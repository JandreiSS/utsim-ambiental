//! A tela Fábrica (plano 3.1): a planta vista de cima com uma mancha de calor em cada
//! máquina, o painel do nó, "Os três vilões", o cartão "De onde veio" e o "Antes e
//! depois" com a diferença pareada.

use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, Sense, Shape, Stroke, pos2, vec2};
use utsim_ambiental_core::conta::{Linha, TipoLinha, comparar};
use utsim_ambiental_core::{Estimativa, Grupo, Indicador, Inventario, NoInventario, mapa, viloes};
use utsim_ui_tema::colors;

use crate::app::Tela;
use crate::projeto::Projeto;
use crate::texto::{casas, com_intervalo, fx, num, tx, vx};

/// O que o mapa pinta.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Medida {
    #[default]
    Energia,
    SemValor,
    Co2,
}

/// O estado da tela (o que está escolhido), fora do projeto.
#[derive(Clone, Debug, Default)]
pub struct Vista {
    pub medida: Medida,
    pub por_peca: bool,
    pub selecionado: Option<String>,
    /// O nó cujo cartão "De onde veio" está aberto.
    pub de_onde_veio: Option<String>,
    /// Pedido de trocar de tela (o cartão "falta fator" leva aos Fatores).
    pub ir_para: Option<Tela>,
}

impl Medida {
    fn unidade(self, por_peca: bool) -> &'static str {
        match (self, por_peca) {
            (Medida::Co2, false) => tx("un-kg_co2e"),
            (Medida::Co2, true) => tx("un-kg_co2e_peca"),
            (_, false) => tx("un-kwh"),
            (_, true) => tx("un-kwh_peca"),
        }
    }
}

/// O valor do nó na medida e na base escolhidas; `None`: sem número (falta fator, nó sem
/// bloco ou sem peça).
fn valor(n: &NoInventario, m: Medida, por_peca: bool) -> Option<f64> {
    if !n.tem_bloco {
        return None;
    }
    let total = match m {
        Medida::Energia => n.energia.media,
        Medida::SemValor => n.sem_valor.media,
        Medida::Co2 => n.co2e.valor()?.media,
    };
    if por_peca {
        (n.pecas.media > 0.0).then(|| total / n.pecas.media)
    } else {
        Some(total)
    }
}

/// A escala sequencial de um matiz só (verde), do apagado ao aceso; legível para quem não
/// distingue cores, porque muda o claro e não o matiz.
pub fn escala(t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let a = [38.0, 66.0, 48.0];
    let b = [196.0, 245.0, 150.0];
    let c = |i: usize| (a[i] + (b[i] - a[i]) * t) as u8;
    Color32::from_rgb(c(0), c(1), c(2))
}

impl Vista {
    /// Desenha a tela; devolve uma mensagem para o rodapé quando algo aconteceu.
    pub fn ui(&mut self, ui: &mut egui::Ui, p: &mut Projeto) -> Option<(String, bool)> {
        let mut msg = None;
        // Barra do alto: o modelo, a origem dos dados e as ações.
        ui.horizontal(|ui| {
            ui.heading(p.nome());
            ui.separator();
            if p.rodando() {
                ui.spinner();
                ui.label(tx("fab-simulando"));
            } else if let Some(inv) = &p.inventario {
                ui.label(fx(
                    "fab-origem",
                    &[
                        ("n", inv.replicacoes.into()),
                        ("semente", vx(&p.modelo.run.seed)),
                    ],
                ));
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(
                        !p.rodando(),
                        utsim_ui_tema::widgets::primary(tx("fab-simular_de_novo")),
                    )
                    .clicked()
                {
                    p.simular();
                }
                if ui
                    .add_enabled(p.mudou(), egui::Button::new(tx("app-salvar")))
                    .on_hover_text(tx("app-salvar_dica"))
                    .clicked()
                {
                    msg = Some(match p.gravar() {
                        Ok(c) => (fx("app-salvo", &[("arquivo", vx(&c.display()))]), false),
                        Err(e) => (fx("app-erro_salvar", &[("e", vx(&e))]), true),
                    });
                }
            });
        });
        ui.horizontal(|ui| {
            let medidas = [Medida::Energia, Medida::SemValor, Medida::Co2];
            let nomes = [tx("fab-energia"), tx("fab-sem_valor"), tx("fab-co2e")];
            let atual = medidas.iter().position(|m| *m == self.medida);
            for (i, r) in utsim_ui_tema::widgets::segmented(ui, &nomes, atual)
                .into_iter()
                .enumerate()
            {
                if r.clicked() {
                    self.medida = medidas[i];
                }
            }
            ui.add_space(16.0);
            let bases = [tx("fab-total"), tx("fab-por_peca")];
            let rs =
                utsim_ui_tema::widgets::segmented(ui, &bases, Some(usize::from(self.por_peca)));
            for (i, r) in rs.into_iter().enumerate() {
                if r.clicked() {
                    self.por_peca = i == 1;
                }
            }
        });
        for pr in p.problemas.iter().filter(|x| x.grave) {
            ui.colored_label(colors::ERROR, problema(pr));
        }
        ui.separator();

        let Some(inv) = p.inventario.clone() else {
            ui.centered_and_justified(|ui| {
                ui.label(tx("fab-esperando_simulacao"));
            });
            return msg;
        };

        egui::Panel::right("painel_fabrica")
            .default_size(370.0)
            .resizable(true)
            .show(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.painel(ui, p, &inv);
                });
            });
        egui::CentralPanel::default().show(ui, |ui| self.mapa(ui, p, &inv));
        self.cartao(ui.ctx(), p, &inv);
        msg
    }

    /// O mapa de calor.
    fn mapa(&mut self, ui: &mut egui::Ui, p: &Projeto, inv: &Inventario) {
        let planta = mapa::planta(&p.modelo);
        let (resp, painter) = ui.allocate_painter(ui.available_size(), Sense::click());
        let area = resp.rect;
        let (x0, y0, x1, y1) = planta.limites();
        let margem = 80.0;
        let w = (x1 - x0).max(1e-9);
        let h = (y1 - y0).max(1e-9);
        let esc = ((area.width() - 2.0 * margem) as f64 / w)
            .min((area.height() - 2.0 * margem - 40.0) as f64 / h)
            .clamp(1.0, 120.0);
        let cx = area.center().x as f64 - (x0 + x1) / 2.0 * esc;
        let cy = area.center().y as f64 - 20.0 - (y0 + y1) / 2.0 * esc;
        let tela = |x: f64, y: f64| pos2((cx + x * esc) as f32, (cy + y * esc) as f32);
        let pts: Vec<Pos2> = planta.pontos.iter().map(|q| tela(q.x, q.y)).collect();

        let valores: Vec<Option<f64>> = planta
            .pontos
            .iter()
            .map(|q| {
                inv.no(&q.nome)
                    .and_then(|n| valor(n, self.medida, self.por_peca))
            })
            .collect();
        let max = valores.iter().flatten().copied().fold(0.0, f64::max);

        for (a, b) in &planta.ligacoes {
            painter.line_segment(
                [pts[*a], pts[*b]],
                Stroke::new(1.0, colors::BORDER.gamma_multiply(1.6)),
            );
        }
        // Manchas primeiro, caixas por cima.
        for (i, q) in planta.pontos.iter().enumerate() {
            if let Some(v) = valores[i]
                && max > 0.0
            {
                let t = (v / max) as f32;
                let r = 20.0 + 46.0 * t.sqrt();
                painter.circle_filled(pts[i], r, escala(t).gamma_multiply(0.55));
                painter.circle_filled(pts[i], r * 0.6, escala(t).gamma_multiply(0.6));
            }
            let _ = q;
        }
        let mut clicado = None;
        for (i, q) in planta.pontos.iter().enumerate() {
            let c = pts[i];
            let no = inv.no(&q.nome);
            let pequeno = q.ponta || (q.espera && no.is_none_or(|n| !n.tem_bloco));
            let lado = if pequeno {
                vec2(16.0, 16.0)
            } else {
                vec2(34.0, 24.0)
            };
            let r = Rect::from_center_size(c, lado);
            let sel = self.selecionado.as_deref() == Some(q.nome.as_str());
            painter.rect_filled(r, 4.0, colors::BUTTON);
            let borda = if sel {
                Stroke::new(2.5, crate::app::COR_APP)
            } else {
                Stroke::new(1.0, colors::BUTTON_BORDER)
            };
            if !pequeno && no.is_some_and(|n| !n.tem_bloco) {
                // Sem bloco ambiental: tracejada, convida a informar a potência.
                let cantos = [
                    r.left_top(),
                    r.right_top(),
                    r.right_bottom(),
                    r.left_bottom(),
                    r.left_top(),
                ];
                painter.extend(Shape::dashed_line(
                    &cantos,
                    Stroke::new(1.2, colors::TEXT_WEAK),
                    4.0,
                    3.0,
                ));
            } else {
                painter.rect_stroke(r, 4.0, borda, egui::StrokeKind::Outside);
            }
            painter.text(
                c + vec2(0.0, lado.y / 2.0 + 4.0),
                Align2::CENTER_TOP,
                &q.nome,
                FontId::proportional(13.0),
                colors::TEXT,
            );
            if let Some(n) = no {
                if self.medida == Medida::Co2 && n.tem_bloco && n.co2e.valor().is_none() {
                    pintar_selo(
                        &painter,
                        c + vec2(-30.0, -lado.y / 2.0 - 20.0),
                        tx("fab-falta_fator"),
                        colors::WARNING,
                        11.0,
                    );
                }
                if let Some(v) = valores[i] {
                    painter.text(
                        c,
                        Align2::CENTER_CENTER,
                        num(v, casas(v)),
                        FontId::proportional(11.0),
                        colors::TEXT_STRONG,
                    );
                }
            }
            if let Some(m) = resp.hover_pos()
                && m.distance(c) < 26.0
            {
                let dica = match (no, valores[i]) {
                    (Some(_), Some(v)) => format!(
                        "{}: {} {}",
                        q.nome,
                        num(v, casas(v)),
                        self.medida.unidade(self.por_peca)
                    ),
                    (Some(n), None) if !n.tem_bloco => {
                        format!("{}: {}", q.nome, tx("fab-informe_potencia"))
                    }
                    _ => q.nome.clone(),
                };
                painter.text(
                    m + vec2(14.0, -14.0),
                    Align2::LEFT_BOTTOM,
                    dica,
                    FontId::proportional(13.0),
                    colors::TEXT_STRONG,
                );
                if resp.clicked() && no.is_some() {
                    clicado = Some(q.nome.clone());
                }
            }
        }
        if resp.clicked() {
            self.selecionado = clicado;
        }
        // Legenda numérica, sempre visível.
        let l0 = pos2(area.left() + 16.0, area.bottom() - 40.0);
        let largura = 220.0;
        let passos = 40;
        for k in 0..passos {
            let t = k as f32 / (passos - 1) as f32;
            let r = Rect::from_min_size(
                l0 + vec2(largura * k as f32 / passos as f32, 0.0),
                vec2(largura / passos as f32 + 0.5, 12.0),
            );
            painter.rect_filled(r, 0.0, escala(t));
        }
        let rotulo = |x: f32, s: String, al: Align2| {
            painter.text(
                l0 + vec2(x, 16.0),
                al,
                s,
                FontId::proportional(12.0),
                colors::TEXT_WEAK,
            )
        };
        rotulo(0.0, "0".into(), Align2::LEFT_TOP);
        rotulo(
            largura,
            format!(
                "{} {}",
                num(max, casas(max)),
                self.medida.unidade(self.por_peca)
            ),
            Align2::RIGHT_TOP,
        );
    }

    /// O painel da direita: a fábrica, o nó escolhido, os vilões, o antes e depois e o que
    /// ainda não é contado.
    fn painel(&mut self, ui: &mut egui::Ui, p: &mut Projeto, inv: &Inventario) {
        ui.heading(tx("fab-a_fabrica"));
        egui::Grid::new("totais")
            .num_columns(2)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                ui.label(tx("fab-energia"));
                ui.label(format!("{} {}", com_intervalo(&inv.energia), tx("un-kwh")));
                ui.end_row();
                ui.label(tx("fab-sem_valor"));
                let pct = if inv.energia.media > 0.0 {
                    100.0 * inv.sem_valor.media / inv.energia.media
                } else {
                    0.0
                };
                ui.label(format!(
                    "{} {} ({} %)",
                    com_intervalo(&inv.sem_valor),
                    tx("un-kwh"),
                    num(pct, 0)
                ));
                ui.end_row();
                ui.label(tx("fab-co2e"));
                self.estimativa(ui, &inv.co2e, tx("un-kg_co2e"));
                ui.end_row();
                if let Some(k) = inv.metrica("ambiental.kwh_por_peca") {
                    ui.label(tx("fab-por_peca_boa"));
                    ui.label(format!("{} {}", com_intervalo(&k), tx("un-kwh")));
                    ui.end_row();
                }
                ui.label(tx("fab-pecas_boas"));
                ui.label(com_intervalo(&inv.pecas_boas));
                ui.end_row();
            });
        for f in &inv.fatores_sem_fonte {
            selo(
                ui,
                &fx("fab-sem_fonte", &[("fluxo", vx(f))]),
                colors::WARNING,
            );
        }
        for f in &inv.fatores_didaticos {
            selo(ui, &fx("fab-didatico", &[("fluxo", vx(f))]), colors::BADGE);
        }
        ui.separator();

        if let Some(nome) = self.selecionado.clone()
            && let Some(n) = inv.no(&nome)
        {
            self.painel_no(ui, p, n);
            ui.separator();
        } else {
            ui.label(egui::RichText::new(tx("fab-clique_numa_maquina")).color(colors::TEXT_WEAK));
            ui.separator();
        }

        ui.heading(tx("fab-viloes"));
        let v = viloes(inv, 3);
        if v.is_empty() {
            ui.label(egui::RichText::new(tx("fab-sem_viloes")).color(colors::TEXT_WEAK));
        }
        for (i, x) in v.iter().enumerate() {
            let chave = match x.grupo {
                Grupo::Esperando => "vilao-esperando",
                Grupo::Parada => "vilao-parada",
                Grupo::ForaDeTurno => "vilao-fora_de_turno",
                Grupo::Preparando => "vilao-preparando",
                Grupo::Produzindo => "vilao-produzindo",
            };
            let t = fx(
                chave,
                &[
                    ("no", vx(&x.no)),
                    ("pct", vx(&num(100.0 * x.parcela_no, 0))),
                    ("pf", vx(&num(100.0 * x.parcela_fabrica, 0))),
                ],
            );
            if ui
                .selectable_label(
                    self.selecionado.as_deref() == Some(x.no.as_str()),
                    format!("{}. {t}", i + 1),
                )
                .clicked()
            {
                self.selecionado = Some(x.no.clone());
            }
        }
        ui.separator();
        self.antes_e_depois(ui, p, inv);

        if !inv.pendencias.is_empty() {
            ui.separator();
            ui.collapsing(tx("fab-ainda_nao_contado"), |ui| {
                for x in &inv.pendencias {
                    let chave = format!("pend-{}", x.chave);
                    ui.label(fx(&chave, &[("no", vx(&x.no))]));
                }
            });
        }
        let avisos: Vec<_> = p.problemas.iter().filter(|x| !x.grave).collect();
        if !avisos.is_empty() {
            ui.separator();
            for a in avisos {
                ui.colored_label(colors::WARNING, problema(a));
            }
        }
    }

    fn estimativa(&mut self, ui: &mut egui::Ui, e: &Estimativa, unidade: &str) {
        match e {
            Estimativa::Valor(i) => {
                ui.label(format!("{} {unidade}", com_intervalo(i)));
            }
            Estimativa::FaltaFator(f) => {
                let r = ui
                    .link(tx("fab-falta_fator"))
                    .on_hover_text(fx("fab-falta_fator_dica", &[("fluxos", vx(&f.join(", ")))]));
                if r.clicked() {
                    self.ir_para = Some(Tela::Fatores);
                }
            }
        }
    }

    /// O painel do nó escolhido: energia por grupo, os números e a potência para editar.
    fn painel_no(&mut self, ui: &mut egui::Ui, p: &mut Projeto, n: &NoInventario) {
        ui.horizontal(|ui| {
            ui.heading(&n.nome);
            if let Some(e) = &n.etapa {
                selo(ui, e, colors::BADGE);
            }
        });
        if !n.tem_bloco {
            ui.colored_label(colors::WARNING, tx("fab-informe_potencia"));
        }
        let e = n.energia.media.max(1e-12);
        for g in Grupo::TODOS {
            let v = n.por_grupo[g.indice()].media;
            ui.horizontal(|ui| {
                ui.add_sized([110.0, 18.0], egui::Label::new(tx_grupo(g)));
                let (r, _) = ui.allocate_exact_size(vec2(120.0, 12.0), Sense::hover());
                ui.painter().rect_filled(r, 2.0, colors::FIELD);
                let f = (v / e) as f32;
                let cor = if g.agrega_valor() {
                    crate::app::COR_APP
                } else if g.sem_valor() {
                    colors::WARNING
                } else {
                    colors::TEXT_WEAK
                };
                ui.painter().rect_filled(
                    Rect::from_min_size(r.min, vec2(r.width() * f.clamp(0.0, 1.0), r.height())),
                    2.0,
                    cor,
                );
                ui.label(format!("{} {}", num(v, casas(v)), tx("un-kwh")));
            });
        }
        egui::Grid::new("numeros_no")
            .num_columns(2)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                ui.label(tx("fab-energia"));
                if ui
                    .link(format!("{} {}", com_intervalo(&n.energia), tx("un-kwh")))
                    .on_hover_text(tx("fab-de_onde_veio_dica"))
                    .clicked()
                {
                    self.de_onde_veio = Some(n.nome.clone());
                }
                ui.end_row();
                ui.label(tx("fab-sem_valor"));
                ui.label(format!("{} {}", com_intervalo(&n.sem_valor), tx("un-kwh")));
                ui.end_row();
                if let Some(k) = &n.kwh_por_peca {
                    ui.label(tx("fab-por_peca"));
                    ui.label(format!("{} {}", com_intervalo(k), tx("un-kwh")));
                    ui.end_row();
                }
                ui.label(tx("fab-co2e"));
                self.estimativa(ui, &n.co2e, tx("un-kg_co2e"));
                ui.end_row();
                for (f, q) in &n.fluxos {
                    let un = p
                        .lateral
                        .environment
                        .fluxo(f)
                        .map(|x| x.unit.clone())
                        .unwrap_or_default();
                    ui.label(f);
                    ui.label(format!("{} {un}", com_intervalo(q)));
                    ui.end_row();
                }
            });
        if ui.button(tx("fab-de_onde_veio")).clicked() {
            self.de_onde_veio = Some(n.nome.clone());
        }
        ui.add_space(6.0);
        ui.label(egui::RichText::new(tx("fab-potencia_kw")).strong());
        if editar_potencia(ui, p, &n.nome) {
            p.recalcular();
        }
    }

    /// "Antes e depois": a base guardada e a diferença pareada, com intervalo.
    fn antes_e_depois(&mut self, ui: &mut egui::Ui, p: &mut Projeto, inv: &Inventario) {
        ui.heading(tx("fab-antes_e_depois"));
        match &p.base {
            None => {
                ui.label(egui::RichText::new(tx("fab-base_explica")).color(colors::TEXT_WEAK));
                if ui.button(tx("fab-guardar_base")).clicked() {
                    p.fixar_base();
                }
            }
            Some(b) => {
                let mut linhas = vec![
                    (
                        tx("fab-energia"),
                        "ambiental.energia_kwh".to_string(),
                        tx("un-kwh"),
                    ),
                    (
                        tx("fab-sem_valor"),
                        "ambiental.energia_sem_valor_kwh".into(),
                        tx("un-kwh"),
                    ),
                    (tx("fab-co2e"), "ambiental.co2e_kg".into(), tx("un-kg_co2e")),
                ];
                if let Some(s) = &self.selecionado {
                    linhas.push((s.as_str(), format!("{s}.energia_kwh"), tx("un-kwh")));
                }
                egui::Grid::new("diferencas")
                    .num_columns(2)
                    .spacing([12.0, 4.0])
                    .show(ui, |ui| {
                        for (nome, chave, un) in &linhas {
                            ui.label(*nome);
                            match comparar(&b.inventario, inv, chave) {
                                Some(d) => {
                                    let cor = if d.media < -1e-9 {
                                        colors::SUCCESS
                                    } else if d.media > 1e-9 {
                                        colors::ERROR
                                    } else {
                                        colors::TEXT_WEAK
                                    };
                                    ui.colored_label(cor, format!("{} {un}", diferenca(&d)));
                                }
                                None => {
                                    ui.label("–");
                                }
                            }
                            ui.end_row();
                        }
                    });
                ui.label(
                    egui::RichText::new(tx("fab-pareado_explica"))
                        .color(colors::TEXT_WEAK)
                        .small(),
                );
                ui.horizontal(|ui| {
                    if ui.button(tx("fab-nova_base")).clicked() {
                        p.fixar_base();
                    }
                    if ui.button(tx("fab-voltar_base")).clicked() {
                        p.voltar_a_base();
                    }
                });
            }
        }
    }

    /// O cartão "De onde veio" do nó: direcionador, coeficiente, quantidade e o fator.
    fn cartao(&mut self, ctx: &egui::Context, p: &Projeto, inv: &Inventario) {
        let Some(nome) = self.de_onde_veio.clone() else {
            return;
        };
        let Some(n) = inv.no(&nome) else {
            self.de_onde_veio = None;
            return;
        };
        let mut aberto = true;
        egui::Window::new(fx("fab-de_onde_veio_titulo", &[("no", vx(&nome))]))
            .open(&mut aberto)
            .collapsible(false)
            .resizable(true)
            .default_width(560.0)
            .show(ctx, |ui| {
                ui.label(egui::RichText::new(tx("fab-formula_energia")).monospace());
                ui.label(
                    egui::RichText::new(fx("fab-media_de", &[("n", inv.replicacoes.into())]))
                        .color(colors::TEXT_WEAK),
                );
                egui::Grid::new("conta")
                    .striped(true)
                    .num_columns(5)
                    .spacing([14.0, 4.0])
                    .show(ui, |ui| {
                        for c in [
                            tx("fab-col_origem"),
                            tx("fab-col_grupo"),
                            tx("fab-col_direcionador"),
                            tx("fab-col_coeficiente"),
                            tx("fab-col_quantidade"),
                        ] {
                            ui.label(egui::RichText::new(c).strong());
                        }
                        ui.end_row();
                        for l in &n.linhas {
                            linha_conta(ui, p, l);
                            ui.end_row();
                        }
                    });
                ui.separator();
                ui.label(fx(
                    "fab-total_energia",
                    &[("kwh", vx(&com_intervalo(&n.energia)))],
                ));
                let eletrico = p
                    .lateral
                    .environment
                    .fluxo_eletrico()
                    .map(|f| f.name.clone());
                match (
                    &n.co2e,
                    eletrico
                        .as_deref()
                        .and_then(|e| p.lateral.environment.fator(e)),
                ) {
                    (Estimativa::Valor(i), Some(f)) => {
                        ui.label(fx(
                            "fab-conta_emissao",
                            &[
                                ("fator", vx(&num(f.kg_co2e.unwrap_or(0.0), 3))),
                                ("fonte", vx(&f.source)),
                                ("escopo", vx(&f.scope)),
                                ("kg", vx(&com_intervalo(i))),
                            ],
                        ));
                    }
                    (Estimativa::FaltaFator(fl), _) => {
                        ui.colored_label(
                            colors::WARNING,
                            fx("fab-falta_fator_dica", &[("fluxos", vx(&fl.join(", ")))]),
                        );
                    }
                    _ => {}
                }
            });
        if !aberto {
            self.de_onde_veio = None;
        }
    }
}

fn linha_conta(ui: &mut egui::Ui, p: &Projeto, l: &Linha) {
    let (origem, dir, coef, qtd) = match l.tipo {
        TipoLinha::Estado => (
            fx("fab-estado", &[("estado", vx(&l.origem))]),
            format!("{} h", num(l.direcionador, casas(l.direcionador))),
            format!("{} kW", num(l.coeficiente, casas(l.coeficiente))),
            format!(
                "{} {}",
                num(l.quantidade, casas(l.quantidade)),
                tx("un-kwh")
            ),
        ),
        TipoLinha::EnergiaPorPeca => (
            tx("fab-pecas_que_entraram").to_string(),
            num(l.direcionador, 0),
            format!(
                "{} {}",
                num(l.coeficiente, casas(l.coeficiente)),
                tx("un-kwh_peca")
            ),
            format!(
                "{} {}",
                num(l.quantidade, casas(l.quantidade)),
                tx("un-kwh")
            ),
        ),
        TipoLinha::Fluxo => {
            let un = p
                .lateral
                .environment
                .fluxo(&l.origem)
                .map(|x| x.unit.clone())
                .unwrap_or_default();
            (
                l.origem.clone(),
                num(l.direcionador, casas(l.direcionador)),
                num(l.coeficiente, casas(l.coeficiente)),
                format!("{} {un}", num(l.quantidade, casas(l.quantidade))),
            )
        }
    };
    ui.label(origem);
    ui.label(l.grupo.map_or("–", tx_grupo));
    ui.label(dir);
    ui.label(coef);
    ui.label(qtd);
}

/// O nome do grupo na língua de agora.
pub fn tx_grupo(g: Grupo) -> &'static str {
    match g {
        Grupo::Produzindo => tx("grupo-produzindo"),
        Grupo::Preparando => tx("grupo-preparando"),
        Grupo::Esperando => tx("grupo-esperando"),
        Grupo::Parada => tx("grupo-parada"),
        Grupo::ForaDeTurno => tx("grupo-fora_de_turno"),
    }
}

/// Uma diferença com sinal e intervalo: "−12,3 ± 0,4".
fn diferenca(d: &Indicador) -> String {
    let s = com_intervalo(d);
    if d.media > 0.0 { format!("+{s}") } else { s }
}

/// Um problema da validação, na língua de agora.
pub fn problema(p: &utsim_ambiental_core::Problema) -> String {
    let args: Vec<(&str, utsim_idiomas::Valor)> = p.args.iter().map(|(k, v)| (*k, vx(v))).collect();
    fx(&format!("val-{}", p.chave), &args)
}

/// Os cinco campos de potência do nó (kW); `true` se algum mudou.
pub fn editar_potencia(ui: &mut egui::Ui, p: &mut Projeto, no: &str) -> bool {
    let mut pot = p
        .lateral
        .objects
        .get(no)
        .map(|d| d.power.clone())
        .unwrap_or_default();
    let mut mudou = false;
    egui::Grid::new(("potencia", no))
        .num_columns(2)
        .spacing([12.0, 4.0])
        .show(ui, |ui| {
            for g in Grupo::TODOS {
                ui.label(tx_grupo(g));
                mudou |= ui
                    .add(
                        egui::DragValue::new(pot.do_grupo_mut(g))
                            .range(0.0..=1.0e6)
                            .speed(0.1)
                            .suffix(" kW"),
                    )
                    .changed();
                ui.end_row();
            }
        });
    if mudou {
        p.bloco_mut(no).power = pot;
    }
    mudou
}

/// Um selo de contorno ("SEM FONTE", "FALTA FATOR") com as fontes do tema comum.
pub fn selo(ui: &mut egui::Ui, texto: &str, cor: Color32) -> egui::Response {
    let galley = ui
        .painter()
        .layout_no_wrap(texto.to_uppercase(), FontId::proportional(11.0), cor);
    let (r, resp) = ui.allocate_exact_size(galley.size() + vec2(12.0, 6.0), Sense::hover());
    if ui.is_rect_visible(r) {
        ui.painter().rect_stroke(
            r.shrink(0.5),
            2.0,
            Stroke::new(1.0, cor.gamma_multiply(0.75)),
            egui::StrokeKind::Inside,
        );
        ui.painter()
            .galley(r.center() - galley.size() / 2.0, galley, cor);
    }
    resp
}

/// [`selo`] pintado no mapa, com a borda esquerda em `esquerda`.
fn pintar_selo(p: &egui::Painter, esquerda: Pos2, texto: &str, cor: Color32, tamanho: f32) {
    let galley = p.layout_no_wrap(texto.to_uppercase(), FontId::proportional(tamanho), cor);
    let r = Rect::from_min_size(
        pos2(esquerda.x, esquerda.y - (galley.size().y + 6.0) / 2.0),
        galley.size() + vec2(12.0, 6.0),
    );
    p.rect_filled(r, 2.0, colors::PANEL);
    p.rect_stroke(
        r.shrink(0.5),
        2.0,
        Stroke::new(1.0, cor.gamma_multiply(0.75)),
        egui::StrokeKind::Inside,
    );
    p.galley(r.center() - galley.size() / 2.0, galley, cor);
}
