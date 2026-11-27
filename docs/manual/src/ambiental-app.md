# UTSim Ambiental

O UTSim Ambiental mostra quanta energia cada máquina da fábrica simulada gasta e quanto CO₂
equivalente isso representa, em cima de um modelo do UTSim.

Os números são simulados, para ensino: não são medição nem inventário oficial.

## Começar

1. Na abertura, clique em **Abrir o exemplo** (uma linha de pintura) ou em **Abrir um
   modelo do UTSim** e escolha um arquivo `.ron`.
2. O app simula o modelo (as replicações do próprio modelo) e abre a tela **Fábrica**.

## Fábrica

- A planta da fábrica vista de cima. Cada máquina tem uma mancha: quanto maior e mais clara,
  mais ela gasta. A legenda embaixo diz a escala.
- No alto, escolha o que pintar: **Energia**, **Energia sem valor** (esperando, parada e
  fora de turno) ou **CO₂ equivalente**; e a base: **Total** ou **Por peça**.
- Máquina tracejada ainda não tem potência informada.
- Clique numa máquina: à direita aparecem a energia por grupo de estado, a energia por peça,
  o CO₂ equivalente e os campos de potência. Mude uma potência e tudo se refaz na hora.
- **Os três vilões**: as máquinas que mais gastam energia sem agregar valor.
- **De onde veio?**: a conta do número: horas em cada estado (da simulação) × potência (sua)
  = energia, e o fator de emissão com a fonte.
- **Antes e depois**: clique em **Guardar como base**, mude uma potência e veja a diferença,
  replicação a replicação, com intervalo de 95 %.
- **Simular de novo** roda a simulação outra vez (por exemplo, depois de mudar o modelo no
  UTSim).

## Curva de carga e "Tocar o dia"

- Embaixo do mapa fica a curva de carga: a potência da fábrica somada, no tempo (primeira
  replicação). O ponto amarelo é o pico.
- **Tocar o dia** reproduz o dia em uns 30 segundos: o mapa mostra a potência de cada
  máquina naquele instante. A reprodução para sozinha no pico e diz quais máquinas estavam
  ligadas juntas.
- Clique na curva para ver um instante. **Arraste** na curva para fazer a conta só naquele
  trecho (um turno, o horário de pico); duplo clique ou **Janela inteira** volta ao todo.

## Máquinas

Uma linha por máquina: a etapa e a potência (kW) em cada grupo de estado. Tire a potência da
placa da máquina, do catálogo do fabricante ou de uma medição.

Em **Detalhes**: a energia por peça, por setup e por partida (ligar depois de parada) e o
**desligamento em espera**: depois de esperar mais que o limite, a máquina conta com a
potência de fora de turno, e cada volta conta uma partida. É uma aproximação: a simulação
não atrasa a máquina para religar.

## Relatórios

**Gerar HTML** ou **Gerar PDF** grava o relatório ao lado do modelo. Ele sempre traz o aviso
de uso didático e a lista de fatores com as fontes.

## Fatores

- A **unidade de tempo do modelo** (segundo, minuto ou hora): as horas em cada estado
  dependem dela.
- Os fluxos (eletricidade, gás…) e o fator de emissão de cada um, em kg de CO₂ equivalente
  por unidade, com a **fonte** e o **ano**. O app não traz número de fator: enquanto o fator
  não é informado, o CO₂ aparece como "falta fator".
- Fator sem fonte aparece com o selo "sem fonte" em todas as telas.

## Tarifa de energia

Em **Fatores**, marque **Usar tarifa neste modelo** e informe o horário de pico, o intervalo
de demanda (15 minutos, se não souber), a demanda contratada e os preços. Nenhum preço vem
pronto: sem um preço que a conta usa, aparece "falta preço".

Na **Fábrica**, a curva de carga mostra o pico sombreado, a demanda de cada intervalo e a
linha da demanda contratada. O painel mostra a demanda máxima, a demanda no pico, a energia
no pico, o fator de carga e a conta de energia simulada da janela.

## Água, resíduos e materiais

- Em **Fatores**: a massa das peças (kg), os destinos de resíduo (com a marca "desviado de
  aterro", fator e custo), o destino da sucata, os parâmetros do efluente (com o limite e a
  fonte que você cadastrar) e os destinos do efluente (remoção de cada parâmetro, energia,
  lodo e custo por m³). Na tabela de fluxos, o preço de cada fluxo e o destino dos resíduos.
- Em **Máquinas → Detalhes**: os fluxos da máquina (água, tinta, borra, gás...), cada um
  por peça, por hora num estado, por setup ou por kg; se o material soma ou tira massa da
  peça; o balanço de água (fração incorporada, evaporada e o reuso) e o efluente.
- Na **Fábrica**: as fichas **Água** e **Resíduos** pintam o mapa; o painel mostra a
  captação, o reuso, o efluente e as cargas antes e depois do tratamento (em vermelho se a
  mistura passa do limite cadastrado: o app compara, não afirma conformidade), os resíduos
  por destino, o rendimento de material, o balanço de massa e a **mochila do refugo**: o que
  as peças refugadas levaram consigo de energia e de emissão.

## Fluxos (diagrama de Sankey)

- Quatro modos: **Massa** (kg), **Energia** (kWh), **Água** (m³) e **Carbono** (kg CO₂e). As
  entradas ficam à esquerda, as máquinas no meio e as saídas à direita; a largura da faixa é
  a quantidade.
- No modo massa, as filas não aparecem: a massa passa direto de uma máquina para a outra.
- Clique numa faixa para acender o caminho dela; **Agrupar por etapa** junta as máquinas da
  mesma etapa (duplo clique abre de novo); **Esconder fluxos menores que** tira os fios finos.
- Uma máquina em que o que entra não bate com o que sai ganha o selo "faltam" ou "sobram";
  clique nela para ver a dica (material somado ou resíduo tirado que faltou cadastrar).
- **Exportar SVG** grava o diagrama ao lado do modelo; **Exportar inventário (CSV)** grava
  todas as métricas com média e intervalo.

## Produto (pegada)

- Escolha o produto (o tipo de peça do modelo), a **unidade funcional** (peça boa ou kg de
  produto) e a **fronteira**: clique em **Matérias-primas** para passar de "portão a portão"
  (só a fabricação) a "berço ao portão" (com os materiais comprados).
- O **rateio** diz como a energia de cada máquina se divide entre os produtos: pelo tempo na
  máquina, pelo número de peças, pela massa, ou com a espera como **estrutura** (fora dos
  produtos). A energia por peça vai sempre direto. Produtos + estrutura = fábrica.
- A **contribuição por máquina** mostra de onde vem a pegada; a **taxa de refugo** mostra
  quanto a pegada da peça boa cresce com mais refugo.
- **Relatório do estudo** grava objetivo, unidade funcional, fronteira, inventário, impacto,
  interpretação (com o efeito da regra de rateio), limitações e fatores com as fontes.

## Cenários

- **+ Novo cenário** cria um cenário; **+ Alavanca** acrescenta uma mudança: potência de uma
  máquina, desligar na espera, fator de emissão, destino de um resíduo, reuso de água,
  eficiência do tratamento, preço da energia, demanda contratada, janela de pico, ou uma
  mudança no modelo (por exemplo `estufa.capacity = 2`).
- As alavancas do bloco ambiental são recalculadas na hora. A mudança no modelo pede
  **Simular este cenário**: o app roda as replicações com a mesma semente do Base, e a
  comparação é pareada.
- A tabela mostra custo e impacto (CO₂e, energia ou água) de cada cenário e a diferença para
  o Base com intervalo de 95 %: **melhor**, **pior** ou **não dá para afirmar**.
- A dispersão custo × impacto liga os cenários que não perdem nos dois critérios.
- Com o **investimento**, a vida útil e as **horas de operação por ano**, o app calcula o
  retorno simples e o custo por tonelada de CO₂e evitada.
- **Salvar cenários** grava `<modelo>.cenarios_ambientais.json` ao lado do modelo.

## Salvar

**Salvar** grava o bloco ambiental num arquivo ao lado do modelo (`<modelo>.ambiental.ron`).
O modelo do UTSim não muda.
