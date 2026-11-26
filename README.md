# UTSim Ambiental

Este repositório tem o **UTSim Ambiental**, o app que mostra o que a fábrica simulada
consome e descarta: energia, emissões em CO₂ equivalente e, nas próximas fases, água,
resíduos e efluentes. Ele faz parte da família UTSim: cada app tem o seu repositório, e as
bibliotecas que os apps compartilham ficam em
[utsim-familia](https://github.com/JandreiSS/utsim-familia).

As versões prontas para instalar ficam na página de
[Releases](https://github.com/JandreiSS/utsim-ambiental/releases). São pacotes cifrados:
instale-os pelo UTSim Launcher.

## A ideia em uma frase

Toda conta do app tem a forma **direcionador simulado × coeficiente do nó × fator do
catálogo**. A simulação mede quanto tempo cada máquina passou em cada estado (produzindo,
esperando, parada…); o aluno informa a potência da máquina em cada estado; o catálogo do
modelo diz quantos kg de CO₂ equivalente sai de cada kWh. O app multiplica, soma e mostra
na planta da fábrica.

O plano completo, com todas as fases (AM0 a AM16), está em
[`docs/plano_ambiental.md`](docs/plano_ambiental.md). O formato dos dados está em
[`docs/formato_ambiental.md`](docs/formato_ambiental.md). O manual de quem usa o app está em
[`docs/manual/src/ambiental-app.md`](docs/manual/src/ambiental-app.md).

## Onde fica cada coisa

- `crates/utsim-ambiental-core/`: o núcleo, sem janela. Toda conta mora aqui.
  - `formato.rs`: os tipos do bloco ambiental (nó e modelo) e o arquivo lateral.
  - `validacao.rs`: o que pode estar errado no bloco.
  - `conta.rs`: energia, emissão, métricas com nome, vilões e a diferença pareada.
  - `mapa.rs`: a planta vista de cima (posições e ligações).
  - `rodar.rs`: as replicações com o rastro (mudanças de estado e passagens das peças).
  - `medida.rs`: horas, peças, setups, partidas e a linha do tempo de cada nó numa janela.
  - `tarifa.rs`: demanda por intervalo, pico, fator de carga e a conta de energia.
  - `materiais.rs`: o caminho de cada peça (massa, sucata, balanço de massa).
  - `balanco.rs`: água, efluentes, resíduos, mochila do refugo e custos.
- `crates/utsim-ambiental-app/`: a janela (egui). Não faz conta, só desenha.
  - `app.rs`: abertura, barra lateral, rodapé. `fabrica.rs`: o mapa de calor.
    `maquinas.rs`: potência de cada máquina. `fatores.rs`: fatores de emissão.
    `projeto.rs`: o modelo aberto, a simulação numa thread e a gravação.
    `relatorio.rs`: o relatório em HTML e PDF.
  - `tests/capturas.rs`: desenha as telas sem janela (com `UTSIM_CAPTURAS=<pasta>` grava PNG).
- `exemplos/`: um modelo de exemplo (linha de pintura) com o bloco ambiental ao lado.
- `idiomas/<língua>/ambiental.ftl`: os textos da interface nas cinco línguas
  (pt-BR, en, es, fr, it). As telas não escrevem texto literal.
- `assets/icone/familia/`: o ícone (folha sobre engrenagem, verde); `fonte/ambiental.py`
  desenha os PNG e o ICO.
- `scripts/verificar.sh`: a verificação completa. Rode antes de cada commit.

## Compilar, rodar e verificar

Precisa do Rust estável (https://rustup.rs) e do Git.

```bash
cargo run --release -p utsim-ambiental-app      # abre a janela
cargo test --workspace                           # os testes
bash scripts/verificar.sh                        # formatação, lints, testes e build; termina com "Tudo certo"
```

Na primeira compilação, o Cargo baixa a família do GitHub, na tag fixada no `Cargo.toml`
(hoje `v0.1.0`). Todos os crates da família usam a mesma tag.

Para abrir direto num modelo (é assim que os outros apps chamam este):

```bash
utsim-ambiental --utsim-abrir=<modelo.ron> --utsim-no=<nome do nó>
```

## Onde mora o bloco ambiental

Enquanto as bibliotecas da família não têm o campo `environment` no modelo, o bloco fica
num arquivo ao lado do modelo: `linha.ron` → `linha.ambiental.ron`. O modelo não muda, e a
simulação não lê esse arquivo (ligar o bloco ambiental não muda nenhum tempo). Quando o
campo entrar na família (fase AM10), o bloco muda de lugar, não de formato.

## Regras de honestidade (valem para todo código novo)

- O app **não traz número de fator de emissão**. O catálogo nasce "a definir"; indicador
  com fator ausente mostra "falta fator", nunca zero.
- Fator sem fonte aparece com o selo "sem fonte"; fator didático, com o selo "didático".
- Todo número é média entre replicações, com intervalo de 95 %.
- Todo número leva a "De onde veio": direcionador, coeficiente, fator e fonte.
- O que a versão ainda não conta aparece na lista "Ainda não contado", nunca some calado.

## O que já existe e o que vem depois

Pronto (fases AM0 a AM3): formato e validação, energia e CO₂e por nó e por grupo de
estado, mapa de calor, vilões, "De onde veio", telas Máquinas e Fatores, recálculo na hora,
"Antes e depois" com diferença pareada, curva de carga pelo rastro, "Tocar o dia", janela de
tempo arrastável, energia por setup e por partida, desligamento em espera, relatório em
HTML e PDF, demanda, horário de pico, fator de carga e conta de energia simulada (AM4),
massa, sucata, mochila do refugo, água, efluentes e resíduos (AM5).

Próximos passos, na ordem do plano: AM6 (Sankey de massa e energia) e AM8 (cenários), depois
AM7 (pegada por produto) e AM9 (metas). A seção 8 do plano lista ideias de TCC por curso.

## Mudar o núcleo da família junto com o app

Quando a mudança precisa mexer na família:

```bash
git clone https://github.com/JandreiSS/utsim-familia ../utsim-familia
bash scripts/familia-local.sh liga         # usa ../utsim-familia em vez da tag
# ... mude os dois, rode os testes dos dois ...
```

Depois:

1. Na família: `bash scripts/verificar.sh`, commit, tag nova (por exemplo `v0.2.0`) e push.
2. Aqui: `bash scripts/familia-local.sh desliga`, depois
   `bash scripts/atualizar-familia.sh v0.2.0`, depois `bash scripts/verificar.sh`, e o commit.

O `verificar.sh` recusa rodar com a família local ligada, para nada ir ao GitHub apontando
para uma pasta do seu computador.

## Regras do projeto

- Não há GitHub Actions: a verificação roda na máquina, com `scripts/verificar.sh`.
- Integração só na branch `main`. Trabalhe numa branch sua e junte em `main` depois de
  verificar.
- Não cite outros softwares de simulação ou de avaliação ambiental pelo nome (código,
  comentários, docs, commits).
- Textos da interface vêm sempre de `idiomas/`.
- Não acrescente arquivo ou texto de licença.

## Licença

Licença ainda não definida; para reutilizar, fale com o autor.

## Publicar uma versão (autor)

`scripts/publicar-ambiental.sh` gera o pacote cifrado e o manifesto local; depois
`utsim-publicar github` sobe o pacote como release deste repositório e o manifesto assinado
para o repositório do launcher.
