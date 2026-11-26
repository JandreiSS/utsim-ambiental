//! A tela Fatores, mínima (plano fase AM2): a unidade de tempo do modelo, os fluxos do
//! catálogo e o fator de emissão de cada um, com fonte, ano, escopo e o rótulo "didático".
//! O catálogo nasce vazio: nenhum número de fator vem de fábrica (decisão A8).

use eframe::egui;
use utsim_ambiental_core::formato::{A_DEFINIR, Direction, Factor, FlowDef, FlowKind, TimeUnit};
use utsim_ui_tema::colors;

use crate::projeto::Projeto;
use crate::texto::{fx, tx, vx};

/// O fluxo novo sendo escrito (guardado na memória da tela).
#[derive(Clone, Debug)]
struct Novo {
    nome: String,
    tipo: FlowKind,
    unidade: String,
}

impl Default for Novo {
    fn default() -> Self {
        Novo {
            nome: String::new(),
            tipo: FlowKind::Fuel,
            unidade: "m3".into(),
        }
    }
}

const TIPOS: [FlowKind; 8] = [
    FlowKind::Energy,
    FlowKind::Fuel,
    FlowKind::Water,
    FlowKind::Material,
    FlowKind::Waste,
    FlowKind::Effluent,
    FlowKind::Emission,
    FlowKind::Other,
];

fn tx_tipo(k: FlowKind) -> &'static str {
    match k {
        FlowKind::Energy => tx("fat-tipo_energia"),
        FlowKind::Fuel => tx("fat-tipo_combustivel"),
        FlowKind::Water => tx("fat-tipo_agua"),
        FlowKind::Material => tx("fat-tipo_material"),
        FlowKind::Waste => tx("fat-tipo_residuo"),
        FlowKind::Effluent => tx("fat-tipo_efluente"),
        FlowKind::Emission => tx("fat-tipo_emissao"),
        FlowKind::Other => tx("fat-tipo_outro"),
    }
}

fn tx_unidade(u: TimeUnit) -> &'static str {
    match u {
        TimeUnit::Second => tx("fat-segundo"),
        TimeUnit::Minute => tx("fat-minuto"),
        TimeUnit::Hour => tx("fat-hora"),
    }
}

pub fn ui(ui: &mut egui::Ui, p: &mut Projeto) -> Option<(String, bool)> {
    let mut msg = None;
    let mut mudou = false;
    ui.horizontal(|ui| {
        ui.heading(tx("fat-titulo"));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add_enabled(p.mudou(), utsim_ui_tema::widgets::primary(tx("app-salvar")))
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
    ui.label(egui::RichText::new(tx("fat-explica")).color(colors::TEXT_WEAK));
    ui.add_space(6.0);

    let cat = &mut p.lateral.environment;
    ui.horizontal(|ui| {
        ui.label(tx("fat-unidade_tempo"));
        egui::ComboBox::from_id_salt("unidade_tempo")
            .selected_text(tx_unidade(cat.clock.time_unit))
            .show_ui(ui, |ui| {
                for u in TimeUnit::TODAS {
                    mudou |= ui
                        .selectable_value(&mut cat.clock.time_unit, u, tx_unidade(u))
                        .changed();
                }
            });
        ui.label(egui::RichText::new(tx("fat-unidade_tempo_dica")).color(colors::TEXT_WEAK));
    });
    ui.add_space(8.0);

    // Todo fluxo tem uma linha de fator (vazia, "a definir", se ainda não tem).
    for f in &cat.flows {
        if cat.fator(&f.name).is_none() {
            cat.factors.push(Factor {
                flow: f.name.clone(),
                scope: if f.kind == FlowKind::Energy { 2 } else { 1 },
                kg_co2e: None,
                source: A_DEFINIR.into(),
                year: None,
                range: None,
                didactic: false,
            });
        }
    }

    let mut tirar = None;
    egui::ScrollArea::both().show(ui, |ui| {
        egui::Grid::new("fatores")
            .striped(true)
            .num_columns(9)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                for c in [
                    tx("fat-fluxo"),
                    tx("fat-tipo"),
                    tx("fat-unidade"),
                    tx("fat-escopo"),
                    tx("fat-fator"),
                    tx("fat-fonte"),
                    tx("fat-ano"),
                    tx("fat-didatico"),
                    "",
                ] {
                    ui.label(egui::RichText::new(c).strong());
                }
                ui.end_row();
                let fluxos: Vec<FlowDef> = cat.flows.clone();
                for (i, f) in fluxos.iter().enumerate() {
                    ui.label(&f.name);
                    ui.label(tx_tipo(f.kind));
                    ui.label(&f.unit);
                    let Some(fa) = cat.factors.iter_mut().find(|x| x.flow == f.name) else {
                        ui.end_row();
                        continue;
                    };
                    egui::ComboBox::from_id_salt(("escopo", i))
                        .width(40.0)
                        .selected_text(fa.scope.to_string())
                        .show_ui(ui, |ui| {
                            for s in 1..=3u8 {
                                mudou |= ui
                                    .selectable_value(&mut fa.scope, s, s.to_string())
                                    .changed();
                            }
                        });
                    ui.horizontal(|ui| {
                        let mut tem = fa.kg_co2e.is_some();
                        if ui
                            .checkbox(&mut tem, "")
                            .on_hover_text(tx("fat-tem_valor"))
                            .changed()
                        {
                            fa.kg_co2e = tem.then_some(0.0);
                            mudou = true;
                        }
                        match &mut fa.kg_co2e {
                            Some(v) => {
                                mudou |= ui
                                    .add(
                                        egui::DragValue::new(v)
                                            .range(0.0..=1.0e6)
                                            .speed(0.001)
                                            .max_decimals(4),
                                    )
                                    .changed();
                                ui.label(fx("fat-kg_por", &[("unidade", vx(&f.unit))]));
                            }
                            None => {
                                ui.colored_label(colors::WARNING, tx("fab-falta_fator"));
                            }
                        }
                    });
                    ui.horizontal(|ui| {
                        mudou |= ui
                            .add(egui::TextEdit::singleline(&mut fa.source).desired_width(200.0))
                            .changed();
                        if fa.sem_fonte() {
                            crate::fabrica::selo(ui, tx("fat-sem_fonte"), colors::WARNING);
                        }
                    });
                    let mut ano = fa.year.map(|a| a.to_string()).unwrap_or_default();
                    if ui
                        .add(egui::TextEdit::singleline(&mut ano).desired_width(50.0))
                        .changed()
                    {
                        fa.year = ano.trim().parse().ok();
                        mudou = true;
                    }
                    mudou |= ui.checkbox(&mut fa.didactic, "").changed();
                    // O fluxo de energia elétrica fica: é nele que a potência vira emissão.
                    let eletrico = f.kind == FlowKind::Energy
                        && fluxos.iter().position(|x| x.kind == FlowKind::Energy) == Some(i);
                    if !eletrico && ui.small_button(tx("fat-tirar")).clicked() {
                        tirar = Some(f.name.clone());
                    }
                    if eletrico {
                        ui.label("");
                    }
                    ui.end_row();
                }
            });
    });
    if let Some(n) = tirar {
        cat.flows.retain(|f| f.name != n);
        cat.factors.retain(|f| f.flow != n);
        mudou = true;
    }

    ui.add_space(8.0);
    ui.label(egui::RichText::new(tx("fat-novo_fluxo")).strong());
    let id = egui::Id::new("fatores-novo");
    let mut novo: Novo = ui.data_mut(|d| d.get_temp(id)).unwrap_or_default();
    ui.horizontal(|ui| {
        ui.label(tx("fat-fluxo"));
        ui.add(egui::TextEdit::singleline(&mut novo.nome).desired_width(140.0));
        egui::ComboBox::from_id_salt("novo_tipo")
            .selected_text(tx_tipo(novo.tipo))
            .show_ui(ui, |ui| {
                for k in TIPOS {
                    ui.selectable_value(&mut novo.tipo, k, tx_tipo(k));
                }
            });
        ui.label(tx("fat-unidade"));
        ui.add(egui::TextEdit::singleline(&mut novo.unidade).desired_width(60.0));
        let nome = novo.nome.trim().to_string();
        let ok = !nome.is_empty() && cat.fluxo(&nome).is_none() && !novo.unidade.trim().is_empty();
        if ui
            .add_enabled(ok, egui::Button::new(tx("fat-acrescentar")))
            .clicked()
        {
            let saida = matches!(
                novo.tipo,
                FlowKind::Waste | FlowKind::Effluent | FlowKind::Emission
            );
            cat.flows.push(FlowDef {
                name: nome,
                kind: novo.tipo,
                unit: novo.unidade.trim().to_string(),
                direction: if saida { Direction::Out } else { Direction::In },
                destination: None,
            });
            novo = Novo::default();
            mudou = true;
        }
    });
    ui.data_mut(|d| d.insert_temp(id, novo));

    ui.add_space(12.0);
    ui.separator();
    mudou |= tarifa(ui, p);
    if mudou {
        p.recalcular();
    }
    msg
}

/// Um número opcional: a caixa diz se tem valor; sem ela, "a definir".
pub fn opcional(ui: &mut egui::Ui, v: &mut Option<f64>, velocidade: f64) -> bool {
    let mut mudou = false;
    ui.horizontal(|ui| {
        let mut tem = v.is_some();
        if ui
            .checkbox(&mut tem, "")
            .on_hover_text(tx("fat-tem_valor"))
            .changed()
        {
            *v = tem.then_some(0.0);
            mudou = true;
        }
        match v {
            Some(x) => {
                mudou |= ui
                    .add(
                        egui::DragValue::new(x)
                            .range(0.0..=1.0e9)
                            .speed(velocidade)
                            .max_decimals(4),
                    )
                    .changed();
            }
            None => {
                ui.colored_label(colors::TEXT_WEAK, tx("rel-a_definir"));
            }
        }
    });
    mudou
}

/// A tarifa de energia (fase AM4): horário de pico, intervalo de demanda, demanda contratada
/// e preços, todos informados pelo usuário.
fn tarifa(ui: &mut egui::Ui, p: &mut Projeto) -> bool {
    use utsim_ambiental_core::formato::Tariff;
    let mut mudou = false;
    ui.heading(tx("fat-tarifa"));
    ui.label(egui::RichText::new(tx("fat-tarifa_explica")).color(colors::TEXT_WEAK));
    let cat = &mut p.lateral.environment;
    let mut usar = cat.tariff.is_some();
    if ui.checkbox(&mut usar, tx("fat-usar_tarifa")).changed() {
        cat.tariff = usar.then(Tariff::inicial);
        mudou = true;
    }
    let Some(t) = &mut cat.tariff else {
        return mudou;
    };
    egui::Grid::new("tarifa")
        .num_columns(2)
        .spacing([12.0, 6.0])
        .show(ui, |ui| {
            ui.label(tx("fat-horario_pico"));
            ui.horizontal(|ui| {
                mudou |= ui
                    .add(egui::TextEdit::singleline(&mut t.peak.from).desired_width(50.0))
                    .changed();
                ui.label("–");
                mudou |= ui
                    .add(egui::TextEdit::singleline(&mut t.peak.to).desired_width(50.0))
                    .changed();
                mudou |= ui
                    .checkbox(&mut t.peak.weekdays_only, tx("fat-dias_uteis"))
                    .changed();
            });
            ui.end_row();
            ui.label(tx("fat-intervalo_demanda"));
            mudou |= ui
                .add(
                    egui::DragValue::new(&mut t.demand_interval_min)
                        .range(1..=60)
                        .suffix(" min"),
                )
                .changed();
            ui.end_row();
            ui.label(tx("fat-contratada"));
            mudou |= opcional(ui, &mut t.contracted_kw, 1.0);
            ui.end_row();
            ui.label(tx("fat-preco_pico"));
            mudou |= opcional(ui, &mut t.price_kwh_peak, 0.01);
            ui.end_row();
            ui.label(tx("fat-preco_fora"));
            mudou |= opcional(ui, &mut t.price_kwh_off_peak, 0.01);
            ui.end_row();
            ui.label(tx("fat-preco_kw"));
            mudou |= opcional(ui, &mut t.price_kw, 0.1);
            ui.end_row();
            ui.label(tx("fat-preco_ultra"));
            mudou |= opcional(ui, &mut t.price_kw_overrun, 0.1);
            ui.end_row();
            ui.label(tx("fat-fonte_tarifa"));
            ui.horizontal(|ui| {
                mudou |= ui
                    .add(egui::TextEdit::singleline(&mut t.source).desired_width(220.0))
                    .changed();
                let s = t.source.trim();
                if s.is_empty() || s.eq_ignore_ascii_case(utsim_ambiental_core::formato::A_DEFINIR)
                {
                    crate::fabrica::selo(ui, tx("fat-sem_fonte"), colors::WARNING);
                }
            });
            ui.end_row();
        });
    mudou
}
