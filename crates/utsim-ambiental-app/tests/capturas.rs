//! Pictures of the windows drawn without a screen: the opening and the Factory screen of the
//! example, with the oven chosen and its "Where it comes from" card open. With
//! `UTSIM_CAPTURAS=<pasta>` the PNGs are saved there for review.

use std::time::{Duration, Instant};

use eframe::egui::{Context, vec2};
use utsim_ambiental_app::{App, Tela};
use utsim_ui_tema::captura::{Captura, Imagem};

fn contexto() -> Context {
    let ctx = Context::default();
    utsim_ui_tema::install(&ctx);
    utsim_ui_tema::usar_cor_do_app(&ctx, utsim_ambiental_app::app::COR_APP);
    ctx
}

fn conferir(img: &Imagem, nome: &str) {
    let mut cores = std::collections::BTreeSet::new();
    for y in (0..img.height).step_by(9) {
        for x in (0..img.width).step_by(9) {
            cores.insert(img.pixel(x, y));
        }
    }
    assert!(cores.len() > 20, "{nome}: a tela quase não pintou nada");
    if let Some(d) = std::env::var_os("UTSIM_CAPTURAS") {
        let d = std::path::PathBuf::from(d);
        std::fs::create_dir_all(&d).unwrap();
        img.salvar_png(&d.join(format!("{nome}.png"))).unwrap();
    }
}

fn pasta(nome: &str) -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("utsim-ambiental-cap-{nome}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&p);
    std::fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn abertura() {
    let d = pasta("abertura");
    let mut app = App::new(Some(d.join("cfg.json")));
    let mut cap = Captura::new(contexto(), vec2(1280.0, 800.0));
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_abertura");
    let _ = std::fs::remove_dir_all(&d);
}

#[test]
fn fabrica_do_exemplo() {
    let d = pasta("fabrica");
    let mut app = App::new(Some(d.join("cfg.json")));
    let modelo = utsim_ambiental_app::projeto::gravar_exemplo(&d.join("exemplo")).unwrap();
    app.abrir(&modelo, Some("estufa"));
    assert_eq!(app.tela, Tela::Fabrica);
    let ctx = contexto();
    let mut cap = Captura::new(ctx.clone(), vec2(1440.0, 860.0));
    let limite = Instant::now() + Duration::from_secs(120);
    loop {
        let _ = cap.imagem(1, |ui| app.draw(ui));
        if app.projeto.as_ref().is_some_and(|p| p.inventario.is_some()) {
            break;
        }
        assert!(Instant::now() < limite, "a simulação não terminou a tempo");
        std::thread::sleep(Duration::from_millis(50));
    }
    // Uma tarifa com pico no meio do turno e demanda contratada.
    {
        let p = app.projeto.as_mut().unwrap();
        let mut t = utsim_ambiental_core::formato::Tariff::inicial();
        t.peak.from = "10:00".into();
        t.peak.to = "12:00".into();
        t.contracted_kw = Some(60.0);
        p.lateral.environment.tariff = Some(t);
        p.recalcular();
    }
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_fabrica");
    app.fabrica.de_onde_veio = Some("estufa".into());
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_de_onde_veio");
    app.fabrica.de_onde_veio = None;
    // "Tocar o dia" parado no pico: o mapa mostra a potência daquele instante.
    let pico = app
        .projeto
        .as_ref()
        .unwrap()
        .inventario
        .as_ref()
        .unwrap()
        .pico()
        .unwrap();
    app.fabrica.toque = Some(pico.0);
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_pico");
    app.fabrica.toque = None;
    app.tela = Tela::Relatorios;
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_relatorios");
    app.tela = Tela::Maquinas;
    ctx.data_mut(|d| {
        d.insert_temp(
            eframe::egui::Id::new("maquinas-escolhida"),
            Some("lavadora".to_string()),
        )
    });
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_maquinas");
    app.fabrica.medida = utsim_ambiental_app::fabrica::Medida::Agua;
    app.fabrica.selecionado = Some("lavadora".into());
    app.tela = Tela::Fabrica;
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_agua");
    for (modo, nome) in [
        (
            utsim_ambiental_core::sankey::Modo::Massa,
            "ambiental_sankey_massa",
        ),
        (
            utsim_ambiental_core::sankey::Modo::Energia,
            "ambiental_sankey_energia",
        ),
        (
            utsim_ambiental_core::sankey::Modo::Agua,
            "ambiental_sankey_agua",
        ),
    ] {
        app.tela = Tela::Fluxos;
        app.fluxos.modo = modo;
        let img = cap.imagem(3, |ui| app.draw(ui));
        conferir(&img, nome);
    }
    app.tela = Tela::Produto;
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_produto");
    {
        let p = app.projeto.as_mut().unwrap();
        p.estudo
            .cenarios
            .push(utsim_ambiental_core::cenarios::Cenario {
                nome: "Estufa desliga na espera".into(),
                alavancas: vec![utsim_ambiental_core::cenarios::Alavanca::Desligamento {
                    no: "estufa".into(),
                    depois: Some(120.0),
                }],
                investimento: Some(5000.0),
                vida_anos: Some(10.0),
            });
        p.estudo.horas_por_ano = Some(4000.0);
        let mut t = p.lateral.environment.tariff.clone().unwrap();
        t.price_kwh_peak = Some(1.2);
        t.price_kwh_off_peak = Some(0.6);
        t.price_kw = Some(30.0);
        t.price_kw_overrun = Some(60.0);
        p.lateral.environment.tariff = Some(t);
        p.recalcular();
    }
    app.tela = Tela::Cenarios;
    app.cenarios.impacto = utsim_ambiental_app::cenarios::Impacto::Energia;
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_cenarios");
    {
        let p = app.projeto.as_mut().unwrap();
        p.lateral
            .environment
            .targets
            .push(utsim_ambiental_core::formato::Target {
                id: "m1".into(),
                indicator: "energia_kwh".into(),
                scope: utsim_ambiental_core::formato::TargetScope::Factory,
                basis: utsim_ambiental_core::formato::Basis::PerGoodItem,
                baseline: Some(1.45),
                target: Some(1.42),
                due: Some("2027-06".into()),
                actions: vec!["Estufa desliga na espera".into()],
            });
        p.recalcular();
    }
    app.tela = Tela::Metas;
    app.metas.aberta = Some("m1".into());
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_metas");
    app.tela = Tela::Fatores;
    let img = cap.imagem(3, |ui| app.draw(ui));
    conferir(&img, "ambiental_fatores");
    let _ = std::fs::remove_dir_all(&d);
}
