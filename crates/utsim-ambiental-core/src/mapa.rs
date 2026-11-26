//! A planta vista de cima, sem estado de tela (plano 3.1 e 5.1): onde cada objeto fica e as
//! ligações. Usa a posição do modelo; quando o modelo não tem posições (todas em zero),
//! monta camadas pela ordem das ligações, da esquerda para a direita.

use std::collections::{BTreeMap, VecDeque};

use utsim_model::{Model, ObjectKind};

/// Um objeto no mapa.
#[derive(Clone, Debug, PartialEq)]
pub struct Ponto {
    pub nome: String,
    pub x: f64,
    pub y: f64,
    /// Fontes e saídas aparecem pequenas, sem mancha.
    pub ponta: bool,
    /// Filas e estantes: sem bloco, aparecem pequenas (em geral não consomem).
    pub espera: bool,
}

/// A planta: os pontos e as ligações (índices em `pontos`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Planta {
    pub pontos: Vec<Ponto>,
    pub ligacoes: Vec<(usize, usize)>,
}

impl Planta {
    /// O retângulo que contém todos os pontos: (x0, y0, x1, y1).
    pub fn limites(&self) -> (f64, f64, f64, f64) {
        let mut l = (f64::MAX, f64::MAX, f64::MIN, f64::MIN);
        for p in &self.pontos {
            l.0 = l.0.min(p.x);
            l.1 = l.1.min(p.y);
            l.2 = l.2.max(p.x);
            l.3 = l.3.max(p.y);
        }
        if self.pontos.is_empty() {
            (0.0, 0.0, 1.0, 1.0)
        } else {
            l
        }
    }
}

/// A planta do modelo. Objetos móveis e de rede de fluidos ficam de fora no MVP.
pub fn planta(modelo: &Model) -> Planta {
    let objs: Vec<_> = modelo
        .objects
        .iter()
        .filter(|o| !utsim_model::is_mobile(&o.kind))
        .collect();
    let indice: BTreeMap<&str, usize> = objs
        .iter()
        .enumerate()
        .map(|(i, o)| (o.name.as_str(), i))
        .collect();
    let ligacoes: Vec<(usize, usize)> = modelo
        .connections
        .iter()
        .filter_map(|(a, b)| Some((*indice.get(a.as_str())?, *indice.get(b.as_str())?)))
        .collect();
    let com_posicao = objs
        .iter()
        .any(|o| o.position[0] != 0.0 || o.position[1] != 0.0);
    let xy: Vec<(f64, f64)> = if com_posicao {
        objs.iter()
            .map(|o| (o.position[0], o.position[1]))
            .collect()
    } else {
        camadas(objs.len(), &ligacoes)
    };
    Planta {
        pontos: objs
            .iter()
            .zip(xy)
            .map(|(o, (x, y))| Ponto {
                nome: o.name.clone(),
                x,
                y,
                ponta: matches!(o.kind, ObjectKind::Source { .. } | ObjectKind::Sink),
                espera: matches!(o.kind, ObjectKind::Queue { .. } | ObjectKind::Rack { .. }),
            })
            .collect(),
        ligacoes,
    }
}

/// Camadas pela distância (em ligações) desde quem não recebe nada; dentro da camada, um
/// abaixo do outro. Ciclos (retrabalho) não aumentam a camada.
fn camadas(n: usize, ligacoes: &[(usize, usize)]) -> Vec<(f64, f64)> {
    let mut entra = vec![0usize; n];
    for (_, b) in ligacoes {
        entra[*b] += 1;
    }
    let mut camada = vec![usize::MAX; n];
    let mut fila: VecDeque<usize> = (0..n).filter(|i| entra[*i] == 0).collect();
    if fila.is_empty() && n > 0 {
        fila.push_back(0);
    }
    for i in &fila {
        camada[*i] = 0;
    }
    while let Some(a) = fila.pop_front() {
        for (x, b) in ligacoes {
            if *x == a && camada[*b] == usize::MAX {
                camada[*b] = camada[a] + 1;
                fila.push_back(*b);
            }
        }
    }
    let fim = camada
        .iter()
        .filter(|c| **c != usize::MAX)
        .max()
        .map_or(0, |c| c + 1);
    let mut linha: BTreeMap<usize, usize> = BTreeMap::new();
    (0..n)
        .map(|i| {
            let c = if camada[i] == usize::MAX {
                fim
            } else {
                camada[i]
            };
            let l = linha.entry(c).or_default();
            let y = *l as f64;
            *l += 1;
            (c as f64 * 4.0, y * 3.0)
        })
        .collect()
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn sem_posicao_monta_camadas_em_ordem() {
        let m = Model::parse(
            r#"Model(name: "m", run: (stop_time: 10.0, warmup: 0.0, seed: 1, replications: 1),
               objects: [(name: "a", kind: Source(interarrival: Const(1.0))),
                         (name: "b", kind: Queue(capacity: 5)),
                         (name: "c", kind: Processor(process: Const(1.0))),
                         (name: "d", kind: Sink)],
               connections: [("a", "b"), ("b", "c"), ("c", "d")])"#,
        )
        .unwrap();
        let p = planta(&m);
        let xs: Vec<f64> = p.pontos.iter().map(|p| p.x).collect();
        assert!(xs.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(p.ligacoes.len(), 3);
        assert!(p.pontos[0].ponta && p.pontos[3].ponta && !p.pontos[2].ponta);
    }
}
