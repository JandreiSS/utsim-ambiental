//! O diagrama de Sankey (fase AM6, plano 3.2), sem estado de tela: o grafo de cada modo
//! (massa, energia, água, carbono), a geometria das caixas e das faixas, o caminho de uma
//! faixa, o balanço de cada nó do meio e a exportação em SVG.
//!
//! A largura de cada faixa é proporcional à quantidade, na mesma escala em todo o desenho.
//! Os valores são médias entre as replicações, na janela da conta.

use std::collections::{BTreeMap, BTreeSet};

use utsim_model::{Model, ObjectKind, SinkRole};

use crate::balanco::{SEM_DESTINO, fator_do_fluxo};
use crate::conta::Inventario;
use crate::formato::{FlowKind, Grupo, Lateral};

/// A grandeza do diagrama.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Modo {
    #[default]
    Massa,
    Energia,
    Agua,
    Carbono,
}

impl Modo {
    pub const TODOS: [Modo; 4] = [Modo::Massa, Modo::Energia, Modo::Agua, Modo::Carbono];

    pub fn unidade(self) -> &'static str {
        match self {
            Modo::Massa => "kg",
            Modo::Energia => "kWh",
            Modo::Agua => "m³",
            Modo::Carbono => "kg CO₂e",
        }
    }
}

/// Onde o nó fica: entradas à esquerda, máquinas no meio, saídas à direita.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Lado {
    Entrada,
    Meio,
    Saida,
}

/// A família do fluxo, que dá a cor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Classe {
    Energia,
    Combustivel,
    Agua,
    Material,
    Produto,
    Residuo,
    Efluente,
    Emissao,
    SemValor,
    Maquina,
}

/// Um nó do diagrama. `chave` identifica o nó (estável entre os modos); `rotulo` é o que se
/// lê (um nome do modelo ou do catálogo, ou uma chave de texto `snk-*` que o app traduz).
#[derive(Clone, Debug, PartialEq)]
pub struct NoS {
    pub chave: String,
    pub rotulo: String,
    pub lado: Lado,
    pub classe: Classe,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ligacao {
    pub de: usize,
    pub para: usize,
    pub valor: f64,
    pub classe: Classe,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Grafo {
    pub modo: Modo,
    pub nos: Vec<NoS>,
    pub ligacoes: Vec<Ligacao>,
}

/// Os textos fixos dos nós sintéticos (o app traduz pelo `rotulo`).
pub mod rotulo {
    pub const AGREGA: &str = "snk-agrega_valor";
    pub const PREPARANDO: &str = "snk-preparando";
    pub const SEM_VALOR: &str = "snk-sem_valor";
    pub const TRATAMENTO: &str = "snk-tratamento";
    pub const CAPTACAO: &str = "snk-captacao";
    pub const EVAPORADA: &str = "snk-evaporada";
    pub const INCORPORADA: &str = "snk-incorporada";
    pub const EFLUENTE_SEM: &str = "snk-efluente_sem_tratamento";
    pub const PRODUTO: &str = "snk-produto";
    pub const SUCATA_E_LODO: &str = "snk-sucata_e_lodo";
    pub const TODOS: [&str; 10] = [
        AGREGA,
        PREPARANDO,
        SEM_VALOR,
        TRATAMENTO,
        CAPTACAO,
        EVAPORADA,
        INCORPORADA,
        EFLUENTE_SEM,
        PRODUTO,
        SUCATA_E_LODO,
    ];
}

impl Grafo {
    fn no(&mut self, chave: &str, rotulo: &str, lado: Lado, classe: Classe) -> usize {
        if let Some(i) = self.nos.iter().position(|n| n.chave == chave) {
            return i;
        }
        self.nos.push(NoS {
            chave: chave.into(),
            rotulo: rotulo.into(),
            lado,
            classe,
        });
        self.nos.len() - 1
    }

    fn ligar(&mut self, de: usize, para: usize, valor: f64, classe: Classe) {
        if valor.is_nan() || valor <= 1e-12 || de == para {
            return;
        }
        match self
            .ligacoes
            .iter_mut()
            .find(|l| l.de == de && l.para == para && l.classe == classe)
        {
            Some(l) => l.valor += valor,
            None => self.ligacoes.push(Ligacao {
                de,
                para,
                valor,
                classe,
            }),
        }
    }

    /// O que entra e o que sai de cada nó.
    pub fn somas(&self) -> Vec<(f64, f64)> {
        let mut s = vec![(0.0, 0.0); self.nos.len()];
        for l in &self.ligacoes {
            s[l.para].0 += l.valor;
            s[l.de].1 += l.valor;
        }
        s
    }

    /// O total do diagrama: a soma das entradas (ou das saídas, se for maior).
    pub fn total(&self) -> f64 {
        let s = self.somas();
        let e: f64 = self
            .nos
            .iter()
            .zip(&s)
            .filter(|(n, _)| n.lado == Lado::Entrada)
            .map(|(_, x)| x.1)
            .sum();
        let o: f64 = self
            .nos
            .iter()
            .zip(&s)
            .filter(|(n, _)| n.lado == Lado::Saida)
            .map(|(_, x)| x.0)
            .sum();
        e.max(o)
    }

    /// Os nós do meio em que o que entra difere do que sai em mais que `tolerancia` (fração):
    /// `(nó, entra, sai)`.
    pub fn sem_balanco(&self, tolerancia: f64) -> Vec<(usize, f64, f64)> {
        self.somas()
            .into_iter()
            .enumerate()
            .filter(|(i, (e, s))| {
                self.nos[*i].lado == Lado::Meio && (e - s).abs() > tolerancia * e.max(*s)
            })
            .map(|(i, (e, s))| (i, e, s))
            .collect()
    }

    /// Esconde as faixas menores que `fracao` do total e os nós que ficam sem faixa.
    pub fn juntar(&self, fracao: f64) -> Grafo {
        let limite = fracao * self.total();
        let ligs: Vec<&Ligacao> = self.ligacoes.iter().filter(|l| l.valor >= limite).collect();
        let usados: BTreeSet<usize> = ligs.iter().flat_map(|l| [l.de, l.para]).collect();
        let novo: BTreeMap<usize, usize> =
            usados.iter().enumerate().map(|(k, i)| (*i, k)).collect();
        Grafo {
            modo: self.modo,
            nos: usados.iter().map(|i| self.nos[*i].clone()).collect(),
            ligacoes: ligs
                .into_iter()
                .map(|l| Ligacao {
                    de: novo[&l.de],
                    para: novo[&l.para],
                    ..l.clone()
                })
                .collect(),
        }
    }

    /// As faixas ligadas à faixa `k`: todo o caminho para trás e para a frente.
    pub fn caminho(&self, k: usize) -> BTreeSet<usize> {
        let mut feitas = BTreeSet::from([k]);
        let Some(l0) = self.ligacoes.get(k) else {
            return feitas;
        };
        // Para a frente a partir do destino; para trás a partir da origem.
        for (inicio, frente) in [(l0.para, true), (l0.de, false)] {
            let mut pilha = vec![inicio];
            let mut vistos = BTreeSet::from([inicio]);
            while let Some(no) = pilha.pop() {
                for (j, l) in self.ligacoes.iter().enumerate() {
                    let (a, b) = if frente {
                        (l.de, l.para)
                    } else {
                        (l.para, l.de)
                    };
                    if a == no {
                        feitas.insert(j);
                        if vistos.insert(b) {
                            pilha.push(b);
                        }
                    }
                }
            }
        }
        feitas
    }
}

/// O grupo de um objeto quando o diagrama agrupa por etapa: a etapa, ou o próprio nome.
fn chave_do_meio(l: &Lateral, nome: &str, por_etapa: bool) -> String {
    if por_etapa && let Some(e) = l.objects.get(nome).and_then(|d| d.stage.clone()) {
        return format!("etapa:{e}");
    }
    format!("no:{nome}")
}

fn rotulo_do_meio(l: &Lateral, nome: &str, por_etapa: bool) -> String {
    if por_etapa && let Some(e) = l.objects.get(nome).and_then(|d| d.stage.clone()) {
        return e;
    }
    nome.to_string()
}

/// Por que o modo não tem diagrama.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SemDiagrama {
    /// A massa pede o rastro e a massa das peças.
    SemMassa,
    /// Nenhuma máquina tem o que o modo mostra.
    Vazio,
    /// O carbono pede os fatores dos fluxos usados.
    FaltaFator(Vec<String>),
}

/// O grafo de um modo, a partir do inventário.
pub fn grafo(
    modelo: &Model,
    l: &Lateral,
    inv: &Inventario,
    modo: Modo,
    por_etapa: bool,
) -> Result<Grafo, SemDiagrama> {
    let mut g = Grafo {
        modo,
        ..Grafo::default()
    };
    let cat = &l.environment;
    let meio = |g: &mut Grafo, nome: &str| {
        g.no(
            &chave_do_meio(l, nome, por_etapa),
            &rotulo_do_meio(l, nome, por_etapa),
            Lado::Meio,
            Classe::Maquina,
        )
    };
    let tipo = |f: &str| cat.fluxo(f).map(|x| (x.kind, x.unit.clone()));
    match modo {
        Modo::Energia => {
            let eletrico = cat
                .fluxo_eletrico()
                .map_or("eletricidade".to_string(), |f| f.name.clone());
            for n in inv.nos.iter().filter(|n| n.energia.media > 0.0) {
                let e = g.no(
                    &format!("fluxo:{eletrico}"),
                    &eletrico,
                    Lado::Entrada,
                    Classe::Energia,
                );
                let m = meio(&mut g, &n.nome);
                g.ligar(e, m, n.energia.media, Classe::Energia);
                let pg = |gr: Grupo| n.por_grupo[gr.indice()].media;
                let agrega = g.no("saida:agrega", rotulo::AGREGA, Lado::Saida, Classe::Produto);
                g.ligar(m, agrega, pg(Grupo::Produzindo), Classe::Energia);
                let prep = g.no(
                    "saida:preparando",
                    rotulo::PREPARANDO,
                    Lado::Saida,
                    Classe::Energia,
                );
                g.ligar(m, prep, pg(Grupo::Preparando), Classe::Energia);
                let sem = g.no(
                    "saida:sem_valor",
                    rotulo::SEM_VALOR,
                    Lado::Saida,
                    Classe::SemValor,
                );
                let sv = pg(Grupo::Esperando) + pg(Grupo::Parada) + pg(Grupo::ForaDeTurno);
                g.ligar(m, sem, sv, Classe::SemValor);
            }
            if let Some(a) = &inv.agua
                && a.tratamento_kwh.media > 0.0
            {
                let e = g.no(
                    &format!("fluxo:{eletrico}"),
                    &eletrico,
                    Lado::Entrada,
                    Classe::Energia,
                );
                let t = g.no(
                    "saida:tratamento",
                    rotulo::TRATAMENTO,
                    Lado::Saida,
                    Classe::Efluente,
                );
                g.ligar(e, t, a.tratamento_kwh.media, Classe::Energia);
            }
        }
        Modo::Agua => {
            for a in &inv.agua_nos {
                let m = meio(&mut g, &a.nome);
                let recebido: f64 = inv
                    .agua_nos
                    .iter()
                    .filter(|x| x.reuso_para.as_deref() == Some(a.nome.as_str()))
                    .map(|x| x.reuso)
                    .sum();
                let cap = g.no(
                    "entrada:captacao",
                    rotulo::CAPTACAO,
                    Lado::Entrada,
                    Classe::Agua,
                );
                g.ligar(cap, m, (a.entrada - recebido).max(0.0), Classe::Agua);
                if let Some(alvo) = &a.reuso_para {
                    let t = meio(&mut g, alvo);
                    if t == m {
                        // Reuso no próprio nó: a água volta para ele (aparece como menos captação).
                    } else {
                        g.ligar(m, t, a.reuso, Classe::Agua);
                    }
                }
                let ev = g.no(
                    "saida:evaporada",
                    rotulo::EVAPORADA,
                    Lado::Saida,
                    Classe::Emissao,
                );
                g.ligar(m, ev, a.evaporada, Classe::Agua);
                let inc = g.no(
                    "saida:incorporada",
                    rotulo::INCORPORADA,
                    Lado::Saida,
                    Classe::Produto,
                );
                g.ligar(m, inc, a.incorporada, Classe::Agua);
                let efl = match &a.destino {
                    Some(d) => g.no(&format!("efluente:{d}"), d, Lado::Saida, Classe::Efluente),
                    None => g.no(
                        "efluente:sem",
                        rotulo::EFLUENTE_SEM,
                        Lado::Saida,
                        Classe::Efluente,
                    ),
                };
                g.ligar(m, efl, a.efluente, Classe::Efluente);
            }
        }
        Modo::Massa => {
            if inv.transferencias.is_empty() {
                return Err(SemDiagrama::SemMassa);
            }
            let total: f64 = inv.transferencias.values().sum();
            if total <= 0.0 {
                return Err(SemDiagrama::SemMassa);
            }
            let no_do_objeto = |g: &mut Grafo, i: usize| -> Option<usize> {
                let o = modelo.objects.get(i)?;
                Some(match &o.kind {
                    ObjectKind::Source { .. } => g.no(
                        &format!("fonte:{}", o.name),
                        &o.name,
                        Lado::Entrada,
                        Classe::Material,
                    ),
                    ObjectKind::Sink => {
                        let classe = if o.sink_role == Some(SinkRole::Scrap) {
                            Classe::Residuo
                        } else {
                            Classe::Produto
                        };
                        g.no(&format!("saida:{}", o.name), &o.name, Lado::Saida, classe)
                    }
                    _ => meio(g, &o.name),
                })
            };
            for ((de, para), kg) in &sem_esperas(modelo, &inv.transferencias) {
                let (Some(a), Some(b)) = (no_do_objeto(&mut g, *de), no_do_objeto(&mut g, *para))
                else {
                    continue;
                };
                let classe = match g.nos[b].classe {
                    Classe::Residuo => Classe::Residuo,
                    Classe::Produto => Classe::Produto,
                    _ => Classe::Material,
                };
                g.ligar(a, b, *kg, classe);
            }
            // Material somado e resíduo tirado em cada nó (fluxos em kg).
            for n in &inv.nos {
                for (f, q) in &n.fluxos {
                    let Some((k, unidade)) = tipo(f) else {
                        continue;
                    };
                    if unidade.trim().to_lowercase() != "kg" {
                        continue;
                    }
                    match k {
                        FlowKind::Material => {
                            let e = g.no(&format!("fluxo:{f}"), f, Lado::Entrada, Classe::Material);
                            let m = meio(&mut g, &n.nome);
                            g.ligar(e, m, q.media, Classe::Material);
                        }
                        FlowKind::Waste => {
                            let dest = cat
                                .fluxo(f)
                                .and_then(|x| x.destination.clone())
                                .unwrap_or_else(|| SEM_DESTINO.into());
                            let s = g.no(
                                &format!("residuo:{f}"),
                                &format!("{f} → {dest}"),
                                Lado::Saida,
                                Classe::Residuo,
                            );
                            let m = meio(&mut g, &n.nome);
                            g.ligar(m, s, q.media, Classe::Residuo);
                        }
                        _ => {}
                    }
                }
            }
        }
        Modo::Carbono => {
            let mut falta = Vec::new();
            let eletrico = cat.fluxo_eletrico().map(|f| f.name.clone());
            let produto = |g: &mut Grafo| {
                g.no(
                    "saida:produto",
                    rotulo::PRODUTO,
                    Lado::Saida,
                    Classe::Produto,
                )
            };
            for n in inv.nos.iter().filter(|n| n.tem_bloco) {
                let mut origens: Vec<(String, f64)> = Vec::new();
                if n.energia.media > 0.0 {
                    match eletrico.as_deref().map(|e| (e, fator_do_fluxo(l, e))) {
                        Some((e, Ok((k, _)))) => origens.push((e.to_string(), n.energia.media * k)),
                        Some((_, Err(x))) => falta.push(x),
                        None => falta.push("eletricidade".into()),
                    }
                }
                for (f, q) in &n.fluxos {
                    if q.media == 0.0 {
                        continue;
                    }
                    match fator_do_fluxo(l, f) {
                        Ok((k, _)) => origens.push((f.clone(), q.media * k)),
                        Err(x) => falta.push(x),
                    }
                }
                let m = meio(&mut g, &n.nome);
                for (f, v) in origens {
                    let e = g.no(&format!("fluxo:{f}"), &f, Lado::Entrada, Classe::Emissao);
                    g.ligar(e, m, v, Classe::Emissao);
                    let p = produto(&mut g);
                    g.ligar(m, p, v, Classe::Emissao);
                }
            }
            falta.sort();
            falta.dedup();
            if !falta.is_empty() {
                return Err(SemDiagrama::FaltaFator(falta));
            }
            // Sucata, lodo e tratamento: a diferença entre a fábrica e a soma dos nós.
            if let Some(total) = inv.co2e.valor() {
                let nos: f64 = g
                    .ligacoes
                    .iter()
                    .filter(|x| g.nos[x.para].lado == Lado::Saida)
                    .map(|x| x.valor)
                    .sum();
                let resto = total.media - nos;
                if resto > 1e-9 {
                    let e = g.no(
                        "fluxo:fabrica",
                        rotulo::SUCATA_E_LODO,
                        Lado::Entrada,
                        Classe::Residuo,
                    );
                    let p = produto(&mut g);
                    g.ligar(e, p, resto, Classe::Emissao);
                }
            }
        }
    }
    if g.ligacoes.is_empty() {
        return Err(SemDiagrama::Vazio);
    }
    // Só os nós do meio que restaram com faixa.
    Ok(g.juntar(0.0))
}

/// As transferências sem as filas, estantes e esteiras (que só guardam ou levam peças): o
/// que sai de uma delas é repartido entre quem a alimentou, na proporção do que cada um
/// mandou. A massa total que chega a cada máquina não muda.
pub fn sem_esperas(
    modelo: &Model,
    t: &BTreeMap<(usize, usize), f64>,
) -> BTreeMap<(usize, usize), f64> {
    let espera = |i: usize| {
        modelo.objects.get(i).is_some_and(|o| {
            matches!(
                o.kind,
                ObjectKind::Queue { .. } | ObjectKind::Rack { .. } | ObjectKind::Conveyor { .. }
            )
        })
    };
    let mut t = t.clone();
    // Uma fila de cada vez, até não sobrar nenhuma (filas em série também).
    for _ in 0..modelo.objects.len() {
        let Some(q) = t.keys().flat_map(|(a, b)| [*a, *b]).find(|i| espera(*i)) else {
            break;
        };
        let entra: Vec<(usize, f64)> = t
            .iter()
            .filter(|((_, b), _)| *b == q)
            .map(|((a, _), v)| (*a, *v))
            .filter(|(a, _)| *a != q)
            .collect();
        let sai: Vec<(usize, f64)> = t
            .iter()
            .filter(|((a, _), _)| *a == q)
            .map(|((_, b), v)| (*b, *v))
            .filter(|(b, _)| *b != q)
            .collect();
        t.retain(|(a, b), _| *a != q && *b != q);
        let soma: f64 = entra.iter().map(|x| x.1).sum();
        for (b, vo) in &sai {
            if soma > 0.0 {
                for (a, vi) in &entra {
                    *t.entry((*a, *b)).or_default() += vo * vi / soma;
                }
            }
        }
    }
    t
}

/// Uma caixa (nó) no desenho.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Caixa {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

/// Uma faixa: sai da borda direita da origem, entre `y0` e `y0 + espessura`, e chega à
/// borda esquerda do destino, entre `y1` e `y1 + espessura`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Faixa {
    pub ligacao: usize,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub espessura: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Desenho {
    pub caixas: Vec<Caixa>,
    pub faixas: Vec<Faixa>,
    /// Unidades de desenho por unidade da grandeza.
    pub escala: f64,
    pub largura: f64,
    pub altura: f64,
}

/// A coluna de cada nó: entradas na 0, saídas na última, o meio pela distância mais longa
/// desde as entradas (ligações que voltam, como o reuso, não empurram a coluna).
fn colunas(g: &Grafo) -> Vec<usize> {
    let n = g.nos.len();
    let mut col = vec![0usize; n];
    // Ordem topológica aproximada: relaxa as ligações para a frente, sem voltar.
    for _ in 0..n {
        let mut mudou = false;
        for l in &g.ligacoes {
            if g.nos[l.para].lado == Lado::Entrada {
                continue;
            }
            let c = col[l.de] + 1;
            if c > col[l.para] && c < n {
                col[l.para] = c;
                mudou = true;
            }
        }
        if !mudou {
            break;
        }
    }
    let fim = col
        .iter()
        .zip(&g.nos)
        .filter(|(_, x)| x.lado != Lado::Saida)
        .map(|(c, _)| *c)
        .max()
        .unwrap_or(0)
        + 1;
    for (i, x) in g.nos.iter().enumerate() {
        match x.lado {
            Lado::Entrada => col[i] = 0,
            Lado::Saida => col[i] = fim,
            Lado::Meio => col[i] = col[i].clamp(1, fim.saturating_sub(1).max(1)),
        }
    }
    col
}

/// A geometria do diagrama numa área de `largura` × `altura`.
pub fn desenhar(g: &Grafo, largura: f64, altura: f64) -> Desenho {
    let n = g.nos.len();
    let col = colunas(g);
    let ncol = col.iter().copied().max().unwrap_or(0) + 1;
    let somas = g.somas();
    let tam: Vec<f64> = somas.iter().map(|(e, s)| e.max(*s)).collect();
    let folga = 14.0;
    let larg_caixa = 12.0;
    // A escala: a coluna mais cheia ocupa a altura toda, menos as folgas.
    let mut escala = f64::INFINITY;
    for c in 0..ncol {
        let nos: Vec<usize> = (0..n).filter(|i| col[*i] == c).collect();
        let soma: f64 = nos.iter().map(|i| tam[*i]).sum();
        let livre = altura - folga * (nos.len().saturating_sub(1)) as f64;
        if soma > 0.0 {
            escala = escala.min(livre.max(1.0) / soma);
        }
    }
    if !escala.is_finite() {
        escala = 1.0;
    }
    let x_de = |c: usize| {
        if ncol <= 1 {
            0.0
        } else {
            (largura - larg_caixa) * c as f64 / (ncol - 1) as f64
        }
    };
    let mut caixas = vec![
        Caixa {
            x0: 0.0,
            y0: 0.0,
            x1: 0.0,
            y1: 0.0
        };
        n
    ];
    for c in 0..ncol {
        let mut y = 0.0;
        // Na ordem em que os nós apareceram (a ordem do modelo).
        for i in (0..n).filter(|i| col[*i] == c) {
            let h = (tam[i] * escala).max(1.0);
            caixas[i] = Caixa {
                x0: x_de(c),
                y0: y,
                x1: x_de(c) + larg_caixa,
                y1: y + h,
            };
            y += h + folga;
        }
    }
    // As faixas: na origem, empilhadas pela altura do destino; no destino, pela da origem.
    let mut ordem: Vec<usize> = (0..g.ligacoes.len()).collect();
    ordem.sort_by(|a, b| {
        let (la, lb) = (&g.ligacoes[*a], &g.ligacoes[*b]);
        caixas[la.para].y0.total_cmp(&caixas[lb.para].y0)
    });
    let mut saida_y: Vec<f64> = caixas.iter().map(|c| c.y0).collect();
    let mut y_origem = vec![0.0; g.ligacoes.len()];
    for k in &ordem {
        let l = &g.ligacoes[*k];
        y_origem[*k] = saida_y[l.de];
        saida_y[l.de] += l.valor * escala;
    }
    ordem.sort_by(|a, b| {
        let (la, lb) = (&g.ligacoes[*a], &g.ligacoes[*b]);
        caixas[la.de].y0.total_cmp(&caixas[lb.de].y0)
    });
    let mut entrada_y: Vec<f64> = caixas.iter().map(|c| c.y0).collect();
    let mut faixas = Vec::with_capacity(g.ligacoes.len());
    for k in &ordem {
        let l = &g.ligacoes[*k];
        let y1 = entrada_y[l.para];
        entrada_y[l.para] += l.valor * escala;
        faixas.push(Faixa {
            ligacao: *k,
            x0: caixas[l.de].x1,
            y0: y_origem[*k],
            x1: caixas[l.para].x0,
            y1,
            espessura: l.valor * escala,
        });
    }
    faixas.sort_by_key(|f| f.ligacao);
    Desenho {
        caixas,
        faixas,
        escala,
        largura,
        altura,
    }
}

/// A cor de cada classe, em `#rrggbb` (a mesma no app e no SVG).
pub fn cor(c: Classe) -> &'static str {
    match c {
        Classe::Energia => "#e8c547",
        Classe::Combustivel => "#e0873a",
        Classe::Agua => "#4fa3e0",
        Classe::Material => "#9aa7b8",
        Classe::Produto => "#5cc86e",
        Classe::Residuo => "#b5835a",
        Classe::Efluente => "#3d7fb0",
        Classe::Emissao => "#b07ad6",
        Classe::SemValor => "#d9534f",
        Classe::Maquina => "#dee3ed",
    }
}

fn escapar(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// O diagrama em SVG, com o título, a legenda das quantidades e os rótulos já traduzidos
/// (`rotulos[i]` para o nó `i`).
pub fn svg(g: &Grafo, d: &Desenho, titulo: &str, rotulos: &[String]) -> String {
    let margem = 160.0;
    let w = d.largura + 2.0 * margem;
    let h = d.altura + 80.0;
    let mut s = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w:.0}\" height=\"{h:.0}\" viewBox=\"0 0 {w:.0} {h:.0}\" font-family=\"sans-serif\" font-size=\"12\">\n<rect width=\"100%\" height=\"100%\" fill=\"#ffffff\"/>\n<text x=\"{margem}\" y=\"24\" font-size=\"16\" font-weight=\"bold\">{}</text>\n<g transform=\"translate({margem},50)\">\n",
        escapar(titulo)
    );
    for f in &d.faixas {
        let l = &g.ligacoes[f.ligacao];
        let mx = (f.x0 + f.x1) / 2.0;
        s.push_str(&format!(
            "<path d=\"M{x0:.2},{y0:.2} C{mx:.2},{y0:.2} {mx:.2},{y1:.2} {x1:.2},{y1:.2} L{x1:.2},{y1e:.2} C{mx:.2},{y1e:.2} {mx:.2},{y0e:.2} {x0:.2},{y0e:.2} Z\" fill=\"{c}\" fill-opacity=\"0.55\"><title>{v:.3} {u}</title></path>\n",
            x0 = f.x0,
            y0 = f.y0,
            x1 = f.x1,
            y1 = f.y1,
            y0e = f.y0 + f.espessura,
            y1e = f.y1 + f.espessura,
            c = cor(l.classe),
            v = l.valor,
            u = g.modo.unidade()
        ));
    }
    let somas = g.somas();
    for (i, c) in d.caixas.iter().enumerate() {
        let no = &g.nos[i];
        s.push_str(&format!(
            "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#444b5a\"/>\n",
            c.x0,
            c.y0,
            c.x1 - c.x0,
            c.y1 - c.y0
        ));
        let valor = somas[i].0.max(somas[i].1);
        let (x, ancora) = if no.lado == Lado::Entrada {
            (c.x0 - 6.0, "end")
        } else {
            (c.x1 + 6.0, "start")
        };
        let texto = rotulos.get(i).cloned().unwrap_or_else(|| no.rotulo.clone());
        s.push_str(&format!(
            "<text x=\"{x:.2}\" y=\"{:.2}\" text-anchor=\"{ancora}\" dominant-baseline=\"middle\">{} ({valor:.2} {})</text>\n",
            (c.y0 + c.y1) / 2.0,
            escapar(&texto),
            g.modo.unidade()
        ));
    }
    s.push_str("</g>\n</svg>\n");
    s
}

#[cfg(test)]
mod testes {
    use super::*;

    fn g() -> Grafo {
        let mut g = Grafo {
            modo: Modo::Energia,
            ..Grafo::default()
        };
        let e = g.no("e", "e", Lado::Entrada, Classe::Energia);
        let a = g.no("a", "a", Lado::Meio, Classe::Maquina);
        let b = g.no("b", "b", Lado::Meio, Classe::Maquina);
        let s1 = g.no("s1", "s1", Lado::Saida, Classe::Produto);
        let s2 = g.no("s2", "s2", Lado::Saida, Classe::SemValor);
        g.ligar(e, a, 10.0, Classe::Energia);
        g.ligar(e, b, 5.0, Classe::Energia);
        g.ligar(a, s1, 7.0, Classe::Energia);
        g.ligar(a, s2, 3.0, Classe::SemValor);
        g.ligar(b, s1, 5.0, Classe::Energia);
        g
    }

    /// A espessura de toda faixa é o valor × escala, e as faixas de um nó somam a caixa.
    #[test]
    fn geometria_confere_com_os_valores() {
        let g = g();
        let d = desenhar(&g, 600.0, 300.0);
        for f in &d.faixas {
            let l = &g.ligacoes[f.ligacao];
            assert!((f.espessura - l.valor * d.escala).abs() < 1e-9);
        }
        let somas = g.somas();
        for (i, c) in d.caixas.iter().enumerate() {
            let t = somas[i].0.max(somas[i].1) * d.escala;
            assert!((c.y1 - c.y0 - t.max(1.0)).abs() < 1e-9);
        }
        // Entradas à esquerda, saídas à direita.
        assert_eq!(d.caixas[0].x0, 0.0);
        assert!(d.caixas[3].x0 > d.caixas[1].x0);
        assert!(g.sem_balanco(0.01).is_empty());
        let svg = svg(&g, &d, "teste", &[]);
        assert_eq!(svg.matches("<path").count(), 5);
    }

    /// O caminho de uma faixa segue para trás e para a frente; juntar esconde as finas.
    #[test]
    fn caminho_e_juntar() {
        let g = g();
        // e→b leva a b→s1, e nada de a.
        let c = g.caminho(1);
        assert_eq!(c, BTreeSet::from([1, 4]));
        let j = g.juntar(0.25);
        assert_eq!(j.ligacoes.len(), 4);
        assert!(j.nos.iter().all(|n| n.chave != "s2"));
    }

    /// Um nó que não fecha aparece no balanço.
    #[test]
    fn balanco_acusa() {
        let mut g = g();
        g.ligacoes[2].valor = 5.0;
        let s = g.sem_balanco(0.02);
        assert_eq!(s.len(), 1);
        assert_eq!(g.nos[s[0].0].chave, "a");
    }

    /// No exemplo de verdade: o modo massa fecha nas máquinas (o que entra sai, menos o que
    /// ficou em processo), e os quatro modos montam.
    #[test]
    fn exemplo_nos_quatro_modos() {
        let pasta = std::env::temp_dir().join(format!("utsim-amb-snk-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&pasta);
        std::fs::create_dir_all(&pasta).unwrap();
        let raiz = concat!(env!("CARGO_MANIFEST_DIR"), "/../../exemplos/");
        let m = Model::load(format!("{raiz}linha_de_pintura.ron")).unwrap();
        let l: Lateral = crate::ler_texto(
            &std::fs::read_to_string(format!("{raiz}linha_de_pintura.ambiental.ron")).unwrap(),
        )
        .unwrap();
        let rod = crate::rodar::simular(&m).unwrap();
        let inv = crate::conta::avaliar(&m, &l, &rod, None);
        let massa = grafo(&m, &l, &inv, Modo::Massa, false).unwrap();
        // Tolerância de 5 %: a peça que está na máquina no fim da janela entrou e não saiu.
        let ruins = massa.sem_balanco(0.05);
        assert!(ruins.is_empty(), "{ruins:?}");
        assert!(grafo(&m, &l, &inv, Modo::Energia, true).is_ok());
        assert!(grafo(&m, &l, &inv, Modo::Agua, false).is_ok());
        assert!(matches!(
            grafo(&m, &l, &inv, Modo::Carbono, false),
            Err(SemDiagrama::FaltaFator(_))
        ));
        // A energia do diagrama é a da fábrica.
        let e = grafo(&m, &l, &inv, Modo::Energia, false).unwrap();
        assert!((e.total() - inv.energia.media).abs() < 1e-6 * inv.energia.media);
    }
}
