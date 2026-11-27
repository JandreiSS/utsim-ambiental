//! Demanda, horário de pico e a conta de energia simulada (fase AM4, plano 2.2).
//!
//! - **Demanda** de um intervalo (padrão 15 minutos, alinhado ao relógio do modelo): a
//!   energia do intervalo dividida pela duração dele. Nas bordas da janela, o intervalo
//!   incompleto usa a duração que está dentro da janela.
//! - **Horário de pico**: um intervalo é de pico quando o meio dele cai na janela de pico do
//!   dia (`from` a `to`). Com `weekdays_only`, só de segunda a sexta, e o primeiro dia da
//!   simulação é uma segunda-feira.
//! - **Fator de carga**: a potência média da janela dividida pela demanda máxima.
//! - **Conta de energia simulada** da janela: energia no pico e fora dele vezes os preços,
//!   mais a demanda cobrada (a contratada ou a medida, a maior) vezes o preço da demanda,
//!   mais a ultrapassagem (o que passa da contratada vezes o preço da ultrapassagem). Sem
//!   todos os preços que a conta usa, não há conta (nunca um preço inventado).

use utsim_core::Time;

use crate::formato::{Clock, Tariff};
use crate::validacao::hora;

/// Um intervalo de demanda.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intervalo {
    pub de: Time,
    pub ate: Time,
    /// Demanda média do intervalo, em kW.
    pub kw: f64,
    pub kwh: f64,
    pub pico: bool,
}

/// A conta de energia simulada, por parcela.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Conta {
    pub energia_pico: f64,
    pub energia_fora: f64,
    pub demanda: f64,
    pub ultrapassagem: f64,
}

impl Conta {
    pub fn total(&self) -> f64 {
        self.energia_pico + self.energia_fora + self.demanda + self.ultrapassagem
    }
}

/// A demanda de uma replicação.
#[derive(Clone, Debug, PartialEq)]
pub struct Demanda {
    pub intervalos: Vec<Intervalo>,
    pub max_kw: f64,
    /// Maior demanda nos intervalos de pico (`None`: nenhum intervalo de pico na janela).
    pub pico_kw: Option<f64>,
    pub energia_pico_kwh: f64,
    pub energia_fora_kwh: f64,
    pub fator_de_carga: f64,
    /// `Err` com os preços que faltam.
    pub conta: Result<Conta, Vec<&'static str>>,
}

/// A energia (kWh) da curva entre `x0` e `x1`, com `hu` horas por unidade de tempo.
fn integral(curva: &[(Time, f64)], x0: Time, x1: Time, hu: f64) -> f64 {
    let mut e = 0.0;
    for (k, (t, kw)) in curva.iter().enumerate() {
        let fim = curva.get(k + 1).map_or(f64::INFINITY, |x| x.0);
        let s = t.max(x0);
        let f = fim.min(x1);
        if f > s {
            e += kw * (f - s) * hu;
        }
    }
    e
}

/// O instante `meio` cai na janela de pico da tarifa?
pub fn em_pico(t: &Tariff, relogio: &Clock, meio: Time) -> bool {
    let (Some(de), Some(ate)) = (hora(&t.peak.from), hora(&t.peak.to)) else {
        return false;
    };
    let (dia, min) = relogio.relogio(meio);
    if t.peak.weekdays_only && dia % 7 >= 5 {
        return false;
    }
    let (de, ate) = (f64::from(de), f64::from(ate));
    if de <= ate {
        min >= de && min < ate
    } else {
        // Janela que passa da meia-noite.
        min >= de || min < ate
    }
}

/// A demanda e a conta de uma replicação, sobre a curva de carga e os impulsos de energia
/// (energia por peça, setup e partida) da janela `(a, b)`.
pub fn demanda(
    curva: &[(Time, f64)],
    impulsos: &[(Time, f64)],
    janela: (Time, Time),
    relogio: &Clock,
    t: &Tariff,
) -> Demanda {
    let (a, b) = janela;
    let hu = relogio.time_unit.em_horas();
    let inicio = f64::from(hora(&relogio.start).unwrap_or(0));
    let delta = f64::from(t.demand_interval_min.max(1));
    // Minutos absolutos do instante e o instante de um minuto absoluto.
    let minuto = |x: Time| inicio + x * hu * 60.0;
    let instante = |m: f64| (m - inicio) / (hu * 60.0);
    let mut intervalos = Vec::new();
    if b > a && hu > 0.0 {
        let mut k = (minuto(a) / delta).floor();
        loop {
            let x0 = instante(k * delta).max(a);
            let x1 = instante((k + 1.0) * delta).min(b);
            if x0 >= b {
                break;
            }
            if x1 > x0 {
                let mut kwh = integral(curva, x0, x1, hu);
                kwh += impulsos
                    .iter()
                    .filter(|(ti, _)| *ti >= x0 && *ti < x1)
                    .map(|x| x.1)
                    .sum::<f64>();
                let horas = (x1 - x0) * hu;
                intervalos.push(Intervalo {
                    de: x0,
                    ate: x1,
                    kw: kwh / horas,
                    kwh,
                    pico: em_pico(t, relogio, (x0 + x1) / 2.0),
                });
            }
            k += 1.0;
        }
    }
    let max_kw = intervalos.iter().map(|x| x.kw).fold(0.0, f64::max);
    let pico_kw = intervalos
        .iter()
        .filter(|x| x.pico)
        .map(|x| x.kw)
        .reduce(f64::max);
    let energia_pico_kwh: f64 = intervalos.iter().filter(|x| x.pico).map(|x| x.kwh).sum();
    let energia_fora_kwh: f64 = intervalos.iter().filter(|x| !x.pico).map(|x| x.kwh).sum();
    let horas = (b - a).max(0.0) * hu;
    let media = if horas > 0.0 {
        (energia_pico_kwh + energia_fora_kwh) / horas
    } else {
        0.0
    };
    let fator_de_carga = if max_kw > 0.0 { media / max_kw } else { 0.0 };

    let mut falta = Vec::new();
    let mut preco = |p: Option<f64>, nome: &'static str| {
        p.unwrap_or_else(|| {
            falta.push(nome);
            0.0
        })
    };
    let p_pico = if energia_pico_kwh > 0.0 {
        preco(t.price_kwh_peak, "price_kwh_peak")
    } else {
        t.price_kwh_peak.unwrap_or(0.0)
    };
    let p_fora = preco(t.price_kwh_off_peak, "price_kwh_off_peak");
    let p_kw = preco(t.price_kw, "price_kw");
    let excesso = t.contracted_kw.map_or(0.0, |c| (max_kw - c).max(0.0));
    let p_ultra = if excesso > 0.0 {
        preco(t.price_kw_overrun, "price_kw_overrun")
    } else {
        0.0
    };
    let cobrada = t.contracted_kw.map_or(max_kw, |c| c.max(max_kw));
    let conta = if falta.is_empty() {
        Ok(Conta {
            energia_pico: energia_pico_kwh * p_pico,
            energia_fora: energia_fora_kwh * p_fora,
            demanda: cobrada * p_kw,
            ultrapassagem: excesso * p_ultra,
        })
    } else {
        Err(falta)
    };
    Demanda {
        intervalos,
        max_kw,
        pico_kw,
        energia_pico_kwh,
        energia_fora_kwh,
        fator_de_carga,
        conta,
    }
}

#[cfg(test)]
mod testes {
    use super::*;
    use crate::formato::{Peak, TimeUnit};

    fn tarifa() -> Tariff {
        Tariff {
            peak: Peak {
                from: "07:00".into(),
                to: "08:00".into(),
                weekdays_only: false,
            },
            demand_interval_min: 15,
            contracted_kw: Some(12.0),
            price_kwh_peak: Some(2.0),
            price_kwh_off_peak: Some(1.0),
            price_kw: Some(10.0),
            price_kw_overrun: Some(30.0),
            source: "teste".into(),
        }
    }

    /// Caso resolvido à mão. Relógio começa às 06:00, unidade minuto, janela de 0 a 120
    /// (06:00 às 08:00). Potência 10 kW o tempo todo e 20 kW de 07:10 a 07:20 (minutos 70 a
    /// 80), atravessando dois intervalos de 15 minutos (07:00–07:15 e 07:15–07:30).
    /// - 07:00–07:15: 10 min a 10 kW + 5 min a 20 kW → (10×10 + 5×20)/15 = 13,33 kW.
    /// - 07:15–07:30: 5 min a 20 kW + 10 a 10 → 13,33 kW. Demanda máxima 13,33 kW, no pico.
    /// - Energia: 2 h × 10 kW + 10 min × 10 kW a mais = 20 + 1,667 = 21,667 kWh; no pico
    ///   (07:00–08:00): 10 + 1,667 = 11,667 kWh; fora: 10 kWh.
    /// - Conta: 11,667×2 + 10×1 + 13,33×10 + (13,33−12)×30 = 23,33 + 10 + 133,3 + 40 = 206,67.
    #[test]
    fn caso_a_mao_pico_cruza_dois_intervalos() {
        let relogio = Clock {
            start: "06:00".into(),
            time_unit: TimeUnit::Minute,
        };
        let curva = vec![(0.0, 10.0), (70.0, 20.0), (80.0, 10.0)];
        let d = demanda(&curva, &[], (0.0, 120.0), &relogio, &tarifa());
        assert_eq!(d.intervalos.len(), 8);
        let quarenta_tres = 200.0 / 15.0;
        assert!((d.max_kw - quarenta_tres).abs() < 1e-9);
        assert!((d.pico_kw.unwrap() - quarenta_tres).abs() < 1e-9);
        assert!((d.energia_pico_kwh - (10.0 + 10.0 / 6.0)).abs() < 1e-9);
        assert!((d.energia_fora_kwh - 10.0).abs() < 1e-9);
        let total =
            (10.0 + 10.0 / 6.0) * 2.0 + 10.0 + quarenta_tres * 10.0 + (quarenta_tres - 12.0) * 30.0;
        assert!((d.conta.unwrap().total() - total).abs() < 1e-9);
        let media = (20.0 + 10.0 / 6.0) / 2.0;
        assert!((d.fator_de_carga - media / quarenta_tres).abs() < 1e-9);
    }

    /// Sem preço, não há conta; os impulsos entram no intervalo deles.
    #[test]
    fn sem_preco_e_impulso() {
        let relogio = Clock {
            start: "06:00".into(),
            time_unit: TimeUnit::Minute,
        };
        let mut t = tarifa();
        t.price_kw = None;
        let d = demanda(&[(0.0, 0.0)], &[(5.0, 2.5)], (0.0, 30.0), &relogio, &t);
        assert_eq!(d.intervalos[0].kwh, 2.5);
        assert!((d.max_kw - 10.0).abs() < 1e-9);
        assert_eq!(d.conta.unwrap_err(), vec!["price_kw"]);
    }

    #[test]
    fn so_dias_uteis() {
        let relogio = Clock {
            start: "07:30".into(),
            time_unit: TimeUnit::Hour,
        };
        let mut t = tarifa();
        t.peak.weekdays_only = true;
        // Dia 5 (sábado) às 07:30 não é pico; dia 0 é.
        assert!(em_pico(&t, &relogio, 0.0));
        assert!(!em_pico(&t, &relogio, 5.0 * 24.0));
    }
}
