//! A tela Fluxos (fase AM6, plano 3.2): o diagrama de Sankey em quatro modos, o caminho de
//! uma faixa, o agrupamento por etapa, o limiar dos fios finos, o selo de balanço dos nós e
//! a exportação em SVG e do inventário em CSV.

use std::path::PathBuf;

use eframe::egui::{self, Align2, Color32, FontId, Mesh, Pos2, Rect, Sense, pos2, vec2};
use utsim_ambiental_core::sankey::{self, Classe, Desenho, Grafo, Lado, Modo, SemDiagrama};
use utsim_ui_tema::colors;

use crate::projeto::Projeto;
use crate::texto::{casas, fx, num, tx, vx};

#[derive(Clone, Debug)]
pub struct Vista {
    pub modo: Modo,
    pub por_etapa: bool,
    /// Esconde as faixas menores que esta fração do total.
    pub limiar: f64,
    /// A faixa escolhida (o caminho dela fica aceso).
    pub escolhida: Option<usize>,
    /// O nó do meio cujo balanço está aberto.
    pub balanco_aberto: Option<String>,
}

impl Default for Vista {
    fn default() -> Self {
        Vista {
            modo: Modo::Massa,
            por_etapa: false,
            limiar: 0.0,
            escolhida: None,
            balanco_aberto: None,
        }
    }
}

fn cor(c: Classe) -> Color32 {
    utsim_ui_tema::familia::cor_hex(sankey::cor(c)).unwrap_or(colors::TEXT)
}

fn tx_modo(m: Modo) -> &'static str {
    match m {
        Modo::Massa => tx("snk-massa"),
        Modo::Energia => tx("snk-energia"),
        Modo::Agua => tx("snk-agua"),
        Modo::Carbono => tx("snk-carbono"),
    }
}

/// O rótulo de um nó na língua de agora.
pub fn rotulo(r: &str) -> String {
    match sankey::rotulo::TODOS.iter().find(|x| **x == r) {
        Some(k) => tx(k).to_string(),
        None => r.to_string(),
    }
}

/// Um ponto da curva de uma faixa: a borda de cima em `t` ∈ [0, 1].
fn bezier(x0: f32, y0: f32, x1: f32, y1: f32, t: f32) -> Pos2 {
    let mx = (x0 + x1) / 2.0;
    let u = 1.0 - t;
    let x = u * u * u * x0 + 3.0 * u * u * t * mx + 3.0 * u * t * t * mx + t * t * t * x1;
    let y = u * u * u * y0 + 3.0 * u * u * t * y0 + 3.0 * u * t * t * y1 + t * t * t * y1;
    pos2(x, y)
}

const AMOSTRAS: usize = 24;

impl Vista {
    pub fn ui(&mut self, ui: &mut egui::Ui, p: &Projeto) -> Option<(String, bool)> {
        let mut msg = None;
        let Some(inv) = &p.inventario else {
            ui.label(tx("fab-esperando_simulacao"));
            return None;
        };
        ui.horizontal(|ui| {
            ui.heading(tx("snk-titulo"));
            ui.add_space(12.0);
            let nomes: Vec<&str> = Modo::TODOS.iter().map(|m| tx_modo(*m)).collect();
            let atual = Modo::TODOS.iter().position(|m| *m == self.modo);
            for (i, r) in utsim_ui_tema::widgets::segmented(ui, &nomes, atual)
                .into_iter()
                .enumerate()
            {
                if r.clicked() {
                    self.modo = Modo::TODOS[i];
                    self.escolhida = None;
                }
            }
            ui.add_space(12.0);
            if ui
                .checkbox(&mut self.por_etapa, tx("snk-por_etapa"))
                .changed()
            {
                self.escolhida = None;
            }
            ui.add_space(12.0);
            ui.label(tx("snk-juntar"));
            let mut pct = self.limiar * 100.0;
            if ui
                .add(
                    egui::Slider::new(&mut pct, 0.0..=20.0)
                        .suffix(" %")
                        .max_decimals(1),
                )
                .changed()
            {
                self.limiar = pct / 100.0;
                self.escolhida = None;
            }
        });
        let grafo = match sankey::grafo(&p.modelo, &p.lateral, inv, self.modo, self.por_etapa) {
            Ok(g) => g.juntar(self.limiar),
            Err(e) => {
                ui.add_space(20.0);
                let t = match e {
                    SemDiagrama::SemMassa => tx("snk-sem_massa").to_string(),
                    SemDiagrama::Vazio => tx("snk-vazio").to_string(),
                    SemDiagrama::FaltaFator(f) => {
                        fx("fab-falta_fator_dica", &[("fluxos", vx(&f.join(", ")))])
                    }
                };
                ui.colored_label(colors::WARNING, t);
                return None;
            }
        };
        let rotulos: Vec<String> = grafo.nos.iter().map(|n| rotulo(&n.rotulo)).collect();
        ui.horizontal(|ui| {
            let total = grafo.total();
            ui.label(fx(
                "snk-total",
                &[
                    ("valor", vx(&num(total, casas(total)))),
                    ("unidade", vx(&self.modo.unidade())),
                ],
            ));
            ui.separator();
            if ui.button(tx("snk-exportar_svg")).clicked() {
                msg = Some(match exportar_svg(p, &grafo, &rotulos, self.modo) {
                    Ok(c) => (fx("rel-gravado", &[("arquivo", vx(&c.display()))]), false),
                    Err(e) => (fx("app-erro_salvar", &[("e", vx(&e))]), true),
                });
            }
            if ui.button(tx("snk-exportar_csv")).clicked() {
                msg = Some(match exportar_csv(p) {
                    Ok(c) => (fx("rel-gravado", &[("arquivo", vx(&c.display()))]), false),
                    Err(e) => (fx("app-erro_salvar", &[("e", vx(&e))]), true),
                });
            }
            ui.label(egui::RichText::new(tx("snk-dica")).color(colors::TEXT_WEAK));
        });
        // O selo de balanço dos nós que não fecham.
        let sem = grafo.sem_balanco(0.02);
        if let Some(chave) = &self.balanco_aberto
            && let Some((i, e, s)) = sem.iter().find(|x| &grafo.nos[x.0].chave == chave)
        {
            ui.colored_label(
                colors::WARNING,
                fx(
                    "snk-balanco_dica",
                    &[
                        ("no", vx(&rotulos[*i])),
                        ("entra", vx(&num(*e, casas(*e)))),
                        ("sai", vx(&num(*s, casas(*s)))),
                    ],
                ),
            );
        }
        let area = ui.available_rect_before_wrap();
        let (resp, pintor) = ui.allocate_painter(area.size(), Sense::click());
        let r = resp.rect;
        let margem_esq = 170.0;
        let margem_dir = 230.0;
        let d = sankey::desenhar(
            &grafo,
            (r.width() - margem_esq - margem_dir).max(100.0) as f64,
            (r.height() - 30.0).max(100.0) as f64,
        );
        let origem = r.min + vec2(margem_esq, 10.0);
        let p2 = |x: f64, y: f64| origem + vec2(x as f32, y as f32);
        let acesas = self.escolhida.map(|k| grafo.caminho(k));
        // As faixas.
        let mut sob_mouse = None;
        for f in &d.faixas {
            let l = &grafo.ligacoes[f.ligacao];
            let a = p2(f.x0, f.y0);
            let b = p2(f.x1, f.y1);
            let esp = (f.espessura as f32).max(0.6);
            let alfa = match &acesas {
                Some(c) if !c.contains(&f.ligacao) => 0.10,
                Some(_) => 0.80,
                None => 0.50,
            };
            let c = cor(l.classe).gamma_multiply(alfa);
            let mut m = Mesh::default();
            for k in 0..=AMOSTRAS {
                let t = k as f32 / AMOSTRAS as f32;
                let topo = bezier(a.x, a.y, b.x, b.y, t);
                m.colored_vertex(topo, c);
                m.colored_vertex(topo + vec2(0.0, esp), c);
                if k > 0 {
                    let i = (2 * k) as u32;
                    m.add_triangle(i - 2, i - 1, i);
                    m.add_triangle(i - 1, i, i + 1);
                }
            }
            pintor.add(m);
            if let Some(mp) = resp.hover_pos()
                && mp.x >= a.x
                && mp.x <= b.x
            {
                // A borda de cima na posição x do mouse, pelas amostras.
                let mut y_topo = a.y;
                for k in 0..AMOSTRAS {
                    let p0 = bezier(a.x, a.y, b.x, b.y, k as f32 / AMOSTRAS as f32);
                    let p1 = bezier(a.x, a.y, b.x, b.y, (k + 1) as f32 / AMOSTRAS as f32);
                    if mp.x >= p0.x && mp.x <= p1.x {
                        let u = if p1.x > p0.x {
                            (mp.x - p0.x) / (p1.x - p0.x)
                        } else {
                            0.0
                        };
                        y_topo = p0.y + (p1.y - p0.y) * u;
                        break;
                    }
                }
                if mp.y >= y_topo && mp.y <= y_topo + esp.max(3.0) {
                    sob_mouse = Some(f.ligacao);
                }
            }
        }
        // As caixas, os rótulos e os selos.
        let somas = grafo.somas();
        let mut no_sob_mouse = None;
        for (i, c) in d.caixas.iter().enumerate() {
            let rc = Rect::from_min_max(p2(c.x0, c.y0), p2(c.x1, c.y1));
            let no = &grafo.nos[i];
            pintor.rect_filled(rc, 1.0, colors::BUTTON_BORDER);
            let valor = somas[i].0.max(somas[i].1);
            let texto = format!(
                "{}  {} {}",
                rotulos[i],
                num(valor, casas(valor)),
                self.modo.unidade()
            );
            let (pos, ancora) = if no.lado == Lado::Entrada {
                (pos2(rc.left() - 6.0, rc.center().y), Align2::RIGHT_CENTER)
            } else {
                (pos2(rc.right() + 6.0, rc.center().y), Align2::LEFT_CENTER)
            };
            pintor.text(pos, ancora, texto, FontId::proportional(12.0), colors::TEXT);
            if let Some((_, e, s)) = sem.iter().find(|x| x.0 == i) {
                let chave = if s < e { "snk-faltam" } else { "snk-sobram" };
                let pct = 100.0 * (e - s).abs() / e.max(*s);
                pintor.text(
                    pos2(rc.center().x, rc.top() - 3.0),
                    Align2::CENTER_BOTTOM,
                    fx(chave, &[("pct", vx(&num(pct, 0)))]),
                    FontId::proportional(11.0),
                    colors::WARNING,
                );
            }
            if let Some(mp) = resp.hover_pos()
                && rc.expand(4.0).contains(mp)
            {
                no_sob_mouse = Some(i);
            }
        }
        if let Some(k) = sob_mouse.filter(|_| no_sob_mouse.is_none())
            && let Some(mp) = resp.hover_pos()
        {
            let l = &grafo.ligacoes[k];
            let total = grafo.total().max(1e-12);
            pintor.text(
                mp + vec2(14.0, -14.0),
                Align2::LEFT_BOTTOM,
                fx(
                    "snk-faixa",
                    &[
                        ("de", vx(&rotulos[l.de])),
                        ("para", vx(&rotulos[l.para])),
                        ("valor", vx(&num(l.valor, casas(l.valor)))),
                        ("unidade", vx(&self.modo.unidade())),
                        ("pct", vx(&num(100.0 * l.valor / total, 1))),
                    ],
                ),
                FontId::proportional(13.0),
                colors::TEXT_STRONG,
            );
        }
        if resp.double_clicked() {
            match no_sob_mouse {
                Some(i) if grafo.nos[i].chave.starts_with("etapa:") => self.por_etapa = false,
                None => {
                    self.escolhida = None;
                    self.por_etapa = false;
                }
                _ => {}
            }
        } else if resp.clicked() {
            if let Some(i) = no_sob_mouse {
                let chave = grafo.nos[i].chave.clone();
                self.balanco_aberto = sem.iter().any(|x| x.0 == i).then_some(chave);
            } else {
                self.escolhida = sob_mouse;
            }
        }
        msg
    }
}

fn base_do_modelo(p: &Projeto) -> String {
    p.caminho
        .file_stem()
        .map_or_else(|| "modelo".into(), |s| s.to_string_lossy().into_owned())
}

/// Grava o diagrama em SVG ao lado do modelo.
fn exportar_svg(p: &Projeto, g: &Grafo, rotulos: &[String], modo: Modo) -> Result<PathBuf, String> {
    let d: Desenho = sankey::desenhar(g, 760.0, 520.0);
    let titulo = fx(
        "snk-titulo_svg",
        &[("modelo", vx(&p.nome())), ("modo", vx(&tx_modo(modo)))],
    );
    let texto = sankey::svg(g, &d, &titulo, rotulos);
    let nome = match modo {
        Modo::Massa => "massa",
        Modo::Energia => "energia",
        Modo::Agua => "agua",
        Modo::Carbono => "carbono",
    };
    let c = p
        .caminho
        .with_file_name(format!("{}.sankey_{nome}.svg", base_do_modelo(p)));
    std::fs::write(&c, texto).map_err(|e| e.to_string())?;
    Ok(c)
}

/// Grava o inventário (todas as métricas com nome) em CSV ao lado do modelo.
pub fn exportar_csv(p: &Projeto) -> Result<PathBuf, String> {
    let inv = p.inventario.as_ref().ok_or("sem inventário")?;
    let mut s = String::from("metrica;media;meia_largura_95;replicacoes\n");
    for (k, v) in &inv.metricas {
        let i = utsim_ambiental_core::Indicador::de(v);
        s.push_str(&format!("{k};{};{};{}\n", i.media, i.meia_largura, i.n));
    }
    let c = p
        .caminho
        .with_file_name(format!("{}.inventario_ambiental.csv", base_do_modelo(p)));
    std::fs::write(&c, s).map_err(|e| e.to_string())?;
    Ok(c)
}
