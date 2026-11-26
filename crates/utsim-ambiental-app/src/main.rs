//! `utsim-ambiental`: abre a janela do UTSim Ambiental.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::ExitCode;

use utsim_ambiental_app::{App, ICONE, TITULO};

/// O valor de `--<nome>=<valor>`, em qualquer posição.
fn argumento(nome: &str) -> Option<String> {
    let prefixo = format!("--{nome}=");
    std::env::args()
        .find_map(|a| a.strip_prefix(&prefixo).map(str::to_string))
        .filter(|v| !v.is_empty())
}

fn main() -> ExitCode {
    // A primeira vez segue a língua do sistema; a guardada vale por cima (`App::new`).
    utsim_idiomas::definir(utsim_idiomas::Idioma::inicial());
    if let Some(c) = utsim_ui_tema::janela::versao_ou_ajuda(
        std::env::args().nth(1).as_deref(),
        "utsim-ambiental",
        env!("CARGO_PKG_VERSION"),
        "utsim-ambiental [--utsim-abrir=<modelo.ron>] [--utsim-no=<nome>]\n  abre a janela do \
         UTSim Ambiental; com --utsim-abrir, já no modelo (e no nó, com --utsim-no)",
    ) {
        return c;
    }
    let abrir = argumento("utsim-abrir");
    let no = argumento("utsim-no");
    let options = eframe::NativeOptions {
        viewport: utsim_ui_tema::janela::viewport(
            TITULO,
            [1240.0, 800.0],
            [860.0, 540.0],
            Some(ICONE),
        ),
        ..Default::default()
    };
    let result = eframe::run_native(
        TITULO,
        options,
        Box::new(move |cc| {
            utsim_ui_tema::install(&cc.egui_ctx);
            utsim_ui_tema::usar_cor_do_app(&cc.egui_ctx, utsim_ambiental_app::app::COR_APP);
            let mut app = App::new(utsim_ambiental_app::config::arquivo());
            if let Some(c) = abrir {
                app.abrir(std::path::Path::new(&c), no.as_deref());
            }
            Ok(Box::new(app))
        }),
    );
    utsim_ui_tema::janela::saida(result)
}
