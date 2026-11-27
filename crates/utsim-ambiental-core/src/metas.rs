//! Metas (fase AM9, plano 3.5): o valor de cada meta em cada replicação, a probabilidade de
//! cumpri-la com intervalo, o semáforo, o orçamento de carbono, a cascata dos cenários
//! ligados e os selos de conquista.
//!
//! As metas moram no bloco do modelo (`targets`), porque são sobre o simulado; as leituras
//! reais ficam à parte, com a origem "Real" (decisão A15).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::conta::{Indicador, Inventario};
use crate::formato::{Basis, Lateral, Target, TargetScope};

/// Um indicador que uma meta pode usar: a chave curta da métrica e se menos é melhor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TipoIndicador {
    pub chave: &'static str,
    pub menor_melhor: bool,
    /// Faz sentido somar nós (energia, CO₂e) ou não (porcentagens).
    pub soma: bool,
}

pub const INDICADORES: [TipoIndicador; 11] = [
    TipoIndicador {
        chave: "co2e_kg",
        menor_melhor: true,
        soma: true,
    },
    TipoIndicador {
        chave: "energia_kwh",
        menor_melhor: true,
        soma: true,
    },
    TipoIndicador {
        chave: "energia_sem_valor_kwh",
        menor_melhor: true,
        soma: true,
    },
    TipoIndicador {
        chave: "agua_m3",
        menor_melhor: true,
        soma: true,
    },
    TipoIndicador {
        chave: "efluente_m3",
        menor_melhor: true,
        soma: true,
    },
    TipoIndicador {
        chave: "custo",
        menor_melhor: true,
        soma: false,
    },
    TipoIndicador {
        chave: "custo_energia",
        menor_melhor: true,
        soma: false,
    },
    TipoIndicador {
        chave: "demanda_max_kw",
        menor_melhor: true,
        soma: false,
    },
    TipoIndicador {
        chave: "desvio_aterro_pct",
        menor_melhor: false,
        soma: false,
    },
    TipoIndicador {
        chave: "rendimento_material_pct",
        menor_melhor: false,
        soma: false,
    },
    TipoIndicador {
        chave: "reuso_pct",
        menor_melhor: false,
        soma: false,
    },
];

pub fn tipo(chave: &str) -> Option<TipoIndicador> {
    INDICADORES.iter().copied().find(|x| x.chave == chave)
}

/// O valor da meta em cada replicação, pelo alcance e pela base. `None`: o inventário não
/// tem o indicador (falta fator, falta tarifa, ou o alcance não o tem).
pub fn valores(inv: &Inventario, l: &Lateral, m: &Target) -> Option<Vec<f64>> {
    let t = tipo(&m.indicator)?;
    let n = inv.replicacoes;
    let total: Vec<f64> = match &m.scope {
        TargetScope::Factory => inv
            .metricas
            .get(&format!("ambiental.{}", m.indicator))?
            .clone(),
        TargetScope::Node(no) => inv.metricas.get(&format!("{no}.{}", m.indicator))?.clone(),
        TargetScope::Stage(e) => {
            if !t.soma {
                return None;
            }
            let nos: Vec<&String> = l
                .objects
                .iter()
                .filter(|(_, d)| d.stage.as_deref() == Some(e.as_str()))
                .map(|(n, _)| n)
                .collect();
            let mut v = vec![0.0; n];
            let mut achou = false;
            for no in nos {
                if let Some(x) = inv.metricas.get(&format!("{no}.{}", m.indicator))
                    && x.len() == n
                {
                    achou = true;
                    for (a, b) in v.iter_mut().zip(x) {
                        *a += b;
                    }
                }
            }
            if !achou {
                return None;
            }
            v
        }
    };
    if total.len() != n {
        return None;
    }
    match m.basis {
        Basis::Total => Some(total),
        Basis::PerGoodItem => {
            if !t.soma && !matches!(m.indicator.as_str(), "custo" | "custo_energia") {
                return None;
            }
            let boas = inv.metricas.get("ambiental.pecas_boas")?;
            Some(
                total
                    .iter()
                    .zip(boas)
                    .map(|(x, b)| if *b > 0.0 { x / b } else { f64::NAN })
                    .filter(|x| x.is_finite())
                    .collect(),
            )
        }
        Basis::PerHour => {
            let h = (inv.janela.1 - inv.janela.0) * l.environment.clock.time_unit.em_horas();
            (h > 0.0).then(|| total.iter().map(|x| x / h).collect())
        }
    }
}

/// O semáforo de uma meta pela probabilidade de cumpri-la nas replicações.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Semaforo {
    Verde,
    Amarelo,
    Vermelho,
}

/// O intervalo de Wilson de 95 % para `k` sucessos em `n`.
pub fn wilson(k: usize, n: usize) -> (f64, f64) {
    if n == 0 {
        return (0.0, 1.0);
    }
    let z = 1.959_963_984_540_054_f64;
    let n = n as f64;
    let p = k as f64 / n;
    let c = p + z * z / (2.0 * n);
    let r = z * ((p * (1.0 - p) + z * z / (4.0 * n)) / n).sqrt();
    let d = 1.0 + z * z / n;
    (((c - r) / d).max(0.0), ((c + r) / d).min(1.0))
}

/// Como a meta está no simulado.
#[derive(Clone, Debug, PartialEq)]
pub struct Situacao {
    pub valor: Indicador,
    /// Fração das replicações que cumprem a meta, e o intervalo de 95 %.
    pub probabilidade: f64,
    pub intervalo: (f64, f64),
    pub semaforo: Semaforo,
    pub menor_melhor: bool,
}

/// Cumpre o alvo?
pub fn cumpre(valor: f64, alvo: f64, menor_melhor: bool) -> bool {
    if menor_melhor {
        valor <= alvo
    } else {
        valor >= alvo
    }
}

/// A situação de uma meta (sem alvo, sem probabilidade: semáforo amarelo).
pub fn situacao(inv: &Inventario, l: &Lateral, m: &Target) -> Option<Situacao> {
    let t = tipo(&m.indicator)?;
    let v = valores(inv, l, m)?;
    if v.is_empty() {
        return None;
    }
    let (p, ic) = match m.target {
        Some(a) => {
            let k = v.iter().filter(|x| cumpre(**x, a, t.menor_melhor)).count();
            (k as f64 / v.len() as f64, wilson(k, v.len()))
        }
        None => (0.0, (0.0, 1.0)),
    };
    let semaforo = if m.target.is_none() {
        Semaforo::Amarelo
    } else if p >= 0.8 {
        Semaforo::Verde
    } else if p > 0.2 {
        Semaforo::Amarelo
    } else {
        Semaforo::Vermelho
    };
    Some(Situacao {
        valor: Indicador::de(&v),
        probabilidade: p,
        intervalo: ic,
        semaforo,
        menor_melhor: t.menor_melhor,
    })
}

/// Um degrau da cascata: da linha de base, cada cenário ligado à meta soma o seu efeito
/// (a diferença média para o Base no indicador da meta).
#[derive(Clone, Debug, PartialEq)]
pub struct Degrau {
    pub nome: String,
    pub efeito: f64,
    /// O valor depois deste degrau.
    pub depois: f64,
}

/// A cascata da linha de base até o resultado com os cenários ligados. Os efeitos são
/// somados como se fossem independentes (aproximação declarada na tela).
pub fn cascata(base: f64, efeitos: &[(String, f64)]) -> Vec<Degrau> {
    let mut v = base;
    efeitos
        .iter()
        .map(|(n, e)| {
            v += e;
            Degrau {
                nome: n.clone(),
                efeito: *e,
                depois: v,
            }
        })
        .collect()
}

/// Uma leitura real (medidor, conta, pesagem), guardada à parte do simulado.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Leitura {
    /// A meta a que a leitura se refere.
    pub meta: String,
    /// O período (`2026-11`, "semana 3"...).
    pub periodo: String,
    pub valor: f64,
    /// De onde veio (medidor, conta, pesagem).
    pub origem: String,
}

/// As leituras reais de um modelo: `<modelo>.leituras_ambientais.json`.
pub fn caminho_leituras(modelo: &Path) -> PathBuf {
    let base = modelo
        .file_stem()
        .map_or_else(|| "modelo".into(), |s| s.to_string_lossy().into_owned());
    modelo.with_file_name(format!("{base}.leituras_ambientais.json"))
}

pub fn ler_leituras(modelo: &Path) -> Vec<Leitura> {
    std::fs::read_to_string(caminho_leituras(modelo))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}

pub fn gravar_leituras(modelo: &Path, l: &[Leitura]) -> Result<PathBuf, String> {
    let c = caminho_leituras(modelo);
    let t = serde_json::to_string_pretty(l).map_err(|x| x.to_string())?;
    utsim_pasta::write_replace(&c, t.as_bytes()).map_err(|x| x.to_string())?;
    Ok(c)
}

/// Os selos de conquista da sala de aula.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Selo {
    PrimeiraMeta,
    BalancoFechado,
    MetadeDesviada,
}

impl Selo {
    pub fn chave(self) -> &'static str {
        match self {
            Selo::PrimeiraMeta => "primeira_meta",
            Selo::BalancoFechado => "balanco_fechado",
            Selo::MetadeDesviada => "metade_desviada",
        }
    }
}

pub fn selos(inv: &Inventario, l: &Lateral) -> Vec<Selo> {
    let mut s = Vec::new();
    if l.environment
        .targets
        .iter()
        .filter_map(|m| situacao(inv, l, m))
        .any(|x| x.semaforo == Semaforo::Verde)
    {
        s.push(Selo::PrimeiraMeta);
    }
    if inv
        .materiais
        .as_ref()
        .is_some_and(|m| m.erro_pct.media < 0.5 && m.entrada_kg.media > 0.0)
    {
        s.push(Selo::BalancoFechado);
    }
    if inv
        .residuos
        .as_ref()
        .is_some_and(|r| r.desvio_pct.media > 50.0)
    {
        s.push(Selo::MetadeDesviada);
    }
    s
}

/// O orçamento de carbono: quanto da emissão do período a janela já usou (fração).
pub fn orcamento(inv: &Inventario, l: &Lateral) -> Option<(f64, f64)> {
    let b = l.environment.carbon_budget_kg?;
    let c = inv.co2e.valor()?.media;
    (b > 0.0).then_some((c, c / b))
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::formato::{EnvironmentCatalog, EnvironmentDef};
    use utsim_model::Model;

    /// Uma meta cumprida em todas as replicações fica verde; no meio, amarela; nenhuma,
    /// vermelha.
    #[test]
    fn semaforo_pelas_replicacoes() {
        let m = Model::parse(
            r#"Model(name: "m", run: (stop_time: 7200.0, warmup: 0.0, seed: 2, replications: 6),
               objects: [(name: "a", kind: Source(interarrival: Exp(mean: 60.0))),
                         (name: "forno", kind: Processor(process: Exp(mean: 40.0))),
                         (name: "b", kind: Sink)],
               connections: [("a", "forno"), ("forno", "b")])"#,
        )
        .unwrap();
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        let mut d = EnvironmentDef::default();
        d.power.producing = 10.0;
        d.power.waiting = 2.0;
        d.stage = Some("calor".into());
        l.objects.insert("forno".into(), d);
        let rod = crate::rodar::simular(&m).unwrap();
        let inv = crate::avaliar(&m, &l, &rod, None);
        let e = &inv.metricas["ambiental.energia_kwh"];
        let mut ord = e.clone();
        ord.sort_by(f64::total_cmp);
        let mut meta = Target {
            id: "m1".into(),
            indicator: "energia_kwh".into(),
            scope: TargetScope::Factory,
            basis: Basis::Total,
            baseline: None,
            target: Some(ord[5] + 1.0),
            due: None,
            actions: Vec::new(),
        };
        assert_eq!(situacao(&inv, &l, &meta).unwrap().semaforo, Semaforo::Verde);
        meta.target = Some((ord[2] + ord[3]) / 2.0);
        let s = situacao(&inv, &l, &meta).unwrap();
        assert_eq!(s.semaforo, Semaforo::Amarelo);
        assert!((s.probabilidade - 0.5).abs() < 1e-12);
        meta.target = Some(ord[0] - 1.0);
        assert_eq!(
            situacao(&inv, &l, &meta).unwrap().semaforo,
            Semaforo::Vermelho
        );
        // Por etapa e por peça boa.
        meta.scope = TargetScope::Stage("calor".into());
        meta.basis = Basis::PerGoodItem;
        let v = valores(&inv, &l, &meta).unwrap();
        let boas = &inv.metricas["ambiental.pecas_boas"];
        assert!((v[0] - e[0] / boas[0]).abs() < 1e-9);
    }

    #[test]
    fn wilson_e_cascata() {
        let (a, b) = wilson(5, 10);
        assert!(a < 0.5 && b > 0.5);
        assert_eq!(wilson(0, 0), (0.0, 1.0));
        let c = cascata(100.0, &[("a".into(), -10.0), ("b".into(), -5.0)]);
        assert_eq!(c[1].depois, 85.0);
    }
}
