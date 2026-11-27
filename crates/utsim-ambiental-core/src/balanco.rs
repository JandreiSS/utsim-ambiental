//! Água, efluentes, resíduos, sucata, a mochila da peça refugada e o custo dos fluxos
//! (fase AM5, plano 2.3, 2.4 e 2.6), por cima da conta de energia.
//!
//! - **Água.** A entrada do nó é a soma dos fluxos do tipo água (em L ou m³). Com o
//!   balanço do nó: `efluente = entrada × (1 − incorporada − evaporada)`; a fração `reuse`
//!   do efluente volta como entrada de `reuse_to`. Captação da fábrica = entradas − reuso.
//! - **Efluente.** `carga (kg) = volume (m³) × concentração (mg/L) ÷ 1000`. O destino do
//!   catálogo tira a fração `removal` de cada parâmetro e soma energia (kWh/m³, que entra na
//!   energia da fábrica), lodo (kg/m³, resíduo sem destino) e custo (por m³).
//! - **Resíduos.** Os fluxos do tipo resíduo (kg) vão para o destino do fluxo; a sucata vai
//!   para `scrap_destination`. Cada destino tem fator (escopo 3), custo e a marca de
//!   "desviado de aterro". A emissão do resíduo de processo entra no nó; a da sucata e a do
//!   lodo, só na fábrica.
//! - **Mochila do refugo.** O que cada peça refugada levou consigo: a energia média por peça
//!   de cada nó por onde passou (energia do nó ÷ peças que entraram) mais os fluxos por
//!   peça desses nós, e a emissão disso.

use std::collections::BTreeMap;

use crate::conta::{Estimativa, Indicador, Inventario};
use crate::formato::{Driver, FlowKind, Lateral};
use crate::materiais::{Materiais, em_m3};

/// O destino dos resíduos sem destino no catálogo.
pub const SEM_DESTINO: &str = "sem_destino";

/// A carga de um parâmetro do efluente da fábrica.
#[derive(Clone, Debug, PartialEq)]
pub struct Carga {
    pub parametro: String,
    pub antes_kg: Indicador,
    pub depois_kg: Indicador,
    /// Concentração da mistura depois do tratamento, em mg/L.
    pub concentracao: Indicador,
    pub limite: Option<f64>,
    pub fonte_limite: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResumoAgua {
    pub captacao_m3: Indicador,
    pub consumo_m3: Indicador,
    pub reuso_pct: Indicador,
    pub efluente_m3: Indicador,
    pub cargas: Vec<Carga>,
    pub tratamento_kwh: Indicador,
    pub lodo_kg: Indicador,
}

/// A água de um nó (médias entre replicações), em m³: o que entrou e para onde foi.
#[derive(Clone, Debug, PartialEq)]
pub struct AguaNo {
    pub indice: usize,
    pub nome: String,
    pub entrada: f64,
    pub evaporada: f64,
    pub incorporada: f64,
    pub reuso: f64,
    pub reuso_para: Option<String>,
    pub efluente: f64,
    /// O destino do efluente do nó (`None`: sem tratamento cadastrado).
    pub destino: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResumoResiduos {
    /// `(destino, kg, desviado de aterro)`.
    pub por_destino: Vec<(String, Indicador, bool)>,
    pub total_kg: Indicador,
    pub desvio_pct: Indicador,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResumoMateriais {
    pub entrada_kg: Indicador,
    pub boa_kg: Indicador,
    pub sucata_kg: Indicador,
    pub refugadas: Indicador,
    pub rendimento_pct: Indicador,
    pub erro_pct: Indicador,
    pub mochila_kwh: Indicador,
    pub mochila_co2e: Estimativa,
    /// A mochila de cada peça refugada da primeira replicação, na janela (fase AM13: o
    /// cartão da peça na cena 3D). A soma dá a mochila da replicação.
    pub pecas: Vec<MochilaPeca>,
}

/// O que uma peça refugada levou consigo: a energia média por peça e os fluxos por peça
/// de cada nó por onde passou.
#[derive(Clone, Debug, PartialEq)]
pub struct MochilaPeca {
    /// A peça (o `item` do rastro).
    pub item: u64,
    /// Os nós por onde passou (posição do objeto no modelo), na ordem.
    pub caminho: Vec<usize>,
    pub kwh: f64,
    /// Quantidade de cada fluxo por peça somada no caminho.
    pub fluxos: BTreeMap<String, f64>,
    /// `Err`: os fatores que faltam.
    pub co2e_kg: Result<f64, Vec<String>>,
}

/// O que a conta de energia passa para o balanço.
pub(crate) struct Entrada<'a> {
    pub lateral: &'a Lateral,
    pub nomes: &'a [String],
    pub mats: &'a [Option<Materiais>],
    /// Por nó (posição do objeto): fluxo → quantidade em cada replicação.
    pub fluxos_no: &'a BTreeMap<usize, BTreeMap<String, Vec<f64>>>,
    /// Por replicação e nó: energia e peças que entraram.
    pub kwh_no: &'a [Vec<f64>],
    pub entradas_no: &'a [Vec<f64>],
}

/// Os totais da fábrica que o balanço completa.
pub(crate) struct Fabrica<'a> {
    pub met: &'a mut BTreeMap<String, Vec<f64>>,
    pub kwh: &'a mut [f64],
    pub co2: &'a mut [f64],
    pub escopo: &'a mut BTreeMap<u8, Vec<f64>>,
    pub falta: &'a mut Vec<String>,
}

fn push(m: &mut BTreeMap<String, Vec<f64>>, k: String, x: f64) {
    m.entry(k).or_default().push(x);
}

/// O fator (kg CO₂e por unidade, escopo) de um fluxo; o de um resíduo é o do destino dele.
pub(crate) fn fator_do_fluxo(l: &Lateral, fluxo: &str) -> Result<(f64, u8), String> {
    let cat = &l.environment;
    match cat.fluxo(fluxo) {
        Some(f) if f.kind == FlowKind::Waste => {
            let dest = f.destination.as_deref().unwrap_or(SEM_DESTINO);
            fator_do_destino(l, dest).ok_or_else(|| format!("{fluxo} → {dest}"))
        }
        _ => cat
            .fator(fluxo)
            .and_then(|f| f.kg_co2e.map(|k| (k, f.scope)))
            .ok_or_else(|| fluxo.to_string()),
    }
}

fn fator_do_destino(l: &Lateral, dest: &str) -> Option<(f64, u8)> {
    l.environment
        .waste_destinations
        .iter()
        .find(|d| d.name == dest)
        .and_then(|d| d.kg_co2e)
        .map(|k| (k, 3))
}

/// Completa o inventário com água, efluentes, resíduos, materiais e custos.
pub(crate) fn completar(e: &Entrada, fab: &mut Fabrica, inv: &mut Inventario) {
    let l = e.lateral;
    let cat = &l.environment;
    let n = e.kwh_no.len();
    if n == 0 {
        return;
    }
    let eletrico = cat.fluxo_eletrico().map(|f| f.name.clone());
    let fator_eletrico = eletrico
        .as_deref()
        .and_then(|x| cat.fator(x))
        .and_then(|f| f.kg_co2e.map(|k| (k, f.scope)));
    let tipo = |f: &str| cat.fluxo(f).map(|x| x.kind);

    let mut usou_agua = false;
    let mut usou_residuo = false;
    // Por replicação.
    let mut agua = vec![0.0; n];
    let mut reuso = vec![0.0; n];
    let mut efluente = vec![0.0; n];
    let mut tratamento = vec![0.0; n];
    let mut lodo = vec![0.0; n];
    let mut antes: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let mut depois: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let mut destinos: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let mut custo = vec![0.0; n];
    let mut tem_custo = false;
    // Por nó, para o painel.
    let mut no_agua: BTreeMap<usize, Vec<f64>> = BTreeMap::new();
    let mut no_efl: BTreeMap<usize, Vec<f64>> = BTreeMap::new();
    let mut no_res: BTreeMap<usize, Vec<f64>> = BTreeMap::new();
    // Por nó: entrada, evaporada, incorporada, reuso e efluente somados nas replicações.
    let mut no_det: BTreeMap<usize, [f64; 5]> = BTreeMap::new();

    for (i, fluxos) in e.fluxos_no {
        let nome = &e.nomes[*i];
        let def = l.objects.get(nome);
        for r in 0..n {
            let mut a = 0.0;
            let mut res = 0.0;
            for (f, q) in fluxos {
                let fd = cat.fluxo(f);
                match tipo(f) {
                    Some(FlowKind::Water) => {
                        if let Some(k) = fd.and_then(|x| em_m3(&x.unit)) {
                            a += q[r] * k;
                        }
                    }
                    Some(FlowKind::Waste) => {
                        res += q[r];
                        let dest = fd
                            .and_then(|x| x.destination.clone())
                            .unwrap_or_else(|| SEM_DESTINO.into());
                        destinos.entry(dest).or_insert_with(|| vec![0.0; n])[r] += q[r];
                    }
                    _ => {}
                }
                if let Some(p) = cat.prices.get(f) {
                    custo[r] += q[r] * p;
                    tem_custo = true;
                }
            }
            if a > 0.0 {
                usou_agua = true;
                let w = def.and_then(|d| d.water.clone()).unwrap_or_default();
                let efl = a * (1.0 - w.incorporated - w.evaporated).max(0.0);
                let reusado = if w.reuse_to.is_some() {
                    efl * w.reuse
                } else {
                    0.0
                };
                let efl = efl - reusado;
                agua[r] += a;
                reuso[r] += reusado;
                efluente[r] += efl;
                let det = no_det.entry(*i).or_default();
                det[0] += a;
                det[1] += a * w.evaporated;
                det[2] += a * w.incorporated;
                det[3] += reusado;
                det[4] += efl;
                no_agua.entry(*i).or_insert_with(|| vec![0.0; n])[r] = a;
                no_efl.entry(*i).or_insert_with(|| vec![0.0; n])[r] = efl;
                if let Some(ed) = def.and_then(|d| d.effluent.as_ref()) {
                    let dest = cat
                        .effluent
                        .destinations
                        .iter()
                        .find(|x| x.name == ed.destination);
                    for (p, c) in &ed.concentration {
                        let kg = efl * c / 1000.0;
                        let fica =
                            1.0 - dest.and_then(|x| x.removal.get(p)).copied().unwrap_or(0.0);
                        antes.entry(p.clone()).or_insert_with(|| vec![0.0; n])[r] += kg;
                        depois.entry(p.clone()).or_insert_with(|| vec![0.0; n])[r] += kg * fica;
                        push(fab.met, format!("{nome}.carga.{p}_kg"), kg);
                    }
                    if let Some(x) = dest {
                        tratamento[r] += efl * x.kwh_per_m3;
                        lodo[r] += efl * x.sludge_kg_per_m3;
                        if let Some(cm) = x.cost_per_m3 {
                            custo[r] += efl * cm;
                            tem_custo = true;
                        }
                    }
                }
                push(fab.met, format!("{nome}.agua_m3"), a);
                push(fab.met, format!("{nome}.efluente_m3"), efl);
            }
            if res > 0.0 {
                usou_residuo = true;
                no_res.entry(*i).or_insert_with(|| vec![0.0; n])[r] = res;
                push(fab.met, format!("{nome}.residuo_kg"), res);
            }
        }
    }

    // Sucata e lodo: resíduos da fábrica.
    let destino_sucata = cat
        .scrap_destination
        .clone()
        .unwrap_or_else(|| SEM_DESTINO.into());
    let mut sucata = vec![0.0; n];
    for r in 0..n {
        if let Some(m) = &e.mats[r] {
            sucata[r] = m.sucata_total_kg;
        }
        if sucata[r] > 0.0 {
            destinos
                .entry(destino_sucata.clone())
                .or_insert_with(|| vec![0.0; n])[r] += sucata[r];
            usou_residuo = true;
        }
        if lodo[r] > 0.0 {
            destinos
                .entry(SEM_DESTINO.into())
                .or_insert_with(|| vec![0.0; n])[r] += lodo[r];
            usou_residuo = true;
        }
    }
    // Emissão e custo da sucata e do lodo (o resíduo de processo já está no nó).
    let extra: Vec<(String, Vec<f64>)> = vec![
        (destino_sucata.clone(), sucata.clone()),
        (SEM_DESTINO.to_string(), lodo.clone()),
    ];
    for (dest, kg) in &extra {
        if kg.iter().all(|x| *x == 0.0) {
            continue;
        }
        match fator_do_destino(l, dest) {
            Some((k, s)) => {
                let sv = fab.escopo.entry(s).or_insert_with(|| vec![0.0; n]);
                for r in 0..n {
                    fab.co2[r] += kg[r] * k;
                    sv[r] += kg[r] * k;
                }
            }
            None => fab.falta.push(format!("{} → {dest}", "sucata/lodo")),
        }
    }
    for (dest, kg) in &destinos {
        if let Some(c) = cat
            .waste_destinations
            .iter()
            .find(|d| &d.name == dest)
            .and_then(|d| d.cost)
        {
            for r in 0..n {
                custo[r] += kg[r] * c;
            }
            tem_custo = true;
        }
    }
    // Energia do tratamento de efluente: entra na energia da fábrica.
    if tratamento.iter().any(|x| *x > 0.0) {
        match fator_eletrico {
            Some((k, s)) => {
                let sv = fab.escopo.entry(s).or_insert_with(|| vec![0.0; n]);
                for r in 0..n {
                    fab.co2[r] += tratamento[r] * k;
                    sv[r] += tratamento[r] * k;
                }
            }
            None => fab
                .falta
                .push(eletrico.clone().unwrap_or_else(|| "eletricidade".into())),
        }
        for (k, t) in fab.kwh.iter_mut().zip(&tratamento) {
            *k += t;
        }
    }
    // Preço da eletricidade, quando o catálogo tem (sem tarifa).
    if let Some(e) = &eletrico
        && cat.tariff.is_none()
        && let Some(p) = cat.prices.get(e)
    {
        for (c, k) in custo.iter_mut().zip(fab.kwh.iter()) {
            *c += k * p;
        }
        tem_custo = true;
    }

    // Por nó, no inventário.
    for no in &mut inv.nos {
        let Some(i) = e.nomes.iter().position(|x| x == &no.nome) else {
            continue;
        };
        no.agua_m3 = no_agua.get(&i).map(|v| Indicador::de(v));
        no.efluente_m3 = no_efl.get(&i).map(|v| Indicador::de(v));
        no.residuo_kg = no_res.get(&i).map(|v| Indicador::de(v));
        let suc: Vec<f64> = e
            .mats
            .iter()
            .filter_map(|m| m.as_ref().map(|m| m.sucata_kg[i]))
            .collect();
        if suc.iter().any(|x| *x > 0.0) {
            no.sucata_kg = Some(Indicador::de(&suc));
            for x in &suc {
                push(fab.met, format!("{}.sucata_kg", no.nome), *x);
            }
        }
    }

    let nr = n as f64;
    inv.agua_nos = no_det
        .iter()
        .map(|(i, d)| {
            let def = l.objects.get(&e.nomes[*i]);
            AguaNo {
                indice: *i,
                nome: e.nomes[*i].clone(),
                entrada: d[0] / nr,
                evaporada: d[1] / nr,
                incorporada: d[2] / nr,
                reuso: d[3] / nr,
                reuso_para: def
                    .and_then(|x| x.water.as_ref())
                    .and_then(|w| w.reuse_to.clone()),
                efluente: d[4] / nr,
                destino: def
                    .and_then(|x| x.effluent.as_ref())
                    .map(|x| x.destination.clone()),
            }
        })
        .collect();
    // A massa que passou entre os nós (média das replicações com rastro).
    let com_rastro: Vec<&Materiais> = e.mats.iter().flatten().collect();
    if !com_rastro.is_empty() {
        let k = com_rastro.len() as f64;
        for m in &com_rastro {
            for (par, kg) in &m.transferido {
                *inv.transferencias.entry(*par).or_default() += kg / k;
            }
        }
    }

    if usou_agua {
        let captacao: Vec<f64> = (0..n).map(|r| (agua[r] - reuso[r]).max(0.0)).collect();
        let consumo: Vec<f64> = (0..n).map(|r| captacao[r] - efluente[r]).collect();
        let pct: Vec<f64> = (0..n)
            .map(|r| {
                if agua[r] > 0.0 {
                    100.0 * reuso[r] / agua[r]
                } else {
                    0.0
                }
            })
            .collect();
        for r in 0..n {
            push(fab.met, "ambiental.agua_m3".into(), captacao[r]);
            push(fab.met, "ambiental.reuso_pct".into(), pct[r]);
            push(fab.met, "ambiental.efluente_m3".into(), efluente[r]);
            push(fab.met, "ambiental.tratamento_kwh".into(), tratamento[r]);
        }
        let mut cargas = Vec::new();
        for (p, a) in &antes {
            let d = &depois[p];
            let conc: Vec<f64> = (0..n)
                .map(|r| {
                    if efluente[r] > 0.0 {
                        d[r] * 1000.0 / efluente[r]
                    } else {
                        0.0
                    }
                })
                .collect();
            for r in 0..n {
                push(fab.met, format!("ambiental.carga.{p}_kg"), a[r]);
                push(fab.met, format!("ambiental.carga_tratada.{p}_kg"), d[r]);
            }
            let par = cat.effluent.parameters.iter().find(|x| &x.name == p);
            cargas.push(Carga {
                parametro: p.clone(),
                antes_kg: Indicador::de(a),
                depois_kg: Indicador::de(d),
                concentracao: Indicador::de(&conc),
                limite: par.and_then(|x| x.limit),
                fonte_limite: par.map(|x| x.limit_source.clone()).unwrap_or_default(),
            });
        }
        inv.agua = Some(ResumoAgua {
            captacao_m3: Indicador::de(&captacao),
            consumo_m3: Indicador::de(&consumo),
            reuso_pct: Indicador::de(&pct),
            efluente_m3: Indicador::de(&efluente),
            cargas,
            tratamento_kwh: Indicador::de(&tratamento),
            lodo_kg: Indicador::de(&lodo),
        });
    }

    if usou_residuo {
        let total: Vec<f64> = (0..n)
            .map(|r| destinos.values().map(|v| v[r]).sum())
            .collect();
        let desviado = |d: &str| {
            cat.waste_destinations
                .iter()
                .find(|x| x.name == d)
                .is_some_and(|x| x.diverted)
        };
        let desvio: Vec<f64> = (0..n)
            .map(|r| {
                let d: f64 = destinos
                    .iter()
                    .filter(|(k, _)| desviado(k))
                    .map(|(_, v)| v[r])
                    .sum();
                if total[r] > 0.0 {
                    100.0 * d / total[r]
                } else {
                    0.0
                }
            })
            .collect();
        for r in 0..n {
            for (d, v) in &destinos {
                push(fab.met, format!("ambiental.residuo_kg.{d}"), v[r]);
            }
            push(fab.met, "ambiental.desvio_aterro_pct".into(), desvio[r]);
        }
        inv.residuos = Some(ResumoResiduos {
            por_destino: destinos
                .iter()
                .map(|(d, v)| (d.clone(), Indicador::de(v), desviado(d)))
                .collect(),
            total_kg: Indicador::de(&total),
            desvio_pct: Indicador::de(&desvio),
        });
    }

    // Massa e mochila do refugo (só com rastro).
    if e.mats.iter().all(Option::is_some) && !cat.item_mass_kg.is_empty()
        || e.mats
            .iter()
            .flatten()
            .any(|m| m.sucata_total_kg > 0.0 || !m.caminhos_refugo.is_empty())
    {
        // Fluxos por peça de cada nó (para a mochila).
        let por_peca: Vec<Vec<(String, f64)>> = e
            .nomes
            .iter()
            .map(|nome| {
                l.objects.get(nome).map_or_else(Vec::new, |d| {
                    d.flows
                        .iter()
                        .filter(|f| matches!(f.driver, Driver::Item))
                        .map(|f| (f.flow.clone(), f.amount))
                        .collect()
                })
            })
            .collect();
        let mut moch_kwh = vec![0.0; n];
        let mut moch_co2 = vec![0.0; n];
        let mut moch_falta: Vec<String> = Vec::new();
        for r in 0..n {
            let Some(m) = &e.mats[r] else {
                continue;
            };
            let mut fl: BTreeMap<&str, f64> = BTreeMap::new();
            for c in &m.caminhos_refugo {
                for i in c {
                    let ent = e.entradas_no[r].get(*i).copied().unwrap_or(0.0);
                    if ent > 0.0 {
                        moch_kwh[r] += e.kwh_no[r][*i] / ent;
                    }
                    for (f, q) in &por_peca[*i] {
                        *fl.entry(f.as_str()).or_default() += q;
                    }
                }
            }
            if moch_kwh[r] > 0.0 {
                match fator_eletrico {
                    Some((k, _)) => moch_co2[r] += moch_kwh[r] * k,
                    None => {
                        moch_falta.push(eletrico.clone().unwrap_or_else(|| "eletricidade".into()))
                    }
                }
            }
            for (f, q) in fl {
                match fator_do_fluxo(l, f) {
                    Ok((k, _)) => moch_co2[r] += q * k,
                    Err(x) => moch_falta.push(x),
                }
            }
        }
        moch_falta.sort();
        moch_falta.dedup();
        // A mochila de cada peça refugada da primeira replicação (mesma conta, peça a peça).
        let mut pecas: Vec<MochilaPeca> = Vec::new();
        if let Some(Some(m)) = e.mats.first() {
            for (c, item) in m.caminhos_refugo.iter().zip(&m.itens_refugo) {
                let mut kwh = 0.0;
                let mut fl: BTreeMap<String, f64> = BTreeMap::new();
                for i in c {
                    let ent = e.entradas_no[0].get(*i).copied().unwrap_or(0.0);
                    if ent > 0.0 {
                        kwh += e.kwh_no[0][*i] / ent;
                    }
                    for (f, q) in &por_peca[*i] {
                        *fl.entry(f.clone()).or_default() += q;
                    }
                }
                let mut co2 = 0.0;
                let mut falta: Vec<String> = Vec::new();
                if kwh > 0.0 {
                    match fator_eletrico {
                        Some((k, _)) => co2 += kwh * k,
                        None => {
                            falta.push(eletrico.clone().unwrap_or_else(|| "eletricidade".into()))
                        }
                    }
                }
                for (f, q) in &fl {
                    match fator_do_fluxo(l, f) {
                        Ok((k, _)) => co2 += q * k,
                        Err(x) => falta.push(x),
                    }
                }
                falta.sort();
                falta.dedup();
                pecas.push(MochilaPeca {
                    item: *item,
                    caminho: c.clone(),
                    kwh,
                    fluxos: fl,
                    co2e_kg: if falta.is_empty() {
                        Ok(co2)
                    } else {
                        Err(falta)
                    },
                });
            }
        }
        let col = |f: &dyn Fn(&Materiais) -> f64| -> Vec<f64> {
            e.mats.iter().flatten().map(f).collect()
        };
        for m in e.mats.iter().flatten() {
            push(
                fab.met,
                "ambiental.rendimento_material_pct".into(),
                m.rendimento_pct(),
            );
            push(
                fab.met,
                "ambiental.balanco_massa_erro_pct".into(),
                m.erro_pct(),
            );
        }
        for r in 0..n {
            push(fab.met, "ambiental.mochila_refugo.kwh".into(), moch_kwh[r]);
            if moch_falta.is_empty() {
                push(
                    fab.met,
                    "ambiental.mochila_refugo.co2e_kg".into(),
                    moch_co2[r],
                );
            }
        }
        inv.materiais = Some(ResumoMateriais {
            entrada_kg: Indicador::de(&col(&|m| m.entrada())),
            boa_kg: Indicador::de(&col(&|m| m.boa_kg)),
            sucata_kg: Indicador::de(&col(&|m| m.sucata_total_kg)),
            refugadas: Indicador::de(&col(&|m| m.caminhos_refugo.len() as f64)),
            rendimento_pct: Indicador::de(&col(&|m| m.rendimento_pct())),
            erro_pct: Indicador::de(&col(&|m| m.erro_pct())),
            mochila_kwh: Indicador::de(&moch_kwh),
            mochila_co2e: if moch_falta.is_empty() {
                Estimativa::Valor(Indicador::de(&moch_co2))
            } else {
                Estimativa::FaltaFator(moch_falta)
            },
            pecas,
        });
    }

    if tem_custo {
        for c in &custo {
            push(fab.met, "ambiental.custo".into(), *c);
        }
        inv.custo = Some(Indicador::de(&custo));
    }
}
