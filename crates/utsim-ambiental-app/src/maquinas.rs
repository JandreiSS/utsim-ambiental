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
    if mudou {
        *p.bloco_mut(no) = d;
    }
    mudou
}
