//! A janela: a abertura comum da família, a barra lateral, as telas e o aviso de uso
//! didático no rodapé de todas as telas.

use std::path::{Path, PathBuf};

use eframe::egui;
use utsim_ui_tema::abertura::{Abertura, Acao, Escolha, Onde, Recente, Rotulos, Simbolo};

use crate::config::Config;
use crate::projeto::Projeto;
use crate::texto::{fx, tx, vx};

/// A cor do app: verde (decisão A2). Só no botão principal e no item selecionado.
pub const COR_APP: egui::Color32 = egui::Color32::from_rgb(0x2F, 0xA2, 0x4A);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tela {
    Inicio,
    Fabrica,
    Maquinas,
    Fatores,
    Fluxos,
    Produto,
    Cenarios,
    Metas,
    Relatorios,
}

pub struct App {
    pub tela: Tela,
    pub config: Config,
    arquivo_config: Option<PathBuf>,
    pub projeto: Option<Projeto>,
    pub fabrica: crate::fabrica::Vista,
    /// A última mensagem do rodapé e se é um erro.
    pub mensagem: Option<(String, bool)>,
}

impl App {
    pub fn new(arquivo_config: Option<PathBuf>) -> App {
        let config = Config::ler(arquivo_config.as_ref());
        if let Some(i) = config
            .idioma
            .as_deref()
            .and_then(utsim_idiomas::Idioma::de_codigo)
        {
            utsim_idiomas::definir(i);
        }
        App {
            tela: Tela::Inicio,
            config,
            arquivo_config,
            projeto: None,
            fabrica: crate::fabrica::Vista::default(),
            mensagem: None,
        }
    }

    fn avisar(&mut self, texto: String, erro: bool) {
        self.mensagem = Some((texto, erro));
    }

    fn gravar_config(&mut self) {
        if let Err(e) = self.config.gravar(self.arquivo_config.as_ref()) {
            self.avisar(fx("app-erro_config", &[("e", vx(&e))]), true);
        }
    }

    /// Abre um modelo, simula e vai para a Fábrica (no nó, se pedido).
    pub fn abrir(&mut self, caminho: &Path, no: Option<&str>) {
        match Projeto::abrir(caminho) {
            Ok(mut p) => {
                p.simular();
                utsim_ui_tema::abertura::lembrar(&mut self.config.recentes, &p.caminho);
                self.gravar_config();
                self.avisar(fx("app-aberto", &[("modelo", vx(&p.nome()))]), false);
                self.projeto = Some(p);
                self.fabrica = crate::fabrica::Vista {
                    selecionado: no.map(str::to_string),
                    ..crate::fabrica::Vista::default()
                };
                self.tela = Tela::Fabrica;
            }
            Err(e) => self.avisar(fx("app-erro_abrir", &[("e", vx(&e))]), true),
        }
    }

    pub fn draw(&mut self, ui: &mut egui::Ui) {
        if let Some(p) = &mut self.projeto {
            match p.receber() {
                Some(Ok(())) => self.mensagem = Some((tx("app-simulado").into(), false)),
                Some(Err(e)) => {
                    self.mensagem = Some((fx("app-erro_simular", &[("e", vx(&e))]), true));
                }
                None => {}
            }
            if p.rodando() {
                ui.ctx()
                    .request_repaint_after(std::time::Duration::from_millis(100));
            }
        }
        egui::Panel::left("navegacao")
            .default_size(utsim_ui_tema::navegacao::LARGURA)
            .resizable(false)
            .show(ui, |ui| self.barra_lateral(ui));
        egui::Panel::bottom("rodape").show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if let Some((m, erro)) = &self.mensagem {
                    let cor = if *erro {
                        utsim_ui_tema::colors::ERROR
                    } else {
                        utsim_ui_tema::colors::TEXT_WEAK
                    };
                    ui.colored_label(cor, m);
                    ui.separator();
                }
                ui.colored_label(utsim_ui_tema::colors::WARNING, tx("app-aviso_didatico"));
            });
        });
        egui::CentralPanel::default().show(ui, |ui| match self.tela {
            Tela::Inicio => self.abertura(ui),
            Tela::Fabrica => {
                if let Some(p) = &mut self.projeto
                    && let Some(m) = self.fabrica.ui(ui, p)
                {
                    self.mensagem = Some(m);
                }
                if let Some(t) = self.fabrica.ir_para.take() {
                    self.tela = t;
                }
            }
            Tela::Maquinas => {
                if let Some(p) = &mut self.projeto
                    && let Some(m) = crate::maquinas::ui(ui, p)
                {
                    self.mensagem = Some(m);
                }
            }
            Tela::Fatores => {
                if let Some(p) = &mut self.projeto
                    && let Some(m) = crate::fatores::ui(ui, p)
                {
                    self.mensagem = Some(m);
                }
            }
            _ => {}
        });
    }

    /// A barra lateral: o app, o modelo aberto e as telas. As telas das próximas fases
    /// aparecem apagadas, com a fase em que chegam.
    fn barra_lateral(&mut self, ui: &mut egui::Ui) {
        use utsim_ui_tema::navegacao::{Barra, Grupo, Item};
        let sem = self.projeto.is_none();
        let motivo = tx("app-abra_um_modelo");
        let it = |t, nome| Item::novo(t, nome).bloqueado_se(sem, motivo);
        let depois = |t, nome, fase| Item::novo(t, nome).bloqueado_se(true, fase);
        let grupos = [
            Grupo {
                titulo: None,
                itens: vec![Item::novo(Tela::Inicio, tx("app-inicio"))],
            },
            Grupo {
                titulo: Some(tx("app-grupo_ver")),
                itens: vec![
                    it(Tela::Fabrica, tx("app-fabrica")),
                    depois(Tela::Fluxos, tx("app-fluxos"), tx("app-fase_am6")),
                    depois(Tela::Produto, tx("app-produto"), tx("app-fase_am7")),
                ],
            },
            Grupo {
                titulo: Some(tx("app-grupo_dados")),
                itens: vec![
                    it(Tela::Maquinas, tx("app-maquinas")),
                    it(Tela::Fatores, tx("app-fatores")),
                ],
            },
            Grupo {
                titulo: Some(tx("app-grupo_decidir")),
                itens: vec![
                    depois(Tela::Cenarios, tx("app-cenarios"), tx("app-fase_am8")),
                    depois(Tela::Metas, tx("app-metas"), tx("app-fase_am9")),
                    depois(Tela::Relatorios, tx("app-relatorios"), tx("app-fase_am3")),
                ],
            },
        ];
        let linhas: Vec<String> = self
            .projeto
            .iter()
            .flat_map(|p| {
                let mut v = vec![p.nome()];
                if p.mudou() {
                    v.push(tx("app-nao_salvo").into());
                }
                v
            })
            .collect();
        Barra {
            nome: crate::TITULO,
            cor: COR_APP,
            icone: Some(crate::ICONE),
            linhas: &linhas,
            grupos: &grupos,
        }
        .mostrar(ui, &mut self.tela);
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            let mut i = utsim_idiomas::atual();
            if utsim_idiomas::seletor(ui, "idioma", &mut i) {
                utsim_idiomas::definir(i);
                self.config.idioma = Some(i.codigo().to_string());
                self.gravar_config();
            }
        });
    }

    /// A abertura: abrir um modelo do UTSim, o exemplo, os recentes e onde fica salvo.
    fn abertura(&mut self, ui: &mut egui::Ui) {
        let acoes = [
            Acao::nova(
                "abrir",
                Simbolo::Modelo,
                tx("app-acao_abrir"),
                tx("app-acao_abrir_desc"),
            )
            .principal(),
            Acao::nova(
                "exemplo",
                Simbolo::Exemplo,
                tx("app-acao_exemplo"),
                tx("app-acao_exemplo_desc"),
            ),
        ];
        let recentes: Vec<Recente> = self.config.recentes.iter().map(Recente::de).collect();
        let aberto = self.projeto.as_ref().map(|p| p.caminho.clone());
        let escolha = Abertura {
            nome: crate::TITULO,
            subtitulo: tx("app-subtitulo"),
            cor: COR_APP,
            icone: Some(crate::ICONE),
            acoes: &acoes,
            titulo_recentes: tx("app-recentes"),
            recentes: &recentes,
            onde: Onde {
                caminho: aberto.as_deref(),
                explicacao: tx("app-onde_explicacao"),
            },
            rotulos: Rotulos::atual(),
        }
        .mostrar(ui, |_| {}, |_| {});
        match escolha {
            Some(Escolha::Acao("abrir")) => {
                let atual = aberto
                    .as_ref()
                    .map(|c| c.display().to_string())
                    .unwrap_or_default();
                if let Some(c) = utsim_ui_tema::arquivos::procurar(
                    utsim_ui_tema::arquivos::Tipo::Abrir,
                    tx("app-acao_abrir"),
                    &[utsim_ui_tema::arquivos::RON],
                    &atual,
                ) {
                    self.abrir(&c, None);
                }
            }
            Some(Escolha::Acao("exemplo")) => {
                let pasta = utsim_ui_tema::arquivos::documentos()
                    .unwrap_or_else(std::env::temp_dir)
                    .join("UTSim Ambiental")
                    .join("exemplos");
                match crate::projeto::gravar_exemplo(&pasta) {
                    Ok(c) => self.abrir(&c, None),
                    Err(e) => self.avisar(fx("app-erro_abrir", &[("e", vx(&e))]), true),
                }
            }
            Some(Escolha::Recente(i)) => {
                if let Some(c) = self.config.recentes.get(i).cloned() {
                    self.abrir(Path::new(&c), None);
                }
            }
            Some(Escolha::Esquecer(i)) => {
                if i < self.config.recentes.len() {
                    self.config.recentes.remove(i);
                    self.gravar_config();
                }
            }
            Some(Escolha::MostrarPasta(c)) => {
                let _ = utsim_ui_tema::arquivos::mostrar_no_explorador(&c);
            }
            Some(Escolha::Acao(_)) | None => {}
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        utsim_ui_tema::maximizar_no_inicio(ui.ctx());
        self.draw(ui);
    }
}
