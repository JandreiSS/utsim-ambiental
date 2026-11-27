//! UTSim Ambiental, a janela (`docs/plano_ambiental.md`, fases AM2 e parte da AM3): abrir um
//! modelo do UTSim, informar a potência das máquinas, simular, ver o mapa de calor de
//! energia e de CO₂ equivalente, achar os vilões, abrir a conta de qualquer número ("De
//! onde veio") e comparar uma mudança com a base, com intervalo.
//!
//! O app não faz conta: tudo vem de `utsim-ambiental-core`. Os módulos sem janela
//! (`projeto`, `config`, `texto`) são testados sem renderizador.

pub mod app;
pub mod config;
pub mod fabrica;
pub mod fatores;
pub mod fluxos;
pub mod maquinas;
pub mod projeto;
pub mod relatorio;
pub mod texto;

pub use app::{App, Tela};

pub const TITULO: &str = "UTSim Ambiental";

/// O ícone do app (o mesmo do lançador), na janela e na abertura.
pub const ICONE: &[u8] = include_bytes!("../../../assets/icone/familia/ambiental_256.png");
/// A cor do app (launcher e capa dos relatórios): verde (decisão A2).
pub const COR: &str = "#2FA24A";

/// O modelo de exemplo, gravado numa pasta do usuário quando ele pede "Abrir o exemplo".
pub const EXEMPLO_MODELO: &str = include_str!("../../../exemplos/linha_de_pintura.ron");
pub const EXEMPLO_LATERAL: &str = include_str!("../../../exemplos/linha_de_pintura.ambiental.ron");
