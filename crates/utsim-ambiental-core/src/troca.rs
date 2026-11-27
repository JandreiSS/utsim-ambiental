//! A conversa com o UTSim LSS e o UTSim Estatística (fase AM11, plano 4.4 e 4.6), pelo
//! formato de pedido e resposta da família (`utsim-troca`).
//!
//! - **O teste da redução.** Para cada cenário, o teste t pareado da diferença
//!   `cenário − Base` nas replicações (H₁: o cenário reduz). O resultado fica gravado no
//!   cenário (`testes`), com a origem: o próprio Ambiental ou a resposta da Estatística.
//! - **Enviar para a Estatística.** Um CSV com uma linha por replicação (`base`, `cenario`,
//!   `diferenca`) e o pedido `analisar` (teste pareado), com o hash do CSV. Enquanto a
//!   Estatística não atende `analisar`, ela abre o CSV como planilha.
//! - **Levar para o LSS.** O pedido `nova_oportunidade`, com o resumo do cenário (alavancas,
//!   diferenças com intervalo, investimento) e a linha ambiental do mapa de fluxo de valor
//!   (energia e CO₂e por peça de cada máquina), mais o arquivo dos cenários com hash.
//!
//! O formato dos dois pedidos está em `docs/troca_ambiental.md`.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use utsim_stats::hypothesis::{Alternative, t_paired};
use utsim_troca::{Pedido, acao};

use crate::conta::{Indicador, Inventario};

/// A ação do pedido de oportunidade para o LSS (texto, como as outras ações da família).
pub const ACAO_OPORTUNIDADE: &str = "nova_oportunidade";

/// Um teste gravado no cenário.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Teste {
    /// A métrica testada (`ambiental.co2e_kg`).
    pub metrica: String,
    /// A diferença média `cenário − Base` e o intervalo de 95 %.
    pub diferenca: f64,
    pub de: f64,
    pub ate: f64,
    /// O p-valor de H₁: o cenário reduz.
    pub p_valor: f64,
    pub n: usize,
    /// Quem fez o teste (`UTSim Ambiental`, `UTSim Estatística`).
    pub origem: String,
}

impl Teste {
    /// A redução é significativa a 5 %?
    pub fn reduz(&self) -> bool {
        self.p_valor < 0.05 && self.diferenca < 0.0
    }
}

/// O teste t pareado de uma métrica: H₁, o cenário tem valor menor que o Base.
pub fn testar(base: &[f64], cen: &[f64], metrica: &str) -> Option<Teste> {
    let t = t_paired(cen, base, 0.0, 0.95, Alternative::Less)?;
    let ic = Indicador::de(&utsim_stats::hypothesis::differences(cen, base));
    Some(Teste {
        metrica: metrica.to_string(),
        diferenca: t.estimate,
        de: ic.media - ic.meia_largura,
        ate: ic.media + ic.meia_largura,
        p_valor: t.p_value,
        n: ic.n,
        origem: "UTSim Ambiental".into(),
    })
}

/// O CSV pareado (`;`, ponto decimal), uma linha por replicação.
pub fn csv_pareado(base: &[f64], cen: &[f64]) -> String {
    let mut s = String::from("replicacao;base;cenario;diferenca\n");
    for (r, (a, b)) in base.iter().zip(cen).enumerate() {
        s.push_str(&format!("{};{a};{b};{}\n", r + 1, b - a));
    }
    s
}

/// A pasta de troca do modelo: `troca/` ao lado dele.
pub fn pasta(modelo: &Path) -> PathBuf {
    modelo.with_file_name("troca")
}

fn seguro(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Grava o CSV pareado e o pedido `analisar` para a Estatística. Devolve o pedido (já com o
/// caminho da resposta) e o caminho dele.
pub fn pedido_analise(
    de: &str,
    modelo: &Path,
    cenario: &str,
    metrica: &str,
    base: &[f64],
    cen: &[f64],
) -> Result<(Pedido, PathBuf), String> {
    let p = pasta(modelo);
    std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
    let csv = p.join(format!("{}_{}.csv", seguro(cenario), seguro(metrica)));
    utsim_pasta::write_replace(&csv, csv_pareado(base, cen).as_bytes())
        .map_err(|e| e.to_string())?;
    let mut ped = Pedido::novo(de, acao::ANALISAR);
    ped.ferramenta = Some("teste_t_pareado".into());
    ped.entrada.arquivos = vec![utsim_troca::Arquivo::de(csv).map_err(|e| e.to_string())?];
    ped.entrada.colunas = vec!["cenario".into(), "base".into()];
    // A métrica vai no pedido: a Estatística não precisa tirá-la do nome do CSV.
    ped.entrada.respostas = vec![metrica.into()];
    ped.contexto.modelo = Some(modelo.to_path_buf());
    ped.somente_leitura = true;
    let c = ped.gravar(&p).map_err(|e| e.to_string())?;
    Ok((ped, c))
}

/// O resumo que vai para o LSS.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Oportunidade {
    pub cenario: String,
    /// As alavancas em texto (o formato do arquivo de cenários).
    pub alavancas: Vec<String>,
    /// `(métrica, diferença média, de, até)` para o Base, por replicação pareada.
    pub diferencas: Vec<(String, f64, f64, f64)>,
    pub investimento: Option<f64>,
    pub vida_anos: Option<f64>,
    /// A linha ambiental do mapa de fluxo de valor: por máquina, kWh e kg CO₂e por peça.
    pub linha_vsm: Vec<LinhaVsm>,
    pub testes: Vec<Teste>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LinhaVsm {
    pub no: String,
    pub etapa: Option<String>,
    pub kwh_por_peca: Option<f64>,
    pub co2e_por_peca: Option<f64>,
}

/// A linha ambiental do mapa de fluxo de valor, a partir do inventário.
pub fn linha_vsm(inv: &Inventario) -> Vec<LinhaVsm> {
    inv.nos
        .iter()
        .filter(|n| n.tem_bloco)
        .map(|n| LinhaVsm {
            no: n.nome.clone(),
            etapa: n.etapa.clone(),
            kwh_por_peca: n.kwh_por_peca.map(|x| x.media),
            co2e_por_peca: match (n.co2e.valor(), n.pecas.media > 0.0) {
                (Some(c), true) => Some(c.media / n.pecas.media),
                _ => None,
            },
        })
        .collect()
}

/// As diferenças pareadas das métricas principais.
pub fn diferencas(base: &Inventario, cen: &Inventario) -> Vec<(String, f64, f64, f64)> {
    [
        "ambiental.co2e_kg",
        "ambiental.energia_kwh",
        "ambiental.agua_m3",
        "ambiental.custo_energia",
        "ambiental.custo",
    ]
    .iter()
    .filter_map(|k| {
        let d = crate::conta::comparar(base, cen, k)?;
        let h = if d.tem_intervalo() {
            d.meia_largura
        } else {
            0.0
        };
        Some((k.to_string(), d.media, d.media - h, d.media + h))
    })
    .collect()
}

/// Grava o pedido `nova_oportunidade` para o LSS, com o arquivo dos cenários (hash).
pub fn pedido_oportunidade(
    de: &str,
    modelo: &Path,
    op: &Oportunidade,
) -> Result<(Pedido, PathBuf), String> {
    let p = pasta(modelo);
    std::fs::create_dir_all(&p).map_err(|e| e.to_string())?;
    let mut ped = Pedido::novo(de, ACAO_OPORTUNIDADE);
    ped.contexto.modelo = Some(modelo.to_path_buf());
    let arq = crate::cenarios::caminho(modelo);
    if arq.is_file() {
        ped.entrada.arquivos = vec![utsim_troca::Arquivo::de(arq).map_err(|e| e.to_string())?];
    }
    ped.entrada.conteudo = Some(serde_json::to_string_pretty(op).map_err(|e| e.to_string())?);
    let caminho = ped.gravar(&p).map_err(|e| e.to_string())?;
    Ok((ped, caminho))
}

#[cfg(test)]
mod testes {
    use super::*;

    /// Uma redução clara dá p pequeno; nenhuma diferença, p alto.
    #[test]
    fn teste_pareado() {
        let base = [10.0, 11.0, 12.0, 10.5, 11.5];
        let cen: Vec<f64> = base.iter().map(|x| x - 1.0 + 0.01 * x).collect();
        let t = testar(&base, &cen, "x").unwrap();
        assert!(t.reduz(), "{t:?}");
        let igual = testar(&base, &base.map(|x| x + 0.0), "x");
        assert!(igual.is_none_or(|t| !t.reduz()));
        assert!(csv_pareado(&base, &cen).lines().count() == 6);
    }

    /// Os pedidos ficam na pasta de troca, com o hash dos arquivos.
    #[test]
    fn pedidos_gravados() {
        let pasta = std::env::temp_dir().join(format!("utsim-amb-troca-{}", std::process::id()));
        std::fs::create_dir_all(&pasta).unwrap();
        let m = pasta.join("linha.ron");
        std::fs::write(&m, "x").unwrap();
        let (p, c) = pedido_analise(
            "ambiental 0.3.0",
            &m,
            "Cenário 1",
            "ambiental.co2e_kg",
            &[1.0, 2.0],
            &[0.5, 1.5],
        )
        .unwrap();
        assert!(c.is_file());
        assert_eq!(p.acao, acao::ANALISAR);
        assert_eq!(p.entrada.respostas, vec!["ambiental.co2e_kg".to_string()]);
        assert!(p.conferir_entrada().is_ok());
        let lido = Pedido::ler(&c).unwrap();
        assert_eq!(
            lido.entrada.colunas,
            vec!["cenario".to_string(), "base".into()]
        );
        let op = Oportunidade {
            cenario: "c".into(),
            ..Oportunidade::default()
        };
        let (p2, c2) = pedido_oportunidade("ambiental 0.3.0", &m, &op).unwrap();
        assert_eq!(p2.acao, ACAO_OPORTUNIDADE);
        let lido: Oportunidade = serde_json::from_str(
            Pedido::ler(&c2)
                .unwrap()
                .entrada
                .conteudo
                .as_deref()
                .unwrap(),
        )
        .unwrap();
        assert_eq!(lido.cenario, "c");
        let _ = std::fs::remove_dir_all(&pasta);
    }
}
