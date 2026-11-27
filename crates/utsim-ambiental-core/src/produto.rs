//! A pegada por produto (fase AM7, plano 2.7 e 3.3): energia, água e CO₂ equivalente de
//! cada tipo de peça, por unidade funcional, com a regra de rateio do catálogo.
//!
//! Rateio, nó a nó, em cada replicação:
//!
//! 1. O que tem direcionador por peça (energia por peça e fluxos por peça) vai direto para o
//!    tipo das peças que entraram no nó.
//! 2. O resto do nó (energia por tempo e por evento, fluxos por tempo) é rateado pela regra:
//!    tempo de permanência de cada tipo no nó (padrão), peças, massa, ou, com `None`
//!    (estrutura), só a parte produzindo e preparando vai aos produtos (pelo tempo) e a
//!    espera, a parada e o fora de turno ficam como **estrutura**.
//! 3. O impacto das peças refugadas fica no tipo delas, e a pegada por peça boa o carrega.
//! 4. O que não é de nenhum nó (tratamento de efluente, sucata e lodo) fica na estrutura.
//!
//! A soma dos produtos mais a estrutura é sempre o total da fábrica, qualquer que seja a
//! regra: a regra muda os produtos, nunca o total.
//!
//! Fronteira: **portão a portão** é a fabricação (energia, combustível, água, resíduo);
//! **berço ao portão** soma os materiais comprados (fluxos do tipo material, com o fator do
//! catálogo). Uso e fim de vida ficam fora.

use std::collections::BTreeMap;

use utsim_model::Model;

use crate::balanco::{Entrada, fator_do_fluxo};
use crate::conta::Inventario;
use crate::formato::{Allocation, Driver, EventDriver, FlowKind, Grupo, Lateral};
use crate::materiais::em_m3;

/// A pegada de um tipo de peça, com os valores de cada replicação (o app divide pela
/// unidade funcional e tira o intervalo).
#[derive(Clone, Debug, PartialEq)]
pub struct Pegada {
    /// 0: peças sem tipo; senão a posição em `item_types` (a partir de 1).
    pub tipo: u32,
    pub nome: String,
    pub boas: Vec<f64>,
    pub massa_boa_kg: Vec<f64>,
    /// Peças deste tipo que foram para o refugo.
    pub refugadas: Vec<f64>,
    pub kwh: Vec<f64>,
    pub agua_m3: Vec<f64>,
    /// CO₂e da fabricação e dos materiais; `Err` com os fatores que faltam.
    pub co2_fabricacao: Result<Vec<f64>, Vec<String>>,
    pub co2_materiais: Result<Vec<f64>, Vec<String>>,
    /// Energia que as peças refugadas deste tipo levaram consigo.
    pub mochila_kwh: Vec<f64>,
    /// Por nó: energia e CO₂e da fabricação (médias), para a contribuição.
    pub por_no: Vec<(String, f64, Option<f64>)>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResumoProdutos {
    pub regra: Allocation,
    pub pegadas: Vec<Pegada>,
    /// O que ficou fora dos produtos (por replicação).
    pub estrutura_kwh: Vec<f64>,
    pub estrutura_co2: Option<Vec<f64>>,
}

/// Por nó: energia e fluxos (médias) de um tipo.
type PorNo = BTreeMap<usize, (f64, BTreeMap<String, f64>)>;

/// O nome de um tipo de peça.
pub fn nome_do_tipo(modelo: &Model, tipo: u32) -> String {
    tipo.checked_sub(1)
        .and_then(|k| modelo.item_types.get(k as usize))
        .map_or_else(|| "*".to_string(), |t| t.name.clone())
}

/// Os pesos de cada tipo num nó pela regra.
fn pesos(regra: Allocation, pt: &BTreeMap<u32, crate::materiais::PorTipo>) -> BTreeMap<u32, f64> {
    pt.iter()
        .map(|(t, x)| {
            let w = match regra {
                Allocation::ByItems => x.entradas,
                Allocation::ByMass => x.massa,
                Allocation::ByProcessTime | Allocation::Structure => x.tempo,
            };
            (*t, w)
        })
        .collect()
}

/// Reparte `valor` pelos pesos; devolve o que não foi repartido (sem peso nenhum).
fn repartir(valor: f64, w: &BTreeMap<u32, f64>, saida: &mut BTreeMap<u32, f64>) -> f64 {
    let soma: f64 = w.values().sum();
    if soma <= 0.0 || valor == 0.0 {
        return valor;
    }
    for (t, x) in w {
        *saida.entry(*t).or_default() += valor * x / soma;
    }
    0.0
}

/// Calcula a pegada de cada tipo de peça (só com rastro).
pub(crate) fn calcular(
    modelo: &Model,
    e: &Entrada,
    grupos_no: &[Vec<[f64; 5]>],
    kwh_fabrica: &[f64],
    inv: &mut Inventario,
) {
    let l: &Lateral = e.lateral;
    let cat = &l.environment;
    let n = e.kwh_no.len();
    if n == 0 || e.mats.iter().any(Option::is_none) {
        return;
    }
    let regra = cat.allocation;
    let eletrico = cat.fluxo_eletrico().map(|f| f.name.clone());
    let fator_e = eletrico.as_deref().map(|x| fator_do_fluxo(l, x));
    let tipo_fluxo = |f: &str| cat.fluxo(f).map(|x| (x.kind, x.unit.clone()));

    // Por tipo, por replicação: kWh, fluxos (quantidade) e por nó.
    let mut kwh: BTreeMap<u32, Vec<f64>> = BTreeMap::new();
    let mut fluxos: BTreeMap<u32, BTreeMap<String, Vec<f64>>> = BTreeMap::new();
    let mut por_no: BTreeMap<u32, PorNo> = BTreeMap::new();
    let mut estrutura_kwh = vec![0.0; n];
    let mut estrutura_fl: BTreeMap<String, Vec<f64>> = BTreeMap::new();

    for (r, m) in e.mats.iter().enumerate() {
        let Some(m) = m else { continue };
        for (i, nome) in e.nomes.iter().enumerate() {
            let pt = &m.por_tipo[i];
            let total = e.kwh_no[r][i];
            let fl = e.fluxos_no.get(&i);
            if total == 0.0 && fl.is_none() {
                continue;
            }
            let def = l.objects.get(nome);
            let ent: BTreeMap<u32, f64> = pt.iter().map(|(t, x)| (*t, x.entradas)).collect();
            let w = pesos(regra, pt);
            let mut parte: BTreeMap<u32, f64> = BTreeMap::new();
            // 1. Energia por peça: direto.
            let kwh_peca = def.map_or(0.0, |d| {
                d.energy_per
                    .iter()
                    .filter(|x| x.driver == EventDriver::Item)
                    .map(|x| x.kwh)
                    .sum::<f64>()
            });
            let direto: f64 = ent.values().sum::<f64>() * kwh_peca;
            for (t, c) in &ent {
                *parte.entry(*t).or_default() += c * kwh_peca;
            }
            // 2. O resto pela regra (com `Structure`, a espera, a parada e o fora de turno
            // ficam na estrutura).
            let resto = (total - direto).max(0.0);
            let sem_valor: f64 = if regra == Allocation::Structure {
                let g = &grupos_no[r][i];
                Grupo::TODOS
                    .iter()
                    .filter(|x| x.sem_valor())
                    .map(|x| g[x.indice()])
                    .sum::<f64>()
                    .min(resto)
            } else {
                0.0
            };
            estrutura_kwh[r] += sem_valor;
            estrutura_kwh[r] += repartir(resto - sem_valor, &w, &mut parte);
            for (t, v) in parte {
                kwh.entry(t).or_insert_with(|| vec![0.0; n])[r] += v;
                por_no.entry(t).or_default().entry(i).or_default().0 += v / n as f64;
            }
            // Fluxos: os por peça direto, os outros pela regra.
            if let (Some(fl), Some(d)) = (fl, def) {
                for (f, q) in fl {
                    let por_peca: f64 = d
                        .flows
                        .iter()
                        .filter(|x| &x.flow == f && matches!(x.driver, Driver::Item))
                        .map(|x| x.amount)
                        .sum();
                    let mut p2: BTreeMap<u32, f64> = BTreeMap::new();
                    let dir: f64 = ent.values().sum::<f64>() * por_peca;
                    for (t, c) in &ent {
                        *p2.entry(*t).or_default() += c * por_peca;
                    }
                    let sobra = repartir((q[r] - dir).max(0.0), &w, &mut p2);
                    estrutura_fl
                        .entry(f.clone())
                        .or_insert_with(|| vec![0.0; n])[r] += sobra;
                    for (t, v) in p2 {
                        fluxos
                            .entry(t)
                            .or_default()
                            .entry(f.clone())
                            .or_insert_with(|| vec![0.0; n])[r] += v;
                        *por_no
                            .entry(t)
                            .or_default()
                            .entry(i)
                            .or_default()
                            .1
                            .entry(f.clone())
                            .or_default() += v / n as f64;
                    }
                }
            }
        }
    }
    // O que não é de nenhum nó (tratamento) também é estrutura.
    for r in 0..n {
        let nos: f64 = e.kwh_no[r].iter().sum();
        estrutura_kwh[r] += (kwh_fabrica[r] - nos).max(0.0);
    }

    // CO₂e de cada tipo.
    let co2 = |kw: &[f64], fl: &BTreeMap<String, Vec<f64>>| {
        let mut fab = vec![0.0; n];
        let mut mat = vec![0.0; n];
        let mut falta_fab = Vec::new();
        let mut falta_mat = Vec::new();
        if kw.iter().any(|x| *x > 0.0) {
            match &fator_e {
                Some(Ok((k, _))) => {
                    for r in 0..n {
                        fab[r] += kw[r] * k;
                    }
                }
                Some(Err(x)) => falta_fab.push(x.clone()),
                None => falta_fab.push("eletricidade".into()),
            }
        }
        for (f, q) in fl {
            if q.iter().all(|x| *x == 0.0) {
                continue;
            }
            let material = matches!(tipo_fluxo(f).map(|x| x.0), Some(FlowKind::Material));
            match fator_do_fluxo(l, f) {
                Ok((k, _)) => {
                    let alvo = if material { &mut mat } else { &mut fab };
                    for r in 0..n {
                        alvo[r] += q[r] * k;
                    }
                }
                Err(x) => {
                    if material {
                        falta_mat.push(x)
                    } else {
                        falta_fab.push(x)
                    }
                }
            }
        }
        falta_fab.sort();
        falta_fab.dedup();
        falta_mat.sort();
        falta_mat.dedup();
        (
            if falta_fab.is_empty() {
                Ok(fab)
            } else {
                Err(falta_fab)
            },
            if falta_mat.is_empty() {
                Ok(mat)
            } else {
                Err(falta_mat)
            },
        )
    };
    let agua = |fl: &BTreeMap<String, Vec<f64>>| -> Vec<f64> {
        let mut a = vec![0.0; n];
        for (f, q) in fl {
            if let Some((FlowKind::Water, u)) = tipo_fluxo(f)
                && let Some(k) = em_m3(&u)
            {
                for r in 0..n {
                    a[r] += q[r] * k;
                }
            }
        }
        a
    };

    // A mochila de cada tipo: a energia média por peça dos nós por onde passou.
    let mut mochila: BTreeMap<u32, Vec<f64>> = BTreeMap::new();
    for (r, m) in e.mats.iter().enumerate() {
        let Some(m) = m else { continue };
        for (c, t) in m.caminhos_refugo.iter().zip(&m.tipos_refugo) {
            let v = mochila.entry(*t).or_insert_with(|| vec![0.0; n]);
            for i in c {
                let ent = e.entradas_no[r].get(*i).copied().unwrap_or(0.0);
                if ent > 0.0 {
                    v[r] += e.kwh_no[r][*i] / ent;
                }
            }
        }
    }

    let mut tipos: Vec<u32> = kwh.keys().copied().collect();
    for m in e.mats.iter().flatten() {
        tipos.extend(m.boas_por_tipo.keys().copied());
    }
    tipos.sort_unstable();
    tipos.dedup();
    let vazio = BTreeMap::new();
    let mut pegadas = Vec::new();
    for t in tipos {
        let k = kwh.get(&t).cloned().unwrap_or_else(|| vec![0.0; n]);
        let fl = fluxos.get(&t).unwrap_or(&vazio);
        let (fab, mat) = co2(&k, fl);
        let col = |f: &dyn Fn(&crate::materiais::Materiais) -> f64| -> Vec<f64> {
            e.mats.iter().map(|m| m.as_ref().map_or(0.0, f)).collect()
        };
        let mut contrib: Vec<(String, f64, Option<f64>)> = por_no
            .get(&t)
            .map(|x| {
                x.iter()
                    .map(|(i, (kw, fls))| {
                        let c = match &fator_e {
                            Some(Ok((fe, _))) => {
                                let mut c = kw * fe;
                                let mut ok = true;
                                for (f, q) in fls {
                                    if matches!(
                                        tipo_fluxo(f).map(|x| x.0),
                                        Some(FlowKind::Material)
                                    ) {
                                        continue;
                                    }
                                    match fator_do_fluxo(l, f) {
                                        Ok((kf, _)) => c += q * kf,
                                        Err(_) => ok = false,
                                    }
                                }
                                ok.then_some(c)
                            }
                            _ => None,
                        };
                        (e.nomes[*i].clone(), *kw, c)
                    })
                    .collect()
            })
            .unwrap_or_default();
        contrib.sort_by(|a, b| b.1.total_cmp(&a.1));
        pegadas.push(Pegada {
            tipo: t,
            nome: nome_do_tipo(modelo, t),
            boas: col(&|m| m.boas_por_tipo.get(&t).map_or(0.0, |x| x.0)),
            massa_boa_kg: col(&|m| m.boas_por_tipo.get(&t).map_or(0.0, |x| x.1)),
            refugadas: col(&|m| m.refugadas_por_tipo.get(&t).copied().unwrap_or(0.0)),
            agua_m3: agua(fl),
            kwh: k,
            co2_fabricacao: fab,
            co2_materiais: mat,
            mochila_kwh: mochila.remove(&t).unwrap_or_else(|| vec![0.0; n]),
            por_no: contrib,
        });
    }
    let (efab, emat) = co2(&estrutura_kwh, &estrutura_fl);
    let estrutura_co2 = match (efab, emat) {
        (Ok(a), Ok(b)) => {
            // Mais o CO₂e da fábrica que não é de nenhum nó (sucata, lodo, tratamento já
            // está na energia): a diferença entre o total e a soma dos nós.
            let mut v: Vec<f64> = a.iter().zip(&b).map(|(x, y)| x + y).collect();
            if let Some(tot) = inv.metricas.get("ambiental.co2e_kg") {
                let soma_nos: Vec<f64> = (0..n)
                    .map(|r| {
                        e.nomes
                            .iter()
                            .filter_map(|nm| {
                                inv.metricas.get(&format!("{nm}.co2e_kg")).map(|x| x[r])
                            })
                            .sum()
                    })
                    .collect();
                let trat: Vec<f64> = match (&fator_e, inv.metricas.get("ambiental.tratamento_kwh"))
                {
                    (Some(Ok((k, _))), Some(t)) => t.iter().map(|x| x * k).collect(),
                    _ => vec![0.0; n],
                };
                for r in 0..n {
                    v[r] += (tot[r] - soma_nos[r] - trat[r]).max(0.0);
                }
            }
            Some(v)
        }
        _ => None,
    };
    inv.produtos = Some(ResumoProdutos {
        regra,
        pegadas,
        estrutura_kwh,
        estrutura_co2,
    });
}

#[cfg(test)]
mod testes {
    use crate::formato::{
        Allocation, EnergyPer, EnvironmentCatalog, EnvironmentDef, EventDriver, Lateral,
    };
    use utsim_model::Model;

    /// Dois tipos de peça: produtos + estrutura = fábrica em toda regra; a regra muda os
    /// produtos (pela massa, o tipo mais pesado leva mais) e a energia por peça vai direto.
    #[test]
    fn rateio_conserva_o_total() {
        let m = Model::parse(
            r#"Model(name: "m", run: (stop_time: 14400.0, warmup: 600.0, seed: 8, replications: 3),
               item_types: [(name: "leve"), (name: "pesada")],
               objects: [(name: "chegada", kind: Source(interarrival: Exp(mean: 60.0),
                            item_type_mix: [("leve", 0.5), ("pesada", 0.5)])),
                         (name: "forno", kind: Processor(process: Exp(mean: 30.0))),
                         (name: "prensa", kind: Processor(process: Exp(mean: 20.0))),
                         (name: "fim", kind: Sink)],
               connections: [("chegada", "forno"), ("forno", "prensa"), ("prensa", "fim")])"#,
        )
        .unwrap();
        let rod = crate::rodar::simular(&m).unwrap();
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        l.environment.factors[0].kg_co2e = Some(0.5);
        l.environment.item_mass_kg.insert("leve".into(), 1.0);
        l.environment.item_mass_kg.insert("pesada".into(), 3.0);
        let mut forno = EnvironmentDef::default();
        forno.power.producing = 10.0;
        forno.power.waiting = 4.0;
        let mut prensa = EnvironmentDef::default();
        prensa.power.producing = 5.0;
        prensa.energy_per.push(EnergyPer {
            driver: EventDriver::Item,
            kwh: 0.2,
        });
        l.objects.insert("forno".into(), forno);
        l.objects.insert("prensa".into(), prensa);
        let mut por_regra = Vec::new();
        for regra in [
            Allocation::ByProcessTime,
            Allocation::ByItems,
            Allocation::ByMass,
            Allocation::Structure,
        ] {
            l.environment.allocation = regra;
            let inv = crate::avaliar(&m, &l, &rod, None);
            let p = inv.produtos.as_ref().unwrap();
            assert_eq!(p.pegadas.len(), 2);
            for r in 0..rod.len() {
                let soma: f64 =
                    p.pegadas.iter().map(|x| x.kwh[r]).sum::<f64>() + p.estrutura_kwh[r];
                let fab = inv.metricas["ambiental.energia_kwh"][r];
                assert!(
                    (soma - fab).abs() < 1e-6 * fab,
                    "{regra:?}: {soma} != {fab}"
                );
                // E o CO₂e também fecha.
                let c: f64 = p
                    .pegadas
                    .iter()
                    .map(|x| x.co2_fabricacao.as_ref().unwrap()[r])
                    .sum::<f64>()
                    + p.estrutura_co2.as_ref().unwrap()[r];
                let fc = inv.metricas["ambiental.co2e_kg"][r];
                assert!((c - fc).abs() < 1e-6 * fc, "{regra:?}: {c} != {fc}");
            }
            if regra != Allocation::Structure {
                let e: f64 = p.estrutura_kwh.iter().sum();
                assert!(e.abs() < 1e-6, "{regra:?}: estrutura {e}");
            }
            por_regra.push(p.pegadas[1].kwh.iter().sum::<f64>());
        }
        // Pela massa, a pesada (3 kg) leva mais que pelo número de peças.
        assert!(por_regra[2] > por_regra[1]);
    }
}
