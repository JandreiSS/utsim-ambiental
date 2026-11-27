//! A tela Metas (fase AM9, plano 3.5): um cartão por meta com o medidor, o semáforo pela
//! probabilidade de cumprir a meta nas replicações, o orçamento de carbono, a cascata dos
//! cenários ligados, as leituras reais ao lado do simulado e os selos de conquista.

use eframe::egui::{self, Align2, Color32, FontId, Rect, Sense, Stroke, pos2, vec2};
use utsim_ambiental_core::formato::{Basis, Target, TargetScope};
use utsim_ambiental_core::metas::{self, Leitura, Semaforo, Situacao};
use utsim_ambiental_core::{Indicador, Inventario};
use utsim_ui_tema::colors;

use crate::fatores::opcional;
use crate::projeto::Projeto;
use crate::texto::{casas, com_intervalo, fx, num, tx, vx};

#[derive(Clone, Debug, Default)]
pub struct Vista {
    /// A meta aberta no detalhe (pelo id).
    pub aberta: Option<String>,
}

pub fn tx_indicador(chave: &str) -> String {
    match chave {
        "co2e_kg" => tx("met-ind_co2e_kg").into(),
        "energia_kwh" => tx("met-ind_energia_kwh").into(),
        "energia_sem_valor_kwh" => tx("met-ind_energia_sem_valor_kwh").into(),
        "agua_m3" => tx("met-ind_agua_m3").into(),
        "efluente_m3" => tx("met-ind_efluente_m3").into(),
        "custo" => tx("met-ind_custo").into(),
        "custo_energia" => tx("met-ind_custo_energia").into(),
        "demanda_max_kw" => tx("met-ind_demanda_max_kw").into(),
        "desvio_aterro_pct" => tx("met-ind_desvio_aterro_pct").into(),
        "rendimento_material_pct" => tx("met-ind_rendimento_material_pct").into(),
        "reuso_pct" => tx("met-ind_reuso_pct").into(),
        outro => outro.to_string(),
    }
}

fn tx_base(b: Basis) -> &'static str {
    match b {
        Basis::Total => tx("met-base_total"),
        Basis::PerGoodItem => tx("met-base_peca"),
        Basis::PerHour => tx("met-base_hora"),
    }
}

fn tx_alcance(s: &TargetScope) -> String {
    match s {
        TargetScope::Factory => tx("met-fabrica").into(),
        TargetScope::Stage(e) => fx("met-etapa", &[("etapa", vx(e))]),
        TargetScope::Node(n) => fx("met-maquina", &[("no", vx(n))]),
    }
}

fn cor(s: Semaforo) -> Color32 {
    match s {
        Semaforo::Verde => colors::SUCCESS,
        Semaforo::Amarelo => colors::WARNING,
        Semaforo::Vermelho => colors::ERROR,
    }
}

impl Vista {
    pub fn ui(&mut self, ui: &mut egui::Ui, p: &mut Projeto) -> Option<(String, bool)> {
        let mut msg = None;
        let Some(inv) = p.inventario.clone() else {
            ui.label(tx("fab-esperando_simulacao"));
            return None;
        };
        ui.horizontal(|ui| {
            ui.heading(tx("met-titulo"));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add_enabled(p.mudou(), utsim_ui_tema::widgets::primary(tx("app-salvar")))
                    .clicked()
                {
                    msg = Some(match p.gravar() {
                        Ok(c) => (fx("app-salvo", &[("arquivo", vx(&c.display()))]), false),
                        Err(e) => (fx("app-erro_salvar", &[("e", vx(&e))]), true),
                    });
                }
            });
        });
        ui.label(egui::RichText::new(tx("met-explica")).color(colors::TEXT_WEAK));
        egui::ScrollArea::vertical().show(ui, |ui| {
            self.orcamento(ui, p, &inv);
            self.selos(ui, p, &inv);
            ui.add_space(6.0);
            if ui.button(tx("met-nova")).clicked() {
                let n = p.lateral.environment.targets.len() + 1;
                let id = format!("m{n}");
                let mut t = Target {
                    id: id.clone(),
                    indicator: "energia_kwh".into(),
                    scope: TargetScope::Factory,
                    basis: Basis::Total,
                    baseline: None,
                    target: None,
                    due: None,
                    actions: Vec::new(),
                };
                if let Some(s) = metas::situacao(&inv, &p.lateral, &t) {
                    t.baseline = Some(s.valor.media);
                    t.target = Some(s.valor.media * 0.9);
                }
                p.lateral.environment.targets.push(t);
                self.aberta = Some(id);
                p.recalcular();
            }
            ui.add_space(6.0);
            let ts = p.lateral.environment.targets.clone();
            let largura = 360.0;
            let por_linha = ((ui.available_width() / (largura + 12.0)).floor() as usize).max(1);
            for grupo in ts.chunks(por_linha) {
                ui.horizontal_top(|ui| {
                    for t in grupo {
                        ui.vertical(|ui| {
                            ui.set_width(largura);
                            self.cartao(ui, p, &inv, t);
                        });
                        ui.add_space(12.0);
                    }
                });
                ui.add_space(8.0);
            }
            if let Some(id) = self.aberta.clone()
                && let Some(k) = p
                    .lateral
                    .environment
                    .targets
                    .iter()
                    .position(|t| t.id == id)
            {
                ui.separator();
                if let Some(m) = self.detalhe(ui, p, &inv, k) {
                    msg = Some(m);
                }
            }
        });
        msg
    }

    fn orcamento(&self, ui: &mut egui::Ui, p: &mut Projeto, inv: &Inventario) {
        ui.horizontal(|ui| {
            ui.label(tx("met-orcamento"));
            if opcional(ui, &mut p.lateral.environment.carbon_budget_kg, 10.0) {
                p.recalcular();
            }
            ui.label(tx("un-kg_co2e"));
        });
        if let Some((usado, frac)) = metas::orcamento(inv, &p.lateral) {
            let (r, _) =
                ui.allocate_exact_size(vec2(ui.available_width().min(600.0), 18.0), Sense::hover());
            ui.painter().rect_filled(r, 3.0, colors::FIELD);
            let cor = if frac > 1.0 {
                colors::ERROR
            } else {
                crate::app::COR_APP
            };
            ui.painter().rect_filled(
                Rect::from_min_size(r.min, vec2(r.width() * frac.min(1.0) as f32, r.height())),
                3.0,
                cor,
            );
            ui.label(fx(
                "met-orcamento_usado",
                &[
                    ("usado", vx(&num(usado, casas(usado)))),
                    ("pct", vx(&num(100.0 * frac, 0))),
                ],
            ));
        }
    }

    fn selos(&self, ui: &mut egui::Ui, p: &mut Projeto, inv: &Inventario) {
        ui.horizontal(|ui| {
            let mut mostrar = !p.lateral.environment.hide_badges;
            if ui.checkbox(&mut mostrar, tx("met-selos")).changed() {
                p.lateral.environment.hide_badges = !mostrar;
                p.recalcular();
            }
            if mostrar {
                let s = metas::selos(inv, &p.lateral);
                if s.is_empty() {
                    ui.label(egui::RichText::new(tx("met-sem_selos")).color(colors::TEXT_WEAK));
                }
                for x in s {
                    crate::fabrica::selo(ui, tx_selo(x), colors::SUCCESS);
                }
            }
        });
    }

    fn cartao(&mut self, ui: &mut egui::Ui, p: &Projeto, inv: &Inventario, t: &Target) {
        let quadro = egui::Frame::group(ui.style()).inner_margin(10.0);
        quadro.show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(tx_indicador(&t.indicator)).strong());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button(tx("met-abrir")).clicked() {
                        self.aberta = Some(t.id.clone());
                    }
                });
            });
            ui.label(
                egui::RichText::new(format!("{} · {}", tx_alcance(&t.scope), tx_base(t.basis)))
                    .color(colors::TEXT_WEAK),
            );
            match metas::situacao(inv, &p.lateral, t) {
                Some(s) => medidor(ui, t, &s),
                None => {
                    ui.colored_label(colors::WARNING, tx("met-sem_valor"));
                }
            }
            if let Some(d) = &t.due {
                ui.label(fx("met-prazo", &[("prazo", vx(d))]));
            }
            let reais: Vec<&Leitura> = p.leituras.iter().filter(|l| l.meta == t.id).collect();
            if let Some(u) = reais.last() {
                ui.label(fx(
                    "met-real",
                    &[
                        ("valor", vx(&num(u.valor, casas(u.valor)))),
                        ("periodo", vx(&u.periodo)),
                        ("origem", vx(&u.origem)),
                    ],
                ));
            }
        });
    }

    /// O detalhe da meta: os campos, os cenários ligados, a cascata e as leituras reais.
    fn detalhe(
        &mut self,
        ui: &mut egui::Ui,
        p: &mut Projeto,
        inv: &Inventario,
        k: usize,
    ) -> Option<(String, bool)> {
        let mut msg = None;
        let mut t = p.lateral.environment.targets[k].clone();
        let mut apagar = false;
        ui.horizontal(|ui| {
            ui.heading(tx("met-detalhe"));
            if ui.button(tx("met-apagar")).clicked() {
                apagar = true;
            }
            if ui.button(tx("met-fechar")).clicked() {
                self.aberta = None;
            }
        });
        let nos: Vec<String> = p
            .lateral
            .objects
            .iter()
            .filter(|(_, d)| !d.power.vazia() || !d.flows.is_empty())
            .map(|(n, _)| n.clone())
            .collect();
        let mut etapas: Vec<String> = p
            .lateral
            .objects
            .values()
            .filter_map(|d| d.stage.clone())
            .collect();
        etapas.sort();
        etapas.dedup();
        egui::Grid::new("meta_campos")
            .num_columns(2)
            .spacing([12.0, 6.0])
            .show(ui, |ui| {
                ui.label(tx("met-indicador"));
                egui::ComboBox::from_id_salt("met_ind")
                    .selected_text(tx_indicador(&t.indicator))
                    .show_ui(ui, |ui| {
                        for i in metas::INDICADORES {
                            ui.selectable_value(
                                &mut t.indicator,
                                i.chave.to_string(),
                                tx_indicador(i.chave),
                            );
                        }
                    });
                ui.end_row();
                ui.label(tx("met-alcance"));
                egui::ComboBox::from_id_salt("met_alc")
                    .selected_text(tx_alcance(&t.scope))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut t.scope, TargetScope::Factory, tx("met-fabrica"));
                        for e in &etapas {
                            let s = TargetScope::Stage(e.clone());
                            let r = tx_alcance(&s);
                            ui.selectable_value(&mut t.scope, s, r);
                        }
                        for n in &nos {
                            let s = TargetScope::Node(n.clone());
                            let r = tx_alcance(&s);
                            ui.selectable_value(&mut t.scope, s, r);
                        }
                    });
                ui.end_row();
                ui.label(tx("met-base"));
                egui::ComboBox::from_id_salt("met_base")
                    .selected_text(tx_base(t.basis))
                    .show_ui(ui, |ui| {
                        for b in [Basis::Total, Basis::PerGoodItem, Basis::PerHour] {
                            ui.selectable_value(&mut t.basis, b, tx_base(b));
                        }
                    });
                ui.end_row();
                ui.label(tx("met-linha_de_base"));
                ui.horizontal(|ui| {
                    opcional(ui, &mut t.baseline, 0.1);
                    if ui.small_button(tx("met-usar_atual")).clicked()
                        && let Some(s) = metas::situacao(inv, &p.lateral, &t)
                    {
                        t.baseline = Some(s.valor.media);
                    }
                });
                ui.end_row();
                ui.label(tx("met-alvo"));
                opcional(ui, &mut t.target, 0.1);
                ui.end_row();
                ui.label(tx("met-prazo_campo"));
                let mut prazo = t.due.clone().unwrap_or_default();
                if ui
                    .add(egui::TextEdit::singleline(&mut prazo).desired_width(120.0))
                    .changed()
                {
                    t.due = (!prazo.trim().is_empty()).then(|| prazo.trim().to_string());
                }
                ui.end_row();
            });

        // Os cenários: quais aproximam a meta, e os ligados como ações.
        ui.add_space(6.0);
        ui.label(egui::RichText::new(tx("met-cenarios")).strong());
        let base = metas::valores(inv, &p.lateral, &t).map(|v| Indicador::de(&v).media);
        let mut efeitos: Vec<(String, f64)> = Vec::new();
        let cenarios = p.estudo.cenarios.clone();
        if cenarios.is_empty() {
            ui.label(egui::RichText::new(tx("met-sem_cenarios")).color(colors::TEXT_WEAK));
        }
        for c in &cenarios {
            let r = p.avaliar_cenario(c).ok().flatten();
            let v = r
                .as_ref()
                .and_then(|i| metas::valores(i, &p.lateral, &t))
                .map(|v| Indicador::de(&v).media);
            let sit = r.as_ref().and_then(|i| metas::situacao(i, &p.lateral, &t));
            ui.horizontal(|ui| {
                let mut ligado = t.actions.contains(&c.nome);
                if ui.checkbox(&mut ligado, &c.nome).changed() {
                    if ligado {
                        t.actions.push(c.nome.clone());
                    } else {
                        t.actions.retain(|x| x != &c.nome);
                    }
                }
                match (v, base) {
                    (Some(v), Some(b)) => {
                        ui.label(fx(
                            "met-cenario_efeito",
                            &[
                                ("valor", vx(&num(v, casas(v)))),
                                ("efeito", vx(&num(v - b, casas(v - b)))),
                            ],
                        ));
                        if let Some(s) = sit {
                            ui.colored_label(
                                cor(s.semaforo),
                                format!("{} %", num(100.0 * s.probabilidade, 0)),
                            );
                        }
                        if t.actions.contains(&c.nome) {
                            efeitos.push((c.nome.clone(), v - b));
                        }
                    }
                    _ => {
                        ui.label(
                            egui::RichText::new(tx("cen-precisa_simular")).color(colors::TEXT_WEAK),
                        );
                    }
                }
            });
        }
        if let (Some(b), false) = (t.baseline.or(base), efeitos.is_empty()) {
            cascata(ui, b, &metas::cascata(b, &efeitos), t.target);
        }

        // Leituras reais.
        ui.add_space(6.0);
        ui.label(egui::RichText::new(tx("met-leituras")).strong());
        ui.label(egui::RichText::new(tx("met-leituras_explica")).color(colors::TEXT_WEAK));
        let mut leituras = p.leituras.clone();
        let mut mudou = false;
        let mut tirar = None;
        for (i, l) in leituras
            .iter_mut()
            .enumerate()
            .filter(|(_, l)| l.meta == t.id)
        {
            ui.horizontal(|ui| {
                mudou |= ui
                    .add(egui::TextEdit::singleline(&mut l.periodo).desired_width(90.0))
                    .changed();
                mudou |= ui
                    .add(egui::DragValue::new(&mut l.valor).speed(0.1))
                    .changed();
                mudou |= ui
                    .add(egui::TextEdit::singleline(&mut l.origem).desired_width(140.0))
                    .changed();
                if ui.small_button(tx("fat-tirar")).clicked() {
                    tirar = Some(i);
                }
            });
        }
        if let Some(i) = tirar {
            leituras.remove(i);
            mudou = true;
        }
        if ui.button(tx("met-nova_leitura")).clicked() {
            leituras.push(Leitura {
                meta: t.id.clone(),
                periodo: String::new(),
                valor: 0.0,
                origem: tx("met-origem_padrao").into(),
            });
            mudou = true;
        }
        if mudou {
            p.leituras = leituras;
            if let Err(e) = metas::gravar_leituras(&p.caminho, &p.leituras) {
                msg = Some((fx("app-erro_salvar", &[("e", vx(&e))]), true));
            }
        }

        if apagar {
            p.lateral.environment.targets.remove(k);
            self.aberta = None;
            p.recalcular();
        } else if t != p.lateral.environment.targets[k] {
            p.lateral.environment.targets[k] = t;
            p.recalcular();
        }
        msg
    }
}

fn tx_selo(s: metas::Selo) -> &'static str {
    match s {
        metas::Selo::PrimeiraMeta => tx("met-selo_primeira_meta"),
        metas::Selo::BalancoFechado => tx("met-selo_balanco_fechado"),
        metas::Selo::MetadeDesviada => tx("met-selo_metade_desviada"),
    }
}

/// O medidor: da linha de base ao alvo, com o valor simulado e o semáforo.
fn medidor(ui: &mut egui::Ui, t: &Target, s: &Situacao) {
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(vec2(22.0, 22.0), Sense::hover());
        ui.painter().circle_filled(r.center(), 9.0, cor(s.semaforo));
        ui.label(egui::RichText::new(com_intervalo(&s.valor)).heading());
    });
    if t.target.is_some() {
        ui.label(fx(
            "met-probabilidade",
            &[
                ("pct", vx(&num(100.0 * s.probabilidade, 0))),
                ("de", vx(&num(100.0 * s.intervalo.0, 0))),
                ("ate", vx(&num(100.0 * s.intervalo.1, 0))),
            ],
        ));
    } else {
        ui.colored_label(colors::TEXT_WEAK, tx("met-sem_alvo"));
    }
    let valores: Vec<f64> = [t.baseline, t.target, Some(s.valor.media)]
        .into_iter()
        .flatten()
        .collect();
    if valores.len() < 2 {
        return;
    }
    let lo = valores.iter().copied().fold(f64::MAX, f64::min);
    let hi = valores.iter().copied().fold(f64::MIN, f64::max);
    let folga = ((hi - lo) * 0.1).max(1e-9);
    let (lo, hi) = (lo - folga, hi + folga);
    let (resp, pintor) = ui.allocate_painter(vec2(ui.available_width(), 34.0), Sense::hover());
    let r = Rect::from_min_size(
        resp.rect.min + vec2(0.0, 6.0),
        vec2(resp.rect.width(), 10.0),
    );
    let x = |v: f64| r.left() + ((v - lo) / (hi - lo)) as f32 * r.width();
    pintor.rect_filled(r, 3.0, colors::FIELD);
    let marca = |v: f64, rotulo: &str, c: Color32| {
        pintor.line_segment(
            [pos2(x(v), r.top() - 4.0), pos2(x(v), r.bottom() + 4.0)],
            Stroke::new(2.0, c),
        );
        pintor.text(
            pos2(x(v), r.bottom() + 5.0),
            Align2::CENTER_TOP,
            rotulo,
            FontId::proportional(11.0),
            c,
        );
    };
    if let Some(b) = t.baseline {
        marca(b, tx("met-base_curta"), colors::TEXT_WEAK);
    }
    if let Some(a) = t.target {
        marca(a, tx("met-alvo_curto"), colors::SUCCESS);
    }
    marca(s.valor.media, tx("met-simulado_curto"), crate::app::COR_APP);
}

/// A cascata: a linha de base, um degrau por cenário ligado, e o alvo.
fn cascata(ui: &mut egui::Ui, base: f64, degraus: &[metas::Degrau], alvo: Option<f64>) {
    ui.label(egui::RichText::new(tx("met-cascata")).strong());
    let mut valores = vec![base];
    valores.extend(degraus.iter().map(|d| d.depois));
    valores.extend(alvo);
    let hi = valores.iter().copied().fold(f64::MIN, f64::max).max(1e-9);
    let n = degraus.len() + 2;
    let (resp, pintor) = ui.allocate_painter(
        vec2((n as f32 * 90.0).min(ui.available_width()), 150.0),
        Sense::hover(),
    );
    let r = resp.rect.shrink2(vec2(4.0, 20.0));
    let w = r.width() / n as f32;
    let y = |v: f64| r.bottom() - (v / hi) as f32 * r.height();
    let barra = |k: usize, de: f64, ate: f64, c: Color32, rotulo: &str| {
        let x0 = r.left() + w * k as f32 + 6.0;
        let rr = Rect::from_x_y_ranges(
            x0..=x0 + w - 12.0,
            y(de.max(ate))..=y(de.min(ate)).max(y(de.max(ate)) + 1.0),
        );
        pintor.rect_filled(rr, 2.0, c);
        pintor.text(
            pos2(rr.center().x, r.bottom() + 3.0),
            Align2::CENTER_TOP,
            rotulo,
            FontId::proportional(11.0),
            colors::TEXT_WEAK,
        );
        pintor.text(
            pos2(rr.center().x, rr.top() - 2.0),
            Align2::CENTER_BOTTOM,
            num(ate, casas(ate)),
            FontId::proportional(11.0),
            colors::TEXT,
        );
    };
    barra(0, 0.0, base, colors::TEXT_WEAK, tx("met-base_curta"));
    let mut atual = base;
    for (k, d) in degraus.iter().enumerate() {
        let c = if d.efeito < 0.0 {
            colors::SUCCESS
        } else {
            colors::ERROR
        };
        barra(k + 1, atual, d.depois, c, &d.nome);
        atual = d.depois;
    }
    if let Some(a) = alvo {
        barra(n - 1, 0.0, a, crate::app::COR_APP, tx("met-alvo_curto"));
    }
    ui.label(
        egui::RichText::new(tx("met-cascata_explica"))
            .color(colors::TEXT_WEAK)
            .small(),
    );
}
