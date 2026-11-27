//! A conversa com os outros apps (fase AM11): o pedido de análise para o UTSim Estatística,
//! a oportunidade para o UTSim LSS e a leitura das respostas. Os pedidos ficam na pasta
//! `troca/` ao lado do modelo; o app chamado abre pelo launcher (ou ao lado deste programa).

use std::path::{Path, PathBuf};

use utsim_ambiental_core::Inventario;
use utsim_ambiental_core::cenarios::Cenario;
use utsim_ambiental_core::troca::{self, Oportunidade, Teste};

use crate::projeto::Projeto;
use crate::texto::{fx, tx, vx};

fn de() -> String {
    format!("ambiental {}", env!("CARGO_PKG_VERSION"))
}

/// O programa de um produto: a versão ativa do launcher, senão ao lado deste programa ou no
/// `PATH`.
pub fn localizar(produto: &str, programa: &str) -> Option<(PathBuf, PathBuf)> {
    use utsim_launcher_core::install::{Layout, launch_command};
    if let Some(raiz) = Layout::default_root()
        && let Ok((exe, _args, dir)) = launch_command(&Layout::new(raiz), produto)
    {
        return Some((exe, dir));
    }
    let exe = utsim_troca::localizar_programa(programa)?;
    let dir = exe.parent()?.to_path_buf();
    Some((exe, dir))
}

fn abrir(produto: &str, programa: &str, args: &[String]) -> bool {
    match localizar(produto, programa) {
        Some((exe, dir)) => std::process::Command::new(exe)
            .args(args)
            .current_dir(dir)
            .spawn()
            .is_ok(),
        None => false,
    }
}

fn pasta_de(c: &Path) -> String {
    c.parent()
        .map_or_else(String::new, |p| p.display().to_string())
}

/// Grava o CSV pareado e o pedido `analisar`; abre a Estatística com o CSV.
pub fn enviar_estatistica(
    p: &mut Projeto,
    cenario: &str,
    metrica: &str,
    base: &[f64],
    cen: &[f64],
) -> (String, bool) {
    match troca::pedido_analise(&de(), &p.caminho, cenario, metrica, base, cen) {
        Ok((ped, c)) => {
            // A Estatística instalada abre o CSV (argumentos diretos); quando ela atender
            // `analisar`, o pedido inteiro vai junto e a resposta volta para cá.
            let mut args = utsim_troca::Pedido::argumentos(&c);
            args.extend(
                ped.entrada
                    .arquivos
                    .iter()
                    .map(|a| format!("--utsim-abrir={}", a.caminho.display())),
            );
            let aberto = abrir("estatistica", "utsim-estatistica", &args);
            p.pedidos.push((cenario.to_string(), ped));
            let chave = if aberto {
                "cen-enviado_estatistica"
            } else {
                "cen-pedido_gravado"
            };
            (fx(chave, &[("pasta", vx(&pasta_de(&c)))]), false)
        }
        Err(e) => (fx("app-erro_salvar", &[("e", vx(&e))]), true),
    }
}

/// Grava a oportunidade para o LSS e abre o LSS com o pedido.
pub fn levar_lss(
    p: &mut Projeto,
    c: &Cenario,
    base: &Inventario,
    inv: &Inventario,
) -> (String, bool) {
    // O arquivo dos cenários vai junto, com hash: grava antes.
    if let Err(e) = utsim_ambiental_core::cenarios::gravar(&p.caminho, &p.estudo) {
        return (fx("app-erro_salvar", &[("e", vx(&e))]), true);
    }
    let op = Oportunidade {
        cenario: c.nome.clone(),
        alavancas: c.alavancas.iter().map(|a| format!("{a:?}")).collect(),
        diferencas: troca::diferencas(base, inv),
        investimento: c.investimento,
        vida_anos: c.vida_anos,
        linha_vsm: troca::linha_vsm(inv),
        testes: c.testes.clone(),
    };
    match troca::pedido_oportunidade(&de(), &p.caminho, &op) {
        Ok((_, caminho)) => {
            let aberto = abrir(
                "lean-seis-sigma",
                "utsim-seis-sigma",
                &utsim_troca::Pedido::argumentos(&caminho),
            );
            let chave = if aberto {
                "cen-enviado_lss"
            } else {
                "cen-pedido_gravado"
            };
            (fx(chave, &[("pasta", vx(&pasta_de(&caminho)))]), false)
        }
        Err(e) => (fx("app-erro_salvar", &[("e", vx(&e))]), true),
    }
}

/// Lê as respostas que chegaram e grava o teste no cenário.
pub fn vigiar(p: &mut Projeto) -> Option<(String, bool)> {
    let mut msg = None;
    let mut fica = Vec::new();
    for (cenario, ped) in std::mem::take(&mut p.pedidos) {
        match utsim_troca::Resposta::ler(&ped) {
            Ok(Some(r)) => {
                if let Some(t) = r
                    .resultado
                    .as_ref()
                    .and_then(|x| teste_da_resposta(&x.dados))
                    && let Some(c) = p.estudo.cenarios.iter_mut().find(|c| c.nome == cenario)
                {
                    c.testes
                        .retain(|y| !(y.metrica == t.metrica && y.origem == t.origem));
                    c.testes.push(t);
                }
                msg = Some((fx("cen-resposta", &[("cenario", vx(&cenario))]), false));
            }
            Ok(None) => fica.push((cenario, ped)),
            Err(e) => msg = Some((fx("app-erro_abrir", &[("e", vx(&e))]), true)),
        }
    }
    p.pedidos = fica;
    msg
}

/// O teste que a Estatística devolve em `dados`: `{ "metrica", "diferenca", "de", "ate",
/// "p_valor", "n" }` (docs/troca_ambiental.md).
fn teste_da_resposta(d: &serde_json::Value) -> Option<Teste> {
    let mut t: Teste = serde_json::from_value(d.clone()).ok()?;
    t.origem = tx("cen-origem_estatistica").to_string();
    Some(t)
}
