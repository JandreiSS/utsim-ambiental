//! UTSim Ambiental, o núcleo: o formato do bloco ambiental, a validação e a conta de
//! energia e de emissão de cada nó a partir do relatório da simulação (`docs/plano_ambiental.md`,
//! fases AM0 e AM1).
//!
//! Toda conta tem a forma `direcionador simulado × coeficiente do nó × fator do catálogo`:
//! o tempo em cada estado vem do relatório da simulação, a potência vem do bloco do nó e o
//! fator de emissão vem do catálogo do modelo. A simulação não lê nenhum destes campos:
//! ligar o bloco ambiental não muda nenhum tempo nem nenhum sorteio.
//!
//! Este crate não conhece a janela: o app (`utsim-ambiental-app`) só desenha o que sai daqui.

pub mod balanco;
pub mod camada3d;
pub mod cenarios;
pub mod conta;
pub mod formato;
pub mod mapa;
pub mod materiais;
pub mod medida;
pub mod metas;
pub mod produto;
pub mod rodar;
pub mod sankey;
pub mod tarifa;
pub mod troca;
pub mod validacao;

use std::path::{Path, PathBuf};

pub use conta::{Estimativa, Indicador, Inventario, NoInventario, Vilao, avaliar, viloes};
pub use formato::{EnvironmentCatalog, EnvironmentDef, Grupo, Lateral};
pub use rodar::{Rodada, simular};
pub use validacao::{Problema, validar};

/// Os erros de leitura e gravação do arquivo lateral.
#[derive(Debug, thiserror::Error)]
pub enum Erro {
    #[error("não consegui ler {caminho}: {erro}")]
    Ler { caminho: PathBuf, erro: String },
    #[error("o arquivo {caminho} não está no formato do bloco ambiental: {erro}")]
    Formato { caminho: PathBuf, erro: String },
    #[error("não consegui gravar {caminho}: {erro}")]
    Gravar { caminho: PathBuf, erro: String },
}

/// O arquivo lateral de um modelo: `linha.ron` → `linha.ambiental.ron`, na mesma pasta.
pub fn caminho_lateral(modelo: &Path) -> PathBuf {
    let base = modelo
        .file_stem()
        .map_or_else(|| "modelo".into(), |s| s.to_string_lossy().into_owned());
    modelo.with_file_name(format!("{base}.ambiental.ron"))
}

/// Lê o texto de um arquivo lateral.
pub fn ler_texto(texto: &str) -> Result<Lateral, ron::error::SpannedError> {
    ron::from_str(texto)
}

/// O arquivo lateral em texto RON, legível e estável (mesma ordem sempre).
pub fn texto(lateral: &Lateral) -> String {
    let cfg = ron::ser::PrettyConfig::new()
        .depth_limit(6)
        .struct_names(false)
        .separate_tuple_members(false)
        .enumerate_arrays(false);
    let corpo = ron::ser::to_string_pretty(lateral, cfg).unwrap_or_default();
    format!(
        "// Bloco ambiental do modelo (UTSim Ambiental). Fica ao lado do modelo até o campo\n\
         // `environment` entrar nas bibliotecas da família (docs/formato_ambiental.md).\n{corpo}\n"
    )
}

/// Lê o arquivo lateral do modelo; sem arquivo, um lateral com o catálogo inicial.
pub fn ler_do_modelo(modelo: &Path) -> Result<Lateral, Erro> {
    let caminho = caminho_lateral(modelo);
    if !caminho.exists() {
        return Ok(Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        });
    }
    let texto = std::fs::read_to_string(&caminho).map_err(|e| Erro::Ler {
        caminho: caminho.clone(),
        erro: e.to_string(),
    })?;
    ler_texto(&texto).map_err(|e| Erro::Formato {
        caminho,
        erro: e.to_string(),
    })
}

/// Grava o arquivo lateral ao lado do modelo (troca o antigo de uma vez, seguro numa pasta
/// sincronizada).
pub fn gravar_do_modelo(modelo: &Path, lateral: &Lateral) -> Result<PathBuf, Erro> {
    let caminho = caminho_lateral(modelo);
    utsim_pasta::write_replace(&caminho, texto(lateral).as_bytes()).map_err(|e| Erro::Gravar {
        caminho: caminho.clone(),
        erro: e.to_string(),
    })?;
    Ok(caminho)
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn caminho_do_lateral_fica_ao_lado_do_modelo() {
        let c = caminho_lateral(Path::new("/tmp/x/linha.ron"));
        assert_eq!(c, Path::new("/tmp/x/linha.ambiental.ron"));
    }

    #[test]
    fn texto_volta_igual() {
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        let mut d = EnvironmentDef::default();
        d.power.producing = 18.0;
        d.power.by_state.insert("blocked".into(), 12.0);
        l.objects.insert("estufa".into(), d);
        let t = texto(&l);
        assert_eq!(ler_texto(&t).unwrap(), l);
    }
}
