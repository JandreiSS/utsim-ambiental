//! O modelo aberto: o modelo do UTSim, o bloco ambiental (arquivo lateral), os relatórios da
//! última rodada e a conta. Sem janela.
//!
//! A simulação roda numa thread; a conta é refeita na hora a cada mudança de potência ou de
//! fator, sobre os mesmos relatórios (não precisa simular de novo: a simulação não lê o
//! bloco ambiental).

use std::path::{Path, PathBuf};
use std::sync::mpsc;

use utsim_ambiental_core::{EnvironmentDef, Inventario, Lateral, Problema, avaliar, validar};
use utsim_core::RunReport;
use utsim_model::Model;

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
    pub relatorios: Vec<RunReport>,
    pub inventario: Option<Inventario>,
    pub base: Option<Base>,
    pub problemas: Vec<Problema>,
    rodando: Option<mpsc::Receiver<Result<Vec<RunReport>, String>>>,
}

impl Projeto {
    /// Abre o modelo e o arquivo lateral (sem ele, o catálogo inicial).
    pub fn abrir(caminho: &Path) -> Result<Projeto, String> {
        let modelo = Model::load(caminho).map_err(|e| e.to_string())?;
        let lateral = utsim_ambiental_core::ler_do_modelo(caminho).map_err(|e| e.to_string())?;
        let caminho = std::fs::canonicalize(caminho).unwrap_or_else(|_| caminho.to_path_buf());
        let mut p = Projeto {
            caminho,
            modelo,
            salvo: lateral.clone(),
            lateral,
            relatorios: Vec::new(),
            inventario: None,
            base: None,
            problemas: Vec::new(),
            rodando: None,
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
            let r = modelo.run_replications().map_err(|e| e.to_string());
            let _ = tx.send(r);
        });
        self.rodando = Some(rx);
    }

    /// Simula aqui mesmo, sem thread (testes).
    pub fn simular_agora(&mut self) -> Result<(), String> {
        self.relatorios = self.modelo.run_replications().map_err(|e| e.to_string())?;
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
                self.relatorios = r;
                self.base = None;
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
        self.inventario = (!self.relatorios.is_empty())
            .then(|| avaliar(&self.modelo, &self.lateral, &self.relatorios));
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
