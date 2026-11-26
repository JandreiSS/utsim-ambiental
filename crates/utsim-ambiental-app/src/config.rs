//! A configuração do usuário (`.../UTSim/ambiental.json`): a língua e os modelos recentes.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub idioma: Option<String>,
    pub recentes: Vec<String>,
}

/// O arquivo de configuração do app.
pub fn arquivo() -> Option<PathBuf> {
    utsim_pasta::config::arquivo("ambiental.json")
}

impl Config {
    pub fn ler(arquivo: Option<&PathBuf>) -> Config {
        arquivo
            .map(|a| utsim_pasta::config::ler(a))
            .unwrap_or_default()
    }

    pub fn gravar(&self, arquivo: Option<&PathBuf>) -> Result<(), String> {
        match arquivo {
            Some(a) => utsim_pasta::config::gravar(a, self),
            None => Ok(()),
        }
    }
}
