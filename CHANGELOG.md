# Changelog

As mudanças deste repositório ficam registradas aqui.

## [Não lançado]

### Novo
- UTSim Ambiental nasce como repositório próprio da família UTSim, usando a família pela tag v0.1.0.
- UTSim Ambiental, fase AM0: formato do bloco ambiental (nó e modelo) no arquivo lateral `<modelo>.ambiental.ron`, com validação.
- UTSim Ambiental, fase AM1: energia por nó e por grupo de estado, kWh por peça, energia sem valor agregado, CO₂ equivalente por escopo e métricas com nome, com intervalo de 95 % entre replicações.
- UTSim Ambiental, fase AM2 (MVP "a fábrica acende"): abertura da família, mapa de calor da fábrica, painel do nó, "Os três vilões", cartão "De onde veio", telas Máquinas e Fatores, exemplo de linha de pintura.
- UTSim Ambiental, fase AM3: a conta se refaz na hora ao mudar uma potência ou um fator, e o "Antes e depois" mostra a diferença pareada com intervalo.
- UTSim Ambiental, fase AM3: rastro da simulação (sem mudar nenhum número), curva de carga, "Tocar o dia" com pausa no pico, janela de tempo arrastável, energia por setup e por partida, desligamento em espera e relatório ambiental em HTML e PDF.
- UTSim Ambiental, fase AM4: tarifa de energia no modelo, demanda por intervalo alinhada ao relógio, horário de pico (só em dias úteis, se pedido), fator de carga e conta de energia simulada, só com os preços informados.
- UTSim Ambiental, fase AM5: massa das peças e balanço de massa, sucata por destino e a mochila da peça refugada, água com evaporação e reuso, efluente com carga antes e depois do tratamento (energia e lodo do tratamento), resíduos por destino com desvio de aterro, preços dos fluxos; editor de fluxos, água e efluente por máquina; exemplo com água, tinta, borra e refugo.
- UTSim Ambiental, fase AM6: tela Fluxos com o diagrama de Sankey em massa, energia, água e carbono, caminho de uma faixa, agrupamento por etapa, limiar dos fios finos, selo de balanço dos nós, exportação em SVG e do inventário em CSV.
- UTSim Ambiental, fase AM7: tela Produto com a pegada de cada tipo de peça por peça boa ou por kg, fronteira portão a portão ou berço ao portão, rateio pelo tempo, pelas peças, pela massa ou com a espera como estrutura (produtos + estrutura = fábrica), contribuição por máquina, taxa de refugo, comparação de dois produtos e relatório do estudo em HTML e PDF.
