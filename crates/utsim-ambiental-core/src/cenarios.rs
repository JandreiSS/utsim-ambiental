//! Cenários "e se" (fase AM8, plano 3.4): um cenário é uma lista de alavancas sobre o bloco
//! ambiental ou sobre o modelo. As alavancas de coeficiente, fator e tarifa são
//! recalculadas sobre o mesmo rastro, na hora; as que mudam o modelo (patches do
//! experimento) pedem simular de novo, com a mesma semente: as replicações usam os mesmos
//! números aleatórios do Base, e a comparação é pareada.
//!
//! Os cenários ficam em `<modelo>.cenarios_ambientais.json`, ao lado do modelo.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use utsim_model::Model;

use crate::conta::{Indicador, Inventario};
use crate::formato::{Grupo, Lateral, StandbyDef};

/// Uma mudança do cenário.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Alavanca {
    /// A potência de um grupo de estado de uma máquina (kW).
    Potencia { no: String, grupo: Grupo, kw: f64 },
    /// Desligar a máquina depois de esperar `depois` unidades de tempo (`None`: não desliga).
    Desligamento { no: String, depois: Option<f64> },
    /// O fator de emissão de um fluxo (kg CO₂e por unidade).
    Fator { fluxo: String, kg_co2e: f64 },
    /// O destino de um fluxo de resíduo.
    DestinoResiduo { fluxo: String, destino: String },
    /// A fração do efluente de uma máquina que é reusada.
    Reuso { no: String, fracao: f64 },
    /// A remoção de um parâmetro num destino de efluente.
    Remocao {
        destino: String,
        parametro: String,
        fracao: f64,
    },
    /// Preços do kWh no pico e fora dele.
    PrecoEnergia {
        pico: Option<f64>,
        fora: Option<f64>,
    },
    /// A demanda contratada (kW).
    Contratada { kw: Option<f64> },
    /// A janela de pico (`HH:MM`).
    JanelaPico { de: String, ate: String },
    /// Um campo do modelo, como nos experimentos (`estufa.capacity`, `estufa.process`):
    /// pede simular de novo.
    Modelo { chave: String, valor: String },
}

impl Alavanca {
    pub fn precisa_simular(&self) -> bool {
        matches!(self, Alavanca::Modelo { .. })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Cenario {
    pub nome: String,
    pub alavancas: Vec<Alavanca>,
    /// Custo de implantação e vida útil em anos (para o retorno e o custo por tonelada).
    pub investimento: Option<f64>,
    pub vida_anos: Option<f64>,
}

impl Cenario {
    pub fn precisa_simular(&self) -> bool {
        self.alavancas.iter().any(Alavanca::precisa_simular)
    }
}

/// Os cenários de um modelo e os parâmetros da comparação.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Estudo {
    pub cenarios: Vec<Cenario>,
    /// Horas de operação por ano, para levar a janela simulada a um ano.
    pub horas_por_ano: Option<f64>,
    /// Preço do carbono por kg CO₂e (opcional, vazio por padrão).
    pub preco_carbono: Option<f64>,
}

/// O arquivo dos cenários de um modelo.
pub fn caminho(modelo: &Path) -> PathBuf {
    let base = modelo
        .file_stem()
        .map_or_else(|| "modelo".into(), |s| s.to_string_lossy().into_owned());
    modelo.with_file_name(format!("{base}.cenarios_ambientais.json"))
}

pub fn ler(modelo: &Path) -> Estudo {
    std::fs::read_to_string(caminho(modelo))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or_default()
}

pub fn gravar(modelo: &Path, e: &Estudo) -> Result<PathBuf, String> {
    let c = caminho(modelo);
    let t = serde_json::to_string_pretty(e).map_err(|x| x.to_string())?;
    utsim_pasta::write_replace(&c, t.as_bytes()).map_err(|x| x.to_string())?;
    Ok(c)
}

/// O modelo e o bloco com as alavancas aplicadas.
pub fn aplicar(modelo: &Model, l: &Lateral, c: &Cenario) -> Result<(Model, Lateral), String> {
    let mut l = l.clone();
    let mut patches: BTreeMap<String, String> = BTreeMap::new();
    for a in &c.alavancas {
        match a {
            Alavanca::Potencia { no, grupo, kw } => {
                *l.objects
                    .entry(no.clone())
                    .or_default()
                    .power
                    .do_grupo_mut(*grupo) = *kw;
            }
            Alavanca::Desligamento { no, depois } => {
                l.objects.entry(no.clone()).or_default().standby =
                    depois.map(|x| StandbyDef { after: x });
            }
            Alavanca::Fator { fluxo, kg_co2e } => {
                match l.environment.factors.iter_mut().find(|f| &f.flow == fluxo) {
                    Some(f) => f.kg_co2e = Some(*kg_co2e),
                    None => return Err(format!("o fluxo {fluxo} não tem fator no catálogo")),
                }
            }
            Alavanca::DestinoResiduo { fluxo, destino } => {
                match l.environment.flows.iter_mut().find(|f| &f.name == fluxo) {
                    Some(f) => f.destination = Some(destino.clone()),
                    None => return Err(format!("o fluxo {fluxo} não está no catálogo")),
                }
            }
            Alavanca::Reuso { no, fracao } => {
                let d = l.objects.entry(no.clone()).or_default();
                let w = d.water.get_or_insert_with(Default::default);
                w.reuse = fracao.clamp(0.0, 1.0);
                if w.reuse_to.is_none() {
                    w.reuse_to = Some(no.clone());
                }
            }
            Alavanca::Remocao {
                destino,
                parametro,
                fracao,
            } => match l
                .environment
                .effluent
                .destinations
                .iter_mut()
                .find(|d| &d.name == destino)
            {
                Some(d) => {
                    d.removal.insert(parametro.clone(), fracao.clamp(0.0, 1.0));
                }
                None => return Err(format!("o destino {destino} não está no catálogo")),
            },
            Alavanca::PrecoEnergia { pico, fora } => {
                let t = l
                    .environment
                    .tariff
                    .get_or_insert_with(crate::formato::Tariff::inicial);
                t.price_kwh_peak = *pico;
                t.price_kwh_off_peak = *fora;
            }
            Alavanca::Contratada { kw } => {
                l.environment
                    .tariff
                    .get_or_insert_with(crate::formato::Tariff::inicial)
                    .contracted_kw = *kw;
            }
            Alavanca::JanelaPico { de, ate } => {
                let t = l
                    .environment
                    .tariff
                    .get_or_insert_with(crate::formato::Tariff::inicial);
                t.peak.from = de.clone();
                t.peak.to = ate.clone();
            }
            Alavanca::Modelo { chave, valor } => {
                patches.insert(chave.clone(), valor.clone());
            }
        }
    }
    let m = if patches.is_empty() {
        modelo.clone()
    } else {
        utsim_model::experiment::apply_patches(modelo, &patches).map_err(|e| e.to_string())?
    };
    Ok((m, l))
}

/// O custo de cada replicação: conta de energia, fluxos, resíduos e efluentes com preço, e
/// o carbono pelo preço do carbono. `None` quando nada tem preço.
pub fn custo(inv: &Inventario, preco_carbono: Option<f64>) -> Option<Vec<f64>> {
    let n = inv.replicacoes;
    let mut v = vec![0.0; n];
    let mut tem = false;
    for k in ["ambiental.custo_energia", "ambiental.custo"] {
        if let Some(x) = inv.metricas.get(k)
            && x.len() == n
        {
            for (a, b) in v.iter_mut().zip(x) {
                *a += b;
            }
            tem = true;
        }
    }
    if let (Some(p), Some(c)) = (preco_carbono, inv.metricas.get("ambiental.co2e_kg"))
        && c.len() == n
    {
        for (a, b) in v.iter_mut().zip(c) {
            *a += b * p;
        }
        tem = true;
    }
    tem.then_some(v)
}

/// A diferença pareada `cenário − base`, replicação a replicação.
pub fn pareado(base: &[f64], cen: &[f64]) -> Option<Indicador> {
    if base.len() != cen.len() || base.is_empty() {
        return None;
    }
    let d: Vec<f64> = base.iter().zip(cen).map(|(a, b)| b - a).collect();
    Some(Indicador::de(&d))
}

/// O veredito de uma diferença em que menos é melhor (custo, impacto).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Veredito {
    Melhor,
    Pior,
    /// O intervalo passa pelo zero (ou não há intervalo).
    NaoDaParaAfirmar,
}

pub fn veredito(d: &Indicador) -> Veredito {
    if !d.tem_intervalo() {
        return if d.media == 0.0 {
            Veredito::NaoDaParaAfirmar
        } else if d.n >= 2 && d.meia_largura == 0.0 && d.media < 0.0 {
            Veredito::Melhor
        } else {
            Veredito::NaoDaParaAfirmar
        };
    }
    if d.media + d.meia_largura < 0.0 {
        Veredito::Melhor
    } else if d.media - d.meia_largura > 0.0 {
        Veredito::Pior
    } else {
        Veredito::NaoDaParaAfirmar
    }
}

/// Os pontos que não perdem nos dois critérios para nenhum outro (menos é melhor nos dois).
pub fn fronteira(pontos: &[(f64, f64)]) -> Vec<usize> {
    (0..pontos.len())
        .filter(|i| {
            let (a, b) = pontos[*i];
            !pontos
                .iter()
                .enumerate()
                .any(|(j, (x, y))| j != *i && *x <= a && *y <= b && (*x < a || *y < b))
        })
        .collect()
}

/// O retorno de um investimento: `(anos para pagar, custo por tonelada evitada)`.
/// `economia` e `evitado_kg` são da janela simulada; `fator_ano` leva a janela a um ano.
pub fn retorno(
    investimento: f64,
    vida_anos: Option<f64>,
    economia: f64,
    evitado_kg: f64,
    fator_ano: f64,
) -> (Option<f64>, Option<f64>) {
    let eco_ano = economia * fator_ano;
    let anos = (eco_ano > 0.0).then(|| investimento / eco_ano);
    let t_ano = evitado_kg * fator_ano / 1000.0;
    let por_t = match vida_anos {
        Some(v) if v > 0.0 && t_ano > 0.0 => Some((investimento / v - eco_ano) / t_ano),
        _ => None,
    };
    (anos, por_t)
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::formato::{EnvironmentCatalog, EnvironmentDef};

    fn modelo() -> Model {
        Model::parse(
            r#"Model(name: "m", run: (stop_time: 7200.0, warmup: 0.0, seed: 3, replications: 4),
               objects: [(name: "chegada", kind: Source(interarrival: Exp(mean: 60.0))),
                         (name: "forno", kind: Processor(process: Exp(mean: 40.0))),
                         (name: "fim", kind: Sink)],
               connections: [("chegada", "forno"), ("forno", "fim")])"#,
        )
        .unwrap()
    }

    fn lateral() -> Lateral {
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        l.environment.factors[0].kg_co2e = Some(0.1);
        let mut d = EnvironmentDef::default();
        d.power.producing = 10.0;
        d.power.waiting = 6.0;
        l.objects.insert("forno".into(), d);
        l
    }

    /// Baixar a potência de espera é recalculado sobre o mesmo rastro, e a diferença pareada
    /// é exata: −(6 − 2) × horas esperando, em cada replicação; o veredito é "melhor".
    #[test]
    fn alavanca_de_coeficiente_e_pareado() {
        let m = modelo();
        let l = lateral();
        let rod = crate::rodar::simular(&m).unwrap();
        let base = crate::avaliar(&m, &l, &rod, None);
        let c = Cenario {
            nome: "espera".into(),
            alavancas: vec![Alavanca::Potencia {
                no: "forno".into(),
                grupo: Grupo::Esperando,
                kw: 2.0,
            }],
            ..Cenario::default()
        };
        assert!(!c.precisa_simular());
        let (m2, l2) = aplicar(&m, &l, &c).unwrap();
        let cen = crate::avaliar(&m2, &l2, &rod, None);
        let d = pareado(
            &base.metricas["forno.energia_kwh"],
            &cen.metricas["forno.energia_kwh"],
        )
        .unwrap();
        let esperando = &base.metricas["forno.energia_kwh.esperando"];
        let esperado: Vec<f64> = esperando.iter().map(|e| -e / 6.0 * 4.0).collect();
        assert!((d.media - Indicador::de(&esperado).media).abs() < 1e-9);
        assert_eq!(veredito(&d), Veredito::Melhor);
    }

    /// Uma alavanca de modelo pede simular, e o patch que não muda nada dá diferença zero
    /// (mesmos números aleatórios do Base).
    #[test]
    fn alavanca_de_modelo_mesma_semente() {
        let m = modelo();
        let l = lateral();
        let c = Cenario {
            nome: "igual".into(),
            alavancas: vec![Alavanca::Modelo {
                chave: "forno.capacity".into(),
                valor: "1".into(),
            }],
            ..Cenario::default()
        };
        assert!(c.precisa_simular());
        let (m2, l2) = aplicar(&m, &l, &c).unwrap();
        let a = crate::avaliar(&m, &l, &crate::rodar::simular(&m).unwrap(), None);
        let b = crate::avaliar(&m2, &l2, &crate::rodar::simular(&m2).unwrap(), None);
        let d = pareado(
            &a.metricas["ambiental.energia_kwh"],
            &b.metricas["ambiental.energia_kwh"],
        )
        .unwrap();
        assert_eq!(d.media, 0.0);
        assert_eq!(veredito(&d), Veredito::NaoDaParaAfirmar);
        // Duas máquinas: o resultado muda.
        let c2 = Cenario {
            alavancas: vec![Alavanca::Modelo {
                chave: "forno.capacity".into(),
                valor: "2".into(),
            }],
            ..c
        };
        assert!(aplicar(&m, &l, &c2).is_ok());
    }

    #[test]
    fn fronteira_e_retorno() {
        let f = fronteira(&[(10.0, 5.0), (8.0, 6.0), (9.0, 7.0), (12.0, 4.0)]);
        assert_eq!(f, vec![0, 1, 3]);
        // Investe 1000, economiza 10 por janela, 100 janelas por ano: paga em 1 ano.
        let (anos, por_t) = retorno(1000.0, Some(10.0), 10.0, 50.0, 100.0);
        assert!((anos.unwrap() - 1.0).abs() < 1e-12);
        // (100 − 1000) / 5 t = −180 por tonelada (ganha dinheiro e reduz).
        assert!((por_t.unwrap() + 180.0).abs() < 1e-9);
    }

    #[test]
    fn grava_e_le() {
        let pasta = std::env::temp_dir().join(format!("utsim-amb-cen-{}", std::process::id()));
        std::fs::create_dir_all(&pasta).unwrap();
        let m = pasta.join("x.ron");
        let e = Estudo {
            cenarios: vec![Cenario {
                nome: "a".into(),
                alavancas: vec![Alavanca::Fator {
                    fluxo: "eletricidade".into(),
                    kg_co2e: 0.2,
                }],
                investimento: Some(10.0),
                vida_anos: None,
            }],
            horas_por_ano: Some(4000.0),
            preco_carbono: None,
        };
        gravar(&m, &e).unwrap();
        assert_eq!(ler(&m), e);
        let _ = std::fs::remove_dir_all(&pasta);
    }
}
