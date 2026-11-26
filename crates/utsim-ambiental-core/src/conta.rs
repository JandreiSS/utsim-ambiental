//! A conta de energia e de emissão (fase AM1): horas em cada estado, energia por nó e por
//! grupo de estado, kWh por peça, energia sem valor agregado, emissão em CO₂ equivalente por
//! escopo e as métricas com nome (plano 2.2, 2.5 e 4.2.5).
//!
//! A conta é feita depois da rodada, sobre os relatórios das replicações: mudar uma potência
//! ou um fator refaz só a conta, sem simular de novo. Todo número é uma métrica por
//! replicação, com média e intervalo de 95 % entre as replicações.

use std::collections::BTreeMap;

use utsim_core::RunReport;
use utsim_core::stats::Summary;
use utsim_model::{Model, ObjectKind, SinkRole};

use crate::formato::{Driver, EnvironmentDef, EventDriver, Grupo, Lateral};

/// Um indicador entre replicações: média e meia largura do intervalo de 95 %.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Indicador {
    pub media: f64,
    /// `NaN` com uma replicação só.
    pub meia_largura: f64,
    pub n: usize,
}

impl Indicador {
    pub fn de(xs: &[f64]) -> Indicador {
        if xs.is_empty() {
            return Indicador {
                media: 0.0,
                meia_largura: f64::NAN,
                n: 0,
            };
        }
        let s = Summary::from_samples(xs);
        Indicador {
            media: s.mean,
            meia_largura: s.half_width,
            n: s.n,
        }
    }

    /// O intervalo existe (duas replicações ou mais).
    pub fn tem_intervalo(&self) -> bool {
        self.meia_largura.is_finite()
    }
}

/// Um número que depende de fator: o valor, ou "falta fator" com os fluxos sem fator.
#[derive(Clone, Debug, PartialEq)]
pub enum Estimativa {
    Valor(Indicador),
    FaltaFator(Vec<String>),
}

impl Estimativa {
    pub fn valor(&self) -> Option<Indicador> {
        match self {
            Estimativa::Valor(i) => Some(*i),
            Estimativa::FaltaFator(_) => None,
        }
    }
}

/// Uma linha do cartão "De onde veio": direcionador × coeficiente = quantidade.
#[derive(Clone, Debug, PartialEq)]
pub struct Linha {
    /// O que foi medido: um estado do motor (`"blocked"`), `"item"` ou o nome de um fluxo.
    pub origem: String,
    pub tipo: TipoLinha,
    /// O grupo de estado, quando a linha é um estado de um dos cinco grupos.
    pub grupo: Option<Grupo>,
    /// O direcionador médio (horas ou peças).
    pub direcionador: f64,
    /// O coeficiente do nó (kW, kWh por peça, quantidade por unidade).
    pub coeficiente: f64,
    /// A quantidade média (kWh ou a unidade do fluxo).
    pub quantidade: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TipoLinha {
    /// Horas no estado × kW.
    Estado,
    /// Peças × kWh por peça.
    EnergiaPorPeca,
    /// Um fluxo do catálogo × o direcionador.
    Fluxo,
}

/// O resultado de um nó.
#[derive(Clone, Debug, PartialEq)]
pub struct NoInventario {
    pub nome: String,
    pub etapa: Option<String>,
    /// O nó tem bloco ambiental com alguma potência ou fluxo.
    pub tem_bloco: bool,
    pub energia: Indicador,
    pub por_grupo: [Indicador; 5],
    pub sem_valor: Indicador,
    /// Energia por peça que saiu do nó; `None` se nenhuma peça saiu.
    pub kwh_por_peca: Option<Indicador>,
    pub pecas: Indicador,
    pub co2e: Estimativa,
    /// As contas, para o cartão "De onde veio".
    pub linhas: Vec<Linha>,
    /// Quantidade de cada fluxo do nó, na unidade do catálogo.
    pub fluxos: BTreeMap<String, Indicador>,
}

/// Algo que a versão atual ainda não conta (o app mostra, nunca esconde).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pendencia {
    pub no: String,
    /// `setup`, `partida`, `espera`, `refugo`, `retrabalho`, `distancia`, `volume`, `massa`.
    pub chave: &'static str,
}

impl Pendencia {
    pub const CHAVES: &'static [&'static str] = &[
        "setup",
        "partida",
        "espera",
        "refugo",
        "retrabalho",
        "distancia",
        "volume",
        "massa",
    ];
}

/// O inventário: o resultado de todos os nós, da fábrica e as métricas por replicação.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Inventario {
    pub nos: Vec<NoInventario>,
    pub replicacoes: usize,
    /// Métrica com nome → valor em cada replicação (plano 4.2.5).
    pub metricas: BTreeMap<String, Vec<f64>>,
    pub energia: Indicador,
    pub sem_valor: Indicador,
    pub co2e: Estimativa,
    /// Peças boas da fábrica: as que chegaram às saídas que não são de refugo.
    pub pecas_boas: Indicador,
    pub pendencias: Vec<Pendencia>,
    /// Os fatores usados que não têm fonte declarada.
    pub fatores_sem_fonte: Vec<String>,
    /// Os fatores usados marcados como didáticos.
    pub fatores_didaticos: Vec<String>,
}

impl Default for Estimativa {
    fn default() -> Self {
        Estimativa::FaltaFator(Vec::new())
    }
}

impl Inventario {
    pub fn no(&self, nome: &str) -> Option<&NoInventario> {
        self.nos.iter().find(|n| n.nome == nome)
    }

    pub fn metrica(&self, chave: &str) -> Option<Indicador> {
        self.metricas.get(chave).map(|v| Indicador::de(v))
    }
}

/// A diferença pareada `novo − base` de uma métrica, replicação a replicação (mesma
/// semente, mesmos números aleatórios). `None` se a métrica falta em um dos dois ou se o
/// número de replicações difere.
pub fn comparar(base: &Inventario, novo: &Inventario, chave: &str) -> Option<Indicador> {
    let a = base.metricas.get(chave)?;
    let b = novo.metricas.get(chave)?;
    if a.len() != b.len() || a.is_empty() {
        return None;
    }
    let d: Vec<f64> = a.iter().zip(b).map(|(x, y)| y - x).collect();
    Some(Indicador::de(&d))
}

/// Os objetos que podem ter bloco ambiental e aparecem no mapa: todos menos fontes e
/// saídas (que só contam peças).
pub fn consome(kind: &ObjectKind) -> bool {
    !matches!(kind, ObjectKind::Source { .. } | ObjectKind::Sink)
}

/// A conta inteira, sobre os relatórios das replicações.
pub fn avaliar(modelo: &Model, lateral: &Lateral, relatorios: &[RunReport]) -> Inventario {
    let cat = &lateral.environment;
    let hu = cat.clock.time_unit.em_horas();
    let eletrico = cat.fluxo_eletrico().map(|f| f.name.clone());
    let vazio = EnvironmentDef::default();
    let mut inv = Inventario {
        replicacoes: relatorios.len(),
        ..Inventario::default()
    };
    let mut met: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let push = |m: &mut BTreeMap<String, Vec<f64>>, k: String, x: f64| {
        m.entry(k).or_default().push(x);
    };
    let mut pendencias = Vec::new();
    let mut usados: Vec<String> = Vec::new();

    // Por replicação: total da fábrica, por escopo.
    let n = relatorios.len();
    let mut fab_kwh = vec![0.0; n];
    let mut fab_sem = vec![0.0; n];
    let mut fab_co2 = vec![0.0; n];
    let mut fab_escopo: BTreeMap<u8, Vec<f64>> = BTreeMap::new();
    let mut fab_falta: Vec<String> = Vec::new();

    for obj in modelo.objects.iter().filter(|o| consome(&o.kind)) {
        let def = lateral.objects.get(&obj.name);
        let d = def.unwrap_or(&vazio);
        let tem_bloco = def
            .is_some_and(|d| !d.power.vazia() || !d.energy_per.is_empty() || !d.flows.is_empty());
        let mut kwh = vec![0.0; n];
        let mut grupos = vec![[0.0f64; 5]; n];
        let mut pecas = vec![0.0; n];
        let mut fluxos: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        // Linhas do cartão: soma das replicações, dividida no fim.
        let mut est: BTreeMap<String, (f64, f64)> = BTreeMap::new(); // estado → (horas, kWh)
        let mut por_peca = (0.0, 0.0); // (peças, kWh)
        let mut fl_dir: BTreeMap<String, (f64, f64)> = BTreeMap::new(); // fluxo → (dir, qtd)

        for (r, rel) in relatorios.iter().enumerate() {
            let Some(b) = rel.blocks.iter().find(|b| b.name == obj.name) else {
                continue;
            };
            let janela = (rel.end_time - rel.stats_from).max(0.0) * hu;
            let horas = |estado: &str| b.states.get(estado).copied().unwrap_or(0.0) * janela;
            for (estado, frac) in &b.states {
                let h = frac * janela;
                let e = h * d.power.no_estado(estado);
                kwh[r] += e;
                if let Some(g) = Grupo::de_estado(estado) {
                    grupos[r][g.indice()] += e;
                }
                let l = est.entry(estado.clone()).or_default();
                l.0 += h;
                l.1 += e;
            }
            pecas[r] = b.output as f64;
            for ep in &d.energy_per {
                if ep.driver == EventDriver::Item {
                    let e = ep.kwh * b.input as f64;
                    kwh[r] += e;
                    grupos[r][Grupo::Produzindo.indice()] += e;
                    por_peca.0 += b.input as f64;
                    por_peca.1 += e;
                }
            }
            for f in &d.flows {
                let dir = match &f.driver {
                    Driver::Item => Some(b.input as f64),
                    Driver::GoodItem => Some(b.output as f64),
                    Driver::Time(estados) => Some(estados.iter().map(|e| horas(e.nome())).sum()),
                    _ => None,
                };
                if let Some(dir) = dir {
                    let q = f.amount * dir;
                    let v = fluxos.entry(f.flow.clone()).or_insert_with(|| vec![0.0; n]);
                    v[r] += q;
                    let l = fl_dir.entry(f.flow.clone()).or_default();
                    l.0 += dir;
                    l.1 += q;
                }
            }
        }

        // O que esta versão ainda não conta.
        for ep in &d.energy_per {
            match ep.driver {
                EventDriver::Setup => pendencias.push(Pendencia {
                    no: obj.name.clone(),
                    chave: "setup",
                }),
                EventDriver::Startup => pendencias.push(Pendencia {
                    no: obj.name.clone(),
                    chave: "partida",
                }),
                EventDriver::Item => {}
            }
        }
        for f in &d.flows {
            let chave = match f.driver {
                Driver::Scrap => "refugo",
                Driver::Rework => "retrabalho",
                Driver::Setup => "setup",
                Driver::Startup => "partida",
                Driver::Distance { .. } => "distancia",
                Driver::Volume => "volume",
                Driver::Mass => "massa",
                _ => continue,
            };
            pendencias.push(Pendencia {
                no: obj.name.clone(),
                chave,
            });
        }
        if d.standby.is_some() {
            pendencias.push(Pendencia {
                no: obj.name.clone(),
                chave: "espera",
            });
        }

        // Emissão do nó: a eletricidade e cada fluxo, com o fator do catálogo.
        let mut falta = Vec::new();
        let mut co2 = vec![0.0; n];
        let mut escopo: BTreeMap<u8, Vec<f64>> = BTreeMap::new();
        let mut emitir = |fluxo: &str, q: &[f64], falta: &mut Vec<String>| {
            if q.iter().all(|x| *x == 0.0) {
                return;
            }
            match cat.fator(fluxo) {
                Some(f) if f.kg_co2e.is_some() => {
                    let k = f.kg_co2e.unwrap_or(0.0);
                    let s = escopo.entry(f.scope).or_insert_with(|| vec![0.0; n]);
                    for r in 0..n {
                        co2[r] += q[r] * k;
                        s[r] += q[r] * k;
                    }
                    if !usados.iter().any(|u| u == fluxo) {
                        usados.push(fluxo.to_string());
                    }
                }
                _ => falta.push(fluxo.to_string()),
            }
        };
        match &eletrico {
            Some(e) => emitir(e, &kwh, &mut falta),
            None if kwh.iter().any(|x| *x != 0.0) => falta.push("eletricidade".into()),
            None => {}
        }
        for (f, q) in &fluxos {
            emitir(f, q, &mut falta);
        }

        // Métricas com nome.
        if tem_bloco {
            let no = &obj.name;
            for r in 0..n {
                push(&mut met, format!("{no}.energia_kwh"), kwh[r]);
                let sem: f64 = Grupo::TODOS
                    .iter()
                    .filter(|g| g.sem_valor())
                    .map(|g| grupos[r][g.indice()])
                    .sum();
                push(&mut met, format!("{no}.energia_sem_valor_kwh"), sem);
                for g in Grupo::TODOS {
                    push(
                        &mut met,
                        format!("{no}.energia_kwh.{}", g.chave()),
                        grupos[r][g.indice()],
                    );
                }
                if pecas[r] > 0.0 {
                    push(&mut met, format!("{no}.kwh_por_peca"), kwh[r] / pecas[r]);
                }
                for (f, q) in &fluxos {
                    push(&mut met, format!("{no}.fluxo.{f}"), q[r]);
                }
                if falta.is_empty() {
                    push(&mut met, format!("{no}.co2e_kg"), co2[r]);
                }
                fab_kwh[r] += kwh[r];
                fab_sem[r] += sem;
                fab_co2[r] += co2[r];
            }
            for (s, v) in &escopo {
                let t = fab_escopo.entry(*s).or_insert_with(|| vec![0.0; n]);
                for r in 0..n {
                    t[r] += v[r];
                }
            }
            fab_falta.extend(falta.iter().cloned());
        }

        let rn = n.max(1) as f64;
        let mut linhas: Vec<Linha> = est
            .into_iter()
            .filter(|(_, (h, _))| *h > 0.0)
            .map(|(estado, (h, e))| Linha {
                grupo: Grupo::de_estado(&estado),
                coeficiente: d.power.no_estado(&estado),
                origem: estado,
                tipo: TipoLinha::Estado,
                direcionador: h / rn,
                quantidade: e / rn,
            })
            .collect();
        linhas.sort_by(|a, b| b.quantidade.total_cmp(&a.quantidade));
        if por_peca.0 > 0.0 {
            linhas.push(Linha {
                origem: "item".into(),
                tipo: TipoLinha::EnergiaPorPeca,
                grupo: Some(Grupo::Produzindo),
                direcionador: por_peca.0 / rn,
                coeficiente: por_peca.1 / por_peca.0,
                quantidade: por_peca.1 / rn,
            });
        }
        for (f, (dir, q)) in fl_dir {
            linhas.push(Linha {
                coeficiente: if dir > 0.0 { q / dir } else { 0.0 },
                origem: f,
                tipo: TipoLinha::Fluxo,
                grupo: None,
                direcionador: dir / rn,
                quantidade: q / rn,
            });
        }

        let sem_valor: Vec<f64> = grupos
            .iter()
            .map(|g| {
                Grupo::TODOS
                    .iter()
                    .filter(|x| x.sem_valor())
                    .map(|x| g[x.indice()])
                    .sum()
            })
            .collect();
        let kpp: Vec<f64> = (0..n)
            .filter(|r| pecas[*r] > 0.0)
            .map(|r| kwh[r] / pecas[r])
            .collect();
        falta.sort();
        falta.dedup();
        inv.nos.push(NoInventario {
            nome: obj.name.clone(),
            etapa: d.stage.clone(),
            tem_bloco,
            energia: Indicador::de(&kwh),
            por_grupo: Grupo::TODOS
                .map(|g| Indicador::de(&grupos.iter().map(|x| x[g.indice()]).collect::<Vec<_>>())),
            sem_valor: Indicador::de(&sem_valor),
            kwh_por_peca: (!kpp.is_empty()).then(|| Indicador::de(&kpp)),
            pecas: Indicador::de(&pecas),
            co2e: if falta.is_empty() {
                Estimativa::Valor(Indicador::de(&co2))
            } else {
                Estimativa::FaltaFator(falta)
            },
            linhas,
            fluxos: fluxos
                .iter()
                .map(|(f, v)| (f.clone(), Indicador::de(v)))
                .collect(),
        });
    }

    // Peças boas: as que chegam às saídas que não são de refugo.
    let boas: Vec<f64> = relatorios
        .iter()
        .map(|rel| {
            modelo
                .objects
                .iter()
                .filter(|o| {
                    matches!(o.kind, ObjectKind::Sink) && o.sink_role != Some(SinkRole::Scrap)
                })
                .filter_map(|o| rel.blocks.iter().find(|b| b.name == o.name))
                .map(|b| b.input as f64)
                .sum()
        })
        .collect();

    for r in 0..n {
        push(&mut met, "ambiental.energia_kwh".into(), fab_kwh[r]);
        push(
            &mut met,
            "ambiental.energia_sem_valor_kwh".into(),
            fab_sem[r],
        );
        if boas[r] > 0.0 {
            push(
                &mut met,
                "ambiental.kwh_por_peca".into(),
                fab_kwh[r] / boas[r],
            );
        }
        if fab_falta.is_empty() {
            push(&mut met, "ambiental.co2e_kg".into(), fab_co2[r]);
            for (s, v) in &fab_escopo {
                push(&mut met, format!("ambiental.co2e_kg.escopo{s}"), v[r]);
            }
        }
    }
    fab_falta.sort();
    fab_falta.dedup();
    pendencias.sort();
    pendencias.dedup();

    inv.energia = Indicador::de(&fab_kwh);
    inv.sem_valor = Indicador::de(&fab_sem);
    inv.co2e = if fab_falta.is_empty() {
        Estimativa::Valor(Indicador::de(&fab_co2))
    } else {
        Estimativa::FaltaFator(fab_falta)
    };
    inv.pecas_boas = Indicador::de(&boas);
    inv.metricas = met;
    inv.pendencias = pendencias;
    for f in &usados {
        if let Some(x) = cat.fator(f) {
            if x.sem_fonte() {
                inv.fatores_sem_fonte.push(f.clone());
            }
            if x.didactic {
                inv.fatores_didaticos.push(f.clone());
            }
        }
    }
    inv
}

/// Um "vilão": um nó que gasta energia sem agregar valor.
#[derive(Clone, Debug, PartialEq)]
pub struct Vilao {
    pub no: String,
    /// O grupo sem valor que mais pesa no nó.
    pub grupo: Grupo,
    /// A parcela da energia do nó gasta nesse grupo (0 a 1).
    pub parcela_no: f64,
    /// A parcela da energia da fábrica gasta sem valor neste nó (0 a 1).
    pub parcela_fabrica: f64,
}

/// Os `n` nós com mais energia sem valor agregado, do maior para o menor.
pub fn viloes(inv: &Inventario, n: usize) -> Vec<Vilao> {
    let total = inv.energia.media;
    let mut v: Vec<&NoInventario> = inv
        .nos
        .iter()
        .filter(|x| x.tem_bloco && x.sem_valor.media > 0.0 && x.energia.media > 0.0)
        .collect();
    v.sort_by(|a, b| b.sem_valor.media.total_cmp(&a.sem_valor.media));
    v.into_iter()
        .take(n)
        .map(|x| {
            let g = Grupo::TODOS
                .into_iter()
                .filter(|g| g.sem_valor())
                .max_by(|a, b| {
                    x.por_grupo[a.indice()]
                        .media
                        .total_cmp(&x.por_grupo[b.indice()].media)
                })
                .unwrap_or(Grupo::Esperando);
            Vilao {
                no: x.nome.clone(),
                grupo: g,
                parcela_no: x.por_grupo[g.indice()].media / x.energia.media,
                parcela_fabrica: if total > 0.0 {
                    x.sem_valor.media / total
                } else {
                    0.0
                },
            }
        })
        .collect()
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::formato::{Direction, FlowKind, TimeUnit};
    use crate::formato::{EnergyPer, EnvironmentCatalog, Estado, Factor, FlowDef, FlowUse};
    use utsim_core::{BlockReport, TallySummary};

    fn bloco(
        nome: &str,
        kind: &str,
        input: u64,
        output: u64,
        estados: &[(&str, f64)],
    ) -> BlockReport {
        BlockReport {
            name: nome.into(),
            kind: kind.into(),
            input,
            output,
            content_now: 0,
            content_mean: 0.0,
            content_max: 0.0,
            staytime: TallySummary {
                count: 0,
                mean: 0.0,
                std_dev: 0.0,
                min: 0.0,
                max: 0.0,
                histogram: None,
                observations: None,
            },
            states: estados.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
            extra: BTreeMap::new(),
            custom: BTreeMap::new(),
            series: Vec::new(),
        }
    }

    fn rel(r: u32, fim: f64, blocos: Vec<BlockReport>) -> RunReport {
        RunReport {
            replication: r,
            seed: 1,
            stats_from: 0.0,
            end_time: fim,
            events: 0,
            blocks: blocos,
        }
    }

    fn modelo() -> Model {
        Model::parse(
            r#"Model(name: "m", run: (stop_time: 7200.0, warmup: 0.0, seed: 1, replications: 2),
               objects: [(name: "chegada", kind: Source(interarrival: Const(60.0))),
                         (name: "forno", kind: Processor(process: Const(30.0))),
                         (name: "prensa", kind: Processor(process: Const(30.0))),
                         (name: "fim", kind: Sink)],
               connections: [("chegada", "forno"), ("forno", "prensa"), ("prensa", "fim")])"#,
        )
        .unwrap()
    }

    /// Caso resolvido à mão: 2 horas (7200 s); o forno fica 60 % processando a 10 kW e
    /// 40 % esperando a 4 kW: 0,6×2×10 + 0,4×2×4 = 12 + 3,2 = 15,2 kWh, dos quais 3,2 sem
    /// valor. 100 peças saem: 0,152 kWh por peça. Com 0,5 kg CO₂e/kWh: 7,6 kg.
    #[test]
    fn caso_a_mao_um_no() {
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        l.environment.factors[0].kg_co2e = Some(0.5);
        let mut d = EnvironmentDef::default();
        d.power.producing = 10.0;
        d.power.waiting = 4.0;
        l.objects.insert("forno".into(), d);
        let r = rel(
            0,
            7200.0,
            vec![
                bloco(
                    "forno",
                    "processor",
                    100,
                    100,
                    &[("processing", 0.6), ("idle", 0.4)],
                ),
                bloco("fim", "sink", 100, 0, &[]),
            ],
        );
        let inv = avaliar(&modelo(), &l, &[r]);
        let f = inv.no("forno").unwrap();
        assert!((f.energia.media - 15.2).abs() < 1e-9);
        assert!((f.sem_valor.media - 3.2).abs() < 1e-9);
        assert!((f.kwh_por_peca.unwrap().media - 0.152).abs() < 1e-12);
        assert!((f.co2e.valor().unwrap().media - 7.6).abs() < 1e-9);
        assert!((inv.metrica("ambiental.co2e_kg.escopo2").unwrap().media - 7.6).abs() < 1e-9);
        assert!((inv.metrica("ambiental.kwh_por_peca").unwrap().media - 0.152).abs() < 1e-12);
        // Sem fonte declarada, o fator aparece na lista.
        assert_eq!(inv.fatores_sem_fonte, vec!["eletricidade".to_string()]);
    }

    /// `by_state` vale no lugar do grupo, a unidade de tempo muda as horas e a energia por
    /// peça entra no grupo produzindo.
    #[test]
    fn por_estado_unidade_e_energia_por_peca() {
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        l.environment.clock.time_unit = TimeUnit::Minute;
        let mut d = EnvironmentDef::default();
        d.power.waiting = 2.0;
        d.power.by_state.insert("blocked".into(), 8.0);
        d.energy_per.push(EnergyPer {
            driver: EventDriver::Item,
            kwh: 0.5,
        });
        l.objects.insert("prensa".into(), d);
        // 120 minutos = 2 h: idle 50 % (1 h × 2 kW) e blocked 50 % (1 h × 8 kW) = 10 kWh,
        // mais 10 peças × 0,5 = 5 kWh: 15 kWh.
        let r = rel(
            0,
            120.0,
            vec![bloco(
                "prensa",
                "processor",
                10,
                10,
                &[("idle", 0.5), ("blocked", 0.5)],
            )],
        );
        let inv = avaliar(&modelo(), &l, &[r]);
        let p = inv.no("prensa").unwrap();
        assert!((p.energia.media - 15.0).abs() < 1e-9);
        assert!((p.por_grupo[Grupo::Produzindo.indice()].media - 5.0).abs() < 1e-9);
        assert!((p.sem_valor.media - 10.0).abs() < 1e-9);
        // Fator a definir: falta fator, nunca zero.
        assert_eq!(p.co2e, Estimativa::FaltaFator(vec!["eletricidade".into()]));
        assert!(inv.metrica("ambiental.co2e_kg").is_none());
    }

    /// Conservação (a soma dos nós é a fábrica, a soma dos grupos é o nó), um fluxo de gás
    /// por tempo com escopo 1 e a diferença pareada entre duas replicações.
    #[test]
    fn conservacao_fluxo_e_pareado() {
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        l.environment.factors[0].kg_co2e = Some(0.1);
        l.environment.flows.push(FlowDef {
            name: "gas".into(),
            kind: FlowKind::Fuel,
            unit: "m3".into(),
            direction: Direction::In,
            destination: None,
        });
        l.environment.factors.push(Factor {
            flow: "gas".into(),
            scope: 1,
            kg_co2e: Some(2.0),
            source: "teste".into(),
            year: None,
            range: None,
            didactic: true,
        });
        let mut forno = EnvironmentDef::default();
        forno.power.producing = 10.0;
        forno.power.waiting = 5.0;
        forno.flows.push(FlowUse {
            flow: "gas".into(),
            amount: 3.0,
            driver: Driver::Time(vec![Estado::Processing]),
            adds_mass: false,
            removes_mass: false,
        });
        let mut prensa = EnvironmentDef::default();
        prensa.power.producing = 4.0;
        prensa.power.off_shift = 1.0;
        l.objects.insert("forno".into(), forno);
        l.objects.insert("prensa".into(), prensa);
        let reps: Vec<RunReport> = [0.5, 0.7]
            .iter()
            .enumerate()
            .map(|(i, p)| {
                rel(
                    i as u32,
                    3600.0,
                    vec![
                        bloco(
                            "forno",
                            "processor",
                            10,
                            10,
                            &[("processing", *p), ("idle", 1.0 - p)],
                        ),
                        bloco(
                            "prensa",
                            "processor",
                            10,
                            10,
                            &[("processing", *p), ("off_shift", 1.0 - p)],
                        ),
                        bloco("fim", "sink", 10, 0, &[]),
                    ],
                )
            })
            .collect();
        let inv = avaliar(&modelo(), &l, &reps);
        let soma: f64 = inv.nos.iter().map(|n| n.energia.media).sum();
        assert!((soma - inv.energia.media).abs() < 1e-9);
        for n in &inv.nos {
            let g: f64 = n.por_grupo.iter().map(|x| x.media).sum();
            assert!((g - n.energia.media).abs() < 1e-9);
        }
        // Gás: 3 m3/h × 0,5 h e × 0,7 h → média 1,8 m3; 3,6 kg CO₂e, escopo 1.
        let gas = inv.no("forno").unwrap().fluxos["gas"];
        assert!((gas.media - 1.8).abs() < 1e-9);
        assert!((inv.metrica("ambiental.co2e_kg.escopo1").unwrap().media - 3.6).abs() < 1e-9);
        assert_eq!(inv.fatores_didaticos, vec!["gas".to_string()]);

        // Baixar a espera do forno de 5 para 1 kW: a diferença pareada é exata.
        let mut l2 = l.clone();
        l2.objects.get_mut("forno").unwrap().power.waiting = 1.0;
        let inv2 = avaliar(&modelo(), &l2, &reps);
        let d = comparar(&inv, &inv2, "forno.energia_kwh").unwrap();
        // Espera: 0,5 h e 0,3 h → −4×0,5 = −2 e −4×0,3 = −1,2; média −1,6.
        assert!((d.media + 1.6).abs() < 1e-9);
        assert!(d.tem_intervalo());

        let v = viloes(&inv, 3);
        assert_eq!(v[0].no, "forno");
        assert_eq!(v[0].grupo, Grupo::Esperando);
    }

    /// Monotonia: subir uma potência nunca baixa a energia.
    #[test]
    fn monotonia() {
        let mut l = Lateral::default();
        let mut d = EnvironmentDef::default();
        d.power.waiting = 1.0;
        l.objects.insert("forno".into(), d);
        let r = rel(
            0,
            3600.0,
            vec![bloco("forno", "processor", 1, 1, &[("idle", 1.0)])],
        );
        let a = avaliar(&modelo(), &l, std::slice::from_ref(&r))
            .energia
            .media;
        l.objects.get_mut("forno").unwrap().power.waiting = 2.0;
        let b = avaliar(&modelo(), &l, &[r]).energia.media;
        assert!(b >= a);
    }

    /// A conta sobre uma rodada de verdade: a mesma semente dá a mesma saída.
    #[test]
    fn reprodutivel_por_semente() {
        let m = modelo();
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        let mut d = EnvironmentDef::default();
        d.power.producing = 10.0;
        d.power.waiting = 3.0;
        l.objects.insert("forno".into(), d);
        let a = avaliar(&m, &l, &m.run_replications().unwrap());
        let b = avaliar(&m, &l, &m.run_replications().unwrap());
        assert_eq!(a.metricas, b.metricas);
        // O forno trabalha 30 s a cada 60 s: perto de metade do tempo produzindo.
        let f = a.no("forno").unwrap();
        assert!(f.energia.media > 0.0);
        assert!(f.por_grupo[Grupo::Produzindo.indice()].media > 0.0);
    }
}
