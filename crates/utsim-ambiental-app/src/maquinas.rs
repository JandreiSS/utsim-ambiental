//! A tela Máquinas, o assistente "Dar vida ao modelo" (plano fase AM2): uma linha por
//! máquina, com a etapa e a potência em cada grupo de estado. Toda mudança refaz a conta na
//! hora.

use eframe::egui;
use utsim_ambiental_core::Grupo;
use utsim_ambiental_core::conta::consome;
use utsim_ui_tema::colors;

use crate::fabrica::tx_grupo;
use crate::projeto::Projeto;
use crate::texto::{com_intervalo, fx, tx, vx};

pub fn ui(ui: &mut egui::Ui, p: &mut Projeto) -> Option<(String, bool)> {
    let mut msg = None;
    ui.horizontal(|ui| {
        ui.heading(tx("maq-titulo"));
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
    ui.label(egui::RichText::new(tx("maq-explica")).color(colors::TEXT_WEAK));
    ui.add_space(6.0);
    let nomes: Vec<String> = p
        .modelo
        .objects
        .iter()
        .filter(|o| consome(&o.kind) && !utsim_model::is_mobile(&o.kind))
        .map(|o| o.name.clone())
        .collect();
    let mut mudou = false;
    let id = egui::Id::new("maquinas-escolhida");
    let mut escolhida: Option<String> = ui.data_mut(|d| d.get_temp(id)).flatten();
    egui::ScrollArea::both().show(ui, |ui| {
        egui::Grid::new("maquinas")
            .striped(true)
            .num_columns(9)
            .spacing([10.0, 6.0])
            .show(ui, |ui| {
                ui.label(egui::RichText::new(tx("maq-maquina")).strong());
                ui.label(egui::RichText::new(tx("maq-etapa")).strong());
                for g in Grupo::TODOS {
                    ui.label(egui::RichText::new(format!("{} (kW)", tx_grupo(g))).strong());
                }
                ui.label(egui::RichText::new(tx("maq-energia")).strong());
                ui.label("");
                ui.end_row();
                for no in &nomes {
                    ui.label(no);
                    let mut etapa = p
                        .lateral
                        .objects
                        .get(no)
                        .and_then(|d| d.stage.clone())
                        .unwrap_or_default();
                    if ui
                        .add_sized([110.0, 22.0], egui::TextEdit::singleline(&mut etapa))
                        .changed()
                    {
                        let e = etapa.trim();
                        p.bloco_mut(no).stage = (!e.is_empty()).then(|| e.to_string());
                        mudou = true;
                    }
                    let mut pot = p
                        .lateral
                        .objects
                        .get(no)
                        .map(|d| d.power.clone())
                        .unwrap_or_default();
                    let mut esta = false;
                    for g in Grupo::TODOS {
                        esta |= ui
                            .add(
                                egui::DragValue::new(pot.do_grupo_mut(g))
                                    .range(0.0..=1.0e6)
                                    .speed(0.1),
                            )
                            .changed();
                    }
                    if esta {
                        p.bloco_mut(no).power = pot;
                        mudou = true;
                    }
                    match p.inventario.as_ref().and_then(|i| i.no(no)) {
                        Some(n) if n.tem_bloco => {
                            ui.label(format!("{} {}", com_intervalo(&n.energia), tx("un-kwh")));
                        }
                        _ => {
                            ui.colored_label(colors::TEXT_WEAK, tx("fab-informe_potencia"));
                        }
                    }
                    let aberta = escolhida.as_deref() == Some(no.as_str());
                    if ui.selectable_label(aberta, tx("maq-detalhes")).clicked() {
                        escolhida = if aberta { None } else { Some(no.clone()) };
                    }
                    ui.end_row();
                }
            });
        if let Some(no) = escolhida.clone()
            && nomes.contains(&no)
        {
            ui.add_space(10.0);
            ui.separator();
            mudou |= detalhes(ui, p, &no);
        }
    });
    ui.data_mut(|d| d.insert_temp(id, escolhida));
    if mudou {
        p.recalcular();
    }
    msg
}

/// Os detalhes de uma máquina: energia por evento e desligamento em espera. `true` se
/// algo mudou.
fn detalhes(ui: &mut egui::Ui, p: &mut Projeto, no: &str) -> bool {
    use utsim_ambiental_core::formato::{EnergyPer, EventDriver, StandbyDef};
    let mut mudou = false;
    let mut d = p.lateral.objects.get(no).cloned().unwrap_or_default();
    ui.heading(fx("maq-detalhes_de", &[("no", vx(&no))]));
    ui.label(egui::RichText::new(tx("maq-eventos")).strong());
    egui::Grid::new(("eventos", no))
        .num_columns(2)
        .spacing([12.0, 4.0])
        .show(ui, |ui| {
            for (ev, rotulo) in [
                (EventDriver::Item, tx("maq-kwh_peca")),
                (EventDriver::Setup, tx("maq-kwh_setup")),
                (EventDriver::Startup, tx("maq-kwh_partida")),
            ] {
                ui.label(rotulo);
                let mut v = d
                    .energy_per
                    .iter()
                    .find(|e| e.driver == ev)
                    .map_or(0.0, |e| e.kwh);
                if ui
                    .add(
                        egui::DragValue::new(&mut v)
                            .range(0.0..=1.0e6)
                            .speed(0.01)
                            .suffix(" kWh"),
                    )
                    .changed()
                {
                    d.energy_per.retain(|e| e.driver != ev);
                    if v > 0.0 {
                        d.energy_per.push(EnergyPer { driver: ev, kwh: v });
                    }
                    mudou = true;
                }
                ui.end_row();
            }
        });
    ui.add_space(6.0);
    let mut liga = d.standby.is_some();
    if ui
        .checkbox(&mut liga, tx("maq-desligar_em_espera"))
        .changed()
    {
        d.standby = liga.then_some(StandbyDef { after: 600.0 });
        mudou = true;
    }
    if let Some(sb) = &mut d.standby {
        ui.horizontal(|ui| {
            ui.label(tx("maq-depois_de"));
            mudou |= ui
                .add(
                    egui::DragValue::new(&mut sb.after)
                        .range(0.0..=1.0e9)
                        .speed(1.0),
                )
                .changed();
            ui.label(tx("maq-unidades_do_modelo"));
        });
        ui.label(egui::RichText::new(tx("maq-espera_aproximacao")).color(colors::TEXT_WEAK));
    }
    ui.add_space(8.0);
    mudou |= fluxos_do_no(ui, p, no, &mut d);
    if mudou {
        *p.bloco_mut(no) = d;
    }
    mudou
}

/// Os direcionadores que a tela oferece (os outros ficam como estão no arquivo).
fn direcionadores() -> Vec<(utsim_ambiental_core::formato::Driver, &'static str)> {
    use utsim_ambiental_core::formato::{Driver, Estado};
    vec![
        (Driver::Item, tx("maq-dir_peca")),
        (Driver::GoodItem, tx("maq-dir_peca_boa")),
        (Driver::Scrap, tx("maq-dir_refugo")),
        (Driver::Setup, tx("maq-dir_setup")),
        (Driver::Startup, tx("maq-dir_partida")),
        (Driver::Mass, tx("maq-dir_massa")),
        (
            Driver::Time(vec![Estado::Processing, Estado::Utilize]),
            tx("maq-dir_tempo_produzindo"),
        ),
        (
            Driver::Time(vec![
                Estado::Processing,
                Estado::Utilize,
                Estado::Setup,
                Estado::Idle,
                Estado::Empty,
                Estado::Blocked,
                Estado::WaitingOperator,
                Estado::WaitingTransport,
            ]),
            tx("maq-dir_tempo_ligada"),
        ),
    ]
}

/// Os fluxos do nó, o balanço de água e o efluente (fase AM5).
fn fluxos_do_no(
    ui: &mut egui::Ui,
    p: &Projeto,
    no: &str,
    d: &mut utsim_ambiental_core::EnvironmentDef,
) -> bool {
    use utsim_ambiental_core::formato::{Driver, EffluentDef, FlowKind, FlowUse, WaterDef};
    let cat = &p.lateral.environment;
    let mut mudou = false;
    ui.label(egui::RichText::new(tx("maq-fluxos")).strong());
    ui.label(egui::RichText::new(tx("maq-fluxos_explica")).color(colors::TEXT_WEAK));
    let nomes: Vec<String> = cat
        .flows
        .iter()
        .filter(|f| f.kind != FlowKind::Energy)
        .map(|f| f.name.clone())
        .collect();
    let dirs = direcionadores();
    let mut tirar = None;
    egui::Grid::new(("fluxos_no", no))
        .num_columns(6)
        .spacing([10.0, 4.0])
        .show(ui, |ui| {
            for (k, f) in d.flows.iter_mut().enumerate() {
                egui::ComboBox::from_id_salt(("fluxo", no, k))
                    .selected_text(f.flow.clone())
                    .show_ui(ui, |ui| {
                        for n in &nomes {
                            mudou |= ui.selectable_value(&mut f.flow, n.clone(), n).changed();
                        }
                    });
                mudou |= ui
                    .add(
                        egui::DragValue::new(&mut f.amount)
                            .range(0.0..=1.0e9)
                            .speed(0.01)
                            .max_decimals(4),
                    )
                    .changed();
                let unidade = cat
                    .fluxo(&f.flow)
                    .map(|x| x.unit.clone())
                    .unwrap_or_default();
                let atual = dirs
                    .iter()
                    .find(|x| x.0 == f.driver)
                    .map_or(tx("maq-dir_outro"), |x| x.1);
                egui::ComboBox::from_id_salt(("dir", no, k))
                    .selected_text(fx(
                        "maq-por",
                        &[("unidade", vx(&unidade)), ("dir", vx(&atual))],
                    ))
                    .show_ui(ui, |ui| {
                        for (dv, r) in &dirs {
                            if ui.selectable_label(&f.driver == dv, *r).clicked() {
                                f.driver = dv.clone();
                                mudou = true;
                            }
                        }
                    });
                let por_peca = matches!(f.driver, Driver::Item | Driver::GoodItem);
                ui.add_enabled_ui(por_peca, |ui| {
                    mudou |= ui
                        .checkbox(&mut f.adds_mass, tx("maq-soma_massa"))
                        .changed();
                });
                ui.add_enabled_ui(por_peca, |ui| {
                    mudou |= ui
                        .checkbox(&mut f.removes_mass, tx("maq-tira_massa"))
                        .changed();
                });
                if ui.small_button(tx("fat-tirar")).clicked() {
                    tirar = Some(k);
                }
                ui.end_row();
            }
        });
    if let Some(k) = tirar {
        d.flows.remove(k);
        mudou = true;
    }
    match nomes.first() {
        Some(primeiro) => {
            if ui.button(tx("maq-novo_fluxo")).clicked() {
                d.flows.push(FlowUse {
                    flow: primeiro.clone(),
                    amount: 0.0,
                    driver: Driver::Item,
                    adds_mass: false,
                    removes_mass: false,
                });
                mudou = true;
            }
        }
        None => {
            ui.colored_label(colors::TEXT_WEAK, tx("maq-sem_fluxos"));
        }
    }

    ui.add_space(6.0);
    let mut tem_agua = d.water.is_some();
    if ui.checkbox(&mut tem_agua, tx("maq-balanco_agua")).changed() {
        d.water = tem_agua.then(WaterDef::default);
        mudou = true;
    }
    if let Some(w) = &mut d.water {
        let alvos: Vec<String> = p
            .modelo
            .objects
            .iter()
            .filter(|o| consome(&o.kind))
            .map(|o| o.name.clone())
            .collect();
        egui::Grid::new(("agua", no))
            .num_columns(2)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                ui.label(tx("maq-incorporada"));
                mudou |= ui
                    .add(
                        egui::DragValue::new(&mut w.incorporated)
                            .range(0.0..=1.0)
                            .speed(0.01),
                    )
                    .changed();
                ui.end_row();
                ui.label(tx("maq-evaporada"));
                mudou |= ui
                    .add(
                        egui::DragValue::new(&mut w.evaporated)
                            .range(0.0..=1.0)
                            .speed(0.01),
                    )
                    .changed();
                ui.end_row();
                ui.label(tx("maq-reuso_para"));
                egui::ComboBox::from_id_salt(("reuso", no))
                    .selected_text(w.reuse_to.clone().unwrap_or_else(|| "–".into()))
                    .show_ui(ui, |ui| {
                        mudou |= ui.selectable_value(&mut w.reuse_to, None, "–").changed();
                        for a in &alvos {
                            mudou |= ui
                                .selectable_value(&mut w.reuse_to, Some(a.clone()), a)
                                .changed();
                        }
                    });
                ui.end_row();
                ui.label(tx("maq-reuso_fracao"));
                mudou |= ui
                    .add(
                        egui::DragValue::new(&mut w.reuse)
                            .range(0.0..=1.0)
                            .speed(0.01),
                    )
                    .changed();
                ui.end_row();
            });
    }

    let destinos: Vec<String> = cat
        .effluent
        .destinations
        .iter()
        .map(|x| x.name.clone())
        .collect();
    let mut tem_efl = d.effluent.is_some();
    if ui.checkbox(&mut tem_efl, tx("maq-efluente")).changed() {
        d.effluent = tem_efl.then(|| EffluentDef {
            destination: destinos.first().cloned().unwrap_or_default(),
            ..EffluentDef::default()
        });
        mudou = true;
    }
    if let Some(e) = &mut d.effluent {
        egui::Grid::new(("efluente", no))
            .num_columns(2)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                ui.label(tx("maq-destino_efluente"));
                egui::ComboBox::from_id_salt(("dest_efl", no))
                    .selected_text(e.destination.clone())
                    .show_ui(ui, |ui| {
                        for x in &destinos {
                            mudou |= ui
                                .selectable_value(&mut e.destination, x.clone(), x)
                                .changed();
                        }
                    });
                ui.end_row();
                for par in &cat.effluent.parameters {
                    ui.label(format!("{} ({})", par.name, par.unit));
                    let mut v = e.concentration.get(&par.name).copied().unwrap_or(0.0);
                    if ui
                        .add(egui::DragValue::new(&mut v).range(0.0..=1.0e7).speed(1.0))
                        .changed()
                    {
                        if v > 0.0 {
                            e.concentration.insert(par.name.clone(), v);
                        } else {
                            e.concentration.remove(&par.name);
                        }
                        mudou = true;
                    }
                    ui.end_row();
                }
            });
        if cat.effluent.parameters.is_empty() {
            ui.colored_label(colors::TEXT_WEAK, tx("maq-sem_parametros"));
        }
    }
    mudou
}
