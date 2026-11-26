//! Os textos da janela, nas cinco línguas da família: as chaves de
//! `idiomas/<código>/ambiental.ftl`. As telas não escrevem texto literal: leem `tx("chave")`
//! ou `fx("chave", &[("nome", vx(&valor))])`.

use std::fmt::Display;

use utsim_idiomas::{Catalogo, Valor};

/// Os textos do app nas cinco línguas, embutidos de `idiomas/`.
pub static TEXTOS: Catalogo = utsim_idiomas::catalogo!("ambiental");

/// O texto de uma chave na língua de agora.
pub fn tx(chave: &'static str) -> &'static str {
    TEXTOS.t(chave)
}

/// O texto de uma chave com valores com nome.
pub fn fx(chave: &str, args: &[(&str, Valor)]) -> String {
    TEXTOS.f(chave, args)
}

/// Um valor de texto como está.
pub fn vx(x: &impl Display) -> Valor<'static> {
    x.to_string().into()
}

/// Um número com `casas` casas, com a marca decimal da língua.
pub fn num(x: f64, casas: usize) -> String {
    if !x.is_finite() {
        return "–".into();
    }
    utsim_idiomas::marca_decimal(&format!("{x:.casas$}"))
}

/// Casas para mostrar um número sem excesso: 0 a partir de 100, 1 a partir de 10, senão 2
/// (3 abaixo de 0,1).
pub fn casas(x: f64) -> usize {
    let a = x.abs();
    if a >= 100.0 {
        0
    } else if a >= 10.0 {
        1
    } else if a >= 0.1 || a == 0.0 {
        2
    } else {
        3
    }
}

/// Um indicador como "12,3 ± 0,4" (sem intervalo com uma replicação só).
pub fn com_intervalo(i: &utsim_ambiental_core::Indicador) -> String {
    let c = casas(i.media);
    if i.tem_intervalo() {
        format!(
            "{} ± {}",
            num(i.media, c),
            num(i.meia_largura, c.max(casas(i.meia_largura)))
        )
    } else {
        num(i.media, c)
    }
}

#[cfg(test)]
mod testes {
    use super::*;

    /// Os arquivos das cinco línguas têm as mesmas chaves, com os mesmos argumentos.
    #[test]
    fn cinco_linguas_completas() {
        let raiz = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../idiomas");
        let r = utsim_idiomas::conferencia::conferir(&raiz);
        assert!(!r.is_empty());
        let ruins: Vec<_> = r.iter().filter(|x| !x.vazio()).collect();
        assert!(
            ruins.is_empty(),
            "{}",
            utsim_idiomas::conferencia::texto(&r)
        );
    }

    /// Os problemas da validação e as pendências da conta têm texto nas cinco línguas.
    #[test]
    fn chaves_do_nucleo_tem_texto() {
        for c in utsim_ambiental_core::Problema::CHAVES {
            assert!(TEXTOS.tem(&format!("val-{c}")), "val-{c}");
        }
        for c in utsim_ambiental_core::conta::Pendencia::CHAVES {
            assert!(TEXTOS.tem(&format!("pend-{c}")), "pend-{c}");
        }
        for g in utsim_ambiental_core::Grupo::TODOS {
            assert!(TEXTOS.tem(&format!("grupo-{}", g.chave())));
            assert!(TEXTOS.tem(&format!("vilao-{}", g.chave())));
        }
    }

    #[test]
    fn numeros() {
        assert_eq!(casas(123.4), 0);
        assert_eq!(casas(0.05), 3);
    }
}
