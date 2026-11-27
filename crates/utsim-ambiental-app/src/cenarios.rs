//! A tela Cenários (fase AM8, plano 3.4): a prateleira de cenários, as alavancas, custo e
//! impacto lado a lado com a diferença pareada para o Base, a dispersão custo × impacto com
//! a fronteira, e o investimento (retorno simples e custo por tonelada evitada).

use eframe::egui::{self, Align2, FontId, Rect, Sense, Stroke, pos2, vec2};
use utsim_ambiental_core::Grupo;
use utsim_ambiental_core::cenarios::{
    self, Alavanca, Cenario, Veredito, custo, fronteira, pareado, retorno, veredito,
};
use utsim_ambiental_core::conta::consome;
use utsim_ambiental_core::formato::FlowKind;
use utsim_ambiental_core::{Indicador, Inventario};
use utsim_ui_tema::colors;

use crate::fabrica::tx_grupo;
use crate::fatores::opcional;
use crate::projeto::Projeto;
use crate::texto::{casas, com_intervalo, fx, num, tx, vx};

/// O que a coluna Impacto mede.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Impacto {
    #[default]
    Co2,
    Energia,
    Agua,
}

impl Impacto {
    fn chave(self) -> &'static str {
        match self {
            Impacto::Co2 => "ambiental.co2e_kg",
            Impacto::Energia => "ambiental.energia_kwh",
            Impacto::Agua => "ambiental.agua_m3",
        }
    }

    fn unidade(self) -> &'static str {
        match self {
            Impacto::Co2 => tx("un-kg_co2e"),
            Impacto::Energia => tx("un-kwh"),
            Impacto::Agua => tx("un-m3"),
        }
    }

    fn nome(self) -> &'static str {
        match self {
            Impacto::Co2 => tx("fab-co2e"),
            Impacto::Energia => tx("fab-energia"),
            Impacto::Agua => tx("fab-agua"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Vista {
    /// O cenário aberto no editor (índice em `estudo.cenarios`).
    pub aberto: Option<usize>,
    pub impacto: Impacto,
}

/// O nome de cada tipo de alavanca.
fn tipos() -> Vec<(&'static str, &'static str)> {
    vec![
        ("potencia", tx("cen-alv_potencia")),
        ("desligamento", tx("cen-alv_desligamento")),
        ("fator", tx("cen-alv_fator")),
        ("destino", tx("cen-alv_destino")),
        ("reuso", tx("cen-alv_reuso")),
        ("remocao", tx("cen-alv_remocao")),
        ("preco", tx("cen-alv_preco")),
        ("contratada", tx("cen-alv_contratada")),
        ("pico", tx("cen-alv_pico")),
        ("modelo", tx("cen-alv_modelo")),
    ]
}

/// Uma alavanca nova, com os valores de agora do modelo.
fn nova(p: &Projeto, tipo: &str) -> Option<Alavanca> {
    let cat = &p.lateral.environment;
    let maquina = p
        .lateral
        .objects
        .iter()
        .find(|(_, d)| !d.power.vazia())
        .map(|(n, _)| n.clone())
        .or_else(|| {
            p.modelo
                .objects
                .iter()
                .find(|o| consome(&o.kind))
                .map(|o| o.name.clone())
        })?;
    let t = cat.tariff.clone();
    Some(match tipo {
        "potencia" => {
            let kw = p
                .lateral
                .objects
                .get(&maquina)
                .map_or(0.0, |d| d.power.waiting);
            Alavanca::Potencia {
                no: maquina,
                grupo: Grupo::Esperando,
                kw,
            }
        }
        "desligamento" => Alavanca::Desligamento {
            no: maquina,
            depois: Some(600.0),
        },
        "fator" => {
            let f = cat.factors.first()?;
            Alavanca::Fator {
                fluxo: f.flow.clone(),
                kg_co2e: f.kg_co2e.unwrap_or(0.0),
            }
        }
        "destino" => {
            let f = cat.flows.iter().find(|f| f.kind == FlowKind::Waste)?;
            let d = cat.waste_destinations.first()?;
            Alavanca::DestinoResiduo {
                fluxo: f.name.clone(),
                destino: d.name.clone(),
            }
        }
        "reuso" => Alavanca::Reuso {
            no: maquina,
            fracao: 0.5,
        },
        "remocao" => {
            let d = cat.effluent.destinations.first()?;
            let par = cat.effluent.parameters.first()?;
            Alavanca::Remocao {
                destino: d.name.clone(),
                parametro: par.name.clone(),
                fracao: d.removal.get(&par.name).copied().unwrap_or(0.0),
            }
        }
        "preco" => Alavanca::PrecoEnergia {
            pico: t.as_ref().and_then(|t| t.price_kwh_peak),
            fora: t.as_ref().and_then(|t| t.price_kwh_off_peak),
        },
        "contratada" => Alavanca::Contratada {
            kw: t.as_ref().and_then(|t| t.contracted_kw),
        },
        "pico" => Alavanca::JanelaPico {
            de: t.as_ref().map_or("18:00".into(), |t| t.peak.from.clone()),
            ate: t.as_ref().map_or("21:00".into(), |t| t.peak.to.clone()),
        },
        _ => Alavanca::Modelo {
            chave: format!("{maquina}.capacity"),
            valor: "2".into(),
        },
    })
}

fn combo(ui: &mut egui::Ui, id: egui::Id, valor: &mut String, opcoes: &[String]) -> bool {
    let mut mudou = false;
    egui::ComboBox::from_id_salt(id)
        .selected_text(valor.clone())
        .show_ui(ui, |ui| {
            for o in opcoes {
                mudou |= ui.selectable_value(valor, o.clone(), o).changed();
            }
        });
    mudou
}

/// O editor de uma alavanca; `true` se mudou.
fn editar(ui: &mut egui::Ui, p: &Projeto, k: usize, a: &mut Alavanca) -> bool {
    let cat = &p.lateral.environment;
    let maquinas: Vec<String> = p
        .modelo
        .objects
        .iter()
        .filter(|o| consome(&o.kind))
        .map(|o| o.name.clone())
        .collect();
    let mut m = false;
    match a {
        Alavanca::Potencia { no, grupo, kw } => {
            m |= combo(ui, egui::Id::new(("no", k)), no, &maquinas);
            egui::ComboBox::from_id_salt(("grupo", k))
                .selected_text(tx_grupo(*grupo))
                .show_ui(ui, |ui| {
                    for g in Grupo::TODOS {
                        m |= ui.selectable_value(grupo, g, tx_grupo(g)).changed();
                    }
                });
            m |= ui
                .add(
                    egui::DragValue::new(kw)
                        .range(0.0..=1.0e6)
                        .speed(0.1)
                        .suffix(" kW"),
                )
                .changed();
        }
        Alavanca::Desligamento { no, depois } => {
            m |= combo(ui, egui::Id::new(("no", k)), no, &maquinas);
            ui.label(tx("maq-depois_de"));
            m |= opcional(ui, depois, 1.0);
        }
        Alavanca::Fator { fluxo, kg_co2e } => {
            let f: Vec<String> = cat.factors.iter().map(|x| x.flow.clone()).collect();
            m |= combo(ui, egui::Id::new(("fluxo", k)), fluxo, &f);
            m |= ui
                .add(
                    egui::DragValue::new(kg_co2e)
                        .range(0.0..=1.0e6)
                        .speed(0.001)
                        .max_decimals(4),
                )
                .changed();
            ui.label(tx("cen-kg_por_unidade"));
        }
        Alavanca::DestinoResiduo { fluxo, destino } => {
            let f: Vec<String> = cat
                .flows
                .iter()
                .filter(|x| x.kind == FlowKind::Waste)
                .map(|x| x.name.clone())
                .collect();
            let d: Vec<String> = cat
                .waste_destinations
                .iter()
                .map(|x| x.name.clone())
                .collect();
            m |= combo(ui, egui::Id::new(("fluxo", k)), fluxo, &f);
            ui.label("→");
            m |= combo(ui, egui::Id::new(("dest", k)), destino, &d);
        }
        Alavanca::Reuso { no, fracao } => {
            m |= combo(ui, egui::Id::new(("no", k)), no, &maquinas);
            m |= ui
                .add(egui::Slider::new(fracao, 0.0..=1.0).max_decimals(2))
                .changed();
        }
        Alavanca::Remocao {
            destino,
            parametro,
            fracao,
        } => {
            let d: Vec<String> = cat
                .effluent
                .destinations
                .iter()
                .map(|x| x.name.clone())
                .collect();
            let pr: Vec<String> = cat
                .effluent
                .parameters
                .iter()
                .map(|x| x.name.clone())
                .collect();
            m |= combo(ui, egui::Id::new(("dest", k)), destino, &d);
            m |= combo(ui, egui::Id::new(("par", k)), parametro, &pr);
            m |= ui
                .add(egui::Slider::new(fracao, 0.0..=1.0).max_decimals(2))
                .changed();
        }
        Alavanca::PrecoEnergia { pico, fora } => {
            ui.label(tx("fat-preco_pico"));
            m |= opcional(ui, pico, 0.01);
            ui.label(tx("fat-preco_fora"));
            m |= opcional(ui, fora, 0.01);
        }
        Alavanca::Contratada { kw } => {
            m |= opcional(ui, kw, 1.0);
        }
        Alavanca::JanelaPico { de, ate } => {
            m |= ui
                .add(egui::TextEdit::singleline(de).desired_width(50.0))
                .changed();
            ui.label("–");
            m |= ui
                .add(egui::TextEdit::singleline(ate).desired_width(50.0))
                .changed();
        }
        Alavanca::Modelo { chave, valor } => {
            m |= ui
                .add(egui::TextEdit::singleline(chave).desired_width(160.0))
                .changed();
            ui.label("=");
            m |= ui
                .add(egui::TextEdit::singleline(valor).desired_width(120.0))
                .changed();
            ui.colored_label(colors::WARNING, tx("cen-precisa_simular"));
        }
    }
    m
}

/// Uma linha da comparação: o cenário, a média e a diferença para o Base.
struct Linha {
    nome: String,
    indice: Option<usize>,
    custo: Option<f64>,
    d_custo: Option<Indicador>,
    impacto: Option<f64>,
    d_impacto: Option<Indicador>,
}

fn tx_veredito(v: Veredito) -> (&'static str, egui::Color32) {
    match v {
        Veredito::Melhor => (tx("cen-melhor"), colors::SUCCESS),
        Veredito::Pior => (tx("cen-pior"), colors::ERROR),
        Veredito::NaoDaParaAfirmar => (tx("cen-nao_da"), colors::TEXT_WEAK),
    }
}

impl Vista {
    pub fn ui(&mut self, ui: &mut egui::Ui, p: &mut Projeto) -> Option<(String, bool)> {
        let mut msg = None;
        let Some(base) = p.inventario.clone() else {
            ui.label(tx("fab-esperando_simulacao"));
            return None;
        };
        if let Some(r) = p.receber_cenario() {
            msg = Some(match r {
                Ok(()) => (tx("cen-simulado").into(), false),
                Err(e) => (fx("app-erro_simular", &[("e", vx(&e))]), true),
            });
        }
        if p.cenario_rodando().is_some() {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(100));
        }
        egui::Panel::left("prateleira")
            .default_size(270.0)
            .resizable(true)
            .show(ui, |ui| {
                if let Some(m) = self.prateleira(ui, p) {
                    msg = Some(m);
                }
            });
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(i) = self.aberto
                    && i < p.estudo.cenarios.len()
                {
                    self.editor(ui, p, i);
                    ui.separator();
                }
                self.comparacao(ui, p, &base);
            });
        });
        msg
    }

    fn prateleira(&mut self, ui: &mut egui::Ui, p: &mut Projeto) -> Option<(String, bool)> {
        let mut msg = None;
        ui.heading(tx("cen-titulo"));
        ui.label(egui::RichText::new(tx("cen-explica")).color(colors::TEXT_WEAK));
        ui.add_space(6.0);
        if ui
            .selectable_label(self.aberto.is_none(), tx("cen-base"))
            .clicked()
        {
            self.aberto = None;
        }
        for (i, c) in p.estudo.cenarios.iter().enumerate() {
            let mut t = c.nome.clone();
            if c.precisa_simular() {
                t.push_str(if p.cenario_simulado(c) {
                    "  ✓"
                } else {
                    "  ⟳"
                });
            }
            if ui.selectable_label(self.aberto == Some(i), t).clicked() {
                self.aberto = Some(i);
            }
        }
        if ui.button(tx("cen-novo")).clicked() {
            let n = p.estudo.cenarios.len() + 1;
            p.estudo.cenarios.push(Cenario {
                nome: fx("cen-nome_padrao", &[("n", n.into())]),
                ..Cenario::default()
            });
            self.aberto = Some(p.estudo.cenarios.len() - 1);
        }
        ui.separator();
        ui.label(egui::RichText::new(tx("cen-parametros")).strong());
        ui.label(tx("cen-horas_por_ano"));
        opcional(ui, &mut p.estudo.horas_por_ano, 10.0);
        ui.label(tx("cen-preco_carbono"));
        opcional(ui, &mut p.estudo.preco_carbono, 0.001);
        ui.add_space(6.0);
        if ui
            .add(utsim_ui_tema::widgets::primary(tx("cen-salvar")))
            .clicked()
        {
            msg = Some(match cenarios::gravar(&p.caminho, &p.estudo) {
                Ok(c) => (fx("app-salvo", &[("arquivo", vx(&c.display()))]), false),
                Err(e) => (fx("app-erro_salvar", &[("e", vx(&e))]), true),
            });
        }
        msg
    }

    fn editor(&mut self, ui: &mut egui::Ui, p: &mut Projeto, i: usize) {
        let mut c = p.estudo.cenarios[i].clone();
        let mut apagar = false;
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut c.nome).desired_width(220.0));
            if ui.button(tx("cen-apagar")).clicked() {
                apagar = true;
            }
        });
        ui.label(egui::RichText::new(tx("cen-alavancas")).strong());
        let mut tirar = None;
        for (k, a) in c.alavancas.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                editar(ui, p, k, a);
                if ui.small_button(tx("fat-tirar")).clicked() {
                    tirar = Some(k);
                }
            });
        }
        if let Some(k) = tirar {
            c.alavancas.remove(k);
        }
        ui.horizontal(|ui| {
            egui::ComboBox::from_id_salt("nova_alavanca")
                .selected_text(tx("cen-nova_alavanca"))
                .show_ui(ui, |ui| {
                    for (t, nome) in tipos() {
                        if ui.selectable_label(false, nome).clicked()
                            && let Some(a) = nova(p, t)
                        {
                            c.alavancas.push(a);
                        }
                    }
                });
            if c.precisa_simular() {
                let rodando = p.cenario_rodando().is_some();
                if ui
                    .add_enabled(!rodando, utsim_ui_tema::widgets::primary(tx("cen-simular")))
                    .clicked()
                {
                    p.simular_cenario(&c);
                }
                if rodando {
                    ui.spinner();
                }
            }
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(tx("cen-investimento"));
            opcional(ui, &mut c.investimento, 100.0);
            ui.label(tx("cen-vida"));
            opcional(ui, &mut c.vida_anos, 1.0);
        });
        if apagar {
            p.estudo.cenarios.remove(i);
            self.aberto = None;
        } else if c != p.estudo.cenarios[i] {
            p.estudo.cenarios[i] = c;
        }
    }

    fn comparacao(&mut self, ui: &mut egui::Ui, p: &mut Projeto, base: &Inventario) {
        ui.horizontal(|ui| {
            ui.heading(tx("cen-comparacao"));
            ui.add_space(12.0);
            ui.label(tx("cen-impacto"));
            let is = [Impacto::Co2, Impacto::Energia, Impacto::Agua];
            let nomes: Vec<&str> = is.iter().map(|x| x.nome()).collect();
            let atual = is.iter().position(|x| *x == self.impacto);
            for (k, r) in utsim_ui_tema::widgets::segmented(ui, &nomes, atual)
                .into_iter()
                .enumerate()
            {
                if r.clicked() {
                    self.impacto = is[k];
                }
            }
        });
        ui.label(
            egui::RichText::new(tx("fab-pareado_explica"))
                .color(colors::TEXT_WEAK)
                .small(),
        );
        let pc = p.estudo.preco_carbono;
        let chave = self.impacto.chave();
        let c_base = custo(base, pc);
        let i_base = base.metricas.get(chave).cloned();
        let media = |v: &Option<Vec<f64>>| v.as_ref().map(|x| Indicador::de(x).media);
        let mut linhas = vec![Linha {
            nome: tx("cen-base").into(),
            indice: None,
            custo: media(&c_base),
            d_custo: None,
            impacto: media(&i_base),
            d_impacto: None,
        }];
        let cenarios = p.estudo.cenarios.clone();
        let mut erros = Vec::new();
        for (k, c) in cenarios.iter().enumerate() {
            match p.avaliar_cenario(c) {
                Ok(Some(inv)) => {
                    let cc = custo(&inv, pc);
                    let ii = inv.metricas.get(chave).cloned();
                    linhas.push(Linha {
                        nome: c.nome.clone(),
                        indice: Some(k),
                        custo: media(&cc),
                        d_custo: c_base
                            .as_ref()
                            .zip(cc.as_ref())
                            .and_then(|(a, b)| pareado(a, b)),
                        impacto: media(&ii),
                        d_impacto: i_base
                            .as_ref()
                            .zip(ii.as_ref())
                            .and_then(|(a, b)| pareado(a, b)),
                    });
                }
                Ok(None) => linhas.push(Linha {
                    nome: format!("{} ({})", c.nome, tx("cen-precisa_simular")),
                    indice: Some(k),
                    custo: None,
                    d_custo: None,
                    impacto: None,
                    d_impacto: None,
                }),
                Err(e) => erros.push(format!("{}: {e}", c.nome)),
            }
        }
        for e in erros {
            ui.colored_label(colors::ERROR, e);
        }
        if c_base.is_none() {
            ui.colored_label(colors::WARNING, tx("cen-sem_custo"));
        }
        if i_base.is_none() {
            ui.colored_label(colors::WARNING, tx("cen-sem_impacto"));
        }
        let max_c = linhas.iter().filter_map(|l| l.custo).fold(0.0, f64::max);
        let max_i = linhas.iter().filter_map(|l| l.impacto).fold(0.0, f64::max);
        let barra = |ui: &mut egui::Ui, v: Option<f64>, max: f64| {
            let (r, _) = ui.allocate_exact_size(vec2(120.0, 12.0), Sense::hover());
            ui.painter().rect_filled(r, 2.0, colors::FIELD);
            if let Some(v) = v
                && max > 0.0
            {
                ui.painter().rect_filled(
                    Rect::from_min_size(r.min, vec2(r.width() * (v / max) as f32, r.height())),
                    2.0,
                    crate::app::COR_APP,
                );
            }
        };
        let dif = |ui: &mut egui::Ui, d: &Option<Indicador>| match d {
            Some(d) => {
                let (t, cor) = tx_veredito(veredito(d));
                let s = com_intervalo(d);
                let s = if d.media > 0.0 { format!("+{s}") } else { s };
                ui.colored_label(cor, format!("{s}  {t}"));
            }
            None => {
                ui.label("–");
            }
        };
        egui::Grid::new("comparacao")
            .striped(true)
            .num_columns(7)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                for c in [
                    tx("cen-cenario"),
                    tx("cen-custo"),
                    "",
                    tx("cen-diferenca"),
                    self.impacto.nome(),
                    "",
                    tx("cen-diferenca"),
                ] {
                    ui.label(egui::RichText::new(c).strong());
                }
                ui.end_row();
                for l in &linhas {
                    if ui
                        .selectable_label(self.aberto == l.indice && l.indice.is_some(), &l.nome)
                        .clicked()
                    {
                        self.aberto = l.indice;
                    }
                    ui.label(l.custo.map_or("–".into(), |v| num(v, casas(v))));
                    barra(ui, l.custo, max_c);
                    dif(ui, &l.d_custo);
                    ui.label(l.impacto.map_or("–".into(), |v| {
                        format!("{} {}", num(v, casas(v)), self.impacto.unidade())
                    }));
                    barra(ui, l.impacto, max_i);
                    dif(ui, &l.d_impacto);
                    ui.end_row();
                }
            });
        ui.add_space(8.0);
        self.dispersao(ui, &linhas);
        ui.add_space(8.0);
        self.investimentos(ui, p, &linhas, base);
    }

    /// Custo × impacto, um ponto por cenário, com a fronteira dos que não perdem nos dois.
    fn dispersao(&mut self, ui: &mut egui::Ui, linhas: &[Linha]) {
        let pts: Vec<(usize, (f64, f64))> = linhas
            .iter()
            .enumerate()
            .filter_map(|(k, l)| Some((k, (l.custo?, l.impacto?))))
            .collect();
        if pts.len() < 2 {
            return;
        }
        ui.label(egui::RichText::new(tx("cen-dispersao")).strong());
        let (resp, pintor) = ui.allocate_painter(vec2(520.0, 260.0), Sense::click());
        let r = Rect::from_min_max(
            resp.rect.min + vec2(50.0, 10.0),
            resp.rect.max - vec2(10.0, 30.0),
        );
        pintor.rect_filled(r, 2.0, colors::FIELD);
        let (mut x0, mut x1, mut y0, mut y1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for (_, (x, y)) in &pts {
            x0 = x0.min(*x);
            x1 = x1.max(*x);
            y0 = y0.min(*y);
            y1 = y1.max(*y);
        }
        let mx = ((x1 - x0) * 0.1).max(1e-9);
        let my = ((y1 - y0) * 0.1).max(1e-9);
        let (x0, x1, y0, y1) = (x0 - mx, x1 + mx, y0 - my, y1 + my);
        let tela = |x: f64, y: f64| {
            pos2(
                r.left() + ((x - x0) / (x1 - x0)) as f32 * r.width(),
                r.bottom() - ((y - y0) / (y1 - y0)) as f32 * r.height(),
            )
        };
        let so_pontos: Vec<(f64, f64)> = pts.iter().map(|x| x.1).collect();
        let mut fr: Vec<usize> = fronteira(&so_pontos);
        fr.sort_by(|a, b| so_pontos[*a].0.total_cmp(&so_pontos[*b].0));
        for w in fr.windows(2) {
            let (a, b) = (so_pontos[w[0]], so_pontos[w[1]]);
            pintor.line_segment(
                [tela(a.0, a.1), tela(b.0, b.1)],
                Stroke::new(1.5, crate::app::COR_APP),
            );
        }
        for (k, (i, (x, y))) in pts.iter().enumerate() {
            let c = tela(*x, *y);
            let na = fr.contains(&k);
            pintor.circle_filled(
                c,
                if na { 6.0 } else { 4.5 },
                if na {
                    crate::app::COR_APP
                } else {
                    colors::TEXT_WEAK
                },
            );
            let (desvio, ancora) = if c.x > r.center().x {
                (vec2(-8.0, -4.0), Align2::RIGHT_BOTTOM)
            } else {
                (vec2(8.0, -4.0), Align2::LEFT_BOTTOM)
            };
            pintor.text(
                c + desvio,
                ancora,
                &linhas[*i].nome,
                FontId::proportional(12.0),
                colors::TEXT,
            );
            if resp.clicked()
                && let Some(m) = resp.interact_pointer_pos()
                && m.distance(c) < 10.0
            {
                self.aberto = linhas[*i].indice;
            }
        }
        pintor.text(
            r.center_bottom() + vec2(0.0, 6.0),
            Align2::CENTER_TOP,
            tx("cen-custo"),
            FontId::proportional(12.0),
            colors::TEXT_WEAK,
        );
        pintor.text(
            r.left_center() - vec2(6.0, 0.0),
            Align2::RIGHT_CENTER,
            self.impacto.unidade(),
            FontId::proportional(12.0),
            colors::TEXT_WEAK,
        );
        ui.label(
            egui::RichText::new(tx("cen-fronteira"))
                .color(colors::TEXT_WEAK)
                .small(),
        );
    }

    /// Retorno simples e custo por tonelada evitada dos cenários com investimento.
    fn investimentos(&self, ui: &mut egui::Ui, p: &Projeto, linhas: &[Linha], base: &Inventario) {
        let com: Vec<(&Linha, &Cenario)> = linhas
            .iter()
            .filter_map(|l| Some((l, p.estudo.cenarios.get(l.indice?)?)))
            .filter(|(_, c)| c.investimento.is_some())
            .collect();
        if com.is_empty() {
            return;
        }
        ui.label(egui::RichText::new(tx("cen-investimentos")).strong());
        let horas_janela =
            (base.janela.1 - base.janela.0) * p.lateral.environment.clock.time_unit.em_horas();
        let Some(h) = p.estudo.horas_por_ano.filter(|_| horas_janela > 0.0) else {
            ui.colored_label(colors::WARNING, tx("cen-sem_horas"));
            return;
        };
        let fator = h / horas_janela;
        egui::Grid::new("investimentos")
            .num_columns(3)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                for c in [tx("cen-cenario"), tx("cen-retorno"), tx("cen-por_tonelada")] {
                    ui.label(egui::RichText::new(c).strong());
                }
                ui.end_row();
                for (l, c) in com {
                    let economia = l.d_custo.map_or(0.0, |d| -d.media);
                    let evitado = if self.impacto == Impacto::Co2 {
                        l.d_impacto.map_or(0.0, |d| -d.media)
                    } else {
                        0.0
                    };
                    let (anos, por_t) = retorno(
                        c.investimento.unwrap_or(0.0),
                        c.vida_anos,
                        economia,
                        evitado,
                        fator,
                    );
                    ui.label(&l.nome);
                    ui.label(anos.map_or(tx("cen-nao_paga").to_string(), |a| {
                        fx("cen-anos", &[("anos", vx(&num(a, 1)))])
                    }));
                    ui.label(por_t.map_or("–".into(), |x| num(x, casas(x))));
                    ui.end_row();
                }
            });
    }
}
