# Pedidos do UTSim Ambiental para os outros apps

Fase AM11 do [plano](plano_ambiental.md) (seções 4.4 e 4.6). Os pedidos usam o formato de
`utsim-troca` da família (`<id>.utsim-pedido.json`, resposta em `<id>.utsim-resposta.json`)
e ficam na pasta `troca/` ao lado do modelo. O Ambiental abre o outro app pelo launcher com
`--utsim-pedido=<caminho>`.

Este documento é o contrato para quem implementa o lado de lá (UTSim Estatística e UTSim
LSS). Enquanto o lado de lá não existir, o Ambiental grava o pedido e segue funcionando: o
teste da redução também é feito no próprio Ambiental.

## Para a Estatística: `analisar` (teste t pareado)

```json
{
  "formato": 1,
  "id": "…",
  "de": "ambiental 0.3.0",
  "acao": "analisar",
  "ferramenta": "teste_t_pareado",
  "entrada": {
    "arquivos": [{ "caminho": ".../troca/Cenario_1_ambiental_co2e_kg.csv", "sha256": "…" }],
    "colunas": ["cenario", "base"]
  },
  "contexto": { "modelo": ".../linha.ron" },
  "resposta": ".../troca/<id>.utsim-resposta.json",
  "somente_leitura": true
}
```

O CSV tem `;` como separador e ponto decimal, uma linha por replicação:
`replicacao;base;cenario;diferenca`. As replicações são pareadas (mesma semente).

A hipótese é H₁: `cenario − base < 0` (o cenário reduz). A resposta esperada traz, em
`resultado.dados`:

```json
{ "metrica": "ambiental.co2e_kg", "diferenca": -12.3, "de": -15.0, "ate": -9.6,
  "p_valor": 0.0012, "n": 10 }
```

O Ambiental grava esse teste no cenário com a origem "UTSim Estatística". Hoje a
Estatística instalada abre o CSV como planilha (`--utsim-abrir=`), que vai junto no
comando.

## Para o LSS: `nova_oportunidade`

```json
{
  "formato": 1,
  "id": "…",
  "de": "ambiental 0.3.0",
  "acao": "nova_oportunidade",
  "entrada": {
    "arquivos": [{ "caminho": ".../linha.cenarios_ambientais.json", "sha256": "…" }],
    "conteudo": "<o resumo abaixo, em JSON>"
  },
  "contexto": { "modelo": ".../linha.ron" }
}
```

O resumo (`entrada.conteudo`):

| Campo | O que é |
|---|---|
| `cenario` | o nome do cenário |
| `alavancas` | as mudanças, em texto |
| `diferencas` | `[métrica, diferença média, de, até]` para o Base (intervalo de 95 %, pareado): CO₂e, energia, água, conta de energia, custo |
| `investimento`, `vida_anos` | o custo de implantação e a vida útil, quando informados |
| `linha_vsm` | a linha ambiental do mapa de fluxo de valor: por máquina, `no`, `etapa`, `kwh_por_peca`, `co2e_por_peca` |
| `testes` | os testes da redução já gravados |

O LSS confere o hash do arquivo de cenários e cria a ideia na carteira. O ganho ambiental
fica ao lado do financeiro, sem converter um no outro (decisão A25).
