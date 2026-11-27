//! A rodada com rastro (fase AM3): as replicações do modelo, com um observador que guarda
//! só o que a conta ambiental usa: as mudanças de estado de cada bloco, a passagem das
//! peças de um bloco para outro e o tipo de cada peça.
//!
//! O observador só lê: a simulação dá os mesmos números com ou sem ele (o relatório de cada
//! replicação é o mesmo de `Model::run_replications`).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use utsim_core::{Observer, RunReport, Time, TraceEvent};
use utsim_model::Model;

/// Uma passagem: a peça `item` entrou em `para` vinda de `de` (`None`: criada ali).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Passagem {
    pub t: Time,
    pub item: u64,
    pub de: Option<usize>,
    pub para: usize,
}

/// O que a conta usa do rastro de uma replicação.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Rastro {
    /// Por bloco (na ordem dos objetos do modelo): `(instante, estado)` a cada mudança.
    pub estados: Vec<Vec<(Time, &'static str)>>,
    /// Todas as passagens, em ordem de tempo.
    pub passagens: Vec<Passagem>,
    /// O tipo de cada peça (1 = o primeiro de `item_types`), quando o modelo tem tipos.
    pub tipos: HashMap<u64, u32>,
}

/// Uma replicação: o relatório e, quando gravado, o rastro.
#[derive(Clone, Debug)]
pub struct Rodada {
    pub relatorio: RunReport,
    pub rastro: Option<Rastro>,
}

impl Rodada {
    /// Uma rodada só com o relatório (sem rastro): a conta usa as frações de tempo.
    pub fn so_relatorio(relatorio: RunReport) -> Rodada {
        Rodada {
            relatorio,
            rastro: None,
        }
    }
}

struct Gravador {
    rastro: Arc<Mutex<Rastro>>,
}

impl Observer for Gravador {
    fn on_event(&mut self, t: Time, e: &TraceEvent) {
        let Ok(mut r) = self.rastro.lock() else {
            return;
        };
        match e {
            TraceEvent::State { block, state } => {
                if r.estados.len() <= *block {
                    r.estados.resize(block + 1, Vec::new());
                }
                r.estados[*block].push((t, state.name()));
            }
            TraceEvent::Created { item, block } | TraceEvent::Unpacked { item, block } => {
                r.passagens.push(Passagem {
                    t,
                    item: *item,
                    de: None,
                    para: *block,
                });
            }
            TraceEvent::Moved { item, from, to } => r.passagens.push(Passagem {
                t,
                item: *item,
                de: Some(*from),
                para: *to,
            }),
            TraceEvent::ItemType { item, item_type } => {
                r.tipos.insert(*item, *item_type);
            }
            _ => {}
        }
    }
}

/// Roda uma replicação do modelo com rastro: a mesma de [`simular`] com o mesmo número
/// (mesma semente), para quem mostra uma replicação só (a cena 3D).
pub fn simular_uma(modelo: &Model, replicacao: u32) -> Result<Rodada, String> {
    modelo.validate().map_err(|e| e.to_string())?;
    uma(modelo, &modelo.nav_set(), replicacao)
}

fn uma(modelo: &Model, nav: &utsim_model::NavSet, r: u32) -> Result<Rodada, String> {
    let rastro = Arc::new(Mutex::new(Rastro::default()));
    let mut sim = modelo.build_with(r, nav);
    sim.set_observer(Box::new(Gravador {
        rastro: rastro.clone(),
    }));
    let relatorio = modelo.drive(&mut sim).map_err(|e| e.to_string())?;
    drop(sim);
    let rastro = Arc::try_unwrap(rastro)
        .ok()
        .and_then(|m| m.into_inner().ok())
        .unwrap_or_default();
    Ok(Rodada {
        relatorio,
        rastro: Some(rastro),
    })
}

/// Roda as replicações do modelo com rastro, em paralelo (uma thread por replicação, no
/// máximo o número de núcleos).
pub fn simular(modelo: &Model) -> Result<Vec<Rodada>, String> {
    modelo.validate().map_err(|e| e.to_string())?;
    let n = modelo.run.replications.max(1);
    let nav = modelo.nav_set();
    let uma = |r: u32| uma(modelo, &nav, r);
    let nucleos = std::thread::available_parallelism().map_or(2, |x| x.get()) as u32;
    let mut saida: Vec<Option<Result<Rodada, String>>> = (0..n).map(|_| None).collect();
    for lote in (0..n).collect::<Vec<_>>().chunks(nucleos.max(1) as usize) {
        let feitos: Vec<(u32, Result<Rodada, String>)> = std::thread::scope(|s| {
            let h: Vec<_> = lote
                .iter()
                .map(|r| {
                    let r = *r;
                    s.spawn(move || (r, uma(r)))
                })
                .collect();
            h.into_iter()
                .map(|x| {
                    x.join()
                        .unwrap_or_else(|_| (0, Err("a replicação parou".into())))
                })
                .collect()
        });
        for (r, x) in feitos {
            saida[r as usize] = Some(x);
        }
    }
    saida
        .into_iter()
        .map(|x| x.unwrap_or_else(|| Err("replicação perdida".into())))
        .collect()
}

#[cfg(test)]
mod testes {
    use super::*;

    /// O observador não muda a simulação: o relatório é o mesmo de `run_replications`.
    #[test]
    fn rastro_nao_muda_o_relatorio() {
        let m = Model::parse(
            r#"Model(name: "m", run: (stop_time: 3600.0, warmup: 300.0, seed: 3, replications: 3),
               objects: [(name: "a", kind: Source(interarrival: Exp(mean: 60.0))),
                         (name: "b", kind: Processor(process: Exp(mean: 40.0))),
                         (name: "c", kind: Sink)],
               connections: [("a", "b"), ("b", "c")])"#,
        )
        .unwrap();
        let com = simular(&m).unwrap();
        let sem = m.run_replications().unwrap();
        assert_eq!(com.len(), 3);
        for (x, y) in com.iter().zip(&sem) {
            assert_eq!(x.relatorio.blocks[1].input, y.blocks[1].input);
            assert_eq!(x.relatorio.blocks[1].states, y.blocks[1].states);
            let r = x.rastro.as_ref().unwrap();
            assert!(!r.estados[1].is_empty());
            assert!(!r.passagens.is_empty());
        }
    }
}
