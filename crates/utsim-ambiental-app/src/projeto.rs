//! O modelo aberto: o modelo do UTSim, o bloco ambiental (arquivo lateral), os relatórios da
//! última rodada e a conta. Sem janela.
//!
//! A simulação roda numa thread; a conta é refeita na hora a cada mudança de potência ou de
//! fator, sobre os mesmos relatórios (não precisa simular de novo: a simulação não lê o
//! bloco ambiental).

use std::path::{Path, PathBuf};
use std::sync::mpsc;

use utsim_ambiental_core::{
    EnvironmentDef, Inventario, Lateral, Problema, Rodada, avaliar, validar,
};
use utsim_model::Model;

/// Onde chega o fim de uma simulação numa thread.
type Recebe = mpsc::Receiver<Result<Vec<Rodada>, String>>;

/// A base guardada para o "Antes e depois": o bloco e a conta daquele momento.
#[derive(Clone, Debug)]
pub struct Base {
    pub lateral: Lateral,
    pub inventario: Inventario,
}

pub struct Projeto {
    pub caminho: PathBuf,
    pub modelo: Model,
    pub lateral: Lateral,
    /// O bloco como está gravado, para saber se há mudança a salvar.
    pub salvo: Lateral,
    pub rodadas: Vec<Rodada>,
    /// A janela de tempo da conta (`None`: a janela de estatística inteira).
    pub janela: Option<(f64, f64)>,
    pub inventario: Option<Inventario>,
    pub base: Option<Base>,
    pub problemas: Vec<Problema>,
    rodando: Option<Recebe>,
    /// Os cenários do modelo (fase AM8) e o que já foi calculado de cada um.
    pub estudo: utsim_ambiental_core::cenarios::Estudo,
    cache_cenarios: std::collections::BTreeMap<String, Inventario>,
    /// As rodadas dos cenários que mudam o modelo, pelas mudanças.
    rodadas_cenarios: std::collections::BTreeMap<String, Vec<Rodada>>,
    cenario_em_curso: Option<(String, Recebe)>,
}

/// A chave das mudanças de modelo de um cenário (as que pedem simular).
fn chave_modelo(c: &utsim_ambiental_core::cenarios::Cenario) -> String {
    let m: Vec<_> = c.alavancas.iter().filter(|a| a.precisa_simular()).collect();
    format!("{m:?}")
}

impl Projeto {
    /// Abre o modelo e o arquivo lateral (sem ele, o catálogo inicial).
    pub fn abrir(caminho: &Path) -> Result<Projeto, String> {
        let modelo = Model::load(caminho).map_err(|e| e.to_string())?;
        let lateral = utsim_ambiental_core::ler_do_modelo(caminho).map_err(|e| e.to_string())?;
        let caminho = std::fs::canonicalize(caminho).unwrap_or_else(|_| caminho.to_path_buf());
        let estudo = utsim_ambiental_core::cenarios::ler(&caminho);
        let mut p = Projeto {
            caminho,
            modelo,
            salvo: lateral.clone(),
            lateral,
            rodadas: Vec::new(),
            janela: None,
            inventario: None,
            base: None,
            problemas: Vec::new(),
            rodando: None,
            estudo,
            cache_cenarios: Default::default(),
            rodadas_cenarios: Default::default(),
            cenario_em_curso: None,
        };
        p.problemas = validar(&p.modelo, &p.lateral);
        Ok(p)
    }

    pub fn nome(&self) -> String {
        if self.modelo.name.trim().is_empty() {
            self.caminho
                .file_stem()
                .map_or_else(String::new, |s| s.to_string_lossy().into_owned())
        } else {
            self.modelo.name.clone()
        }
    }

    /// Começa a simulação numa thread (as replicações do modelo).
    pub fn simular(&mut self) {
        let modelo = self.modelo.clone();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let r = utsim_ambiental_core::simular(&modelo);
            let _ = tx.send(r);
        });
        self.rodando = Some(rx);
    }

    /// Simula aqui mesmo, sem thread (testes).
    pub fn simular_agora(&mut self) -> Result<(), String> {
        self.rodadas = utsim_ambiental_core::simular(&self.modelo)?;
        self.recalcular();
        Ok(())
    }

    pub fn rodando(&self) -> bool {
        self.rodando.is_some()
    }

    /// Recebe o fim da simulação, se já terminou: `Some(Ok)` quando chegou agora.
    pub fn receber(&mut self) -> Option<Result<(), String>> {
        let rx = self.rodando.as_ref()?;
        match rx.try_recv() {
            Ok(Ok(r)) => {
                self.rodando = None;
                self.rodadas = r;
                self.base = None;
                self.cache_cenarios.clear();
                self.rodadas_cenarios.clear();
                self.janela = None;
                self.recalcular();
                Some(Ok(()))
            }
            Ok(Err(e)) => {
                self.rodando = None;
                Some(Err(e))
            }
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => {
                self.rodando = None;
                Some(Err("a simulação parou sem resposta".into()))
            }
        }
    }

    /// Refaz a validação e a conta sobre os relatórios que já existem.
    pub fn recalcular(&mut self) {
        self.problemas = validar(&self.modelo, &self.lateral);
        self.inventario = (!self.rodadas.is_empty())
            .then(|| avaliar(&self.modelo, &self.lateral, &self.rodadas, self.janela));
    }

    /// O inventário de um cenário: recalculado sobre o rastro do Base quando só muda o bloco
    /// ambiental; com as rodadas do cenário quando muda o modelo (`None`: falta simular).
    pub fn avaliar_cenario(
        &mut self,
        c: &utsim_ambiental_core::cenarios::Cenario,
    ) -> Result<Option<Inventario>, String> {
        let chave = format!("{:?}|{:?}|{:?}", c.alavancas, self.lateral, self.janela);
        if let Some(i) = self.cache_cenarios.get(&chave) {
            return Ok(Some(i.clone()));
        }
        let (m, l) = utsim_ambiental_core::cenarios::aplicar(&self.modelo, &self.lateral, c)?;
        let rodadas = if c.precisa_simular() {
            match self.rodadas_cenarios.get(&chave_modelo(c)) {
                Some(r) => r,
                None => return Ok(None),
            }
        } else {
            &self.rodadas
        };
        if rodadas.is_empty() {
            return Ok(None);
        }
        let inv = avaliar(&m, &l, rodadas, self.janela);
        if self.cache_cenarios.len() > 64 {
            self.cache_cenarios.clear();
        }
        self.cache_cenarios.insert(chave, inv.clone());
        Ok(Some(inv))
    }

    /// O cenário que muda o modelo já foi simulado?
    pub fn cenario_simulado(&self, c: &utsim_ambiental_core::cenarios::Cenario) -> bool {
        self.rodadas_cenarios.contains_key(&chave_modelo(c))
    }

    /// Simula o cenário numa thread, com a mesma semente do Base (comparação pareada).
    pub fn simular_cenario(&mut self, c: &utsim_ambiental_core::cenarios::Cenario) {
        let Ok((m, _)) = utsim_ambiental_core::cenarios::aplicar(&self.modelo, &self.lateral, c)
        else {
            return;
        };
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(utsim_ambiental_core::simular(&m));
        });
        self.cenario_em_curso = Some((chave_modelo(c), rx));
    }

    pub fn cenario_rodando(&self) -> Option<&str> {
        self.cenario_em_curso.as_ref().map(|x| x.0.as_str())
    }

    /// Recebe o fim da simulação de um cenário, se já terminou.
    pub fn receber_cenario(&mut self) -> Option<Result<(), String>> {
        let (chave, rx) = self.cenario_em_curso.as_ref()?;
        match rx.try_recv() {
            Ok(Ok(r)) => {
                let chave = chave.clone();
                self.rodadas_cenarios.insert(chave, r);
                self.cache_cenarios.clear();
                self.cenario_em_curso = None;
                Some(Ok(()))
            }
            Ok(Err(e)) => {
                self.cenario_em_curso = None;
                Some(Err(e))
            }
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => {
                self.cenario_em_curso = None;
                Some(Err("a simulação parou sem resposta".into()))
            }
        }
    }

    /// O bloco do nó, criado vazio se ainda não existe.
    pub fn bloco_mut(&mut self, no: &str) -> &mut EnvironmentDef {
        self.lateral.objects.entry(no.to_string()).or_default()
    }

    /// Há mudança no bloco que não foi gravada.
    pub fn mudou(&self) -> bool {
        self.lateral != self.salvo
    }

    /// Grava o arquivo lateral ao lado do modelo.
    pub fn gravar(&mut self) -> Result<PathBuf, String> {
        // Blocos vazios não vão para o arquivo.
        self.lateral
            .objects
            .retain(|_, d| d != &EnvironmentDef::default());
        let c = utsim_ambiental_core::gravar_do_modelo(&self.caminho, &self.lateral)
            .map_err(|e| e.to_string())?;
        self.salvo = self.lateral.clone();
        Ok(c)
    }

    /// Guarda o bloco e a conta de agora como a base do "Antes e depois".
    pub fn fixar_base(&mut self) {
        if let Some(inv) = &self.inventario {
            self.base = Some(Base {
                lateral: self.lateral.clone(),
                inventario: inv.clone(),
            });
        }
    }

    /// Volta o bloco ao da base.
    pub fn voltar_a_base(&mut self) {
        if let Some(b) = &self.base {
            self.lateral = b.lateral.clone();
            self.recalcular();
        }
    }
}

/// Grava o exemplo (modelo e bloco) em `pasta` e devolve o caminho do modelo. Não
/// sobrescreve um exemplo que o usuário já mudou.
pub fn gravar_exemplo(pasta: &Path) -> Result<PathBuf, String> {
    std::fs::create_dir_all(pasta).map_err(|e| e.to_string())?;
    let modelo = pasta.join("linha_de_pintura.ron");
    let lateral = utsim_ambiental_core::caminho_lateral(&modelo);
    for (c, t) in [
        (&modelo, crate::EXEMPLO_MODELO),
        (&lateral, crate::EXEMPLO_LATERAL),
    ] {
        if !c.exists() {
            std::fs::write(c, t).map_err(|e| e.to_string())?;
        }
    }
    Ok(modelo)
}

#[cfg(test)]
mod testes {
    use super::*;

    fn pasta_temp(nome: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("utsim-ambiental-{nome}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        p
    }

    /// O exemplo abre, passa na validação, simula e acende: toda máquina com potência tem
    /// energia, e a fábrica tem vilões.
    #[test]
    fn exemplo_abre_simula_e_acende() {
        let pasta = pasta_temp("exemplo");
        let c = gravar_exemplo(&pasta).unwrap();
        let mut p = Projeto::abrir(&c).unwrap();
        assert!(p.problemas.iter().all(|x| !x.grave), "{:?}", p.problemas);
        p.simular_agora().unwrap();
        let inv = p.inventario.as_ref().unwrap();
        assert!(inv.energia.media > 0.0);
        for (no, d) in &p.lateral.objects {
            if !d.power.vazia() {
                assert!(inv.no(no).unwrap().energia.media > 0.0, "{no}");
            }
        }
        assert!(!utsim_ambiental_core::viloes(inv, 3).is_empty());
        let _ = std::fs::remove_dir_all(&pasta);
    }

    /// Mudar uma potência muda a conta sem simular de novo, e a gravação volta igual.
    #[test]
    fn editar_recalcular_e_gravar() {
        let pasta = pasta_temp("editar");
        let c = gravar_exemplo(&pasta).unwrap();
        let mut p = Projeto::abrir(&c).unwrap();
        p.simular_agora().unwrap();
        p.fixar_base();
        let antes = p.inventario.as_ref().unwrap().energia.media;
        let no = p.lateral.objects.keys().next().unwrap().clone();
        p.bloco_mut(&no).power.waiting += 50.0;
        p.recalcular();
        assert!(p.mudou());
        let depois = p.inventario.as_ref().unwrap().energia.media;
        assert!(depois > antes);
        let d = utsim_ambiental_core::conta::comparar(
            &p.base.as_ref().unwrap().inventario,
            p.inventario.as_ref().unwrap(),
            "ambiental.energia_kwh",
        )
        .unwrap();
        assert!(d.media > 0.0);
        p.gravar().unwrap();
        assert!(!p.mudou());
        let de_novo = Projeto::abrir(&c).unwrap();
        assert_eq!(de_novo.lateral, p.lateral);
        let _ = std::fs::remove_dir_all(&pasta);
    }
}
