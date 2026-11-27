//! O formato do bloco ambiental (`docs/formato_ambiental.md`, plano seção 4.2).
//!
//! Dois blocos: [`EnvironmentDef`] em cada objeto (potência por grupo de estado, fluxos,
//! água, efluente) e [`EnvironmentCatalog`] no modelo (fluxos, fatores, destinos, tarifa,
//! metas). Desde a fase AM10 os tipos moram na família (`utsim_model::environment`) e o
//! bloco pode ficar dentro do modelo (`environment` do objeto e do modelo, `mass_kg` do
//! tipo de item), editado também na aba Ambiental do UTSim.
//!
//! Um modelo que ainda não tem o bloco usa o arquivo ao lado dele,
//! `<modelo>.ambiental.ron` ([`Lateral`]), com os blocos dos objetos pelo nome do objeto. O
//! formato é o mesmo nos dois lugares: só o lugar muda (decisão A4).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use utsim_model::Model;

pub use utsim_model::environment::*;

/// O bloco ambiental inteiro de um modelo: o catálogo e os blocos dos objetos, pelo nome do
/// objeto. É o arquivo lateral `<modelo>.ambiental.ron` e também a forma em memória do
/// bloco que mora dentro do modelo.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Lateral {
    pub environment: EnvironmentCatalog,
    pub objects: BTreeMap<String, EnvironmentDef>,
}

impl Lateral {
    /// O bloco que está dentro do modelo; `None` se o modelo não tem bloco nenhum. A massa
    /// de cada tipo de item (`mass_kg`) entra em `item_mass_kg` do catálogo.
    pub fn do_modelo(m: &Model) -> Option<Lateral> {
        let objects: BTreeMap<String, EnvironmentDef> = m
            .objects
            .iter()
            .filter_map(|o| {
                o.environment
                    .as_deref()
                    .map(|e| (o.name.clone(), e.clone()))
            })
            .collect();
        let massas = m.item_types.iter().any(|t| t.mass_kg.is_some());
        if m.environment.is_none() && objects.is_empty() && !massas {
            return None;
        }
        let mut environment = m
            .environment
            .as_deref()
            .cloned()
            .unwrap_or_else(EnvironmentCatalog::inicial);
        for t in &m.item_types {
            if let Some(k) = t.mass_kg {
                environment.item_mass_kg.insert(t.name.clone(), k);
            }
        }
        Some(Lateral {
            environment,
            objects,
        })
    }

    /// O catálogo como ele vai para o modelo: sem as massas dos tipos de item que o modelo
    /// tem (essas vão para `mass_kg` de cada tipo).
    pub fn catalogo_para_o_modelo(&self, m: &Model) -> EnvironmentCatalog {
        let mut c = self.environment.clone();
        c.item_mass_kg
            .retain(|k, _| !m.item_types.iter().any(|t| &t.name == k));
        c
    }
}
