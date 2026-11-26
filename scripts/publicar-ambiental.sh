#!/usr/bin/env bash
# Publica o UTSim Ambiental pelo launcher, como produto próprio (`ambiental`): monta a
# pasta do programa, cifra, assina e grava o pacote e o manifesto na pasta de publicação (local). Não envia nada à internet: depois, 'utsim-publicar github' sobe o pacote e o
# manifesto para o GitHub. O launcher já instalado passa a mostrar o produto; ele não precisa mudar.
#
# Uso: bash scripts/publicar-ambiental.sh <pasta_de_publicação> [canal] [versão]
#   canal   padrão: estavel (o launcher 0.4.0 mostra todos; o canal só conta para launchers antigos)
#   versão  padrão: a de crates/utsim-ambiental-app/Cargo.toml (versão própria, tag am-v<versão>)
#
# Antes: cargo build --release -p utsim-ambiental-app -p utsim-publicar
# Notas da versão: a seção "## [am-<versão>]" do CHANGELOG.md, se existir; senão, os itens
# da seção "## [Não lançado]" que citam o UTSim Ambiental.

set -eu

cd "$(dirname "$0")/.." || exit 1

[ $# -ge 1 ] || { echo "uso: $0 <pasta_de_publicação> [canal] [versão]" >&2; exit 2; }
SAIDA=$1
CANAL=${2:-estavel}
VERSAO=${3:-$(awk -F'"' '/^version = /{print $2; exit}' crates/utsim-ambiental-app/Cargo.toml)}
VERSAO=${VERSAO#am-}
VERSAO=${VERSAO#v}

PUBLICAR=target/release/utsim-publicar
[ -x "$PUBLICAR" ] || PUBLICAR=target/release/utsim-publicar.exe
[ -x "$PUBLICAR" ] || { echo "erro: rode antes cargo build --release -p utsim-publicar" >&2; exit 1; }

case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*) PLATAFORMA=windows-x64; NOME=utsim-ambiental.exe ;;
    Linux) PLATAFORMA=linux-x64; NOME=utsim-ambiental ;;
    *) echo "erro: plataforma não suportada" >&2; exit 1 ;;
esac
BIN=target/release/$NOME
[ -f "$BIN" ] || { echo "erro: $BIN não encontrado; rode antes cargo build --release -p utsim-ambiental-app" >&2; exit 1; }

PASTA=$(mktemp -d)
trap 'rm -rf "$PASTA"' EXIT
mkdir -p "$PASTA/ambiental/bin" "$PASTA/ambiental/manual" "$PASTA/ambiental/exemplos"
cp "$BIN" "$PASTA/ambiental/bin/"
cp docs/manual/src/ambiental-app.md "$PASTA/ambiental/manual/"
cp exemplos/*.ron "$PASTA/ambiental/exemplos/"

if grep -q "^## \[am-$VERSAO\]" CHANGELOG.md; then
    awk -v s="## [am-$VERSAO]" 'index($0, s) == 1 { on = 1; next } /^## \[/ { on = 0 } on' CHANGELOG.md > "$PASTA/notas.md"
else
    awk '/^## \[/ { n++ } n == 1 && /^- / && /UTSim Ambiental/ { print }' CHANGELOG.md > "$PASTA/notas.md"
fi
[ -s "$PASTA/notas.md" ] || echo "(sem notas)" > "$PASTA/notas.md"

"$PUBLICAR" publicar \
    --saida "$SAIDA" \
    --produto ambiental \
    --nome "UTSim Ambiental" \
    --descricao "Energia, emissões e impacto ambiental da fábrica simulada, da família UTSim" \
    --subtitulo "Energia e impacto ambiental" \
    --autor "UTSim" \
    --cor "#2FA24A" \
    --icone assets/icone/familia/ambiental_256.png \
    --versao "$VERSAO" \
    --canal "$CANAL" \
    --plataforma "$PLATAFORMA" \
    --pasta "$PASTA/ambiental" \
    --executavel "bin/$NOME" \
    --notas "$PASTA/notas.md"
