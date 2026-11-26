//! O relatório ambiental do modelo (fase AM3), em HTML e PDF, pelo gerador de relatórios da
//! família (`utsim-ferramentas::relatorios`). Todo relatório traz o aviso de uso didático e
//! a lista de fatores com as fontes (AMB-RF-12), mesmo quando nenhum fator foi informado.

use std::path::PathBuf;

use eframe::egui;
use utsim_ambiental_core::{Estimativa, Grupo, Inventario, viloes};
use utsim_ferramentas::relatorios::{
    Block, Chart, ChartKind, Cover, Document, Kpi, Template, Tone,
};
use utsim_ui_tema::colors;

use crate::fabrica::tx_grupo;
use crate::projeto::Projeto;
use crate::texto::{casas, com_intervalo, fx, num, tx, vx};

/// A data de hoje (UTC) no formato da língua.
fn hoje() -> String {
    let dias = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() / 86_400) as i64;
    // Dias desde 1970-01-01 para ano, mês e dia (calendário gregoriano).
    let z = dias + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = (yoe + era * 400 + i64::from(m <= 2)) as i32;
    utsim_idiomas::data(y, m, d)
}

fn texto_estimativa(e: &Estimativa) -> String {
    match e {
        Estimativa::Valor(i) => format!("{} {}", com_intervalo(i), tx("un-kg_co2e")),
        Estimativa::FaltaFator(_) => tx("fab-falta_fator").to_string(),
    }
}

/// Uma tabela com cabeçalho já traduzido.
fn tabela(doc: &mut Document, nome: &str, cabecalho: Vec<String>, linhas: Vec<Vec<String>>) {
    doc.blocks.push(Block::Table {
        name: nome.into(),
        header: cabecalho,
        rows: linhas,
        widths: Vec::new(),
    });
}

/// O documento do relatório.
pub fn documento(p: &Projeto, inv: &Inventario) -> Document {
    let cat = &p.lateral.environment;
    let relogio = &cat.clock;
    let mut doc = Document::new(
        &fx("rel-titulo", &[("modelo", vx(&p.nome()))]),
        tx("app-subtitulo"),
    );
    doc.app = crate::TITULO.into();
    doc.toc = true;
    let origem = fx(
        "rel-origem",
        &[
            ("n", inv.replicacoes.into()),
            ("semente", vx(&p.modelo.run.seed)),
            ("de", vx(&relogio.hora(inv.janela.0))),
            ("ate", vx(&relogio.hora(inv.janela.1))),
        ],
    );
    doc.cover = Some(Cover {
        app: crate::TITULO.into(),
        title: doc.title.clone(),
        subtitle: tx("app-subtitulo").into(),
        author: String::new(),
        date: hoje(),
        source: origem.clone(),
        colour: crate::COR.into(),
        icon_png: crate::ICONE.to_vec(),
    });
    doc.callout(Tone::Warn, tx("rel-aviso_titulo"), tx("rel-aviso"));

    doc.heading(tx("rel-resumo"));
    doc.para(&origem);
    let pct = if inv.energia.media > 0.0 {
        100.0 * inv.sem_valor.media / inv.energia.media
    } else {
        0.0
    };
    let mut kpis = vec![
        Kpi {
            label: tx("fab-energia").into(),
            value: format!(
                "{} {}",
                num(inv.energia.media, casas(inv.energia.media)),
                tx("un-kwh")
            ),
            note: com_intervalo(&inv.energia),
        },
        Kpi {
            label: tx("fab-sem_valor").into(),
            value: format!("{} %", num(pct, 0)),
            note: format!("{} {}", com_intervalo(&inv.sem_valor), tx("un-kwh")),
        },
        Kpi {
            label: tx("fab-co2e").into(),
            value: texto_estimativa(&inv.co2e),
            note: String::new(),
        },
    ];
    if let Some(k) = inv.metrica("ambiental.kwh_por_peca") {
        kpis.push(Kpi {
            label: tx("fab-por_peca_boa").into(),
            value: format!("{} {}", num(k.media, casas(k.media)), tx("un-kwh")),
            note: com_intervalo(&k),
        });
    }
    doc.kpis(kpis);

    doc.heading(tx("rel-por_maquina"));
    let nos: Vec<_> = inv.nos.iter().filter(|n| n.tem_bloco).collect();
    tabela(
        &mut doc,
        tx("rel-por_maquina"),
        vec![
            tx("maq-maquina").into(),
            tx("maq-etapa").into(),
            format!("{} ({})", tx("fab-energia"), tx("un-kwh")),
            format!("{} ({})", tx("fab-sem_valor"), tx("un-kwh")),
            tx("un-kwh_peca").into(),
            tx("fab-co2e").into(),
        ],
        nos.iter()
            .map(|n| {
                vec![
                    n.nome.clone(),
                    n.etapa.clone().unwrap_or_default(),
                    com_intervalo(&n.energia),
                    com_intervalo(&n.sem_valor),
                    n.kwh_por_peca.as_ref().map_or("–".into(), com_intervalo),
                    texto_estimativa(&n.co2e),
                ]
            })
            .collect(),
    );
    doc.blocks.push(Block::Chart(Chart {
        title: tx("rel-energia_por_grupo").into(),
        bars: true,
        labels: nos.iter().map(|n| n.nome.clone()).collect(),
        series: Grupo::TODOS
            .iter()
            .map(|g| {
                (
                    tx_grupo(*g).to_string(),
                    nos.iter().map(|n| n.por_grupo[g.indice()].media).collect(),
                )
            })
            .collect(),
        kind: ChartKind::Bars,
        ..Chart::default()
    }));

    let v = viloes(inv, 3);
    if !v.is_empty() {
        doc.heading(tx("fab-viloes"));
        doc.bullets(
            v.iter()
                .map(|x| {
                    fx(
                        &format!("vilao-{}", x.grupo.chave()),
                        &[
                            ("no", vx(&x.no)),
                            ("pct", vx(&num(100.0 * x.parcela_no, 0))),
                            ("pf", vx(&num(100.0 * x.parcela_fabrica, 0))),
                        ],
                    )
                })
                .collect(),
        );
    }

    if !inv.curva.is_empty() {
        doc.heading(tx("fab-curva_de_carga"));
        // A potência média em 48 faixas da janela inteira.
        let (a, b) = inv.janela_total;
        let k = 48;
        let passo = (b - a) / k as f64;
        let mut labels = Vec::new();
        let mut serie = Vec::new();
        for i in 0..k {
            let x0 = a + passo * i as f64;
            let x1 = x0 + passo;
            let mut e = 0.0;
            for (j, (t, kw)) in inv.curva.iter().enumerate() {
                let fim = inv.curva.get(j + 1).map_or(b, |x| x.0);
                let s = t.max(x0);
                let f = fim.min(x1);
                if f > s {
                    e += kw * (f - s);
                }
            }
            labels.push(if i % 6 == 0 {
                relogio.hora(x0)
            } else {
                String::new()
            });
            serie.push(e / passo);
        }
        doc.blocks.push(Block::Chart(Chart {
            title: tx("rel-potencia_media").into(),
            labels,
            series: vec![(tx("un-kw").into(), serie)],
            kind: ChartKind::Lines,
            ..Chart::default()
        }));
        if let Some((tp, kw)) = inv.pico() {
            doc.para(&fx(
                "fab-pico",
                &[
                    ("kw", vx(&num(kw, casas(kw)))),
                    ("hora", vx(&relogio.hora(tp))),
                ],
            ));
        }
    }

    // Os fatores, sempre, com as fontes.
    doc.heading(tx("rel-fatores"));
    let linhas: Vec<Vec<String>> = cat
        .factors
        .iter()
        .map(|f| {
            let unidade = cat
                .fluxo(&f.flow)
                .map(|x| x.unit.clone())
                .unwrap_or_default();
            let mut selos = Vec::new();
            if f.sem_fonte() {
                selos.push(tx("fat-sem_fonte").to_string());
            }
            if f.didactic {
                selos.push(tx("fat-didatico").to_lowercase());
            }
            vec![
                f.flow.clone(),
                f.scope.to_string(),
                f.kg_co2e.map_or(tx("rel-a_definir").to_string(), |k| {
                    format!("{} kg CO₂e/{unidade}", num(k, 4))
                }),
                f.source.clone(),
                f.year.map(|y| y.to_string()).unwrap_or_default(),
                selos.join(", "),
            ]
        })
        .collect();
    if linhas.is_empty() {
        doc.para(tx("rel-sem_fatores"));
    } else {
        tabela(
            &mut doc,
            tx("rel-fatores"),
            vec![
                tx("fat-fluxo").into(),
                tx("fat-escopo").into(),
                tx("fat-fator").into(),
                tx("fat-fonte").into(),
                tx("fat-ano").into(),
                String::new(),
            ],
            linhas,
        );
    }
    if !inv.pendencias.is_empty() {
        doc.heading(tx("fab-ainda_nao_contado"));
        doc.bullets(
            inv.pendencias
                .iter()
                .map(|x| fx(&format!("pend-{}", x.chave), &[("no", vx(&x.no))]))
                .collect(),
        );
    }
    doc
}

/// O modelo de página: a cor do app.
pub fn modelo_de_pagina() -> Template {
    Template {
        colour: crate::COR.into(),
        ..Template::default()
    }
}

/// Grava o relatório ao lado do modelo, em HTML (`pdf = false`) ou PDF.
pub fn gravar(p: &Projeto, inv: &Inventario, pdf: bool) -> Result<PathBuf, String> {
    let doc = documento(p, inv);
    let t = modelo_de_pagina();
    let base = p
        .caminho
        .file_stem()
        .map_or_else(|| "modelo".into(), |s| s.to_string_lossy().into_owned());
    let ext = if pdf { "pdf" } else { "html" };
    let destino = p
        .caminho
        .with_file_name(format!("{base}.relatorio_ambiental.{ext}"));
    let gerado = hoje();
    let bytes = if pdf {
        utsim_ferramentas::relatorios::pdf::render(&doc, &t, &gerado)
    } else {
        utsim_ferramentas::relatorios::html::render(&doc, &t, &gerado).into_bytes()
    };
    std::fs::write(&destino, bytes).map_err(|e| e.to_string())?;
    Ok(destino)
}

/// A tela Relatórios.
pub fn ui(ui: &mut egui::Ui, p: &Projeto, feito: &mut Option<PathBuf>) -> Option<(String, bool)> {
    let mut msg = None;
    ui.heading(tx("rel-tela"));
    ui.label(egui::RichText::new(tx("rel-explica")).color(colors::TEXT_WEAK));
    ui.add_space(8.0);
    let Some(inv) = &p.inventario else {
        ui.label(tx("fab-esperando_simulacao"));
        return None;
    };
    ui.horizontal(|ui| {
        for (pdf, rotulo) in [(false, tx("rel-gerar_html")), (true, tx("rel-gerar_pdf"))] {
            if ui.add(utsim_ui_tema::widgets::primary(rotulo)).clicked() {
                msg = Some(match gravar(p, inv, pdf) {
                    Ok(c) => {
                        let m = fx("rel-gravado", &[("arquivo", vx(&c.display()))]);
                        *feito = Some(c);
                        (m, false)
                    }
                    Err(e) => (fx("app-erro_salvar", &[("e", vx(&e))]), true),
                });
            }
        }
    });
    if let Some(c) = feito.clone() {
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label(c.display().to_string());
            if ui.button(tx("rel-mostrar")).clicked() {
                let _ = utsim_ui_tema::arquivos::mostrar_no_explorador(&c);
            }
        });
    }
    ui.add_space(10.0);
    ui.label(egui::RichText::new(tx("rel-conteudo")).strong());
    for h in documento(p, inv).headings() {
        ui.label(format!("• {h}"));
    }
    msg
}

#[cfg(test)]
mod testes {
    use super::*;

    /// O relatório sai com o aviso e a lista de fatores, em HTML e em PDF.
    #[test]
    fn relatorio_tem_aviso_e_fatores() {
        let pasta =
            std::env::temp_dir().join(format!("utsim-ambiental-rel-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&pasta);
        let c = crate::projeto::gravar_exemplo(&pasta).unwrap();
        let mut p = Projeto::abrir(&c).unwrap();
        p.simular_agora().unwrap();
        let inv = p.inventario.clone().unwrap();
        let doc = documento(&p, &inv);
        let texto = doc.plain_text();
        assert!(texto.contains(tx("rel-aviso")));
        assert!(doc.headings().contains(&tx("rel-fatores")));
        assert!(texto.contains("eletricidade"));
        let h = gravar(&p, &inv, false).unwrap();
        assert!(
            std::fs::read_to_string(&h)
                .unwrap()
                .contains("eletricidade")
        );
        let pdf = gravar(&p, &inv, true).unwrap();
        assert!(std::fs::read(&pdf).unwrap().starts_with(b"%PDF"));
        let _ = std::fs::remove_dir_all(&pasta);
    }
}
