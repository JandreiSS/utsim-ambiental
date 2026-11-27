//! Massa, sucata e a "mochila" da peça refugada (fase AM5, plano 2.4), pelo caminho de
//! cada peça no rastro.
//!
//! - Cada peça nasce com a massa do seu tipo (`item_mass_kg` do catálogo; `"*"` vale para
//!   as peças sem tipo ou de tipo sem massa).
//! - Ao sair de um nó, a peça ganha a massa dos fluxos por peça com `adds_mass` e perde a
//!   dos fluxos por peça com `removes_mass` (em kg).
//! - A peça que chega a uma saída de refugo (`sink_role: Scrap`) vira sucata com a massa
//!   que tem ali, atribuída ao nó de onde veio (o que a rejeitou).
//! - Balanço da janela: o que entrou (peças criadas mais material somado) é igual ao que
//!   saiu (peças boas, sucata, material tirado) mais a variação da massa em processo.

use std::collections::{BTreeMap, HashMap};

use utsim_core::Time;
use utsim_model::{Model, ObjectKind, SinkRole};

use crate::formato::{Driver, Lateral};
use crate::rodar::Rodada;

/// A chave da massa das peças sem tipo.
pub const QUALQUER: &str = "*";

/// O que o caminho das peças dá numa replicação, na janela.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Materiais {
    /// Por nó (posição do objeto): kg que entraram no nó.
    pub massa_entrada: Vec<f64>,
    /// Por nó: peças que saíram do nó direto para o refugo.
    pub refugadas: Vec<f64>,
    /// Por nó: kg de sucata atribuídos ao nó.
    pub sucata_kg: Vec<f64>,
    /// Massa das peças criadas na janela.
    pub criada_kg: f64,
    /// Material somado e tirado nos nós, na janela.
    pub somada_kg: f64,
    pub tirada_kg: f64,
    /// Massa que chegou às saídas que não são de refugo.
    pub boa_kg: f64,
    pub sucata_total_kg: f64,
    /// Massa em processo no começo e no fim da janela.
    pub em_processo_inicio: f64,
    pub em_processo_fim: f64,
    /// O caminho (nós por onde passou) de cada peça refugada na janela.
    pub caminhos_refugo: Vec<Vec<usize>>,
    /// O tipo de cada peça refugada (0: sem tipo), na ordem de `caminhos_refugo`.
    pub tipos_refugo: Vec<u32>,
    /// A peça (o `item` do rastro) de cada caminho de `caminhos_refugo`, na mesma ordem.
    pub itens_refugo: Vec<u64>,
    /// kg que passaram de um nó para outro na janela, por par `(de, para)`.
    pub transferido: BTreeMap<(usize, usize), f64>,
    /// Por nó e por tipo de peça (0: sem tipo): o que entrou e o tempo que ficou.
    pub por_tipo: Vec<BTreeMap<u32, PorTipo>>,
    /// Por tipo: peças boas, massa boa e peças refugadas na janela.
    pub boas_por_tipo: BTreeMap<u32, (f64, f64)>,
    pub refugadas_por_tipo: BTreeMap<u32, f64>,
}

/// O que um tipo de peça fez num nó, na janela.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PorTipo {
    pub entradas: f64,
    pub massa: f64,
    /// Tempo de permanência somado (unidades do modelo), recortado na janela.
    pub tempo: f64,
}

impl Materiais {
    /// Entrada da janela: peças criadas mais material somado.
    pub fn entrada(&self) -> f64 {
        self.criada_kg + self.somada_kg
    }

    /// Erro de fechamento do balanço, em % da entrada (0 quando não entrou nada).
    pub fn erro_pct(&self) -> f64 {
        let e = self.entrada();
        if e <= 0.0 {
            return 0.0;
        }
        let saida = self.boa_kg + self.sucata_total_kg + self.tirada_kg;
        let variacao = self.em_processo_fim - self.em_processo_inicio;
        100.0 * (e - saida - variacao).abs() / e
    }

    /// Rendimento de material: massa boa sobre a entrada, em %.
    pub fn rendimento_pct(&self) -> f64 {
        let e = self.entrada();
        if e > 0.0 {
            100.0 * self.boa_kg / e
        } else {
            0.0
        }
    }
}

/// A massa inicial de uma peça do tipo `tipo` (1 = o primeiro de `item_types`).
fn massa_inicial(modelo: &Model, l: &Lateral, tipo: Option<u32>) -> f64 {
    let m = &l.environment.item_mass_kg;
    tipo.and_then(|k| modelo.item_types.get(k.checked_sub(1)? as usize))
        .and_then(|t| m.get(&t.name))
        .or_else(|| m.get(QUALQUER))
        .copied()
        .unwrap_or(0.0)
}

/// Quanto a peça ganha e perde ao sair de cada nó (kg por peça).
fn variacao_por_no(modelo: &Model, l: &Lateral) -> Vec<(f64, f64)> {
    modelo
        .objects
        .iter()
        .map(|o| {
            let Some(d) = l.objects.get(&o.name) else {
                return (0.0, 0.0);
            };
            let mut v = (0.0, 0.0);
            for f in &d.flows {
                if !matches!(f.driver, Driver::Item | Driver::GoodItem) {
                    continue;
                }
                if f.adds_mass {
                    v.0 += f.amount;
                }
                if f.removes_mass {
                    v.1 += f.amount;
                }
            }
            v
        })
        .collect()
}

/// Percorre as passagens da replicação e mede a massa na janela `(a, b)`. Sem rastro,
/// `None`.
pub fn medir(modelo: &Model, l: &Lateral, r: &Rodada, janela: (Time, Time)) -> Option<Materiais> {
    let rastro = r.rastro.as_ref()?;
    let (a, b) = janela;
    let n = modelo.objects.len();
    let refugo: Vec<bool> = modelo
        .objects
        .iter()
        .map(|o| matches!(o.kind, ObjectKind::Sink) && o.sink_role == Some(SinkRole::Scrap))
        .collect();
    let saida: Vec<bool> = modelo
        .objects
        .iter()
        .map(|o| matches!(o.kind, ObjectKind::Sink))
        .collect();
    let fonte: Vec<bool> = modelo
        .objects
        .iter()
        .map(|o| matches!(o.kind, ObjectKind::Source { .. }))
        .collect();
    let var = variacao_por_no(modelo, l);
    let mut m = Materiais {
        massa_entrada: vec![0.0; n],
        refugadas: vec![0.0; n],
        sucata_kg: vec![0.0; n],
        por_tipo: vec![BTreeMap::new(); n],
        ..Materiais::default()
    };
    struct Viva {
        massa: f64,
        caminho: Vec<usize>,
        tipo: u32,
        no: usize,
        desde: Time,
    }
    let mut vivas: HashMap<u64, Viva> = HashMap::new();
    let mut inicio_marcado = false;
    let em_processo = |v: &HashMap<u64, Viva>| v.values().map(|x| x.massa).sum::<f64>();
    for p in &rastro.passagens {
        if !inicio_marcado && p.t >= a {
            m.em_processo_inicio = em_processo(&vivas);
            inicio_marcado = true;
        }
        if p.t >= b {
            break;
        }
        let dentro = p.t >= a;
        let tipo = rastro.tipos.get(&p.item).copied().unwrap_or(0);
        let e: &mut Viva = match p.de {
            None => {
                let k = massa_inicial(modelo, l, rastro.tipos.get(&p.item).copied());
                if dentro {
                    m.criada_kg += k;
                }
                vivas.entry(p.item).or_insert(Viva {
                    massa: k,
                    caminho: Vec::new(),
                    tipo,
                    no: p.para,
                    desde: p.t,
                })
            }
            Some(de) => {
                let e = vivas.entry(p.item).or_insert(Viva {
                    massa: 0.0,
                    caminho: Vec::new(),
                    tipo,
                    no: de,
                    desde: p.t,
                });
                if let Some((mais, menos)) = var.get(de) {
                    let menos = menos.min(e.massa + mais);
                    e.massa += mais - menos;
                    if dentro {
                        m.somada_kg += mais;
                        m.tirada_kg += menos;
                    }
                }
                // O tempo que a peça ficou no nó de onde sai, recortado na janela.
                let t0 = e.desde.max(a);
                if p.t > t0 && de < n {
                    m.por_tipo[de].entry(e.tipo).or_default().tempo += p.t - t0;
                }
                if dentro {
                    *m.transferido.entry((de, p.para)).or_default() += e.massa;
                }
                e
            }
        };
        e.no = p.para;
        e.desde = p.t;
        let destino = p.para;
        if saida.get(destino).copied().unwrap_or(false) {
            let massa_final = e.massa;
            let tipo = e.tipo;
            let caminho = std::mem::take(&mut e.caminho);
            vivas.remove(&p.item);
            if dentro {
                if refugo[destino] {
                    m.sucata_total_kg += massa_final;
                    if let Some(de) = p.de {
                        m.refugadas[de] += 1.0;
                        m.sucata_kg[de] += massa_final;
                    }
                    m.caminhos_refugo.push(caminho);
                    m.tipos_refugo.push(tipo);
                    m.itens_refugo.push(p.item);
                    *m.refugadas_por_tipo.entry(tipo).or_default() += 1.0;
                } else {
                    m.boa_kg += massa_final;
                    let x = m.boas_por_tipo.entry(tipo).or_default();
                    x.0 += 1.0;
                    x.1 += massa_final;
                }
            }
        } else {
            if dentro && destino < n {
                m.massa_entrada[destino] += e.massa;
                let x = m.por_tipo[destino].entry(e.tipo).or_default();
                x.entradas += 1.0;
                x.massa += e.massa;
            }
            if !fonte.get(destino).copied().unwrap_or(true) {
                e.caminho.push(destino);
            }
        }
    }
    if !inicio_marcado {
        m.em_processo_inicio = em_processo(&vivas);
    }
    // As peças que seguem no nó no fim da janela: o tempo até o fim.
    for v in vivas.values() {
        let t0 = v.desde.max(a);
        if b > t0 && v.no < n {
            m.por_tipo[v.no].entry(v.tipo).or_default().tempo += b - t0;
        }
    }
    m.em_processo_fim = em_processo(&vivas);
    Some(m)
}

/// Um volume de água na unidade do fluxo, em m³ (`L` ou `m3`); outra unidade, `None`.
pub fn em_m3(unidade: &str) -> Option<f64> {
    match unidade.trim().to_lowercase().as_str() {
        "l" | "litro" | "litros" => Some(0.001),
        "m3" | "m³" => Some(1.0),
        _ => None,
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::formato::{EnvironmentDef, FlowUse};

    fn modelo() -> Model {
        Model::parse(
            r#"Model(name: "m", run: (stop_time: 7200.0, warmup: 600.0, seed: 9, replications: 2),
               objects: [(name: "chegada", kind: Source(interarrival: Exp(mean: 60.0))),
                         (name: "corte", kind: Processor(process: Exp(mean: 40.0))),
                         (name: "pintura", kind: Processor(process: Exp(mean: 30.0))),
                         (name: "inspecao", kind: Processor(process: Const(5.0)),
                          output: Weighted([0.8, 0.2])),
                         (name: "fim", kind: Sink),
                         (name: "refugo", kind: Sink, sink_role: Some(Scrap))],
               connections: [("chegada", "corte"), ("corte", "pintura"), ("pintura", "inspecao"),
                             ("inspecao", "fim"), ("inspecao", "refugo")])"#,
        )
        .unwrap()
    }

    fn fluxo(nome: &str, kg: f64, soma: bool) -> FlowUse {
        FlowUse {
            flow: nome.into(),
            amount: kg,
            driver: Driver::Item,
            adds_mass: soma,
            removes_mass: !soma,
        }
    }

    /// O balanço fecha, a peça boa sai com 2 − 0,3 + 0,05 kg e a sucata vai para a inspeção.
    #[test]
    fn balanco_de_massa_fecha() {
        let m = modelo();
        let rod = crate::rodar::simular(&m).unwrap();
        let mut l = Lateral::default();
        l.environment.item_mass_kg.insert(QUALQUER.into(), 2.0);
        let mut corte = EnvironmentDef::default();
        corte.flows.push(fluxo("cavaco", 0.3, false));
        let mut pintura = EnvironmentDef::default();
        pintura.flows.push(fluxo("tinta", 0.05, true));
        l.objects.insert("corte".into(), corte);
        l.objects.insert("pintura".into(), pintura);
        for r in &rod {
            let x = medir(&m, &l, r, (r.relatorio.stats_from, r.relatorio.end_time)).unwrap();
            assert!(x.erro_pct() < 1e-9, "erro {}", x.erro_pct());
            assert!(x.sucata_total_kg > 0.0);
            assert!((x.sucata_kg[3] - x.sucata_total_kg).abs() < 1e-9);
            // Cada peça refugada tem 1,75 kg e passou por corte, pintura e inspeção.
            assert!((x.sucata_total_kg / x.refugadas[3] - 1.75).abs() < 1e-9);
            assert!(x.caminhos_refugo.iter().all(|c| c == &vec![1, 2, 3]));
            assert!(x.rendimento_pct() < 100.0);
        }
    }

    #[test]
    fn unidades_de_agua() {
        assert_eq!(em_m3("L"), Some(0.001));
        assert_eq!(em_m3("m3"), Some(1.0));
        assert_eq!(em_m3("kg"), None);
    }
}
