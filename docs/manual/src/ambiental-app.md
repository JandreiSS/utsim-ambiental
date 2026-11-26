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

## Máquinas

Uma linha por máquina: a etapa e a potência (kW) em cada grupo de estado. Tire a potência da
placa da máquina, do catálogo do fabricante ou de uma medição.

## Fatores

- A **unidade de tempo do modelo** (segundo, minuto ou hora): as horas em cada estado
  dependem dela.
- Os fluxos (eletricidade, gás…) e o fator de emissão de cada um, em kg de CO₂ equivalente
  por unidade, com a **fonte** e o **ano**. O app não traz número de fator: enquanto o fator
  não é informado, o CO₂ aparece como "falta fator".
- Fator sem fonte aparece com o selo "sem fonte" em todas as telas.

## Salvar

**Salvar** grava o bloco ambiental num arquivo ao lado do modelo (`<modelo>.ambiental.ron`).
O modelo do UTSim não muda.
