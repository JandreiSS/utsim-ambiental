//! O que a simulação mediu de cada nó numa janela de tempo (fase AM3): as horas em cada
//! estado, as peças que entraram e saíram, os setups e as partidas, e a linha do tempo da
//! potência (para a curva de carga e para "Tocar o dia").
//!
//! Com rastro, tudo sai das mudanças de estado e das passagens, em qualquer janela. Sem
//! rastro (só o relatório), as horas saem das frações de tempo da janela de estatística, e
//! setups e partidas ficam de fora.

use std::collections::BTreeMap;

use utsim_core::Time;

use crate::formato::{EnvironmentDef, Grupo};
use crate::rodar::Rodada;

/// O estado sintético do desligamento em espera (`standby`): a máquina esperou mais que
/// `after` e foi desligada; conta com a potência de fora de turno, no grupo esperando.
pub const STANDBY: &str = "standby";

/// Estados em que a máquina está desligada: sair deles para outro conta uma partida.
pub fn desligada(estado: &str) -> bool {
    matches!(estado, "off_shift" | "down" | "stopped" | STANDBY)
}

/// O grupo de um estado, incluindo o desligamento em espera.
pub fn grupo(estado: &str) -> Option<Grupo> {
    if estado == STANDBY {
        Some(Grupo::Esperando)
    } else {
        Grupo::de_estado(estado)
    }
}

/// A potência no estado (o desligamento em espera usa a de fora de turno).
pub fn potencia(d: &EnvironmentDef, estado: &str) -> f64 {
    if estado == STANDBY {
        d.power.off_shift
    } else {
        d.power.no_estado(estado)
    }
}

/// Um trecho da linha do tempo de um nó: de `de` a `ate`, no estado, com a potência.
#[derive(Clone, Debug, PartialEq)]
pub struct Trecho {
    pub de: Time,
    pub ate: Time,
    pub estado: &'static str,
    pub kw: f64,
}

/// A medida de um nó numa replicação.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Medida {
    /// Horas por estado (inclui `standby`).
    pub horas: BTreeMap<&'static str, f64>,
    pub entradas: f64,
    pub saidas: f64,
    /// `None`: sem rastro, não medido.
    pub setups: Option<f64>,
    pub partidas: Option<f64>,
    /// Os trechos dentro da janela (vazio sem rastro).
    pub trechos: Vec<Trecho>,
    /// Instantes de entrada de peças, de início de setup e de partida (para os impulsos de
    /// energia na curva de demanda).
    pub t_entradas: Vec<Time>,
    pub t_setups: Vec<Time>,
    pub t_partidas: Vec<Time>,
}

/// A janela de estatística da replicação: do fim do aquecimento ao fim.
pub fn janela_da_rodada(r: &Rodada) -> (Time, Time) {
    (r.relatorio.stats_from, r.relatorio.end_time)
}

/// A janela pedida, recortada na de estatística da replicação.
pub fn recorte(r: &Rodada, pedida: Option<(Time, Time)>) -> (Time, Time) {
    let (a, b) = janela_da_rodada(r);
    match pedida {
        Some((x, y)) => {
            let x = x.max(a).min(b);
            let y = y.min(b).max(x);
            (x, y)
        }
        None => (a, b),
    }
}

/// Mede o nó `indice` (posição do objeto no modelo) na replicação, na janela `(a, b)`, com
/// `hu` horas por unidade de tempo.
pub fn medir(
    r: &Rodada,
    indice: usize,
    nome: &str,
    d: &EnvironmentDef,
    janela: (Time, Time),
    hu: f64,
) -> Medida {
    let (a, b) = janela;
    let Some(rastro) = &r.rastro else {
        // Só o relatório: frações da janela de estatística.
        let mut m = Medida::default();
        if let Some(bl) = r.relatorio.blocks.iter().find(|x| x.name == nome) {
            let (sa, sb) = janela_da_rodada(r);
            let total = (sb - sa).max(0.0);
            let parte = if total > 0.0 { (b - a) / total } else { 0.0 };
            for (e, f) in &bl.states {
                let nome_estado = estatico(e);
                *m.horas.entry(nome_estado).or_default() += f * (b - a) * hu;
            }
            m.entradas = bl.input as f64 * parte;
            m.saidas = bl.output as f64 * parte;
        }
        return m;
    };

    // A linha do tempo inteira do nó, com o desligamento em espera aplicado.
    let fim = r.relatorio.end_time;
    let mudancas = rastro.estados.get(indice).map_or(&[][..], |v| v.as_slice());
    let mut trechos: Vec<(Time, Time, &'static str)> = Vec::new();
    for (k, (t, e)) in mudancas.iter().enumerate() {
        let ate = mudancas.get(k + 1).map_or(fim, |x| x.0);
        if ate > *t {
            trechos.push((*t, ate, e));
        }
    }
    if let Some(sb) = &d.standby {
        trechos = aplicar_espera(&trechos, sb.after);
    }

    let mut m = Medida {
        setups: Some(0.0),
        partidas: Some(0.0),
        ..Medida::default()
    };
    let mut anterior: Option<&'static str> = None;
    for (t0, t1, e) in &trechos {
        // Transições dentro da janela.
        if *t0 >= a && *t0 < b {
            if *e == "setup" && anterior != Some("setup") {
                *m.setups.get_or_insert(0.0) += 1.0;
                m.t_setups.push(*t0);
            }
            if anterior.is_some_and(desligada) && !desligada(e) {
                *m.partidas.get_or_insert(0.0) += 1.0;
                m.t_partidas.push(*t0);
            }
        }
        anterior = Some(e);
        let x0 = t0.max(a);
        let x1 = t1.min(b);
        if x1 > x0 {
            *m.horas.entry(e).or_default() += (x1 - x0) * hu;
            m.trechos.push(Trecho {
                de: x0,
                ate: x1,
                estado: e,
                kw: potencia(d, e),
            });
        }
    }
    for p in &rastro.passagens {
        if p.t < a || p.t >= b {
            continue;
        }
        if p.para == indice {
            m.entradas += 1.0;
            m.t_entradas.push(p.t);
        }
        if p.de == Some(indice) {
            m.saidas += 1.0;
        }
    }
    m
}

/// Troca a parte de cada espera que passa de `apos` pelo desligamento em espera.
/// Uma espera é uma sequência de trechos seguidos no grupo esperando.
fn aplicar_espera(
    trechos: &[(Time, Time, &'static str)],
    apos: f64,
) -> Vec<(Time, Time, &'static str)> {
    let mut saida = Vec::with_capacity(trechos.len());
    let mut i = 0;
    while i < trechos.len() {
        if Grupo::de_estado(trechos[i].2) != Some(Grupo::Esperando) {
            saida.push(trechos[i]);
            i += 1;
            continue;
        }
        let inicio = trechos[i].0;
        let corte = inicio + apos.max(0.0);
        while i < trechos.len() && Grupo::de_estado(trechos[i].2) == Some(Grupo::Esperando) {
            let (t0, t1, e) = trechos[i];
            if t1 <= corte {
                saida.push((t0, t1, e));
            } else if t0 >= corte {
                match saida.last_mut() {
                    Some(u) if u.2 == STANDBY && u.1 == t0 => u.1 = t1,
                    _ => saida.push((t0, t1, STANDBY)),
                }
            } else {
                saida.push((t0, corte, e));
                saida.push((corte, t1, STANDBY));
            }
            i += 1;
        }
    }
    saida
}

/// O nome do estado como `&'static str` (os nomes do motor; outro nome fica "outro").
fn estatico(e: &str) -> &'static str {
    const NOMES: [&str; 23] = [
        "idle",
        "empty",
        "generating",
        "releasing",
        "collecting",
        "setup",
        "processing",
        "blocked",
        "waiting_operator",
        "waiting_transport",
        "down",
        "off_shift",
        "travel_empty",
        "travel_loaded",
        "loading",
        "unloading",
        "utilize",
        "stopped",
        "waiting_repair",
        "maintenance",
        "resting",
        "entering",
        "leaving",
    ];
    NOMES.into_iter().find(|n| *n == e).unwrap_or("outro")
}

/// A curva de carga: `(instante, kW)` a cada mudança da soma das potências dos trechos.
pub fn curva(trechos: &[&[Trecho]]) -> Vec<(Time, f64)> {
    let mut passos: Vec<(Time, f64)> = Vec::new();
    for ts in trechos {
        for t in *ts {
            if t.kw != 0.0 {
                passos.push((t.de, t.kw));
                passos.push((t.ate, -t.kw));
            }
        }
    }
    passos.sort_by(|x, y| x.0.total_cmp(&y.0));
    let mut saida: Vec<(Time, f64)> = Vec::new();
    let mut p = 0.0;
    for (t, d) in passos {
        p += d;
        let p = if p.abs() < 1e-9 { 0.0 } else { p };
        match saida.last_mut() {
            Some(u) if (u.0 - t).abs() < 1e-12 => u.1 = p,
            _ => saida.push((t, p)),
        }
    }
    saida
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn espera_longa_vira_desligamento() {
        // idle 0..100, processing 100..150, idle 150..170 (curta), blocked 170..180.
        let t = [
            (0.0, 100.0, "idle"),
            (100.0, 150.0, "processing"),
            (150.0, 170.0, "idle"),
            (170.0, 180.0, "blocked"),
        ];
        let s = aplicar_espera(&t, 30.0);
        assert_eq!(
            s,
            vec![
                (0.0, 30.0, "idle"),
                (30.0, 100.0, STANDBY),
                (100.0, 150.0, "processing"),
                (150.0, 170.0, "idle"),
                (170.0, 180.0, "blocked"),
            ]
        );
    }

    #[test]
    fn curva_soma_os_trechos() {
        let a = [Trecho {
            de: 0.0,
            ate: 10.0,
            estado: "processing",
            kw: 5.0,
        }];
        let b = [
            Trecho {
                de: 5.0,
                ate: 8.0,
                estado: "idle",
                kw: 2.0,
            },
            Trecho {
                de: 8.0,
                ate: 12.0,
                estado: "processing",
                kw: 3.0,
            },
        ];
        let c = curva(&[&a, &b]);
        assert_eq!(
            c,
            vec![(0.0, 5.0), (5.0, 7.0), (8.0, 8.0), (10.0, 3.0), (12.0, 0.0)]
        );
    }
}
