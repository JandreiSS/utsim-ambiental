# Formato do bloco ambiental

Este documento descreve o formato dos dados do UTSim Ambiental (fase AM0 do
[plano](plano_ambiental.md), seção 4.2). Os tipos ficam em
`crates/utsim-ambiental-core/src/formato.rs`.

## Onde fica

Enquanto as bibliotecas da família não têm o campo `environment`, o bloco mora num
arquivo ao lado do modelo, com o mesmo nome e o fim `.ambiental.ron`:

```text
linha.ron              o modelo do UTSim (não muda)
linha.ambiental.ron    o bloco ambiental
```

O arquivo lateral tem dois campos:

```ron
(
    environment: ( ... ),        // o bloco do modelo (catálogo, fatores, relógio)
    objects: {                   // o bloco de cada objeto, pelo nome do objeto
        "estufa": ( ... ),
    },
)
```

Quando o campo entrar no modelo (fase AM10), `environment` vai para o modelo e cada bloco de
`objects` vai para o objeto de mesmo nome. A estrutura não muda.

A simulação não lê nenhum destes campos: a conta é feita depois da rodada, sobre o
relatório. Ligar o bloco ambiental não muda nenhum tempo nem nenhum sorteio.

## Bloco do objeto

Todos os campos são opcionais.

| Campo | Tipo | O que é |
|---|---|---|
| `stage` | texto | a etapa do processo ("pintura") |
| `power` | potências em kW | `producing`, `preparing`, `waiting`, `stopped`, `off_shift` e `by_state` |
| `energy_per` | lista | energia por evento: `(driver: Item, kwh: 0.2)`, `Setup` ou `Startup` (partida: sair de `off_shift`, `down`, `stopped` ou do desligamento em espera) |
| `flows` | lista | `(flow: "gas_natural", amount: 4.0, driver: Time([Processing, Idle]))` |
| `water` | frações | `incorporated`, `evaporated`, `reuse_to`, `reuse` (conta na AM5) |
| `effluent` | destino e concentrações | conta na AM5 |
| `standby` | `(after: 600.0)` | desligamento em espera: a espera além de `after` (unidades do modelo) conta com a potência `off_shift`, e a volta conta uma partida |

### Grupos de estado

| Grupo | Campo | Estados do motor |
|---|---|---|
| produzindo | `producing` | `processing`, `utilize` |
| preparando | `preparing` | `setup`, `entering`, `leaving`, `collecting`, `releasing` |
| esperando | `waiting` | `idle`, `empty`, `blocked`, `waiting_operator`, `waiting_transport` |
| parada | `stopped` | `down`, `waiting_repair`, `maintenance`, `stopped` |
| fora de turno | `off_shift` | `off_shift` |

`by_state: {"blocked": 16.0}` vale no lugar do grupo para aquele estado. Estados fora dos
cinco grupos (`generating`, viagens de veículos) só contam com potência em `by_state`.

**Energia sem valor agregado** = esperando + parada + fora de turno.

### Direcionadores dos fluxos

| Direcionador | Também aceito | Conta hoje |
|---|---|---|
| `Item` | `Peca` | sim: peças que entraram no nó |
| `GoodItem` | `PecaBoa` | sim: peças que saíram do nó |
| `Time([estados])` | `Tempo` | sim: horas nos estados dados |
| `Scrap`, `Rework` | `Refugo`, `Retrabalho` | fase AM5 |
| `Setup`, `Startup` | `Partida` | sim (pelo rastro) |
| `Distance(loaded, empty)`, `Volume`, `Mass` | `Distancia`, `Massa` | fase AM5 |

O que ainda não conta aparece na tela, na lista "Ainda não contado".

## Bloco do modelo

| Campo | O que é |
|---|---|
| `clock` | `start` ("06:00") e `time_unit` (`Second`, `Minute`, `Hour`): a unidade de tempo do modelo |
| `flows` | os fluxos: `(name, kind, unit, direction)`; `kind` é `Energy`, `Fuel`, `Water`, `Material`, `Waste`, `Effluent`, `Emission` ou `Other` |
| `factors` | `(flow, scope, kg_co2e, source, year)`, mais `range` e `didactic` opcionais |
| `gases`, `waste_destinations`, `effluent`, `tariff`, `prices`, `allocation`, `targets` | das fases AM4 a AM9 |

A potência dos nós é contada no **primeiro fluxo do tipo `Energy`** (o app cria
"eletricidade", em kWh).

`kg_co2e: None` quer dizer "a definir": o indicador que depende do fator mostra "falta
fator", nunca zero. O app não traz número de fator. Fator com `source` vazia ou "a
definir" aparece com o selo "sem fonte"; com `didactic: true`, com o selo "didático".

## Validação

`validar(modelo, lateral)` devolve a lista de problemas (graves ou avisos):

- objeto do bloco que não existe no modelo (aviso);
- fluxo usado num nó que não está no catálogo;
- potência, quantidade ou fator negativos;
- frações de água fora de 0 a 1 ou somando mais que 1; reuso para objeto inexistente;
- parâmetro de efluente fora do catálogo;
- janela de pico fora de um dia, intervalo de demanda zero;
- fluxo repetido no catálogo, escopo fora de 1 a 3;
- catálogo sem fluxo de energia (aviso);
- fluxo que soma ou tira massa sem massa nos tipos de item (aviso).

## Métricas com nome

Por replicação, como qualquer métrica da simulação (média e intervalo de 95 % entre
replicações):

| Chave | O que é |
|---|---|
| `<nó>.energia_kwh` | energia do nó |
| `<nó>.energia_kwh.<grupo>` | por grupo: `produzindo`, `preparando`, `esperando`, `parada`, `fora_de_turno` |
| `<nó>.energia_sem_valor_kwh` | esperando, parada e fora de turno |
| `<nó>.kwh_por_peca` | energia por peça que saiu do nó |
| `<nó>.fluxo.<fluxo>` | quantidade do fluxo, na unidade do catálogo |
| `<nó>.co2e_kg` | emissão do nó (só quando todos os fatores usados existem) |
| `ambiental.energia_kwh`, `ambiental.energia_sem_valor_kwh` | soma dos nós |
| `ambiental.kwh_por_peca` | energia por peça boa (as que chegam às saídas que não são de refugo) |
| `ambiental.co2e_kg`, `ambiental.co2e_kg.escopo<n>` | emissão total e por escopo |
