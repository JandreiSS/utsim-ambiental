# Changelog

As mudanças deste repositório ficam registradas aqui.

## [Não lançado]

### Novo
- UTSim Ambiental, fase AM9: tela Metas com um cartão por meta (indicador, alcance fábrica/etapa/máquina, base total/por peça boa/por hora, linha de base, alvo e prazo), semáforo pela fração das replicações que cumprem a meta com intervalo de Wilson, orçamento de carbono, cenários ligados como ações com a cascata da linha de base ao alvo, leituras reais ao lado do simulado e selos de conquista que o professor pode desligar.

## [am-0.2.0] - 2026-11-27

### Novo
- UTSim Ambiental, fase AM6: tela Fluxos com o diagrama de Sankey em massa, energia, água e carbono, caminho de uma faixa, agrupamento por etapa, limiar dos fios finos, selo de balanço dos nós, exportação em SVG e do inventário em CSV.
- UTSim Ambiental, fase AM7: tela Produto com a pegada de cada tipo de peça por peça boa ou por kg, fronteira portão a portão ou berço ao portão, rateio pelo tempo, pelas peças, pela massa ou com a espera como estrutura (produtos + estrutura = fábrica), contribuição por máquina, taxa de refugo, comparação de dois produtos e relatório do estudo em HTML e PDF.
- UTSim Ambiental, fase AM8: tela Cenários com a prateleira de cenários e as alavancas (potência, desligamento em espera, fator, destino de resíduo, reuso, eficiência do tratamento, preço da energia, demanda contratada, janela de pico e mudanças no modelo), recálculo na hora sobre o mesmo rastro ou simulação com a mesma semente do Base, custo e impacto com a diferença pareada e o veredito, dispersão custo × impacto com a fronteira, retorno simples e custo por tonelada evitada; cenários gravados ao lado do modelo.

## [am-0.1.0] - 2026-11-26

### Novo
- UTSim Ambiental nasce como repositório próprio da família UTSim, usando a família pela tag v0.1.0.
- UTSim Ambiental, fase AM0: formato do bloco ambiental (nó e modelo) no arquivo lateral `<modelo>.ambiental.ron`, com validação.
- UTSim Ambiental, fase AM1: energia por nó e por grupo de estado, kWh por peça, energia sem valor agregado, CO₂ equivalente por escopo e métricas com nome, com intervalo de 95 % entre replicações.
- UTSim Ambiental, fase AM2 (MVP "a fábrica acende"): abertura da família, mapa de calor da fábrica, painel do nó, "Os três vilões", cartão "De onde veio", telas Máquinas e Fatores, exemplo de linha de pintura.
- UTSim Ambiental, fase AM3: a conta se refaz na hora ao mudar uma potência ou um fator, e o "Antes e depois" mostra a diferença pareada com intervalo.
- UTSim Ambiental, fase AM3: rastro da simulação (sem mudar nenhum número), curva de carga, "Tocar o dia" com pausa no pico, janela de tempo arrastável, energia por setup e por partida, desligamento em espera e relatório ambiental em HTML e PDF.
- UTSim Ambiental, fase AM4: tarifa de energia no modelo, demanda por intervalo alinhada ao relógio, horário de pico (só em dias úteis, se pedido), fator de carga e conta de energia simulada, só com os preços informados.
- UTSim Ambiental, fase AM5: massa das peças e balanço de massa, sucata por destino e a mochila da peça refugada, água com evaporação e reuso, efluente com carga antes e depois do tratamento (energia e lodo do tratamento), resíduos por destino com desvio de aterro, preços dos fluxos; editor de fluxos, água e efluente por máquina; exemplo com água, tinta, borra e refugo.
