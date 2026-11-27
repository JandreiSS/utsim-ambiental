//! A tela Produto (fase AM7, plano 3.3): a pegada de cada tipo de peça por unidade
//! funcional, a fronteira (portão a portão ou berço ao portão), a regra de rateio, a
//! contribuição por máquina, a estrutura, a taxa de refugo e o relatório do estudo.

use eframe::egui::{self, Align2, FontId, Rect, Sense, Stroke, vec2};
use utsim_ambiental_core::Indicador;
use utsim_ambiental_core::formato::Allocation;
use utsim_ambiental_core::produto::{Pegada, ResumoProdutos};
use utsim_ui_tema::colors;

use crate::projeto::Projeto;
use crate::texto::{casas, com_intervalo, fx, num, tx, vx};

/// A unidade funcional.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Unidade {
    #[default]
    PecaBoa,
    Kg,
}

/// A fronteira do estudo.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Fronteira {
    #[default]
    PortaoAPortao,
    BercoAoPortao,
}

#[derive(Clone, Debug, Default)]
pub struct Vista {
    pub tipo: Option<u32>,
    pub comparar: Option<u32>,
    pub unidade: Unidade,
    pub fronteira: Fronteira,
    /// A taxa de refugo do controle deslizante (`None`: a simulada).
    pub taxa: Option<f64>,
}

/// Uma quantidade por unidade funcional, replicação a replicação.
pub fn por_unidade(v: &[f64], den: &[f64]) -> Option<Indicador> {
    let xs: Vec<f64> = v
        .iter()
        .zip(den)
        .filter(|(_, d)| **d > 0.0)
        .map(|(x, d)| x / d)
        .collect();
    (!xs.is_empty()).then(|| Indicador::de(&xs))
}

/// O CO₂e da pegada na fronteira escolhida, por replicação (`Err`: fatores que faltam).
pub fn co2(pg: &Pegada, f: Fronteira) -> Result<Vec<f64>, Vec<String>> {
    let fab = pg.co2_fabricacao.clone()?;
    match f {
        Fronteira::PortaoAPortao => Ok(fab),
        Fronteira::BercoAoPortao => {
            let mat = pg.co2_materiais.clone()?;
            Ok(fab.iter().zip(&mat).map(|(a, b)| a + b).collect())
        }
    }
}

/// O denominador da unidade funcional, por replicação.
pub fn denominador(pg: &Pegada, u: Unidade) -> &[f64] {
    match u {
        Unidade::PecaBoa => &pg.boas,
        Unidade::Kg => &pg.massa_boa_kg,
    }
}

pub fn tx_unidade(u: Unidade) -> &'static str {
    match u {
        Unidade::PecaBoa => tx("prd-peca_boa"),
        Unidade::Kg => tx("prd-kg_produto"),
    }
}

pub fn tx_regra(r: Allocation) -> &'static str {
    match r {
        Allocation::ByProcessTime => tx("prd-regra_tempo"),
        Allocation::ByItems => tx("prd-regra_pecas"),
        Allocation::ByMass => tx("prd-regra_massa"),
        Allocation::Structure => tx("prd-regra_estrutura"),
    }
}

fn nome(pg: &Pegada) -> String {
    if pg.nome == "*" {
        tx("prd-sem_tipo").to_string()
    } else {
        pg.nome.clone()
    }
}

/// A taxa de refugo simulada do tipo: refugadas ÷ (boas + refugadas).
pub fn taxa_simulada(pg: &Pegada) -> f64 {
    let boas: f64 = pg.boas.iter().sum();
    let ref_: f64 = pg.refugadas.iter().sum();
    if boas + ref_ > 0.0 {
        ref_ / (boas + ref_)
    } else {
        0.0
    }
}

impl Vista {
    pub fn ui(&mut self, ui: &mut egui::Ui, p: &mut Projeto) -> Option<(String, bool)> {
        let mut msg = None;
        ui.heading(tx("prd-titulo"));
        ui.colored_label(colors::WARNING, tx("prd-aviso"));
        let Some(inv) = p.inventario.clone() else {
            ui.label(tx("fab-esperando_simulacao"));
            return None;
        };
        let Some(prod) = inv.produtos.clone() else {
            ui.label(tx("prd-sem_rastro"));
            return None;
        };
        if prod.pegadas.is_empty() {
            ui.label(tx("prd-sem_pecas"));
            return None;
        }
        let tipo = self
            .tipo
            .filter(|t| prod.pegadas.iter().any(|x| x.tipo == *t))
            .unwrap_or(prod.pegadas[0].tipo);
        self.tipo = Some(tipo);
        ui.add_space(6.0);
        self.estacoes(ui);
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(tx("prd-produto"));
            egui::ComboBox::from_id_salt("produto")
                .selected_text(
                    prod.pegadas
                        .iter()
                        .find(|x| x.tipo == tipo)
                        .map(nome)
                        .unwrap_or_default(),
                )
                .show_ui(ui, |ui| {
                    for pg in &prod.pegadas {
                        if ui.selectable_label(pg.tipo == tipo, nome(pg)).clicked() {
                            self.tipo = Some(pg.tipo);
                            self.taxa = None;
                        }
                    }
                });
            ui.add_space(10.0);
            ui.label(tx("prd-unidade"));
            let us = [Unidade::PecaBoa, Unidade::Kg];
            let nomes: Vec<&str> = us.iter().map(|u| tx_unidade(*u)).collect();
            let atual = us.iter().position(|u| *u == self.unidade);
            for (i, r) in utsim_ui_tema::widgets::segmented(ui, &nomes, atual)
                .into_iter()
                .enumerate()
            {
                if r.clicked() {
                    self.unidade = us[i];
                }
            }
            ui.add_space(10.0);
            ui.label(tx("prd-regra"));
            let mut regra = p.lateral.environment.allocation;
            egui::ComboBox::from_id_salt("regra")
                .selected_text(tx_regra(regra))
                .show_ui(ui, |ui| {
                    for r in [
                        Allocation::ByProcessTime,
                        Allocation::ByItems,
                        Allocation::ByMass,
                        Allocation::Structure,
                    ] {
                        ui.selectable_value(&mut regra, r, tx_regra(r));
                    }
                });
            if regra != p.lateral.environment.allocation {
                p.lateral.environment.allocation = regra;
                p.recalcular();
            }
            ui.add_space(10.0);
            ui.label(tx("prd-comparar"));
            egui::ComboBox::from_id_salt("comparar")
                .selected_text(
                    self.comparar
                        .and_then(|t| prod.pegadas.iter().find(|x| x.tipo == t))
                        .map_or("–".into(), nome),
                )
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.comparar, None, "–");
                    for pg in prod.pegadas.iter().filter(|x| x.tipo != tipo) {
                        ui.selectable_value(&mut self.comparar, Some(pg.tipo), nome(pg));
                    }
                });
        });
        ui.separator();
        let pg = prod.pegadas.iter().find(|x| x.tipo == tipo)?;
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(430.0);
                    self.cartao(ui, pg);
                });
                if let Some(c) = self
                    .comparar
                    .and_then(|t| prod.pegadas.iter().find(|x| x.tipo == t))
                {
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.set_width(430.0);
                        self.cartao(ui, c);
                    });
                }
            });
            ui.add_space(8.0);
            self.contribuicao(ui, pg);
            ui.add_space(8.0);
            self.estrutura(ui, &prod, &inv);
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                for (pdf, rotulo) in [(false, tx("rel-gerar_html")), (true, tx("rel-gerar_pdf"))] {
                    if ui
                        .add(utsim_ui_tema::widgets::primary(&format!(
                            "{} · {}",
                            tx("prd-relatorio"),
                            rotulo
                        )))
                        .clicked()
                    {
                        msg = Some(match crate::relatorio::gravar_estudo(p, &inv, self, pdf) {
                            Ok(c) => (fx("rel-gravado", &[("arquivo", vx(&c.display()))]), false),
                            Err(e) => (fx("app-erro_salvar", &[("e", vx(&e))]), true),
                        });
                    }
                }
            });
        });
        msg
    }

    /// As estações do ciclo de vida: a fabricação sempre dentro; as matérias-primas entram
    /// com "berço ao portão"; o resto fica fora nesta versão.
    fn estacoes(&mut self, ui: &mut egui::Ui) {
        let nomes = [
            (tx("prd-materias_primas"), Some(Fronteira::BercoAoPortao)),
            (tx("prd-transporte"), None),
            (tx("prd-fabricacao"), Some(Fronteira::PortaoAPortao)),
            (tx("prd-distribuicao"), None),
            (tx("prd-uso"), None),
            (tx("prd-fim_de_vida"), None),
        ];
        let (resp, pintor) =
            ui.allocate_painter(vec2(ui.available_width().min(900.0), 46.0), Sense::click());
        let w = resp.rect.width() / nomes.len() as f32;
        for (k, (rotulo, f)) in nomes.iter().enumerate() {
            let r = Rect::from_min_size(
                resp.rect.min + vec2(w * k as f32 + 4.0, 4.0),
                vec2(w - 8.0, 38.0),
            );
            let dentro = match f {
                Some(Fronteira::PortaoAPortao) => true,
                Some(Fronteira::BercoAoPortao) => self.fronteira == Fronteira::BercoAoPortao,
                _ => false,
            };
            let fundo = if dentro {
                crate::app::COR_APP.gamma_multiply(0.55)
            } else {
                colors::FIELD
            };
            pintor.rect_filled(r, 6.0, fundo);
            pintor.rect_stroke(
                r,
                6.0,
                Stroke::new(1.0, colors::BUTTON_BORDER),
                egui::StrokeKind::Inside,
            );
            pintor.text(
                r.center(),
                Align2::CENTER_CENTER,
                *rotulo,
                FontId::proportional(13.0),
                if dentro {
                    colors::TEXT_STRONG
                } else {
                    colors::TEXT_WEAK
                },
            );
            if resp.clicked()
                && let Some(m) = resp.interact_pointer_pos()
                && r.contains(m)
                && *f == Some(Fronteira::BercoAoPortao)
            {
                self.fronteira = if self.fronteira == Fronteira::BercoAoPortao {
                    Fronteira::PortaoAPortao
                } else {
                    Fronteira::BercoAoPortao
                };
            }
        }
        let t = match self.fronteira {
            Fronteira::PortaoAPortao => tx("prd-portao_a_portao"),
            Fronteira::BercoAoPortao => tx("prd-berco_ao_portao"),
        };
        ui.label(
            egui::RichText::new(fx("prd-fronteira", &[("fronteira", vx(&t))]))
                .color(colors::TEXT_WEAK),
        );
    }

    /// O cartão de um produto: pegada por unidade, energia, água e a taxa de refugo.
    fn cartao(&mut self, ui: &mut egui::Ui, pg: &Pegada) {
        let den = denominador(pg, self.unidade);
        let un = tx_unidade(self.unidade);
        ui.label(egui::RichText::new(nome(pg)).heading());
        egui::Grid::new(("pegada", pg.tipo))
            .num_columns(2)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                ui.label(tx("prd-unidades"));
                ui.label(com_intervalo(&Indicador::de(den)));
                ui.end_row();
                ui.label(fx("prd-co2_por", &[("unidade", vx(&un))]));
                match co2(pg, self.fronteira) {
                    Ok(v) => match por_unidade(&v, den) {
                        Some(i) => {
                            ui.label(
                                egui::RichText::new(format!(
                                    "{} {}",
                                    com_intervalo(&i),
                                    tx("un-kg_co2e")
                                ))
                                .strong(),
                            );
                        }
                        None => {
                            ui.label("–");
                        }
                    },
                    Err(f) => {
                        ui.colored_label(colors::WARNING, tx("fab-falta_fator"))
                            .on_hover_text(fx(
                                "fab-falta_fator_dica",
                                &[("fluxos", vx(&f.join(", ")))],
                            ));
                    }
                }
                ui.end_row();
                for (chave, v, u) in [
                    ("prd-energia_por", &pg.kwh, tx("un-kwh")),
                    ("prd-agua_por", &pg.agua_m3, tx("un-m3")),
                    ("prd-mochila_por", &pg.mochila_kwh, tx("un-kwh")),
                ] {
                    if let Some(i) = por_unidade(v, den) {
                        ui.label(fx(chave, &[("unidade", vx(&un))]));
                        ui.label(format!("{} {u}", com_intervalo(&i)));
                        ui.end_row();
                    }
                }
            });
        // A pegada sobe com o refugo: a mesma energia gasta sai em menos peças boas.
        let sim = taxa_simulada(pg);
        let mut taxa = self.taxa.unwrap_or(sim) * 100.0;
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(tx("prd-taxa_refugo"));
            if ui
                .add(
                    egui::Slider::new(&mut taxa, 0.0..=50.0)
                        .suffix(" %")
                        .max_decimals(1),
                )
                .changed()
            {
                self.taxa = Some(taxa / 100.0);
            }
            if ui.small_button(tx("prd-voltar_simulada")).clicked() {
                self.taxa = None;
            }
        });
        let fator = (1.0 - sim) / (1.0 - taxa / 100.0).max(0.01);
        if let Ok(v) = co2(pg, self.fronteira)
            && let Some(i) = por_unidade(&v, den)
        {
            ui.label(fx(
                "prd-com_refugo",
                &[
                    ("taxa", vx(&num(taxa, 1))),
                    ("simulada", vx(&num(sim * 100.0, 1))),
                    ("valor", vx(&num(i.media * fator, casas(i.media * fator)))),
                ],
            ));
        } else if let Some(i) = por_unidade(&pg.kwh, den) {
            ui.label(fx(
                "prd-com_refugo_kwh",
                &[
                    ("taxa", vx(&num(taxa, 1))),
                    ("simulada", vx(&num(sim * 100.0, 1))),
                    ("valor", vx(&num(i.media * fator, casas(i.media * fator)))),
                ],
            ));
        }
    }

    /// A contribuição de cada máquina para a pegada (energia e CO₂e por unidade).
    fn contribuicao(&self, ui: &mut egui::Ui, pg: &Pegada) {
        ui.label(egui::RichText::new(tx("prd-contribuicao")).strong());
        let den = Indicador::de(denominador(pg, self.unidade)).media;
        if den <= 0.0 {
            return;
        }
        let total: f64 = pg.por_no.iter().map(|x| x.1).sum::<f64>().max(1e-12);
        egui::Grid::new("contribuicao")
            .striped(true)
            .num_columns(4)
            .spacing([12.0, 4.0])
            .show(ui, |ui| {
                for c in [
                    tx("maq-maquina"),
                    tx("prd-kwh_por_unidade"),
                    tx("prd-co2_por_unidade"),
                    "",
                ] {
                    ui.label(egui::RichText::new(c).strong());
                }
                ui.end_row();
                for (no, kwh, co2) in &pg.por_no {
                    ui.label(no);
                    ui.label(num(kwh / den, casas(kwh / den)));
                    ui.label(co2.map_or("–".into(), |c| num(c / den, casas(c / den))));
                    let (r, _) = ui.allocate_exact_size(vec2(160.0, 10.0), Sense::hover());
                    ui.painter().rect_filled(r, 2.0, colors::FIELD);
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            r.min,
                            vec2(r.width() * (kwh / total) as f32, r.height()),
                        ),
                        2.0,
                        crate::app::COR_APP,
                    );
                    ui.end_row();
                }
            });
    }

    /// A estrutura: o que não foi para nenhum produto, e a conferência do total.
    fn estrutura(
        &self,
        ui: &mut egui::Ui,
        prod: &ResumoProdutos,
        inv: &utsim_ambiental_core::Inventario,
    ) {
        let soma: f64 = prod
            .pegadas
            .iter()
            .map(|x| Indicador::de(&x.kwh).media)
            .sum();
        let est = Indicador::de(&prod.estrutura_kwh);
        ui.label(fx(
            "prd-estrutura",
            &[
                ("regra", vx(&tx_regra(prod.regra))),
                ("produtos", vx(&num(soma, casas(soma)))),
                ("estrutura", vx(&num(est.media, casas(est.media)))),
                (
                    "fabrica",
                    vx(&num(inv.energia.media, casas(inv.energia.media))),
                ),
            ],
        ));
    }
}
