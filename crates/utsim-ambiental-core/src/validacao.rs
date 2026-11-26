//! A validação do bloco ambiental (plano 4.2.4). Cada problema é uma chave de texto
//! (`val-<chave>` em `ambiental.ftl`) com os valores com nome, para o app mostrar na língua
//! de quem usa; `Display` dá o texto em português.

use std::fmt;

use utsim_model::Model;

use crate::formato::{Direction, Driver, FlowKind, Lateral};

/// Um problema do bloco ambiental: grave (a conta não confia) ou só um aviso.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Problema {
    pub grave: bool,
    pub chave: &'static str,
    pub args: Vec<(&'static str, String)>,
}

impl Problema {
    fn erro(chave: &'static str, args: &[(&'static str, &str)]) -> Self {
        Problema {
            grave: true,
            chave,
            args: args.iter().map(|(k, v)| (*k, v.to_string())).collect(),
        }
    }

    fn aviso(chave: &'static str, args: &[(&'static str, &str)]) -> Self {
        Problema {
            grave: false,
            ..Self::erro(chave, args)
        }
    }

    /// Todas as chaves, para conferir que as cinco línguas têm cada uma.
    pub const CHAVES: &'static [&'static str] = &[
        "objeto_inexistente",
        "fluxo_inexistente",
        "fluxo_de_saida_como_entrada",
        "negativo",
        "agua_fracao",
        "agua_soma",
        "reuso_inexistente",
        "parametro_inexistente",
        "pico_fora_do_dia",
        "intervalo_zero",
        "massa_sem_tipo",
        "sem_fluxo_eletrico",
        "fluxo_repetido",
        "escopo_invalido",
    ];
}

impl fmt::Display for Problema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = |k: &str| {
            self.args
                .iter()
                .find(|(n, _)| *n == k)
                .map_or("", |(_, v)| v.as_str())
        };
        let t = match self.chave {
            "objeto_inexistente" => format!("o objeto {} não existe no modelo", a("no")),
            "fluxo_inexistente" => {
                format!("{}: o fluxo {} não está no catálogo", a("no"), a("fluxo"))
            }
            "fluxo_de_saida_como_entrada" => format!(
                "{}: o fluxo {} é de saída e soma massa à peça",
                a("no"),
                a("fluxo")
            ),
            "negativo" => format!("{}: {} não pode ser negativo", a("no"), a("campo")),
            "agua_fracao" => format!("{}: as frações de água ficam entre 0 e 1", a("no")),
            "agua_soma" => format!("{}: as frações de água somam mais que 1", a("no")),
            "reuso_inexistente" => format!(
                "{}: o reuso vai para {}, que não existe",
                a("no"),
                a("alvo")
            ),
            "parametro_inexistente" => format!(
                "{}: o parâmetro de efluente {} não está no catálogo",
                a("no"),
                a("parametro")
            ),
            "pico_fora_do_dia" => "a janela de pico precisa ficar dentro de um dia (HH:MM)".into(),
            "intervalo_zero" => "o intervalo de demanda precisa ser maior que zero".into(),
            "massa_sem_tipo" => format!(
                "{}: o fluxo soma ou tira massa, mas os tipos de item não têm massa",
                a("no")
            ),
            "sem_fluxo_eletrico" => {
                "o catálogo não tem fluxo de energia: a potência não vira emissão".into()
            }
            "fluxo_repetido" => format!("o fluxo {} aparece duas vezes no catálogo", a("fluxo")),
            "escopo_invalido" => format!("o fator de {} tem escopo fora de 1, 2 ou 3", a("fluxo")),
            outra => outra.to_string(),
        };
        f.write_str(&t)
    }
}

/// Os problemas do bloco ambiental contra o modelo. Lista vazia: tudo certo.
pub fn validar(modelo: &Model, lateral: &Lateral) -> Vec<Problema> {
    let mut v = Vec::new();
    let cat = &lateral.environment;
    let existe = |n: &str| modelo.objects.iter().any(|o| o.name == n);

    for (i, f) in cat.flows.iter().enumerate() {
        if cat.flows[..i].iter().any(|g| g.name == f.name) {
            v.push(Problema::erro("fluxo_repetido", &[("fluxo", &f.name)]));
        }
    }
    for f in &cat.factors {
        if !(1..=3).contains(&f.scope) {
            v.push(Problema::erro("escopo_invalido", &[("fluxo", &f.flow)]));
        }
        if f.kg_co2e.is_some_and(|x| x < 0.0) {
            v.push(Problema::erro(
                "negativo",
                &[("no", &f.flow), ("campo", "kg_co2e")],
            ));
        }
    }
    if !cat.flows.iter().any(|f| f.kind == FlowKind::Energy) {
        v.push(Problema::aviso("sem_fluxo_eletrico", &[]));
    }
    if let Some(t) = &cat.tariff {
        if hora(&t.peak.from).is_none() || hora(&t.peak.to).is_none() {
            v.push(Problema::erro("pico_fora_do_dia", &[]));
        }
        if t.demand_interval_min == 0 {
            v.push(Problema::erro("intervalo_zero", &[]));
        }
    }
    let tem_massa = false; // `mass_kg` ainda não existe nos tipos de item (fase AM5).

    for (no, d) in &lateral.objects {
        if !existe(no) {
            v.push(Problema::aviso("objeto_inexistente", &[("no", no)]));
        }
        let p = &d.power;
        let potencias = [
            ("producing", p.producing),
            ("preparing", p.preparing),
            ("waiting", p.waiting),
            ("stopped", p.stopped),
            ("off_shift", p.off_shift),
        ];
        for (campo, x) in potencias {
            if x < 0.0 {
                v.push(Problema::erro("negativo", &[("no", no), ("campo", campo)]));
            }
        }
        for (estado, x) in &p.by_state {
            if *x < 0.0 {
                v.push(Problema::erro("negativo", &[("no", no), ("campo", estado)]));
            }
        }
        for e in &d.energy_per {
            if e.kwh < 0.0 {
                v.push(Problema::erro("negativo", &[("no", no), ("campo", "kwh")]));
            }
        }
        for f in &d.flows {
            match cat.fluxo(&f.flow) {
                None => v.push(Problema::erro(
                    "fluxo_inexistente",
                    &[("no", no), ("fluxo", &f.flow)],
                )),
                Some(def) => {
                    if f.adds_mass && def.direction == Direction::Out {
                        v.push(Problema::erro(
                            "fluxo_de_saida_como_entrada",
                            &[("no", no), ("fluxo", &f.flow)],
                        ));
                    }
                }
            }
            if f.amount < 0.0 {
                v.push(Problema::erro(
                    "negativo",
                    &[("no", no), ("campo", &f.flow)],
                ));
            }
            if let Driver::Distance { loaded, empty } = f.driver
                && (loaded < 0.0 || empty < 0.0)
            {
                v.push(Problema::erro(
                    "negativo",
                    &[("no", no), ("campo", &f.flow)],
                ));
            }
            if (f.adds_mass || f.removes_mass) && !tem_massa {
                v.push(Problema::aviso("massa_sem_tipo", &[("no", no)]));
            }
        }
        if let Some(w) = &d.water {
            let fr = [w.incorporated, w.evaporated, w.reuse];
            if fr.iter().any(|x| !(0.0..=1.0).contains(x)) {
                v.push(Problema::erro("agua_fracao", &[("no", no)]));
            } else if w.incorporated + w.evaporated > 1.0 + 1e-9 {
                v.push(Problema::erro("agua_soma", &[("no", no)]));
            }
            if let Some(alvo) = &w.reuse_to
                && !existe(alvo)
            {
                v.push(Problema::erro(
                    "reuso_inexistente",
                    &[("no", no), ("alvo", alvo)],
                ));
            }
        }
        if let Some(e) = &d.effluent {
            for p in e.concentration.keys() {
                if !cat.effluent.parameters.iter().any(|q| &q.name == p) {
                    v.push(Problema::erro(
                        "parametro_inexistente",
                        &[("no", no), ("parametro", p)],
                    ));
                }
            }
        }
    }
    v
}

/// `"HH:MM"` em minutos desde a meia-noite, dentro de um dia.
pub fn hora(s: &str) -> Option<u32> {
    let (h, m) = s.trim().split_once(':')?;
    let (h, m): (u32, u32) = (h.parse().ok()?, m.parse().ok()?);
    (h < 24 && m < 60).then_some(h * 60 + m)
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::formato::{EnvironmentCatalog, EnvironmentDef, FlowUse, WaterDef};

    fn modelo() -> Model {
        Model::parse(
            r#"Model(name: "m", run: (stop_time: 10.0, warmup: 0.0, seed: 1, replications: 1),
               objects: [(name: "a", kind: Source(interarrival: Const(1.0))),
                         (name: "b", kind: Sink)],
               connections: [("a", "b")])"#,
        )
        .unwrap()
    }

    #[test]
    fn catalogo_inicial_passa() {
        let l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        assert!(validar(&modelo(), &l).is_empty());
    }

    #[test]
    fn acusa_fluxo_ausente_agua_e_negativo() {
        let mut l = Lateral {
            environment: EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        let mut d = EnvironmentDef::default();
        d.power.waiting = -1.0;
        d.flows.push(FlowUse {
            flow: "gas".into(),
            amount: 1.0,
            driver: Driver::Item,
            adds_mass: false,
            removes_mass: false,
        });
        d.water = Some(WaterDef {
            incorporated: 0.7,
            evaporated: 0.6,
            reuse_to: Some("x".into()),
            reuse: 0.0,
        });
        l.objects.insert("b".into(), d);
        l.objects
            .insert("fantasma".into(), EnvironmentDef::default());
        let chaves: Vec<_> = validar(&modelo(), &l).iter().map(|p| p.chave).collect();
        for c in [
            "negativo",
            "fluxo_inexistente",
            "agua_soma",
            "reuso_inexistente",
            "objeto_inexistente",
        ] {
            assert!(chaves.contains(&c), "faltou {c} em {chaves:?}");
        }
    }

    #[test]
    fn relogio_do_modelo() {
        let c = crate::formato::Clock::default();
        assert_eq!(c.hora(3600.0), "07:00");
        assert_eq!(c.hora(20.0 * 3600.0), "02:00 +1");
    }

    #[test]
    fn hora_do_dia() {
        assert_eq!(hora("18:30"), Some(1110));
        assert_eq!(hora("24:00"), None);
        assert_eq!(hora("x"), None);
    }
}
