//! A camada Ambiental da cena 3D (fase AM13, plano 4.8): o que a cena em Godot desenha
//! sobre a reprodução de uma replicação.
//!
//! - o brilho no piso em volta de cada máquina, na cor da energia dela na janela;
//! - a coluna sobre a máquina, com a altura proporcional à potência no instante;
//! - a mochila de cada peça refugada (o cartão que abre ao clicar na peça);
//! - o medidor da fábrica: a potência instantânea, o pico e se o instante cai na janela de
//!   pico da tarifa.
//!
//! A conta é a mesma do app ([`avaliar`] sobre a replicação com rastro de
//! [`simular_uma`]): a potência da fábrica num instante é o degrau da curva de carga do
//! app nesse instante. A cena só lê estes números: nenhuma conta é feita no Godot.

use utsim_core::Time;
use utsim_model::Model;

use crate::balanco::MochilaPeca;
use crate::conta::{Inventario, avaliar};
use crate::formato::{Clock, Lateral, Tariff};
use crate::medida::Trecho;
use crate::rodar::{Rodada, simular_uma};

/// Um nó com bloco ambiental, pronto para a cena.
#[derive(Clone, Debug, PartialEq)]
pub struct No3d {
    pub nome: String,
    /// A posição do objeto no modelo (o índice do bloco na cena).
    pub indice: usize,
    /// Energia e energia sem produzir na janela de estatística, em kWh.
    pub energia_kwh: f64,
    pub sem_valor_kwh: f64,
    /// `None`: falta fator.
    pub co2e_kg: Option<f64>,
    /// A maior potência do nó na janela, em kW.
    pub kw_max: f64,
    /// A energia do nó dividida pela do nó que mais gasta (0 a 1): a cor do brilho.
    pub fracao_energia: f64,
}

/// A camada de uma replicação.
#[derive(Clone, Debug, PartialEq)]
pub struct Camada3d {
    pub nos: Vec<No3d>,
    /// Os nomes de todos os objetos do modelo, pela posição (para o caminho da mochila).
    pub nomes: Vec<String>,
    /// A curva de carga da fábrica: `(instante, kW)` a cada mudança (a mesma do app).
    pub curva: Vec<(Time, f64)>,
    /// A janela de estatística (do fim do aquecimento ao fim): a curva vale nela.
    pub janela: (Time, Time),
    /// O primeiro instante da potência máxima e a potência.
    pub pico: Option<(Time, f64)>,
    /// A maior potência de um nó (a escala das colunas).
    pub kw_max_no: f64,
    pub energia_kwh: f64,
    pub co2e_kg: Option<f64>,
    /// As peças refugadas na janela, com a mochila de cada uma.
    pub refugo: Vec<MochilaPeca>,
    trechos: Vec<Vec<Trecho>>,
    tarifa: Option<(Tariff, Clock)>,
}

impl Camada3d {
    /// Roda a replicação `replicacao` do modelo com rastro e monta a camada.
    pub fn calcular(modelo: &Model, lateral: &Lateral, replicacao: u32) -> Result<Self, String> {
        let r = simular_uma(modelo, replicacao)?;
        Ok(Self::de_rodada(modelo, lateral, &r))
    }

    /// A camada de uma replicação já rodada (com rastro).
    pub fn de_rodada(modelo: &Model, lateral: &Lateral, r: &Rodada) -> Self {
        let inv = avaliar(modelo, lateral, std::slice::from_ref(r), None);
        Self::do_inventario(modelo, lateral, &inv)
    }

    fn do_inventario(modelo: &Model, lateral: &Lateral, inv: &Inventario) -> Self {
        let mut nos = Vec::new();
        let mut trechos = Vec::new();
        for n in inv.nos.iter().filter(|n| n.tem_bloco) {
            let Some(indice) = modelo.objects.iter().position(|o| o.name == n.nome) else {
                continue;
            };
            let ts = inv.trechos.get(&n.nome).cloned().unwrap_or_default();
            let kw_max = ts.iter().map(|t| t.kw).fold(0.0, f64::max);
            nos.push(No3d {
                nome: n.nome.clone(),
                indice,
                energia_kwh: n.energia.media,
                sem_valor_kwh: n.sem_valor.media,
                co2e_kg: n.co2e.valor().map(|i| i.media),
                kw_max,
                fracao_energia: 0.0,
            });
            trechos.push(ts);
        }
        let emax = nos.iter().map(|n| n.energia_kwh).fold(0.0, f64::max);
        for n in &mut nos {
            n.fracao_energia = if emax > 0.0 {
                n.energia_kwh / emax
            } else {
                0.0
            };
        }
        let kw_max_no = nos.iter().map(|n| n.kw_max).fold(0.0, f64::max);
        Camada3d {
            nos,
            nomes: modelo.objects.iter().map(|o| o.name.clone()).collect(),
            curva: inv.curva.clone(),
            janela: inv.janela_total,
            pico: inv.pico(),
            kw_max_no,
            energia_kwh: inv.energia.media,
            co2e_kg: inv.co2e.valor().map(|i| i.media),
            refugo: inv
                .materiais
                .as_ref()
                .map(|m| m.pecas.clone())
                .unwrap_or_default(),
            trechos,
            tarifa: lateral
                .environment
                .tariff
                .clone()
                .map(|t| (t, lateral.environment.clock.clone())),
        }
    }

    /// O instante está na janela de estatística (fora dela a curva não vale).
    pub fn na_janela(&self, t: Time) -> bool {
        t >= self.janela.0 && t < self.janela.1
    }

    /// A potência da fábrica em `t`, em kW: o degrau da curva de carga (`None` fora da
    /// janela de estatística).
    pub fn potencia(&self, t: Time) -> Option<f64> {
        if !self.na_janela(t) {
            return None;
        }
        let k = self.curva.partition_point(|(x, _)| *x <= t);
        Some(if k == 0 { 0.0 } else { self.curva[k - 1].1 })
    }

    /// A potência e o estado de cada nó de [`Self::nos`] em `t` (0 kW e `""` fora da janela).
    pub fn potencias(&self, t: Time) -> Vec<(f64, &'static str)> {
        self.trechos
            .iter()
            .map(|ts| {
                let k = ts.partition_point(|x| x.de <= t);
                match k.checked_sub(1).map(|i| &ts[i]) {
                    Some(x) if t < x.ate => (x.kw, x.estado),
                    _ => (0.0, ""),
                }
            })
            .collect()
    }

    /// A altura da coluna de cada nó em `t`, de 0 a 1 (a potência sobre a maior potência
    /// de um nó).
    pub fn alturas(&self, t: Time) -> Vec<f64> {
        self.potencias(t)
            .into_iter()
            .map(|(kw, _)| {
                if self.kw_max_no > 0.0 {
                    (kw / self.kw_max_no).clamp(0.0, 1.0)
                } else {
                    0.0
                }
            })
            .collect()
    }

    /// O instante cai na janela de pico da tarifa? `None`: o modelo não tem tarifa.
    pub fn em_pico(&self, t: Time) -> Option<bool> {
        self.tarifa
            .as_ref()
            .map(|(tar, rel)| crate::tarifa::em_pico(tar, rel, t))
    }

    /// A mochila da peça `item`, se ela foi refugada na janela.
    pub fn mochila(&self, item: u64) -> Option<&MochilaPeca> {
        self.refugo.iter().find(|p| p.item == item)
    }

    /// Os nomes dos nós do caminho de uma peça.
    pub fn caminho(&self, p: &MochilaPeca) -> Vec<String> {
        p.caminho
            .iter()
            .filter_map(|i| self.nomes.get(*i).cloned())
            .collect()
    }
}

/// A cor de uma fração de 0 a 1 numa escala sequencial de um só matiz (verde), do claro ao
/// escuro: a ordem fica legível também para quem não distingue cores.
pub fn cor(fracao: f64) -> [f32; 3] {
    let x = fracao.clamp(0.0, 1.0) as f32;
    let claro = [0.85, 0.95, 0.80];
    let escuro = [0.0, 0.38, 0.17];
    [
        claro[0] + (escuro[0] - claro[0]) * x,
        claro[1] + (escuro[1] - claro[1]) * x,
        claro[2] + (escuro[2] - claro[2]) * x,
    ]
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::formato::{Driver, EnvironmentDef, Factor, FlowUse};

    fn modelo() -> Model {
        Model::parse(
            r#"Model(name: "m", run: (stop_time: 7200.0, warmup: 600.0, seed: 9, replications: 3),
               objects: [(name: "chegada", kind: Source(interarrival: Exp(mean: 60.0))),
                         (name: "corte", kind: Processor(process: Exp(mean: 40.0))),
                         (name: "pintura", kind: Processor(process: Exp(mean: 30.0))),
                         (name: "inspecao", kind: Processor(process: Const(5.0)),
                          output: Weighted([0.8, 0.2])),
                         (name: "fim", kind: Sink),
                         (name: "refugo", kind: Sink, sink_role: Some(Scrap))],
               connections: [("chegada", "corte"), ("corte", "pintura"), ("pintura", "inspecao"),
                             ("inspecao", "fim"), ("inspecao", "refugo")])"#,
        )
        .unwrap()
    }

    fn lateral() -> Lateral {
        let mut l = Lateral {
            environment: crate::formato::EnvironmentCatalog::inicial(),
            ..Lateral::default()
        };
        let mut corte = EnvironmentDef::default();
        corte.power.producing = 12.0;
        corte.power.waiting = 4.0;
        let mut pintura = EnvironmentDef::default();
        pintura.power.producing = 8.0;
        pintura.power.waiting = 5.0;
        pintura.flows.push(FlowUse {
            flow: "tinta".into(),
            amount: 0.05,
            driver: Driver::Item,
            adds_mass: false,
            removes_mass: false,
        });
        l.objects.insert("corte".into(), corte);
        l.objects.insert("pintura".into(), pintura);
        let el = l
            .environment
            .fluxo_eletrico()
            .map(|f| f.name.clone())
            .unwrap();
        for (f, k) in [(el.as_str(), 0.09), ("tinta", 2.0)] {
            l.environment.factors.retain(|x| x.flow != f);
            l.environment.factors.push(Factor {
                flow: f.into(),
                scope: 2,
                kg_co2e: Some(k),
                source: "teste".into(),
                year: None,
                range: None,
                didactic: false,
            });
        }
        l
    }

    /// A potência da cena em cada instante é a da curva de carga do app (as replicações
    /// juntas, a primeira na curva), no mesmo instante.
    #[test]
    fn mesma_potencia_que_a_curva_do_app() {
        let m = modelo();
        let l = lateral();
        let app = avaliar(&m, &l, &crate::rodar::simular(&m).unwrap(), None);
        let c = Camada3d::calcular(&m, &l, 0).unwrap();
        assert!(app.curva.len() > 10);
        assert_eq!(c.curva, app.curva);
        for w in app.curva.windows(2) {
            let (t0, p0) = w[0];
            let t1 = w[1].0;
            for t in [t0, (t0 + t1) / 2.0] {
                if c.na_janela(t) {
                    assert_eq!(c.potencia(t), Some(p0), "t = {t}");
                    // A soma das colunas é o medidor da fábrica.
                    let soma: f64 = c.potencias(t).iter().map(|x| x.0).sum();
                    assert!((soma - p0).abs() < 1e-9, "t = {t}: {soma} != {p0}");
                }
            }
        }
        assert_eq!(c.potencia(c.janela.0 - 1.0), None);
        assert_eq!(c.pico, app.pico());
        assert_eq!(c.nos.len(), 2);
        assert!(c.nos.iter().any(|n| (n.fracao_energia - 1.0).abs() < 1e-12));
        assert!(
            c.alturas(c.janela.0 + 100.0)
                .iter()
                .all(|h| (0.0..=1.0).contains(h))
        );
        assert_eq!(c.em_pico(1000.0), None);
    }

    /// A mochila de cada peça refugada soma a mochila da replicação do app.
    #[test]
    fn mochilas_somam_a_do_app() {
        let m = modelo();
        let l = lateral();
        let rod = crate::rodar::simular(&m).unwrap();
        let app = avaliar(&m, &l, &rod, None);
        let c = Camada3d::de_rodada(&m, &l, &rod[0]);
        assert!(!c.refugo.is_empty());
        let kwh: f64 = c.refugo.iter().map(|p| p.kwh).sum();
        let co2: f64 = c.refugo.iter().map(|p| *p.co2e_kg.as_ref().unwrap()).sum();
        assert!((kwh - app.metricas["ambiental.mochila_refugo.kwh"][0]).abs() < 1e-9);
        assert!((co2 - app.metricas["ambiental.mochila_refugo.co2e_kg"][0]).abs() < 1e-9);
        let p = &c.refugo[0];
        assert_eq!(c.mochila(p.item), Some(p));
        assert_eq!(c.caminho(p), vec!["corte", "pintura", "inspecao"]);
        assert!((p.fluxos["tinta"] - 0.05).abs() < 1e-12);
    }

    #[test]
    fn cor_escurece_com_o_valor() {
        let a = cor(0.0);
        let b = cor(1.0);
        assert!(a.iter().sum::<f32>() > b.iter().sum::<f32>());
        assert_eq!(cor(2.0), b);
    }
}
