//! A conta de energia e de emissão (fases AM1 e AM3): horas em cada estado, energia por nó
//! e por grupo de estado, kWh por peça, energia sem valor agregado, energia por setup e por
//! partida, desligamento em espera, emissão em CO₂ equivalente por escopo, a curva de carga
//! e as métricas com nome (plano 2.2, 2.5 e 4.2.5).
//!
//! A conta é feita depois da rodada, sobre o relatório e o rastro das replicações: mudar uma
//! potência, um fator ou a janela de tempo refaz só a conta, sem simular de novo. Todo
//! número é uma métrica por replicação, com média e intervalo de 95 % entre as replicações.

use std::collections::BTreeMap;

use utsim_core::Time;
use utsim_core::stats::Summary;
use utsim_model::{Model, ObjectKind, SinkRole};

use crate::formato::{Driver, EnvironmentDef, EventDriver, Grupo, Lateral};
use crate::medida::{self, Trecho, janela_da_rodada, medir, recorte};
use crate::rodar::Rodada;
use crate::tarifa::{Conta, Intervalo, demanda};

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

impl Default for Estimativa {
    fn default() -> Self {
        Estimativa::FaltaFator(Vec::new())
    }
}

/// Uma linha do cartão "De onde veio": direcionador × coeficiente = quantidade.
#[derive(Clone, Debug, PartialEq)]
pub struct Linha {
    /// O que foi medido: um estado do motor (`"blocked"`), `"item"`, `"setup"`, `"partida"`
    /// ou o nome de um fluxo.
    pub origem: String,
    pub tipo: TipoLinha,
    /// O grupo de estado, quando a linha é de energia.
    pub grupo: Option<Grupo>,
    /// O direcionador médio (horas ou eventos).
    pub direcionador: f64,
    /// O coeficiente do nó (kW, kWh por evento, quantidade por unidade).
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
    /// Setups × kWh por setup.
    EnergiaPorSetup,
    /// Partidas × kWh por partida.
    EnergiaPorPartida,
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
    /// Água que entrou no nó e efluente que saiu, em m³ (fase AM5).
    pub agua_m3: Option<Indicador>,
    pub efluente_m3: Option<Indicador>,
    /// Resíduo de processo e sucata atribuída ao nó, em kg.
    pub residuo_kg: Option<Indicador>,
    pub sucata_kg: Option<Indicador>,
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
    /// A janela da conta e a janela de estatística inteira, da primeira replicação.
    pub janela: (Time, Time),
    pub janela_total: (Time, Time),
    /// A curva de carga da fábrica na janela inteira, da primeira replicação
    /// (`(instante, kW)`); vazia sem rastro.
    pub curva: Vec<(Time, f64)>,
    /// A linha do tempo de cada nó com bloco na janela inteira, da primeira replicação.
    pub trechos: BTreeMap<String, Vec<Trecho>>,
    /// Energia por evento (peça, setup, partida) da primeira replicação: `(instante, kWh)`.
    pub impulsos: Vec<(Time, f64)>,
    /// Demanda, pico e conta de energia (fase AM4), quando o modelo tem tarifa e rastro.
    pub demanda: Option<ResumoDemanda>,
    /// Água e efluentes, resíduos, massa e mochila do refugo, e custo dos fluxos (fase AM5).
    pub agua: Option<crate::balanco::ResumoAgua>,
    pub residuos: Option<crate::balanco::ResumoResiduos>,
    pub materiais: Option<crate::balanco::ResumoMateriais>,
    pub custo: Option<Indicador>,
    /// A água de cada nó e a massa que passou entre os nós (médias), para o Sankey.
    pub agua_nos: Vec<crate::balanco::AguaNo>,
    pub transferencias: BTreeMap<(usize, usize), f64>,
}

/// A demanda e a conta de energia entre as replicações.
#[derive(Clone, Debug, PartialEq)]
pub struct ResumoDemanda {
    /// Os intervalos da primeira replicação, na janela da conta.
    pub intervalos: Vec<Intervalo>,
    pub max_kw: Indicador,
    pub pico_kw: Option<Indicador>,
    pub energia_pico_kwh: Indicador,
    pub fator_de_carga: Indicador,
    /// O total e as parcelas médias, ou os preços que faltam.
    pub custo: Result<(Indicador, Conta), Vec<&'static str>>,
}

impl Inventario {
    pub fn no(&self, nome: &str) -> Option<&NoInventario> {
        self.nos.iter().find(|n| n.nome == nome)
    }

    pub fn metrica(&self, chave: &str) -> Option<Indicador> {
        self.metricas.get(chave).map(|v| Indicador::de(v))
    }

    /// O instante e a potência do pico da curva de carga.
    pub fn pico(&self) -> Option<(Time, f64)> {
        // O primeiro instante em que a potência chega ao máximo.
        self.curva
            .iter()
            .copied()
            .fold(None, |m: Option<(Time, f64)>, x| match m {
                Some(y) if y.1 >= x.1 => Some(y),
                _ => Some(x),
            })
    }

    /// A potência e o estado de cada nó com bloco no instante `t` (primeira replicação).
    pub fn potencias_em(&self, t: Time) -> Vec<(&str, f64, &'static str)> {
        self.trechos
            .iter()
            .filter_map(|(n, ts)| {
                ts.iter()
                    .find(|x| x.de <= t && t < x.ate)
                    .map(|x| (n.as_str(), x.kw, x.estado))
            })
            .collect()
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

/// A conta inteira, sobre as replicações, na janela pedida (`None`: a janela de estatística
/// inteira).
pub fn avaliar(
    modelo: &Model,
    lateral: &Lateral,
    rodadas: &[Rodada],
    janela: Option<(Time, Time)>,
) -> Inventario {
    let cat = &lateral.environment;
    let hu = cat.clock.time_unit.em_horas();
    let eletrico = cat.fluxo_eletrico().map(|f| f.name.clone());
    let vazio = EnvironmentDef::default();
    let n = rodadas.len();
    let com_rastro = rodadas.iter().all(|r| r.rastro.is_some()) && n > 0;
    let mut inv = Inventario {
        replicacoes: n,
        ..Inventario::default()
    };
    if let Some(r0) = rodadas.first() {
        inv.janela = recorte(r0, janela);
        inv.janela_total = janela_da_rodada(r0);
    }
    let mut met: BTreeMap<String, Vec<f64>> = BTreeMap::new();
    let push = |m: &mut BTreeMap<String, Vec<f64>>, k: String, x: f64| {
        m.entry(k).or_default().push(x);
    };
    let mut pendencias = Vec::new();
    let mut usados: Vec<String> = Vec::new();

    // Por replicação: total da fábrica, por escopo.
    let mut fab_kwh = vec![0.0; n];
    let mut fab_sem = vec![0.0; n];
    let mut fab_co2 = vec![0.0; n];
    let mut fab_escopo: BTreeMap<u8, Vec<f64>> = BTreeMap::new();
    let mut fab_falta: Vec<String> = Vec::new();
    let mut trechos_r0: Vec<Vec<Trecho>> = Vec::new();
    // Massa, sucata e caminhos das peças (fase AM5), por replicação.
    let mats: Vec<Option<crate::materiais::Materiais>> = rodadas
        .iter()
        .map(|rod| crate::materiais::medir(modelo, lateral, rod, recorte(rod, janela)))
        .collect();
    let nomes: Vec<String> = modelo.objects.iter().map(|o| o.name.clone()).collect();
    let mut kwh_no = vec![vec![0.0; nomes.len()]; n];
    let mut entradas_no = vec![vec![0.0; nomes.len()]; n];
    let mut fluxos_no: BTreeMap<usize, BTreeMap<String, Vec<f64>>> = BTreeMap::new();
    // Por replicação, na janela da conta: a linha do tempo e os impulsos de cada nó.
    let mut trechos_rep: Vec<Vec<Vec<Trecho>>> = vec![Vec::new(); n];
    let mut impulsos_rep: Vec<Vec<(Time, f64)>> = vec![Vec::new(); n];

    for (indice, obj) in modelo.objects.iter().enumerate() {
        if !consome(&obj.kind) {
            continue;
        }
        let def = lateral.objects.get(&obj.name);
        let d = def.unwrap_or(&vazio);
        let tem_bloco = def
            .is_some_and(|d| !d.power.vazia() || !d.energy_per.is_empty() || !d.flows.is_empty());
        let mut kwh = vec![0.0; n];
        let mut grupos = vec![[0.0f64; 5]; n];
        let mut pecas = vec![0.0; n];
        let mut fluxos: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        // Linhas do cartão: soma das replicações, dividida no fim.
        let mut est: BTreeMap<&'static str, (f64, f64)> = BTreeMap::new();
        let mut eventos: [(f64, f64); 3] = [(0.0, 0.0); 3]; // peça, setup, partida
        let mut fl_dir: BTreeMap<String, (f64, f64)> = BTreeMap::new();

        for (r, rod) in rodadas.iter().enumerate() {
            let win = recorte(rod, janela);
            let m = medir(rod, indice, &obj.name, d, win, hu);
            for (estado, h) in &m.horas {
                let e = h * medida::potencia(d, estado);
                kwh[r] += e;
                if let Some(g) = medida::grupo(estado) {
                    grupos[r][g.indice()] += e;
                }
                let l = est.entry(estado).or_default();
                l.0 += h;
                l.1 += e;
            }
            pecas[r] = m.saidas;
            entradas_no[r][indice] = m.entradas;
            for ep in &d.energy_per {
                let (k, conta, g) = match ep.driver {
                    EventDriver::Item => (0, Some(m.entradas), Grupo::Produzindo),
                    EventDriver::Setup => (1, m.setups, Grupo::Preparando),
                    EventDriver::Startup => (2, m.partidas, Grupo::Preparando),
                };
                if let Some(c) = conta {
                    let e = ep.kwh * c;
                    kwh[r] += e;
                    grupos[r][g.indice()] += e;
                    eventos[k].0 += c;
                    eventos[k].1 += e;
                }
            }
            for f in &d.flows {
                let dir = match &f.driver {
                    Driver::Item => Some(m.entradas),
                    Driver::GoodItem => Some(m.saidas),
                    Driver::Time(estados) => Some(
                        estados
                            .iter()
                            .map(|e| m.horas.get(e.nome()).copied().unwrap_or(0.0))
                            .sum(),
                    ),
                    Driver::Setup => m.setups,
                    Driver::Startup => m.partidas,
                    Driver::Scrap => mats[r].as_ref().map(|x| x.refugadas[indice]),
                    Driver::Mass => mats[r].as_ref().map(|x| x.massa_entrada[indice]),
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
            if tem_bloco {
                for ep in &d.energy_per {
                    let ts = match ep.driver {
                        EventDriver::Item => &m.t_entradas,
                        EventDriver::Setup => &m.t_setups,
                        EventDriver::Startup => &m.t_partidas,
                    };
                    impulsos_rep[r].extend(ts.iter().map(|t| (*t, ep.kwh)));
                }
                trechos_rep[r].push(m.trechos.clone());
            }
            if r == 0 && tem_bloco {
                // A linha do tempo e os impulsos da janela inteira, para a curva.
                let total = if janela.is_some() {
                    medir(rod, indice, &obj.name, d, janela_da_rodada(rod), hu)
                } else {
                    m
                };
                for ep in &d.energy_per {
                    let ts = match ep.driver {
                        EventDriver::Item => &total.t_entradas,
                        EventDriver::Setup => &total.t_setups,
                        EventDriver::Startup => &total.t_partidas,
                    };
                    inv.impulsos.extend(ts.iter().map(|t| (*t, ep.kwh)));
                }
                inv.trechos.insert(obj.name.clone(), total.trechos.clone());
                trechos_r0.push(total.trechos);
            }
        }

        // O que esta versão ainda não conta.
        let mut pend = |chave: &'static str| {
            pendencias.push(Pendencia {
                no: obj.name.clone(),
                chave,
            })
        };
        if !com_rastro {
            for ep in &d.energy_per {
                match ep.driver {
                    EventDriver::Setup => pend("setup"),
                    EventDriver::Startup => pend("partida"),
                    EventDriver::Item => {}
                }
            }
            if d.standby.is_some() {
                pend("espera");
            }
        }
        for f in &d.flows {
            let chave = match f.driver {
                Driver::Scrap if !com_rastro => "refugo",
                Driver::Rework => "retrabalho",
                Driver::Setup if !com_rastro => "setup",
                Driver::Startup if !com_rastro => "partida",
                Driver::Distance { .. } => "distancia",
                Driver::Volume => "volume",
                Driver::Mass if !com_rastro => "massa",
                _ => continue,
            };
            pend(chave);
        }

        // Emissão do nó: a eletricidade e cada fluxo, com o fator do catálogo.
        let mut falta = Vec::new();
        let mut co2 = vec![0.0; n];
        let mut escopo: BTreeMap<u8, Vec<f64>> = BTreeMap::new();
        let mut emitir = |fluxo: &str, q: &[f64], falta: &mut Vec<String>| {
            if q.iter().all(|x| *x == 0.0) {
                return;
            }
            match crate::balanco::fator_do_fluxo(lateral, fluxo) {
                Ok((k, s)) => {
                    let sv = escopo.entry(s).or_insert_with(|| vec![0.0; n]);
                    for (r, x) in q.iter().enumerate() {
                        co2[r] += x * k;
                        sv[r] += x * k;
                    }
                    if !usados.iter().any(|u| u == fluxo) {
                        usados.push(fluxo.to_string());
                    }
                }
                Err(x) => falta.push(x),
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

        for r in 0..n {
            kwh_no[r][indice] = kwh[r];
        }
        if !fluxos.is_empty() {
            fluxos_no.insert(indice, fluxos.clone());
        }

        // Métricas com nome.
        if tem_bloco {
            let no = &obj.name;
            for r in 0..n {
                push(&mut met, format!("{no}.energia_kwh"), kwh[r]);
                push(
                    &mut met,
                    format!("{no}.energia_sem_valor_kwh"),
                    sem_valor[r],
                );
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
                fab_sem[r] += sem_valor[r];
                fab_co2[r] += co2[r];
            }
            for (s, v) in &escopo {
                let t = fab_escopo.entry(*s).or_insert_with(|| vec![0.0; n]);
                for (r, x) in v.iter().enumerate() {
                    t[r] += x;
                }
            }
            fab_falta.extend(falta.iter().cloned());
        }

        let rn = n.max(1) as f64;
        let mut linhas: Vec<Linha> = est
            .into_iter()
            .filter(|(_, (h, _))| *h > 0.0)
            .map(|(estado, (h, e))| Linha {
                grupo: medida::grupo(estado),
                coeficiente: medida::potencia(d, estado),
                origem: estado.to_string(),
                tipo: TipoLinha::Estado,
                direcionador: h / rn,
                quantidade: e / rn,
            })
            .collect();
        linhas.sort_by(|a, b| b.quantidade.total_cmp(&a.quantidade));
        let tipos = [
            ("item", TipoLinha::EnergiaPorPeca, Grupo::Produzindo),
            ("setup", TipoLinha::EnergiaPorSetup, Grupo::Preparando),
            ("partida", TipoLinha::EnergiaPorPartida, Grupo::Preparando),
        ];
        for (k, (origem, tipo, g)) in tipos.into_iter().enumerate() {
            let (c, e) = eventos[k];
            if c > 0.0 {
                linhas.push(Linha {
                    origem: origem.into(),
                    tipo,
                    grupo: Some(g),
                    direcionador: c / rn,
                    coeficiente: e / c,
                    quantidade: e / rn,
                });
            }
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
            agua_m3: None,
            efluente_m3: None,
            residuo_kg: None,
            sucata_kg: None,
        });
    }

    // Peças boas: as que chegam às saídas que não são de refugo, na janela.
    let boas: Vec<f64> = rodadas
        .iter()
        .map(|rod| {
            let win = recorte(rod, janela);
            modelo
                .objects
                .iter()
                .enumerate()
                .filter(|(_, o)| {
                    matches!(o.kind, ObjectKind::Sink) && o.sink_role != Some(SinkRole::Scrap)
                })
                .map(|(i, o)| medir(rod, i, &o.name, &vazio, win, hu).entradas)
                .sum()
        })
        .collect();

    // Água, efluentes, resíduos, massa, mochila e custos (fase AM5).
    crate::balanco::completar(
        &crate::balanco::Entrada {
            lateral,
            nomes: &nomes,
            mats: &mats,
            fluxos_no: &fluxos_no,
            kwh_no: &kwh_no,
            entradas_no: &entradas_no,
        },
        &mut crate::balanco::Fabrica {
            met: &mut met,
            kwh: &mut fab_kwh,
            co2: &mut fab_co2,
            escopo: &mut fab_escopo,
            falta: &mut fab_falta,
        },
        &mut inv,
    );

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

    // Demanda, pico e conta de energia (fase AM4).
    if let Some(t) = &cat.tariff
        && com_rastro
    {
        let ds: Vec<_> = rodadas
            .iter()
            .enumerate()
            .map(|(r, rod)| {
                let refs: Vec<&[Trecho]> = trechos_rep[r].iter().map(|v| v.as_slice()).collect();
                let mut imp = impulsos_rep[r].clone();
                imp.sort_by(|a, b| a.0.total_cmp(&b.0));
                demanda(
                    &medida::curva(&refs),
                    &imp,
                    recorte(rod, janela),
                    &cat.clock,
                    t,
                )
            })
            .collect();
        for d in &ds {
            push(&mut met, "ambiental.demanda_max_kw".into(), d.max_kw);
            if let Some(p) = d.pico_kw {
                push(&mut met, "ambiental.demanda_pico_kw".into(), p);
            }
            push(
                &mut met,
                "ambiental.energia_pico_kwh".into(),
                d.energia_pico_kwh,
            );
            push(
                &mut met,
                "ambiental.fator_de_carga".into(),
                d.fator_de_carga,
            );
            if let Ok(c) = &d.conta {
                push(&mut met, "ambiental.custo_energia".into(), c.total());
            }
        }
        let col = |f: &dyn Fn(&crate::tarifa::Demanda) -> f64| -> Indicador {
            Indicador::de(&ds.iter().map(f).collect::<Vec<_>>())
        };
        let picos: Vec<f64> = ds.iter().filter_map(|d| d.pico_kw).collect();
        let custo = match ds.iter().find_map(|d| d.conta.as_ref().err()) {
            Some(f) => Err(f.clone()),
            None => {
                let contas: Vec<Conta> = ds.iter().filter_map(|d| d.conta.clone().ok()).collect();
                let k = contas.len().max(1) as f64;
                let media = Conta {
                    energia_pico: contas.iter().map(|c| c.energia_pico).sum::<f64>() / k,
                    energia_fora: contas.iter().map(|c| c.energia_fora).sum::<f64>() / k,
                    demanda: contas.iter().map(|c| c.demanda).sum::<f64>() / k,
                    ultrapassagem: contas.iter().map(|c| c.ultrapassagem).sum::<f64>() / k,
                };
                Ok((
                    Indicador::de(&contas.iter().map(Conta::total).collect::<Vec<_>>()),
                    media,
                ))
            }
        };
        inv.demanda = Some(ResumoDemanda {
            intervalos: ds.first().map(|d| d.intervalos.clone()).unwrap_or_default(),
            max_kw: col(&|d| d.max_kw),
            pico_kw: (!picos.is_empty()).then(|| Indicador::de(&picos)),
            energia_pico_kwh: col(&|d| d.energia_pico_kwh),
            fator_de_carga: col(&|d| d.fator_de_carga),
            custo,
        });
    }

    let refs: Vec<&[Trecho]> = trechos_r0.iter().map(|v| v.as_slice()).collect();
    inv.curva = medida::curva(&refs);
    inv.impulsos.sort_by(|a, b| a.0.total_cmp(&b.0));
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
    use utsim_core::{BlockReport, RunReport, TallySummary};

    fn so(v: Vec<RunReport>) -> Vec<Rodada> {
        v.into_iter().map(Rodada::so_relatorio).collect()
    }

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
        let inv = avaliar(&modelo(), &l, &so(vec![r]), None);
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
        let inv = avaliar(&modelo(), &l, &so(vec![r]), None);
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
        let inv = avaliar(&modelo(), &l, &so(reps.clone()), None);
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
        let inv2 = avaliar(&modelo(), &l2, &so(reps.clone()), None);
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
        let a = avaliar(&modelo(), &l, &so(vec![r.clone()]), None)
            .energia
            .media;
        l.objects.get_mut("forno").unwrap().power.waiting = 2.0;
        let b = avaliar(&modelo(), &l, &so(vec![r]), None).energia.media;
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
        let a = avaliar(&m, &l, &crate::rodar::simular(&m).unwrap(), None);
        let b = avaliar(&m, &l, &crate::rodar::simular(&m).unwrap(), None);
        assert_eq!(a.metricas, b.metricas);
        // O forno trabalha 30 s a cada 60 s: perto de metade do tempo produzindo.
        let f = a.no("forno").unwrap();
        assert!(f.energia.media > 0.0);
        assert!(f.por_grupo[Grupo::Produzindo.indice()].media > 0.0);
    }

    fn modelo_setup() -> Model {
        Model::parse(
            r#"Model(name: "m", run: (stop_time: 36000.0, warmup: 3600.0, seed: 5, replications: 3),
               objects: [(name: "chegada", kind: Source(interarrival: Exp(mean: 300.0))),
                         (name: "forno", kind: Processor(setup: Const(20.0), process: Const(60.0))),
                         (name: "fim", kind: Sink)],
               connections: [("chegada", "forno"), ("forno", "fim")])"#,
        )
        .unwrap()
    }

    /// Com rastro, as horas por estado batem com as frações do relatório, os setups e as
    /// partidas aparecem, a curva integra a energia da janela e a janela pela metade dá
    /// menos energia.
    #[test]
    fn rastro_setup_janela_e_curva() {
        let m = modelo_setup();
        let rod = crate::rodar::simular(&m).unwrap();
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        let mut d = EnvironmentDef::default();
        d.power.producing = 10.0;
        d.power.preparing = 6.0;
        d.power.waiting = 2.0;
        d.energy_per.push(EnergyPer {
            driver: EventDriver::Setup,
            kwh: 1.0,
        });
        l.objects.insert("forno".into(), d);
        let com = avaliar(&m, &l, &rod, None);
        let so_rel: Vec<Rodada> = rod
            .iter()
            .map(|r| Rodada::so_relatorio(r.relatorio.clone()))
            .collect();
        let sem = avaliar(&m, &l, &so_rel, None);
        // Sem a energia por setup, as duas contas dão a mesma energia de estados.
        let f = com.no("forno").unwrap();
        let setup = f
            .linhas
            .iter()
            .find(|x| x.tipo == TipoLinha::EnergiaPorSetup)
            .expect("setups medidos");
        assert!(setup.direcionador > 10.0);
        let estados_com: f64 = f
            .linhas
            .iter()
            .filter(|x| x.tipo == TipoLinha::Estado)
            .map(|x| x.quantidade)
            .sum();
        let estados_sem: f64 = sem
            .no("forno")
            .unwrap()
            .linhas
            .iter()
            .filter(|x| x.tipo == TipoLinha::Estado)
            .map(|x| x.quantidade)
            .sum();
        assert!((estados_com - estados_sem).abs() < 1e-6 * estados_sem.max(1.0));
        assert!(com.pendencias.is_empty());
        assert!(sem.pendencias.iter().any(|p| p.chave == "setup"));
        // A curva integra a energia dos estados da primeira replicação.
        let (a, b) = com.janela_total;
        let mut integral = 0.0;
        for w in com.curva.windows(2) {
            integral += w[0].1 * (w[1].0 - w[0].0);
        }
        if let Some(u) = com.curva.last() {
            integral += u.1 * (b - u.0).max(0.0);
        }
        let _ = a;
        let horas = integral / 3600.0;
        let r0: f64 =
            com.metricas["forno.energia_kwh"][0] - com.impulsos.iter().map(|x| x.1).sum::<f64>();
        assert!((horas - r0).abs() < 1e-6 * r0.max(1.0), "{horas} {r0}");
        // Metade da janela, menos energia.
        let meio = avaliar(&m, &l, &rod, Some((a, (a + b) / 2.0)));
        assert!(meio.energia.media < com.energia.media * 0.75);
        assert!(meio.energia.media > com.energia.media * 0.25);
    }

    /// O desligamento em espera: com espera longa e potência fora de turno menor que a de
    /// espera, a energia cai e aparecem partidas.
    #[test]
    fn desligamento_em_espera() {
        let m = modelo_setup();
        let rod = crate::rodar::simular(&m).unwrap();
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        let mut d = EnvironmentDef::default();
        d.power.producing = 10.0;
        d.power.waiting = 8.0;
        d.power.off_shift = 0.5;
        d.energy_per.push(EnergyPer {
            driver: EventDriver::Startup,
            kwh: 0.2,
        });
        l.objects.insert("forno".into(), d.clone());
        let antes = avaliar(&m, &l, &rod, None);
        d.standby = Some(crate::formato::StandbyDef { after: 120.0 });
        l.objects.insert("forno".into(), d);
        let depois = avaliar(&m, &l, &rod, None);
        assert!(depois.energia.media < antes.energia.media);
        let partidas = depois
            .no("forno")
            .unwrap()
            .linhas
            .iter()
            .find(|x| x.tipo == TipoLinha::EnergiaPorPartida)
            .map_or(0.0, |x| x.direcionador);
        assert!(partidas > 5.0);
        assert!(
            depois
                .no("forno")
                .unwrap()
                .linhas
                .iter()
                .any(|x| x.origem == crate::medida::STANDBY)
        );
    }

    /// Com tarifa, a energia dos intervalos de demanda (pico + fora) é a energia da fábrica,
    /// replicação a replicação, e a conta só sai com todos os preços.
    #[test]
    fn demanda_fecha_com_a_energia() {
        let m = modelo_setup();
        let rod = crate::rodar::simular(&m).unwrap();
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        let mut t = crate::formato::Tariff::inicial();
        t.peak.from = "08:00".into();
        t.peak.to = "10:00".into();
        l.environment.tariff = Some(t);
        let mut d = EnvironmentDef::default();
        d.power.producing = 10.0;
        d.power.waiting = 2.0;
        d.energy_per.push(EnergyPer {
            driver: EventDriver::Item,
            kwh: 0.3,
        });
        l.objects.insert("forno".into(), d);
        let inv = avaliar(&m, &l, &rod, None);
        let fab = &inv.metricas["ambiental.energia_kwh"];
        let pico = &inv.metricas["ambiental.energia_pico_kwh"];
        let dem = inv.demanda.as_ref().unwrap();
        assert!(dem.max_kw.media > 2.0 && dem.max_kw.media <= 10.0 + 1e-9 + 0.3 * 4.0 * 3.0);
        assert!(dem.custo.is_err());
        let fora: f64 = dem
            .intervalos
            .iter()
            .filter(|x| !x.pico)
            .map(|x| x.kwh)
            .sum();
        let p0: f64 = dem
            .intervalos
            .iter()
            .filter(|x| x.pico)
            .map(|x| x.kwh)
            .sum();
        assert!((p0 - pico[0]).abs() < 1e-9);
        assert!(
            (p0 + fora - fab[0]).abs() < 1e-6 * fab[0],
            "{} {}",
            p0 + fora,
            fab[0]
        );
        // Com os preços, a conta sai.
        let t = l.environment.tariff.as_mut().unwrap();
        t.price_kwh_peak = Some(1.0);
        t.price_kwh_off_peak = Some(0.5);
        t.price_kw = Some(20.0);
        let inv = avaliar(&m, &l, &rod, None);
        assert!(inv.metrica("ambiental.custo_energia").unwrap().media > 0.0);
    }
}
