# Formato do bloco ambiental

Este documento descreve o formato dos dados do UTSim Ambiental (fase AM0 do
[plano](plano_ambiental.md), seção 4.2). Os tipos ficam em
`crates/utsim-ambiental-core/src/formato.rs`.

## Onde fica

Desde a família 0.2.0 (fase AM10), o bloco mora dentro do modelo: `environment` em cada
objeto e no modelo, e `mass_kg` em cada tipo de item, com os mesmos tipos
(`utsim_model::environment`). O UTSim edita a etapa e a potência na aba Ambiental do nó.
Um modelo sem o bloco usa ainda o arquivo lateral abaixo.

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
| `water` | frações | `incorporated`, `evaporated`, `reuse_to`, `reuse`: o efluente é a entrada de água × (1 − incorporada − evaporada), e a fração `reuse` dele volta como entrada de `reuse_to` |
| `effluent` | destino e concentrações | `(destination: "tratamento_proprio", concentration: {"materia_organica": 350.0})`, em mg/L |
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
| `Scrap` | `Refugo` | sim: peças que saíram do nó direto para uma saída de refugo |
| `Rework` | `Retrabalho` | ainda não |
| `Setup`, `Startup` | `Partida` | sim (pelo rastro) |
| `Mass` | `Massa` | sim: kg que entraram no nó |
| `Distance(loaded, empty)`, `Volume` | `Distancia` | ainda não (veículos e rede de fluidos) |

O que ainda não conta aparece na tela, na lista "Ainda não contado".

## Bloco do modelo

| Campo | O que é |
|---|---|
| `clock` | `start` ("06:00") e `time_unit` (`Second`, `Minute`, `Hour`): a unidade de tempo do modelo |
| `flows` | os fluxos: `(name, kind, unit, direction)`; `kind` é `Energy`, `Fuel`, `Water`, `Material`, `Waste`, `Effluent`, `Emission` ou `Other` |
| `factors` | `(flow, scope, kg_co2e, source, year)`, mais `range` e `didactic` opcionais |
| `tariff` | a tarifa (fase AM4), abaixo |
| `item_mass_kg` | massa de cada tipo de item, `{"*": 2.5}` para as peças sem tipo (vai para `item_types` na AM10) |
| `scrap_destination` | o destino da sucata |
| `waste_destinations` | `(name, diverted, kg_co2e, cost, source)`: o destino dos resíduos, com o fator (escopo 3) e o custo por kg |
| `effluent` | `parameters: [(name, unit, limit, limit_source)]` e `destinations: [(name, removal, kwh_per_m3, sludge_kg_per_m3, cost_per_m3)]` |
| `prices` | preço por unidade de cada fluxo |
| `allocation` | a regra de rateio para o produto: `ByProcessTime` (padrão), `ByItems`, `ByMass` ou `None` (a espera fica como estrutura) |
| `targets` | as metas: `(id, indicator, scope: Factory \| Stage("…") \| Node("…"), basis: Total \| PerGoodItem \| PerHour, baseline, target, due, actions)` |
| `carbon_budget_kg`, `hide_badges` | o orçamento de carbono do período e a chave dos selos de conquista |
| `gases` | fase posterior |

As leituras reais das metas ficam em `<modelo>.leituras_ambientais.json`, e os cenários em
`<modelo>.cenarios_ambientais.json`, os dois ao lado do modelo.

Fluxos de água ficam em `L` ou `m3`. Fluxos de resíduo ficam em kg e levam o `destination` do
fluxo; a emissão deles é a do destino. A sucata e o lodo do tratamento entram nos resíduos
da fábrica; a energia do tratamento entra na energia da fábrica.

### Tarifa

```ron
tariff: Some((
    peak: (from: "18:00", to: "21:00", weekdays_only: true),
    demand_interval_min: 15,
    contracted_kw: Some(60.0),
    price_kwh_peak: None, price_kwh_off_peak: None, price_kw: None,
    price_kw_overrun: None,           // preço de cada kW acima da contratada
    source: "a definir",
)),
```

- A demanda de um intervalo é a energia dele (curva de carga mais a energia por evento)
  dividida pela duração. Os intervalos seguem o relógio do modelo (`clock.start`); nas
  bordas da janela, o intervalo incompleto usa a duração que está dentro dela.
- Um intervalo é de pico quando o meio dele cai entre `from` e `to`. Com `weekdays_only`, o
  primeiro dia da simulação é uma segunda-feira.
- A conta da janela: energia no pico e fora × preços + demanda cobrada (a contratada ou a
  medida, a maior) × `price_kw` + ultrapassagem × `price_kw_overrun`. Faltando um preço que a
  conta usa, não há conta.

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
| `ambiental.demanda_max_kw`, `ambiental.demanda_pico_kw` | maior demanda na janela e nos intervalos de pico |
| `ambiental.energia_pico_kwh`, `ambiental.fator_de_carga` | energia no pico, demanda média sobre a máxima |
| `ambiental.custo_energia` | conta de energia simulada (só com todos os preços) |
| `<nó>.agua_m3`, `<nó>.efluente_m3`, `<nó>.carga.<parâmetro>_kg` | água, efluente e carga do nó |
| `<nó>.residuo_kg`, `<nó>.sucata_kg` | resíduo de processo e sucata atribuída ao nó |
| `ambiental.agua_m3`, `ambiental.reuso_pct`, `ambiental.efluente_m3`, `ambiental.tratamento_kwh` | captação, reuso, efluente e energia do tratamento |
| `ambiental.carga.<p>_kg`, `ambiental.carga_tratada.<p>_kg` | carga antes e depois do tratamento |
| `ambiental.residuo_kg.<destino>`, `ambiental.desvio_aterro_pct` | resíduos por destino e fração desviada |
| `ambiental.rendimento_material_pct`, `ambiental.balanco_massa_erro_pct` | massa boa sobre a entrada e erro de fechamento |
| `ambiental.mochila_refugo.kwh`, `ambiental.mochila_refugo.co2e_kg` | o que as peças refugadas levaram |
| `ambiental.custo` | custo dos fluxos, resíduos e efluentes com preço |
