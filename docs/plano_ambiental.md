# Plano: UTSim Ambiental

Documento de desenho, sem implementação, do **UTSim Ambiental**, app novo da família UTSim pedido pelo dono em 2026-11-24. O app mostra o que a fábrica simulada consome e descarta: energia, água, materiais, resíduos, efluentes e emissões. Ele parte do que a simulação já produz (tempo em cada estado, peças, refugo, retrabalho, viagens, volumes) e transforma isso em indicadores por máquina, por etapa e por produto, em telas que o aluno entende olhando.

O plano define público e objetivos, a conta de cada indicador, as telas, o formato dos dados, a integração com os outros apps, as fases de entrega, ideias de TCC e as decisões que ficam com o dono. Os requisitos sugeridos usam o prefixo `AMB-RF-nn` e as decisões o prefixo `A`.

Documentos da família que este plano segue e não repete (todos em `docs/` do repositório principal):

- `plano_integracao_familia.md`: uma fonte por dado, pedido e resposta entre apps, dado do nó em mão dupla, trava de um escritor;
- `formato_qualidade_seguranca.md`: modelo de como um bloco novo entra no nó sem mudar os modelos antigos;
- `plano_qualidade_e_seguranca.md`: defeito, inspeção, retrabalho, refugo e custos, que alimentam a conta de sucata;
- `plano_lean_seis_sigma.md`: projeto DMAIC, cenários, `estudo.json`, controle de ganhos;
- `plano_ensino_e_jogo.md`: formato da missão e dos pacotes `.utse`;
- `plano_estatistica.md`: pedido `analisar` e receitas de análise;
- `arquitetura.md`: estados dos blocos, rastro, fluidos, estatísticas e métricas do relatório.

**Aviso que vale para o plano inteiro.** O UTSim Ambiental apoia o estudo, o ensino e a organização das informações. Ele não emite inventário oficial de emissões, laudo, declaração ambiental de produto nem documento de licenciamento, e não substitui o profissional habilitado. A avaliação de ciclo de vida do app é simplificada e didática. Os fatores de emissão e de caracterização são sempre dados do usuário, com a fonte declarada por ele. Normas e métodos aparecem só por nome e tema, sem reprodução de texto.

---

## 0. Ponto de partida

### 0.1 O que já existe e será reaproveitado

| Peça | Onde | Uso neste app |
|---|---|---|
| Fração de tempo em cada estado, por bloco (`StateTracker`) | `utsim-core` (`State`: `processing`, `setup`, `idle`, `blocked`, `down`, `off_shift`, `waiting_operator`, `waiting_repair`, `maintenance`, `travel_empty`, `travel_loaded` e os demais) | base da energia, da água de resfriamento e de tudo que depende de tempo |
| Rastro com mudança de estado no tempo (`TraceEvent::State`) | `utsim-core/src/trace.rs` | curva de potência, demanda por intervalo, horário de pico, animação do mapa |
| Entradas, saídas, permanência por item | estatísticas do motor | consumo por peça, intensidade por peça boa |
| Registro por item (`Model::run_item_log`) | `utsim-model/src/items.rs` | caminho de cada peça, para a pegada por produto e para a "mochila" da peça refugada |
| Defeito, inspeção, retrabalho, `sink_role: Scrap`, rótulo `passagens` | `utsim-model/src/quality.rs` | sucata, retrabalho e o impacto embutido neles |
| Rede de fluidos (tanques, tubos, conversores, volume por nó) | `utsim-fluid`, seção 4l da arquitetura | água, efluente e insumos líquidos quando o modelo tem a rede |
| Turnos programados e motivos de parada | seção 4b da arquitetura | relógio do dia, horário de pico, consumo fora de turno |
| Manutenção, quebras, OEE | `utsim-model/src/maintenance.rs` | energia parada em quebra, efeito do desgaste no consumo |
| Tipos de item (`item_types`, `item_type` na `Source`) | plano de chegadas e equipes | indicador por produto |
| Energia de veículos (`Transporter.energy`) | plano de análises e dinâmica | consumo de empilhadeiras e AGVs |
| Experimentos com `patches`, sementes e diferença pareada | `utsim-model/src/experiment.rs`, `utsim-stats` | cenários "e se" com números aleatórios comuns |
| Otimização de parâmetros | `utsim-opt` | busca do melhor compromisso entre custo e impacto |
| Comandos de edição com desfazer, trava do modelo | `utsim-edit`, `utsim-pasta` | edição da aba Ambiental nos dois sentidos |
| Pedido e resposta entre apps | `utsim-troca` | conversa com LSS, Estatística, Qualidade, Ensino e Produto |
| Relatórios com capa, sumário, gráficos, HTML e PDF | `utsim-ferramentas::relatorios` | relatório ambiental do modelo e do estudo |
| Pasta de unidade com diário, compartilhada pelo Drive | `utsim-posto`, `utsim-posto-ui` | leituras reais de medidor, contas, registros de resíduo |
| Tema comum, abertura da família, cinco línguas | `utsim-ui-tema`, `utsim-idiomas` | a cara do app |
| KPI com intervalo e semáforo por probabilidade | `utsim-dash` | painel de metas |
| Cena 3D em Godot, mapa de calor da exposição | `utsim-godot`, `utsim-exposicao` | camada ambiental no 3D, numa fase posterior |

### 0.2 Lacunas principais

- O backlog já lista "Energia e emissões por estado de cada objeto", mas nada foi feito. Nenhum objeto tem potência, consumo de água ou geração de resíduo.
- Não há massa do item nem do material. Sem massa não há balanço de massa nem Sankey de materiais.
- O relatório traz a fração de tempo por estado na rodada inteira, mas não a curva no tempo. A demanda e o pico precisam do rastro.
- Não há catálogo de fatores (emissão, custo, destino de resíduo) nem lugar para a tarifa de energia.
- Não há diagrama de Sankey nem mapa da planta colorido por indicador nas bibliotecas de gráfico da família.
- O nome de crate `utsim-env` já é do ambiente de aprendizado por reforço. As crates deste app usam o prefixo `utsim-ambiental-`.

### 0.3 Princípios

- **A simulação é a fonte dos direcionadores.** O app não pede ao aluno "quantos kWh a máquina gastou". Ele pede a potência em cada estado e multiplica pelo tempo que a simulação mediu. Toda conta tem a forma `direcionador simulado × coeficiente do nó × fator do catálogo`.
- **Ver antes de ler.** A primeira coisa na tela é a fábrica colorida. Tabela e fórmula ficam a um clique, nunca na frente.
- **Toda conta é auditável.** Um clique em qualquer número abre "de onde veio": o direcionador, o coeficiente, o fator, a fonte do fator e a fórmula. Serve à banca do TCC e ao professor.
- **Fator sem fonte fica marcado.** O app não traz número de fator apresentado como oficial. Fator sem fonte declarada aparece com o selo "sem fonte" em toda tela e todo relatório que o usa.
- **Origem do dado sempre visível.** Simulado, Real, Gêmeo, Manual ou Importado, como no resto da família. Indicador simulado nunca aparece como medido.
- **Cálculo no Rust, interface fina.** Toda conta mora numa crate pura, conferida contra casos resolvidos à mão.
- **Modelo antigo não muda.** Os campos novos são opcionais. Modelo sem o bloco ambiental abre, roda e dá os mesmos números de antes.
- **Custo zero.** Sem servidor. Trabalho em equipe por pasta no Drive.

---

## 1. Propósito e público

### 1.1 Propósito

1. Mostrar o impacto ambiental da fábrica simulada onde ele acontece: na máquina, no estado da máquina, na peça e no produto.
2. Deixar o aluno testar mudanças (desligar em espera, mudar o turno, reusar água, reduzir refugo, trocar combustível) e ver custo e impacto lado a lado antes de mexer em qualquer coisa real.
3. Ensinar balanço de massa e de energia com um modelo que se move, em vez de uma planilha parada.
4. Ligar o tema ambiental ao que a família já faz: Lean Seis Sigma (projeto de redução), Qualidade (refugo e retrabalho têm impacto embutido), Estatística (a redução é significativa?), Ensino (missões) e Produto (a pegada como requisito de projeto).
5. Ser um repositório que um aluno assume no TCC: escopo claro, crates pequenas, casos de teste resolvidos à mão e uma lista de extensões possíveis.

### 1.2 Público

| Público | Uso principal |
|---|---|
| Aluno de graduação em TCC | usa o app num estudo de caso ou estende o app (seção 8) |
| Professor | aula com o mapa e o Sankey no projetor, missões do Ensino, trabalhos em grupo |
| Belt do UTSim LSS | projeto de redução de energia, água ou resíduo com linha de base e ganho simulados |
| Engenheiro de empresa piloto | estudo de cenário antes do investimento, com o aviso de que o número é de simulação |

### 1.3 O que cada curso ganha

| Curso | O que encontra no app | Disciplinas próximas |
|---|---|---|
| Engenharia ambiental | inventário de entradas e saídas por processo, balanço de massa, emissões por escopo, efluentes com carga, destino de resíduos, avaliação de ciclo de vida simplificada, metas | gestão ambiental, avaliação de impactos, tratamento de efluentes, resíduos sólidos, produção mais limpa |
| Engenharia de alimentos | água e energia por lote, limpeza entre lotes (enxágue e sanitização como setup com consumo próprio), vapor e frio, perda de produto, carga orgânica do efluente, aproveitamento de subproduto | operações unitárias, higiene industrial, projeto de indústria, tratamento de resíduos da agroindústria |
| Engenharia elétrica | curva de carga da fábrica, demanda por intervalo, horário de pico, fator de carga, custo por posto tarifário, deslocamento de carga, geração própria e armazenamento como cenário | eficiência energética, instalações industriais, qualidade e tarifação de energia |
| Engenharia de produção | os desperdícios do Lean vistos como kWh, litros e quilos, custo e impacto no mesmo cenário, indicador ambiental no mapa de fluxo de valor, projeto DMAIC verde | sistemas de produção, gestão ambiental, engenharia econômica, simulação |
| Engenharia mecânica e química (alcance futuro) | ar comprimido, calor de processo, fornos, balanço em reatores e tanques pela rede de fluidos | sistemas térmicos, operações unitárias |
| Computação (apoio) | diagramas interativos, otimização com mais de um objetivo, importação de dados de medidor | visualização, otimização |

### 1.4 Fora do escopo

- Banco de dados de inventário de ciclo de vida embutido. O app guarda só o que o usuário cadastra.
- Dispersão de poluentes no ar ou na água, ruído ambiental, estudo de solo.
- Projeto elétrico (curto-circuito, harmônicos, dimensionamento de condutores). O app fica na energia e na demanda.
- Emissão de qualquer documento com valor legal.

---

## 2. Indicadores

### 2.1 Ideia central: direcionador, coeficiente e fator

Toda quantidade ambiental do app sai de três peças.

| Peça | O que é | Quem fornece |
|---|---|---|
| **Direcionador** | o que aconteceu na simulação: horas em cada estado, peças processadas, setups, peças refugadas, passagens de retrabalho, metros rodados, volume que passou no nó | a simulação, sem intervenção do aluno |
| **Coeficiente** | quanto o nó consome ou gera por unidade do direcionador: kW no estado, litros por peça, kg de apara por peça, litros por limpeza | o aluno, na aba Ambiental do nó |
| **Fator** | conversão do fluxo em emissão ou em custo: kg de CO₂ equivalente por kWh, por m³, por kg de resíduo em cada destino, preço por unidade | o catálogo do modelo, editável, com fonte |

`quantidade = direcionador × coeficiente` e `emissão = quantidade × fator`. O aluno aprende que o indicador muda por três caminhos diferentes: mudar o processo (direcionador), mudar o equipamento (coeficiente) ou mudar a origem do insumo (fator).

Direcionadores disponíveis, todos lidos da simulação:

| Direcionador | Unidade | De onde vem |
|---|---|---|
| `Tempo(estados)` | hora | `StateTracker` do bloco, na janela de estatística; a curva no tempo vem de `TraceEvent::State` |
| `Peca` | peça que entrou no nó | contador de entradas |
| `PecaBoa` | peça que saiu por porta que não leva ao refugo | saídas e `sink_role` |
| `Refugo` | peça refugada atribuída ao nó onde o defeito nasceu, ou ao nó que a rejeitou | rótulos `defeito.<tipo>` e registro por item |
| `Retrabalho` | passagem de retrabalho | rótulo `passagens` |
| `Setup` | entrada no estado `setup` | contagem de transições no rastro |
| `Partida` | saída de `off_shift`, `down` ou `stopped` para um estado ligado | contagem de transições no rastro |
| `Distancia(carregado, vazio)` | metro | recurso móvel (`travel_loaded`, `travel_empty`) |
| `Volume` | m³ que passou no nó da rede de fluidos | métricas dos fluidos |
| `Massa` | kg processado (peça × massa do tipo de item) | contador de entradas e `item_types` |

### 2.2 Energia

**Coeficientes do nó.** Potência em kW por grupo de estado, mais consumo por evento.

| Grupo | Estados do motor | Leitura para o aluno |
|---|---|---|
| produzindo | `processing`, `utilize` | energia que agrega valor |
| preparando | `setup`, `entering`, `leaving`, `collecting`, `releasing` | necessária, mas não agrega |
| esperando | `idle`, `empty`, `blocked`, `waiting_operator`, `waiting_transport` | ligada sem produzir |
| parada | `down`, `waiting_repair`, `maintenance`, `stopped` | em quebra ou manutenção |
| fora de turno | `off_shift` | consumo de base (o que fica ligado à noite) |

Um estado pode ter potência própria (`by_state`), que vale no lugar da do grupo. Há ainda energia por peça (kWh por peça, para processos em que o consumo depende mais da carga que do tempo), por setup e por partida (o pico de aquecer um forno).

**Contas.**

- Energia do nó: `E = Σ_estados P_estado × T_estado + e_peça × peças + e_setup × setups + e_partida × partidas`, com T em horas.
- Energia por peça boa do nó: `E ÷ peças boas`.
- **Energia sem valor agregado:** a parcela dos grupos esperando, parada e fora de turno. É o número que mais chama a atenção em sala e a ponte com o Lean.
- Energia da etapa e da fábrica: soma dos nós.
- **Curva de carga:** `P(t) = Σ_nós P_estado(t)`, montada com as mudanças de estado do rastro. Como a potência é constante entre dois eventos, a curva é uma escada exata.
- **Demanda:** média de `P(t)` em cada intervalo de integração (padrão de 15 minutos, editável). Demanda máxima é o maior intervalo. O app mostra também a demanda máxima dentro e fora do horário de pico.
- **Horário de pico:** janela do dia cadastrada na tarifa do modelo. A energia consumida dentro dela é a integral de `P(t)` na janela. O relógio do dia vem do calendário de turnos do modelo, ou da hora de início cadastrada quando o modelo não tem turnos.
- **Fator de carga:** demanda média ÷ demanda máxima.
- **Conta de energia simulada:** energia no pico e fora do pico vezes os preços da tarifa, mais a demanda (contratada ou medida, a maior) vezes o preço da demanda, mais a ultrapassagem quando houver. Os preços são campos da tarifa do modelo, sem valor padrão.
- **Outros energéticos:** gás, vapor, ar comprimido, frio. Cada um é um fluxo do catálogo com a própria unidade e segue a mesma conta (por tempo em estado, por peça, por evento). O app converte tudo para kWh só quando o catálogo traz o fator de conversão do fluxo.
- **Veículos:** quando o recurso móvel tem `energy`, a energia sai de lá. Senão, o coeficiente é consumo por metro carregado e vazio.

### 2.3 Água

- **Entrada de água do nó:** litros por peça, litros por hora em determinados estados (resfriamento, lavagem contínua), litros por setup (limpeza entre lotes) e litros por partida. Quando o nó está ligado a um nó da rede de fluidos, o volume medido pela rede vale no lugar do coeficiente.
- **Balanço do nó:** `entrada = incorporada ao produto + evaporada ou perdida + efluente + reuso`. O aluno informa as frações de incorporada e de evaporada. O efluente é o que sobra. Se as frações passam de 1, a validação recusa.
- **Reuso:** fração do efluente de um nó que volta como entrada de outro nó, ou do mesmo. A captação da fábrica é a soma das entradas menos o reuso.
- **Indicadores:** captação total, consumo (captação menos efluente devolvido), litros por peça boa, litros por kg de produto, fração de reuso.

### 2.4 Resíduos e sucata

A base é o balanço de massa, que pede a massa do item.

- **Massa:** cada tipo de item tem massa. Um nó pode somar massa (material adicionado, com o nome do material e os kg por peça) e tirar massa (apara, cavaco, rebarba, casca, com os kg por peça). A massa da peça ao sair do nó é a de entrada mais o adicionado menos o removido.
- **Resíduo de processo:** `kg = coeficiente × direcionador`, por classe de resíduo. O direcionador comum é `Peca`. Lodo, embalagem e óleo usado costumam ir por `Tempo` ou por `Setup`.
- **Sucata:** cada peça que chega a um `Sink` com `sink_role: Scrap` vira resíduo com a massa que tinha naquele ponto. O app a atribui de duas formas, à escolha na tela: ao nó onde o defeito nasceu (causa) ou ao nó que a rejeitou (detecção).
- **A mochila da peça refugada:** a peça refugada carrega tudo o que foi gasto nela até ali: energia, água, material, emissões. O app soma esse impacto embutido pelo registro por item e o mostra separado. É a forma de o aluno ver que refugar no fim da linha custa muito mais ao ambiente que refugar no começo.
- **Retrabalho:** a energia das passagens extras já aparece, porque o tempo é simulado. Os fluxos por peça são contados a cada passagem.
- **Destino:** cada classe de resíduo tem um destino do catálogo (reciclagem, reuso interno, compostagem, aterro, tratamento térmico, outro). O destino traz o fator de emissão, o custo ou a receita por kg e a marcação de "desviado de aterro".
- **Indicadores:** kg por classe e por destino, kg por peça boa, rendimento de material (massa de produto bom ÷ massa de entrada), fração desviada de aterro, erro de fechamento do balanço de massa.

### 2.5 Emissões em CO₂ equivalente

- `emissão = Σ_fluxos quantidade × fator`. Cada fator do catálogo tem valor, unidade, fonte, ano e escopo. Todos são editáveis.
- **Escopos**, como classificação do catálogo:
  - escopo 1: queima de combustível dentro da fábrica (forno, caldeira, empilhadeira) e emissão de processo;
  - escopo 2: energia elétrica e vapor comprados;
  - escopo 3: material comprado, transporte, destino do resíduo, tratamento do efluente fora da fábrica.
- **Outros gases:** o catálogo tem uma tabela de conversão para CO₂ equivalente por gás, também editável e com fonte. Um fluxo de emissão direta (por exemplo, fuga de gás refrigerante por hora) informa o gás e a quantidade.
- **Fator variável no tempo (fase posterior):** fator da eletricidade por hora do dia, para o aluno ver que deslocar carga muda a emissão além do custo.
- **Incerteza:** o fator pode ter mínimo e máximo. A incerteza da simulação já vem das replicações. Numa fase posterior, o app sorteia os fatores dentro da faixa e soma as duas fontes.
- **Indicadores:** kg de CO₂ equivalente por nó, etapa, produto e escopo, e por peça boa.

**Regra de honestidade.** O catálogo novo nasce vazio. Um indicador que depende de fator ausente mostra "falta fator" em vez de zero. Um fator sem fonte mostra o selo "sem fonte". As missões do Ensino podem trazer um conjunto de fatores marcado como "didático", e esse rótulo acompanha os números até o relatório.

### 2.6 Efluentes

- **Volume:** vem do balanço de água do nó (seção 2.3).
- **Carga:** o nó informa a concentração de cada parâmetro no efluente que gera (o catálogo define os parâmetros que a turma usa: matéria orgânica, sólidos, óleos e graxas, temperatura, pH e outros). `carga (kg) = volume × concentração`. Uma alternativa por peça existe para quando o aluno tem a carga por unidade produzida.
- **Destino:** tratamento próprio, rede coletora, reuso ou outro, no catálogo. O tratamento tem eficiência de remoção por parâmetro, energia por m³ tratado (que volta para a conta de energia), custo por m³ e geração de lodo por m³ (que volta para a conta de resíduos).
- **Mistura:** o efluente da fábrica é a soma dos nós. A concentração da mistura é a carga total dividida pelo volume total.
- **Limite:** o usuário cadastra o limite de cada parâmetro e a fonte do limite. O app compara e pinta, sem afirmar conformidade legal.
- **Indicadores:** m³ por nó e por peça, carga por parâmetro antes e depois do tratamento, concentração da mistura, fração reusada.

### 2.7 Por máquina, por etapa e por produto

| Nível | Como se forma |
|---|---|
| Máquina (nó) | conta direta das seções acima |
| Etapa | grupo de nós com o mesmo `stage` no bloco ambiental, ou grupo desenhado no mapa. Soma dos nós. |
| Produto | tipo de item (`item_type`). Usa o registro por item para saber por onde cada peça passou e por quanto tempo. |

**Rateio para o produto.**

1. O que tem direcionador por peça vai direto para a peça.
2. A energia do grupo produzindo e preparando vai para a peça que estava no nó naquele intervalo. Se várias peças estavam juntas (forno, lote), divide-se por igual ou pela massa.
3. O que não tem peça associada (esperando, parada, fora de turno, base da fábrica) é rateado por uma regra escolhida na tela: pelo tempo de processamento de cada produto no nó (padrão), pelo número de peças, pela massa, ou não rateado e mostrado como "estrutura".
4. O impacto das peças refugadas vai para as peças boas do mesmo tipo.
5. A tela sempre mostra a regra em uso, e trocar a regra é um clique. O aluno vê que o rateio é uma escolha e que a pegada muda com ela.

### 2.8 Variabilidade

Todo indicador é uma métrica por replicação. O app mostra média e intervalo de confiança de 95 % entre replicações, como o resto da família, e nunca um número sozinho de uma rodada só sem avisar.

---

## 3. Telas

O app é egui, no tema escuro comum, com a cor de destaque própria (proposta: verde, decisão A2). A janela abre maximizada. Barra lateral: **Fábrica**, **Fluxos**, **Produto**, **Cenários**, **Metas**, **Fatores**, **Relatórios** e **Unidade**. No alto de toda tela ficam o modelo aberto, a origem dos dados (Simulado, com semente e número de replicações), o seletor de janela de tempo e o botão "Simular de novo".

Três gestos valem em toda tela:

- **clicar num número** abre o cartão "De onde veio" (direcionador, coeficiente, fator, fonte, fórmula e intervalo de confiança);
- **arrastar na linha do tempo** escolhe a janela (um turno, o horário de pico, o dia inteiro), e tudo na tela se recalcula para ela;
- **botão direito num nó** oferece "Abrir no UTSim", "Ver nos Fluxos", "Criar cenário a partir daqui" e "Levar para o LSS".

### 3.1 Fábrica: mapa de calor por impacto

É a tela de entrada, pelo princípio da família de que o app começa no modelo com resultados por nó.

**O que o aluno vê.** A planta da fábrica vista de cima, montada com as posições dos objetos do modelo e com a planta CAD ao fundo quando o modelo tem uma. Cada máquina é um bloco com o ícone do tipo. Em volta de cada bloco há uma mancha de calor cuja cor e tamanho seguem o indicador escolhido. As ligações aparecem como linhas finas. À direita, a lista "Os três vilões", com frases prontas montadas a partir dos números, por exemplo: "A estufa gasta 41 % da energia esperando peça". Embaixo, uma faixa com a curva de carga da fábrica, a faixa do horário de pico sombreada e a linha da demanda contratada.

**O que ele clica.**

- Fichas no alto trocam o indicador: Energia, Água, Resíduos, Efluentes, CO₂ equivalente, Custo.
- Um seletor troca a base: total, por peça boa, por hora, só a parcela sem valor agregado.
- Clicar numa máquina abre o painel lateral: rosca da energia por grupo de estado, barras dos outros indicadores, posição da máquina no ranking, os coeficientes cadastrados com o lápis de editar e o botão "Abrir no UTSim".
- "Tocar o dia" reproduz o dia simulado em cerca de 30 segundos. As manchas pulsam conforme as máquinas mudam de estado e um cursor corre na curva de carga. No pico de demanda o app pausa sozinho e marca quais máquinas estavam ligadas juntas.
- "Camadas" liga zonas, etapas, rotas de veículos e a rede de fluidos.

**O que ele arrasta.**

- Arrastar na curva de carga escolhe a janela de tempo, e o mapa passa a mostrar só aquela janela.
- Arrastar um retângulo sobre várias máquinas soma o grupo e oferece "Salvar como etapa".
- No modo "Antes e depois", uma cortina vertical divide o mapa entre o cenário base e o cenário escolhido. Arrastar a cortina revela um ou outro, no mesmo desenho.
- Arrastar com o botão do meio move a planta, e a roda aproxima.

**Detalhes de desenho.** Escala de cor sequencial de um só matiz, legível para daltônicos, com legenda numérica sempre visível. Máquina sem bloco ambiental aparece tracejada com o convite "Informe a potência". Máquina com fator ausente aparece com o selo "falta fator". Numa fase posterior, "Ver em 3D" abre a cena em Godot com a mesma camada (seção 4.6).

### 3.2 Fluxos: massa e energia num diagrama de Sankey

**O que o aluno vê.** Um diagrama de Sankey da esquerda para a direita. À esquerda, as entradas (eletricidade, combustíveis, água, cada material). No meio, as etapas e as máquinas. À direita, as saídas (produto bom, sucata, cada classe de resíduo por destino, efluente, água evaporada, emissões). A largura de cada faixa é proporcional à quantidade. As cores seguem a família do fluxo.

Como um Sankey só faz sentido numa unidade, a tela tem quatro modos no alto: **Massa** (kg), **Energia** (kWh), **Água** (m³) e **Carbono** (kg de CO₂ equivalente). No modo Carbono, as entradas são as origens da emissão e a saída é o produto que a carrega.

**O que ele clica.**

- Clicar numa faixa destaca o caminho inteiro dela, para trás e para a frente, e apaga o resto. Um balão mostra quantidade, fração do total e intervalo.
- Duplo clique numa etapa a abre nas suas máquinas. Duplo clique no fundo fecha.
- A chave "Mostrar o que não agrega" divide cada faixa de energia em duas: a parte produzindo, cheia, e a parte esperando, parada e fora de turno, hachurada. A fábrica passa a mostrar um rio largo de energia indo para lugar nenhum.
- A chave "Mochila do refugo" pinta, dentro das faixas de entrada, a parte que acabou em peça refugada.
- O selo **Balanço** de cada nó diz se o que entra é igual ao que sai. Verde quando fecha dentro da tolerância. Amarelo com "faltam 3 %" quando não fecha, e o clique lista os fluxos que o aluno provavelmente esqueceu de cadastrar. É o exercício de balanço de massa embutido na tela.

**O que ele arrasta.**

- Arrastar um nó para cima ou para baixo reorganiza o desenho, e o app guarda a posição.
- Arrastar um nó sobre outro propõe agrupar os dois numa etapa.
- Arrastar na régua de tempo abaixo do diagrama muda a janela, e as faixas engordam e afinam na hora.
- Arrastar o limiar "Juntar fluxos menores que" esconde os fios finos em "outros".

**Saída.** Exportação em SVG e PNG com legenda, pronta para a monografia, e a tabela do inventário em CSV.

### 3.3 Produto: pegada numa avaliação de ciclo de vida simplificada

**O que o aluno vê.** No alto, uma faixa com as fases do ciclo de vida desenhadas como estações: matérias-primas, transporte de chegada, **fabricação** (a parte que a simulação cobre, destacada), distribuição, uso e fim de vida. Abaixo, o cartão do produto escolhido, com a pegada por unidade funcional e uma barra empilhada por fase. Ao lado, a mesma pegada aberta por etapa da fábrica e a árvore de contribuição.

**Passos da tela, na ordem de um estudo.**

1. **Unidade funcional:** uma peça boa, um kg de produto, um lote, ou um texto livre com o fator de conversão.
2. **Fronteira:** o aluno clica nas estações para incluí-las ou não. "Portão a portão" liga só a fabricação e sai inteira da simulação. "Berço ao portão" soma materiais e transporte de chegada, com os fatores do catálogo. Uso e fim de vida são fases de entrada manual, numa fase posterior.
3. **Inventário:** tabela de entradas e saídas por unidade funcional, com a origem de cada linha (Simulado, Catálogo, Manual).
4. **Impacto:** no MVP desta tela, mudança do clima em CO₂ equivalente, mais os indicadores de inventário (energia, água, resíduo). Outras categorias entram depois, só com fatores de caracterização cadastrados pelo usuário.
5. **Interpretação:** contribuição por fase e por etapa, a mochila do refugo como fatia própria e o efeito da regra de rateio.

**O que ele clica e arrasta.**

- Clicar numa fatia da barra abre a contribuição um nível abaixo (fase, etapa, máquina, fluxo).
- Arrastar um segundo produto para o lado do primeiro compara os dois na mesma escala.
- Trocar a regra de rateio num seletor redesenha a barra, com a diferença em destaque.
- Um controle deslizante de "taxa de refugo" mostra na hora quanto a pegada da peça boa cresce com o refugo. O valor simulado fica marcado na régua.
- "Gerar relatório do estudo" monta o documento com objetivo, unidade funcional, fronteira, inventário, impacto, interpretação, limitações e a lista dos fatores com as fontes.

O relatório sempre traz a frase de que o estudo é simplificado e didático, e lista o que ficou fora da fronteira.

### 3.4 Cenários: "e se", com custo e impacto lado a lado

**O que o aluno vê.** À esquerda, a prateleira de cenários em cartões: Base e os que ele criar. No centro, duas colunas de mesma largura, **Custo** e **Impacto**, com uma linha por cenário. Cada linha tem a barra do valor, a diferença para o Base com intervalo de confiança e uma etiqueta: melhor, pior, ou "não dá para afirmar". Abaixo, um gráfico de dispersão com custo num eixo e CO₂ equivalente no outro, um ponto por cenário, e a fronteira dos cenários que não perdem nos dois critérios destacada.

**Alavancas.** Um cenário é um conjunto de mudanças no modelo, gravadas como `patches` do experimento. O aluno as monta numa lista de alavancas com controles deslizantes:

| Alavanca | O que muda | Precisa simular de novo |
|---|---|---|
| Potência em espera, desligamento automático depois de x minutos ocioso | coeficiente de energia | não |
| Fator de emissão, fonte da energia, combustível | catálogo | não |
| Destino do resíduo, fração de reuso de água, eficiência do tratamento | catálogo e coeficiente | não |
| Preço da energia, janela do horário de pico, demanda contratada | tarifa | não |
| Turnos, horário de partida de uma máquina, sequência de partida | calendário | sim |
| Tamanho de lote, número de máquinas, velocidade | processo | sim |
| Taxa de defeito, plano de inspeção, limite de retrabalho | qualidade | sim |
| Manutenção preventiva | manutenção | sim |

A coluna da direita é uma decisão de desenho que deixa a tela viva. Alavanca que só mexe em coeficiente, fator ou tarifa é recalculada sobre o mesmo rastro, na hora, enquanto o aluno arrasta. Alavanca que muda o fluxo ganha o selo "precisa simular", e o botão "Simular" roda as replicações com os mesmos números aleatórios do Base, para a comparação ser pareada.

**O que ele clica e arrasta.**

- Arrastar um controle deslizante e ver as barras de custo e impacto se moverem juntas.
- Arrastar um cartão de cenário para a área de comparação. Até quatro ao mesmo tempo.
- Clicar num ponto do gráfico de dispersão abre o cenário.
- "Investimento" abre os campos de custo de implantação e vida útil, e o app calcula o retorno simples e o custo por tonelada evitada.
- "Buscar o melhor" (fase posterior) entrega as alavancas ao otimizador com dois objetivos e devolve a fronteira.
- "Enviar para a Estatística" pede o teste de hipótese da redução (seção 4.5).
- "Levar para o LSS" transforma o cenário em ação de melhoria de um projeto (seção 4.3).

**Custos.** Energia (pela tarifa), água e efluente (preço por m³), material (preço por kg), destino do resíduo (custo ou receita por kg), mais o custo de operação quando o modelo tiver os custos do backlog. Um preço de carbono é um campo opcional, vazio por padrão.

### 3.5 Metas: o painel

**O que o aluno vê.** Uma grade de cartões, um por meta. Cada cartão tem o indicador, o alvo, a linha de base, o prazo, um medidor grande e uma linha do tempo com a trajetória planejada e os pontos realizados. Um semáforo mostra a probabilidade de cumprir a meta nas replicações (verde, amarelo, vermelho, com o mesmo cálculo de proporção com intervalo já usado no painel da família). No alto, o **orçamento de carbono** do período: uma barra que se esvazia conforme a produção simulada avança.

**O que ele clica e arrasta.**

- "Nova meta" pede o indicador, o alcance (fábrica, etapa, máquina ou produto), a base (total ou por peça boa), a linha de base (o cenário Base atual, com um clique), o alvo e o prazo.
- Arrastar o marcador do alvo no medidor ajusta a meta, e o semáforo se recalcula.
- Clicar no cartão abre o detalhe: quais cenários aproximam a meta, quanto cada um contribui (gráfico em cascata da linha de base até o alvo) e quais ações estão ligadas a ela.
- Arrastar um cenário da prateleira para dentro de uma meta o liga como ação. A cascata ganha um degrau.
- "Dado real" permite lançar a leitura do mês (medidor, conta, pesagem de resíduo), guardada na pasta de unidade. O cartão passa a mostrar o simulado e o real lado a lado, cada um com a sua origem.
- "Carta de controle" abre o indicador no UTSim Qualidade.

Para a sala de aula, o painel tem selos de conquista simples (primeira meta cumprida, balanço de massa fechado, fábrica com mais de metade do resíduo desviado de aterro), que o professor pode desligar.

### 3.6 Telas de apoio

- **Fatores:** o catálogo do modelo em tabela. Fluxos, fatores de emissão por escopo, gases, destinos de resíduo, destinos de efluente, parâmetros de efluente e limites, tarifa de energia. Cada linha tem valor, unidade, fonte, ano e faixa de incerteza. Importa e exporta CSV. Um conjunto pode ser salvo como biblioteca na pasta de unidade e reaproveitado em outro modelo. A coluna "usado em" mostra quais nós dependem de cada linha.
- **Relatórios:** relatório ambiental do modelo (indicadores, mapa, Sankey, cenários, metas, fatores com fontes) e relatório do estudo de produto, em PDF e HTML, pelo gerador comum da família, com capa, sumário e o aviso da seção de abertura.
- **Unidade:** o que não é de um nó. Leituras reais de medidores, contas, registros de resíduo destinado, biblioteca de fatores. Mesma pasta de unidade dos outros apps.
- **Primeiro uso:** ao abrir um modelo sem nenhum bloco ambiental, o assistente "Dar vida ao modelo" lista as máquinas e pede só a potência produzindo e esperando de cada uma. Com isso o mapa já acende. O resto é opcional e vem depois.

---

## 4. Integração com a família

### 4.1 Onde cada dado mora

Pela regra de uma fonte por dado do plano de integração.

| Dado | Onde mora | Quem edita |
|---|---|---|
| Coeficientes do nó (potência por estado, água, material, resíduo, efluente, etapa) | bloco `environment` do objeto, no `.ron` do modelo | aba Ambiental do UTSim e UTSim Ambiental, com a trava |
| Catálogo de fluxos e fatores, tarifa, relógio, metas ligadas ao modelo | bloco `environment` do modelo, no `.ron` | UTSim Ambiental |
| Massa do tipo de item | `item_types` do modelo | UTSim, UTSim Ambiental, UTSim Produto |
| Cenários e resultados | `estudo.json` ao lado do modelo, no formato de evidência que o LSS já usa | UTSim Ambiental |
| Leituras reais, contas, registros de resíduo, biblioteca de fatores | pasta de unidade | UTSim Ambiental |
| Projeto de redução | pasta `.lss` | UTSim LSS |

### 4.2 Formato dos dados

No espírito de `formato_qualidade_seguranca.md`. Todos os campos são opcionais. O formato do modelo não muda de versão, porque nenhum campo antigo muda de sentido. Modelo sem estes blocos abre e roda como antes, e os golden não mudam. A simulação não lê nenhum destes campos: a conta ambiental é feita depois da rodada, sobre o relatório, o rastro e o registro por item. Por isso ligar o bloco ambiental não muda nenhum tempo nem nenhum sorteio.

#### 4.2.1 Campos novos

| Onde | Campo | Tipo | Lido pela simulação |
|---|---|---|---|
| objeto | `environment` | `EnvironmentDef` | não, só o cálculo ambiental |
| modelo | `environment` | `EnvironmentCatalog` | não |
| tipo de item | `mass_kg` | número | não |
| modelo | `packs` | ganha o valor `"ambiental"` | não, só interface |

`environment` vale em `Processor`, `Combiner`, `Separator`, `Queue`, `Conveyor`, `Rack`, nos recursos móveis e nos nós da rede de fluidos.

#### 4.2.2 `environment` do objeto

```ron
(name: "estufa", kind: Processor(process: Const(900.0)),
 environment: Some((
     stage: Some("pintura"),
     power: (                         // kW por grupo de estado
         producing: 18.0, preparing: 12.0, waiting: 9.0, stopped: 1.5, off_shift: 0.5,
         by_state: {"blocked": 12.0}, // estado com potência própria
     ),
     energy_per: [(driver: Startup, kwh: 6.0)],
     flows: [
         (flow: "gas_natural", amount: 2.4,  driver: Time([Processing, Setup])),   // m3 por hora
         (flow: "agua",        amount: 0.8,  driver: Item),                         // L por peça
         (flow: "agua",        amount: 120.0, driver: Setup),                       // L por limpeza
         (flow: "tinta",       amount: 0.05, driver: Item, adds_mass: true),        // kg por peça
         (flow: "borra_tinta", amount: 0.004, driver: Item),                        // kg por peça
     ],
     water: Some((incorporated: 0.0, evaporated: 0.15, reuse_to: Some("lavadora"), reuse: 0.3)),
     effluent: Some((
         destination: "tratamento_proprio",
         concentration: {"materia_organica": 350.0, "solidos": 120.0},              // mg/L
     )),
     standby: None,                   // Some((after: 600.0)) desliga depois de 600 s esperando
 ))),
```

- **`power`.** Os cinco grupos da seção 2.2. Grupo ausente vale zero. `by_state` aceita o nome de qualquer estado do motor.
- **`energy_per`.** Energia por evento: `Item`, `Setup` ou `Startup`.
- **`flows`.** Cada linha é `quantidade = amount × direcionador`. `flow` precisa existir no catálogo do modelo, que dá a unidade e diz se é entrada ou saída. Direcionadores: `Item`, `GoodItem`, `Scrap`, `Rework`, `Setup`, `Startup`, `Time([estados])`, `Distance(loaded: .., empty: ..)`, `Volume` e `Mass`. `adds_mass` e `removes_mass` dizem se o fluxo entra na massa da peça ou sai dela.
- **`water`.** Frações do balanço de água do nó e o reuso.
- **`effluent`.** Destino e concentração por parâmetro do catálogo.
- **`standby`.** Regra de desligamento em espera, aplicada no cálculo: o tempo esperando além de `after` conta com a potência de `off_shift`, e a volta conta uma partida. É uma aproximação declarada na tela, porque a simulação não atrasa a máquina. Um desligamento que atrasa a produção é modelado no próprio UTSim.
- Os nomes dos campos são em inglês, como `quality`, `inspection` e `safety`. As variantes aceitam o nome em português como alternativa (`Peca`, `Tempo`, `Partida`), como já fazem `Scrap` e `Refugo`.

#### 4.2.3 `environment` do modelo

```ron
environment: Some((
    clock: (start: "06:00", time_unit: Second),
    flows: [
        (name: "eletricidade", kind: Energy,   unit: "kWh", direction: In),
        (name: "gas_natural",  kind: Fuel,     unit: "m3",  direction: In),
        (name: "agua",         kind: Water,    unit: "L",   direction: In),
        (name: "tinta",        kind: Material, unit: "kg",  direction: In),
        (name: "borra_tinta",  kind: Waste,    unit: "kg",  direction: Out, destination: Some("tratamento_termico")),
    ],
    factors: [
        (flow: "eletricidade", scope: 2, kg_co2e: None, source: "a definir", year: None),
        (flow: "gas_natural",  scope: 1, kg_co2e: None, source: "a definir", year: None),
        (flow: "tinta",        scope: 3, kg_co2e: None, source: "a definir", year: None),
    ],
    gases: [],                        // (name, kg_co2e por kg, source)
    waste_destinations: [
        (name: "reciclagem",        diverted: true,  kg_co2e: None, cost: None, source: "a definir"),
        (name: "tratamento_termico", diverted: true, kg_co2e: None, cost: None, source: "a definir"),
        (name: "aterro",            diverted: false, kg_co2e: None, cost: None, source: "a definir"),
    ],
    effluent: (
        parameters: [(name: "materia_organica", unit: "mg/L", limit: None, limit_source: "a definir")],
        destinations: [(name: "tratamento_proprio", removal: {"materia_organica": 0.0},
                        kwh_per_m3: 0.0, sludge_kg_per_m3: 0.0, cost_per_m3: None)],
    ),
    tariff: Some((
        peak: (from: "18:00", to: "21:00", weekdays_only: true),
        demand_interval_min: 15,
        contracted_kw: None,
        price_kwh_peak: None, price_kwh_off_peak: None, price_kw: None,
        source: "a definir",
    )),
    prices: {},                       // preço por unidade de cada fluxo, sem padrão
    allocation: ByProcessTime,        // regra de rateio para o produto
    targets: [
        (id: "m1", indicator: "co2e_kg", scope: Factory, basis: PerGoodItem,
         baseline: None, target: None, due: None),
    ],
))
```

- `kind`: `Energy`, `Fuel`, `Water`, `Material`, `Waste`, `Effluent`, `Emission` e `Other`.
- Fator `None` quer dizer "a definir". O indicador que depende dele mostra "falta fator". Os valores do exemplo acima estão vazios de propósito: este plano não propõe número de fator.
- `factor` aceita também `range: Some((min, max))` para a incerteza e, numa fase posterior, `by_hour` com 24 valores.
- `allocation`: `ByProcessTime`, `ByItems`, `ByMass` ou `None` (estrutura à parte).

#### 4.2.4 Validação

- Todo `flow` usado num nó existe no catálogo, e a direção é coerente com o uso.
- Frações de `water` entre 0 e 1 e com soma no máximo 1.
- `reuse_to` aponta para um objeto que existe.
- Potências e quantidades não negativas.
- Parâmetro de efluente usado no nó existe no catálogo.
- Janela de pico dentro de um dia, intervalo de demanda maior que zero.
- Aviso, sem erro: nó com `adds_mass` ou `removes_mass` num modelo cujos tipos de item não têm `mass_kg`.

#### 4.2.5 Métricas no relatório

Para experimentos, otimização e as missões do Ensino lerem o ambiental como qualquer outra métrica, o cálculo publica chaves com nome, só nos modelos que usam os blocos.

| Chave | Onde | O que é |
|---|---|---|
| `<nó>.energia_kwh` | nó com `power` | energia do nó na janela de estatística |
| `<nó>.energia_kwh.<grupo>` | idem | por grupo: `produzindo`, `preparando`, `esperando`, `parada`, `fora_de_turno` |
| `<nó>.energia_sem_valor_kwh` | idem | esperando, parada e fora de turno |
| `<nó>.kwh_por_peca` | idem | energia por peça boa do nó |
| `<nó>.fluxo.<fluxo>` | nó com `flows` | quantidade do fluxo, na unidade do catálogo |
| `<nó>.agua_m3`, `<nó>.efluente_m3` | nó com água | captação e efluente do nó |
| `<nó>.carga.<parâmetro>_kg` | nó com `effluent` | carga antes do tratamento |
| `<nó>.residuo_kg`, `<nó>.sucata_kg` | nó com resíduo ou refugo | resíduo de processo e sucata atribuída |
| `<nó>.co2e_kg` | nó com algum fluxo com fator | emissão do nó |
| `ambiental.energia_kwh`, `ambiental.energia_sem_valor_kwh` | modelo | soma dos nós |
| `ambiental.demanda_max_kw`, `ambiental.demanda_pico_kw` | modelo com `tariff` | maior demanda no intervalo, no dia e na janela de pico |
| `ambiental.energia_pico_kwh`, `ambiental.fator_de_carga` | idem | energia na janela de pico, demanda média sobre a máxima |
| `ambiental.custo_energia` | idem, com preços | conta de energia simulada |
| `ambiental.agua_m3`, `ambiental.reuso_pct`, `ambiental.efluente_m3` | modelo | captação, fração reusada, efluente |
| `ambiental.residuo_kg.<destino>`, `ambiental.desvio_aterro_pct` | modelo | resíduo por destino e fração desviada |
| `ambiental.rendimento_material_pct`, `ambiental.balanco_massa_erro_pct` | modelo com massas | massa de produto bom sobre a de entrada, e o erro de fechamento |
| `ambiental.co2e_kg`, `ambiental.co2e_kg.escopo<n>` | modelo | emissão total e por escopo |
| `ambiental.mochila_refugo.co2e_kg` | modelo com refugo | emissão embutida nas peças refugadas |
| `produto.<tipo>.co2e_kg_por_peca`, `.kwh_por_peca`, `.agua_l_por_peca` | modelo com tipos de item | pegada portão a portão por peça boa |
| `ambiental.custo` | modelo com preços | soma dos custos ambientais |

As contagens valem na janela de estatística, depois do aquecimento. As chaves de demanda e as de produto pedem o rastro e o registro por item, que custam memória mas não mudam nenhum número da simulação.

#### 4.2.6 Comandos de edição

`SetEnvironment` (troca o bloco inteiro do nó, um passo de desfazer) e `SetEnvironmentCatalog` (troca o bloco do modelo), em `utsim-edit`, ao lado de `SetQuality` e `SetSafety`. O mesmo comando serve à aba do UTSim e ao app. Um escritor por modelo, pela trava `.<modelo>.trava`.

#### 4.2.7 Enquanto as bibliotecas da família não aceitam o campo

O repositório principal está congelado para a separação em vários repositórios. Para o app nascer sem esperar, os mesmos tipos (`EnvironmentDef` e `EnvironmentCatalog`, com o mesmo serde) são gravados num arquivo ao lado do modelo, `<modelo>.ambiental.ron`, com os blocos de nó guardados por nome do objeto. Quando o campo entrar no `utsim-model`, um assistente move os blocos para dentro do modelo, mostra o que vai mudar antes de gravar e apaga o arquivo lateral. O formato não muda, só o lugar (decisão A4).

### 4.3 Aba "Ambiental" em cada nó do UTSim

Mesma mecânica das abas Qualidade, Segurança e Manutenção.

**O que a aba mostra, de cima para baixo.**

1. **Resultado da última rodada**, só leitura: energia do nó com a rosca por grupo de estado, kWh por peça, água, resíduo, CO₂ equivalente e a posição do nó no ranking da fábrica.
2. **Energia:** os cinco campos de potência, com um desenho pequeno da máquina que acende mais ou menos conforme o valor. "Mais estados" abre o `by_state`.
3. **Fluxos:** tabela de linhas (fluxo, quantidade, por quê), com o fluxo escolhido do catálogo.
4. **Água e efluente:** frações, reuso, destino e concentrações.
5. **Etapa** a que o nó pertence.
6. Botão **"Abrir no UTSim Ambiental"**.

O botão salva o modelo e chama, como combinado para os outros apps:

```text
utsim-ambiental --utsim-abrir=<modelo .ron, caminho absoluto> --utsim-no=<nome do nó>
```

O app abre na tela Fábrica com o nó selecionado e o painel lateral aberto. O 3D cede a trava e fica só de leitura enquanto o app edita. Quando o app solta a trava, o 3D relê o modelo. O catálogo de fluxos e fatores é editado só no app. A aba do nó escolhe da lista e, quando falta um fluxo, oferece abrir o app na tela Fatores.

Na vista de resultados do UTSim, o nó ganha uma etiqueta pequena com o indicador ambiental escolhido, ao lado da utilização.

### 4.4 UTSim LSS: o Lean verde

A ideia é mostrar que os desperdícios do Lean têm um lado ambiental, e que um projeto DMAIC pode ter como Y um indicador deste app.

| Desperdício do Lean | Como aparece no Ambiental |
|---|---|
| Espera | energia do grupo esperando, água de resfriamento correndo sem peça |
| Superprodução | energia, água e material gastos em peça que não tem pedido |
| Defeitos | sucata, retrabalho e a mochila do refugo |
| Transporte e movimentação | combustível ou energia dos veículos por metro rodado |
| Estoque | climatização e iluminação do armazém, perda por validade |
| Processamento excessivo | energia e insumo além do necessário, potência produzindo acima do que a peça pede |
| Talento não usado | sem indicador direto, entra como texto no projeto |

**O que o Ambiental manda para o LSS.**

- **Oportunidade para a carteira.** "Levar para o LSS", num vilão do mapa ou num cenário, cria uma ideia na carteira com o indicador, a linha de base simulada com intervalo, o ganho estimado do cenário e o hash do modelo. Segue o caminho que o 8D do Qualidade já usa.
- **Linha de base e Y do termo de abertura.** O termo de um projeto verde recebe o indicador (por exemplo, kWh por peça boa da etapa de pintura), a linha de base e a meta, com origem Simulado.
- **Linha ambiental do mapa de fluxo de valor.** Para cada caixa de processo do mapa do LSS, o Ambiental entrega kWh por peça, litros por peça, kg de resíduo por peça e a fração de energia sem valor agregado. O mapa ganha uma linha ambiental embaixo da linha do tempo.
- **Dados para Medir e Analisar.** Conjunto de dados por replicação e por nó, para o Pareto de energia por máquina e por estado.

**O que o LSS manda para o Ambiental.**

- Pedido `abrir_ferramenta` com `ferramenta` igual a `mapa_impacto`, `sankey`, `pegada` ou `cenario_ambiental`, e o projeto e o entregável no contexto. O resultado volta como entregável do projeto, no formato de troca.
- As ações da fase Melhorar viram cenários. O `estudo.json` do cenário, com hash do modelo, sementes e replicações, é o pacote de evidência do tollgate.

**Ganhos.** No controle de ganhos do LSS, um projeto verde tem o ganho financeiro (energia, água, material e destino de resíduo, pelos preços do catálogo) e o ganho ambiental (kWh, m³, kg e toneladas de CO₂ equivalente evitadas), lado a lado e sem converter um no outro.

**Controlar.** A meta do projeto vira um cartão do painel de metas. A carta de controle do indicador é feita no UTSim Qualidade.

### 4.5 UTSim Ensino: casos e missões ambientais

O Ensino lê métricas com nome do relatório e mexe em chaves do modelo. Como o Ambiental publica as métricas da seção 4.2.5 e os coeficientes são campos do nó, as missões ambientais não pedem código novo no Ensino.

Pacote proposto, `.utse` "Fábrica verde", com `requer: ["ambiental >= 0.1"]`:

| Missão | Conceito | O que o aluno faz |
|---|---|---|
| Quem deixou a luz acesa? | energia por estado | acha no mapa a máquina que mais gasta parada e escolhe onde instalar o desligamento automático, com orçamento |
| O pico das seis | demanda e horário de pico | reorganiza turnos e partidas para tirar a demanda da janela de pico sem perder produção |
| Feche o balanço | balanço de massa | completa os fluxos que faltam até o Sankey de massa fechar |
| A mochila da peça | impacto embutido no refugo | decide onde pôr a inspeção para o refugo acontecer mais cedo |
| Água que volta | reuso e efluente | liga o reuso entre dois processos e respeita o limite de concentração cadastrado |
| Para onde vai o resto? | destino de resíduos | escolhe destinos com custo e emissão diferentes |
| Duas embalagens | pegada por produto | compara dois produtos pela avaliação de ciclo de vida simplificada e explica o efeito do rateio |
| Barato ou limpo? | compromisso entre custo e impacto | acha um cenário na fronteira que cumpre a meta de carbono dentro do orçamento |
| Caso final: projeto verde | DMAIC | leva o caso ao LSS como projeto de treino |

Exemplo de missão, no formato atual:

```ron
Missao(
    id: "verde/luz_acesa",
    formato: 1,
    titulo: "Quem deixou a luz acesa?",
    conceitos: ["energia_por_estado", "desperdicio_de_espera"],
    modelo: "modelos/pintura.ron",
    texto: "textos/luz_acesa.md",
    previsao: [ Escolha(pergunta: "Qual máquina gasta mais energia sem produzir?",
                        opcoes_de: Objetos(["lavadora", "estufa", "cabine"])) ],
    alavancas: [
        Quantidade(rotulo: "Desligamento automático na estufa", chave: "estufa.environment.standby", de: 0, ate: 1, custo_unitario: 12000),
        Quantidade(rotulo: "Desligamento automático na cabine", chave: "cabine.environment.standby", de: 0, ate: 1, custo_unitario: 9000),
    ],
    orcamento: 12000,
    metricas: [ Metrica(nome: "ambiental.energia_sem_valor_kwh", objetivo: Menor, meta: 120.0, peso: 1.0, tolerancia: Relativa(0.02)) ],
    rodadas: (replicacoes: 10, duracao: "run.duration"),
    analisar_em: Some("ambiental"),
    referencia: Calcular(otimizador: Exaustivo),
)
```

- "Analisar no UTSim Ambiental" abre o modelo da tentativa com `--utsim-abrir`.
- Os fatores das missões são um conjunto marcado como "didático" dentro do pacote. O texto da missão diz isso ao aluno.
- A correção usa a métrica, sem depender da tela do Ambiental.
- O construtor de casos do professor ganha as métricas ambientais na lista de objetivos.

### 4.6 UTSim Estatística: a redução é de verdade?

A pergunta de todo projeto de redução é se a diferença é real ou é variação da simulação. O Ambiental não refaz os testes. Ele pede ao Estatística.

| Pergunta do aluno | Análise pedida | Dados que vão |
|---|---|---|
| O cenário reduziu o kWh por peça? | teste t pareado, com números aleatórios comuns | uma linha por replicação: base e cenário |
| Qual de vários cenários é diferente? | ANOVA de um fator com comparações múltiplas | uma coluna por cenário |
| Quanto da energia é fixa e quanto varia com a produção? | regressão linear da energia sobre a produção (a linha de base energética) | energia e produção por período simulado ou medido |
| Quais alavancas mais mexem no CO₂ equivalente? | experimento fatorial com o indicador como resposta | pedido `planejar_experimento` com as alavancas como fatores |
| A redução medida na fábrica real se confirmou? | teste antes e depois sobre as leituras reais da pasta de unidade | leituras com origem Real |
| Quantas replicações bastam para ver uma redução de x %? | tamanho de amostra | desvio do indicador nas replicações já feitas |

Pedido, no formato do `utsim-troca`:

```json
{
  "formato": 1,
  "id": "amb-2026-0001",
  "de": "ambiental 0.1.0",
  "acao": "analisar",
  "entrada": {
    "arquivos": [{ "caminho": "troca/amb-2026-0001.csv", "hash": "..." }],
    "colunas": ["replicacao", "base_kwh_por_peca", "cenario_kwh_por_peca"],
    "receita": { "tipo": "t_pareado", "hipotese": "menor", "diferenca_minima": 0.0 }
  },
  "contexto": { "modelo": "pintura.ron", "cenario": "desligar_estufa", "indicador": "ambiental.energia_kwh" },
  "resposta": "troca/amb-2026-0001.utsim-resposta.json"
}
```

A resposta traz a receita, o resultado e o relatório com hash. O Ambiental grava a conclusão no cenário, e a etiqueta da linha na tela Cenários passa de "não dá para afirmar" para o resultado do teste, com o valor de p e o intervalo da diferença. Sem o Estatística instalado, o app mostra só o intervalo da diferença pareada, que ele já calcula pela `utsim-stats`, e oferece abrir o launcher.

### 4.7 Outros apps

| De | Para | Vai | Volta |
|---|---|---|---|
| UTSim | Ambiental | modelo e nó, rastro, registro por item | coeficientes do nó e catálogo gravados no modelo |
| Ambiental | Qualidade | pedido `abrir_ferramenta` `carta` ou `pareto` sobre um indicador | a ferramenta pronta, citada na meta |
| Qualidade | Ambiental | defeitos, refugo e retrabalho simulados do nó | a mochila do refugo por tipo de defeito, para o Pareto pesado por impacto |
| Produto | Ambiental | estrutura do produto com materiais e massas, modelo gerado | pegada por peça e por material, para virar requisito ou comparação de variantes |
| Segurança | Ambiental | zonas de calor e de ruído, perigos com produto químico | nós com fluxo perigoso, para o inventário do posto |
| Manutenção (aba do nó) | Ambiental | paradas, preventivas e desgaste | energia parada em quebra, e o consumo que cresce com o desgaste numa fase posterior |
| Gêmeo | Ambiental | leituras reais de medidor por tag | comparação do real com o simulado no painel de metas |
| Launcher | Ambiental | onde o app está instalado | nada |

### 4.8 3D

Numa fase posterior, a cena em Godot ganha a camada Ambiental, ligada pela mesma lista de camadas que já mostra zonas e mapa de calor de exposição:

- brilho no piso em volta de cada máquina, na cor do indicador;
- uma coluna translúcida sobre a máquina, com a altura proporcional ao consumo no instante da reprodução;
- a peça refugada com um contorno e, ao clicar, o cartão da mochila dela;
- o medidor da fábrica no canto, com a potência instantânea e a janela de pico.

A cena só lê os números que a biblioteca calcula. Nenhuma conta é feita no Godot.

---

## 5. Arquitetura e repositório

### 5.1 Repositório `utsim-ambiental`

```text
utsim-ambiental/
  crates/
    utsim-ambiental-core/   tipos do bloco, validação, cálculo puro, casos de teste
    utsim-ambiental-app/    janela egui, telas, pedidos e respostas
  exemplos/                 modelos pequenos com o bloco preenchido
  pacotes/fabrica_verde/    missões do Ensino
  docs/                     este plano, o formato e o guia do aluno que assume o repositório
```

| Crate | Papel | Depende de |
|---|---|---|
| `utsim-ambiental-core` | `EnvironmentDef`, `EnvironmentCatalog`, validação, `avaliar(modelo, relatório, rastro, itens) -> Inventario`, curva de carga, balanços, rateio, pegada, cenários de recálculo, grafo do Sankey, grade do mapa | `utsim-core`, `utsim-model`, `utsim-stats` |
| `utsim-ambiental-app` | telas, desenho do mapa e do Sankey, relatórios, troca com os outros apps | o core, `utsim-ui-tema`, `utsim-idiomas`, `utsim-edit`, `utsim-pasta`, `utsim-posto`, `utsim-troca`, `utsim-ferramentas` |

Regras: o core não conhece egui. O app não faz conta. As bibliotecas da família entram como dependência com versão fixada, do jeito que a separação em repositórios definir. O desenho do Sankey e o do mapa ficam em módulos sem estado de tela, para poderem subir para uma biblioteca comum de gráficos se outro app precisar.

### 5.2 Fluxo do cálculo

1. O app abre o modelo e pede as replicações à biblioteca, com rastro e registro por item.
2. `avaliar` percorre o rastro uma vez e monta, por nó, as horas por estado, as contagens de eventos e a escada de potência.
3. Aplica coeficientes e fatores e monta o `Inventario`: quantidades por nó, fluxo e intervalo de tempo.
4. O registro por item dá o rateio por produto e a mochila do refugo.
5. Mapa, Sankey, pegada, cenários e metas são vistas do mesmo `Inventario`. Mudança de coeficiente ou de fator refaz só os passos 3 a 5.

### 5.3 Testes

- Casos resolvidos à mão para cada indicador: uma máquina, dois estados, uma tarifa, um fator.
- Conservação: a soma dos nós é o total da fábrica, a soma dos grupos de estado é a energia do nó, a soma dos produtos mais a estrutura é o total.
- Balanço de massa e de água fecham em modelos de exemplo completos.
- Monotonia: nenhum indicador cai quando uma potência, uma quantidade ou um fator sobe.
- Fator zero ou ausente nunca produz emissão diferente de zero ou de "falta fator".
- Demanda: curva em escada conhecida, com o pico atravessando a fronteira de dois intervalos.
- Reprodutibilidade por semente.
- Modelo sem o bloco dá o mesmo relatório de antes, byte a byte.
- Recalcular um cenário sobre o mesmo rastro dá o mesmo número que simular de novo, quando a alavanca não muda o fluxo.
- Nenhum relatório sai sem o aviso e sem a lista de fatores com as fontes.

---

## 6. Fases de entrega

O tamanho é relativo: P cabe em poucas semanas de um aluno, M em um a dois meses, G é um TCC inteiro ou mais de uma pessoa.

### 6.1 MVP: "a fábrica acende"

O MVP entrega uma aula completa e um estudo de caso de energia. O aluno abre um modelo que já existe, informa a potência das máquinas no assistente, vê o mapa de calor de energia e de CO₂ equivalente, toca o dia, acha o vilão, muda a potência em espera e vê a diferença com intervalo. São as fases AM0 a AM3.

### 6.2 Fases

| Fase | Conteúdo | Tamanho | Depende de | Pronto quer dizer |
|---|---|---|---|---|
| **AM0 Formato** (MVP) | tipos do bloco do nó e do modelo, validação, arquivo lateral, documento de formato | P | decisões A3 e A4 | o documento de formato foi aceito pelo dono, um modelo de exemplo com o bloco passa na validação, e todos os exemplos antigos abrem sem ganhar campo novo |
| **AM1 Energia e emissão no core** (MVP) | horas por estado, energia por nó e por grupo, kWh por peça, energia sem valor agregado, fatores de escopo 1 e 2, métricas com nome | M | AM0 | três casos resolvidos à mão batem, a soma dos nós é o total, a mesma semente dá a mesma saída, fator ausente mostra "falta fator" |
| **AM2 App com a tela Fábrica** (MVP) | janela da família, abrir modelo, assistente "Dar vida ao modelo", mapa de calor, painel do nó, "Os três vilões", cartão "De onde veio", tela Fatores mínima | G | AM1 | um aluno que nunca viu o app abre um modelo de exemplo e chega ao mapa aceso em menos de cinco minutos, sem ler manual, e todo número do painel abre a sua conta |
| **AM3 Tocar o dia e recalcular** (MVP) | curva de carga pelo rastro, reprodução, janela de tempo arrastável, recálculo na hora ao editar potência ou fator, relatório ambiental em HTML e PDF | M | AM2 | a reprodução para sozinha no pico e mostra as máquinas ligadas, a edição de uma potência atualiza o mapa em menos de um segundo, o relatório sai com fontes e aviso |
| **AM4 Demanda, pico e tarifa** | demanda por intervalo, janela de pico, fator de carga, conta de energia simulada | M | AM3 | a conta de um caso resolvido à mão bate, incluindo o pico que cruza dois intervalos, e as métricas `ambiental.demanda_*` aparecem no relatório |
| **AM5 Massa, água, resíduos e efluentes** | `flows` completos, massa do item, balanço de massa e de água, sucata e mochila do refugo, destinos, carga do efluente e tratamento | G | AM1 | um modelo de exemplo fecha o balanço de massa e o de água dentro da tolerância, e a mochila de uma peça refugada confere com a soma à mão do caminho dela |
| **AM6 Fluxos em Sankey** | grafo, desenho, quatro modos, destaque de caminho, abrir etapa, selo de balanço, exportação | G | AM5 | a largura de toda faixa confere com o inventário, o selo acusa um fluxo retirado de propósito, e o SVG exportado abre igual à tela |
| **AM7 Pegada por produto** | rateio, unidade funcional, fronteira, tela Produto, relatório do estudo | G | AM5, tipos de item | a soma das pegadas dos produtos mais a estrutura dá o total da fábrica, trocar a regra de rateio muda os produtos e não muda o total, o relatório lista o que ficou fora da fronteira |
| **AM8 Cenários** | prateleira, alavancas, recálculo na hora e simulação pareada, custo e impacto lado a lado, dispersão com fronteira, investimento e retorno, `estudo.json` | G | AM3, AM4 | um cenário de alavanca de processo roda com os mesmos números aleatórios do Base, a diferença sai com intervalo, e outra pessoa reproduz o número a partir do `estudo.json` |
| **AM9 Metas** | metas no modelo, painel, semáforo por probabilidade, orçamento de carbono, cascata por cenário, leituras reais na pasta de unidade | M | AM8 | uma meta cumprida em todas as replicações fica verde e uma cumprida em metade fica amarela, e o dado real aparece ao lado do simulado com a origem de cada um |
| **AM10 Aba no nó e mão dupla** | campo `environment` nas bibliotecas da família, `SetEnvironment`, aba no UTSim, `--utsim-abrir` e `--utsim-no`, migração do arquivo lateral | M | AM0, descongelamento das bibliotecas | editar a potência na aba do UTSim aparece no app ao recarregar, e o contrário também, com desfazer nos dois e sem dois escritores ao mesmo tempo |
| **AM11 Conversa com LSS e Estatística** | pedidos e respostas, oportunidade para a carteira, linha ambiental do mapa de fluxo de valor, teste da redução gravado no cenário | M | AM8, `utsim-troca` | um cenário vira ideia na carteira com hash conferido, e o resultado do teste pareado volta e aparece na etiqueta do cenário |
| **AM12 Pacote "Fábrica verde"** | as missões da seção 4.5, com modelos, textos, fatores didáticos e gabaritos | M | AM4, AM5, AM10 | cada gabarito confere com a simulação dentro da tolerância, e o pacote instala no Ensino sem código novo |
| **AM13 Camada no 3D** | brilho, colunas, cartão da mochila, medidor da fábrica | M | AM10 | a reprodução no 3D mostra a mesma potência instantânea que a curva do app, no mesmo instante |
| **AM14 Dados reais e gêmeo** | importação de leituras de medidor em CSV, tags do gêmeo, comparação do real com o simulado, calibração dos coeficientes | G | AM9 | uma gravação de teste reproduz o mesmo indicador que o CSV equivalente, e o app sugere o coeficiente que aproxima o simulado do medido |
| **AM15 Incerteza e busca** | faixa nos fatores com sorteio, fator por hora do dia, otimização com dois objetivos, curva de custo por tonelada evitada | G | AM8 | o intervalo com incerteza de fator nunca é mais estreito que o só da simulação, e a fronteira do otimizador contém os melhores cenários feitos à mão |
| **AM16 Outras categorias de impacto** | categorias além do clima, com fatores de caracterização cadastrados pelo usuário | M | AM7 | uma categoria cadastrada pelo aluno aparece na tela Produto e no relatório, com a fonte, sem nenhum fator vindo de fábrica |

### 6.3 Ordem sugerida

1. AM0, AM1, AM2 e AM3 fecham o MVP, com o arquivo lateral.
2. AM4 e AM5 em paralelo, porque mexem em partes diferentes do core.
3. AM6 e AM8, que são as telas de maior efeito em sala.
4. AM7 e AM9.
5. AM10 quando as bibliotecas da família estiverem separadas e descongeladas, e depois AM11, AM12 e AM13.
6. AM14, AM15 e AM16 são extensões naturais para TCC.

### 6.4 Requisitos sugeridos

| Id | Requisito |
|---|---|
| AMB-RF-01 | O nó guarda potência por grupo de estado e fluxos por direcionador, e o modelo guarda o catálogo de fluxos e fatores |
| AMB-RF-02 | O app calcula energia, água, resíduos, efluentes e CO₂ equivalente por nó, etapa e produto, a partir do relatório, do rastro e do registro por item |
| AMB-RF-03 | Todo fator é editável e tem fonte, e indicador com fator ausente não mostra número |
| AMB-RF-04 | O app calcula a curva de carga, a demanda por intervalo e a energia na janela de pico |
| AMB-RF-05 | O mapa de calor mostra qualquer indicador por nó, em qualquer janela de tempo |
| AMB-RF-06 | O Sankey mostra massa, energia, água e carbono, e acusa balanço que não fecha |
| AMB-RF-07 | A pegada por produto declara unidade funcional, fronteira e regra de rateio |
| AMB-RF-08 | Um cenário compara custo e impacto com o Base por diferença pareada com intervalo |
| AMB-RF-09 | O painel de metas mostra a probabilidade de cumprir cada meta e separa simulado de real |
| AMB-RF-10 | Os indicadores saem como métricas com nome no relatório da simulação |
| AMB-RF-11 | O bloco do nó é editado na aba do UTSim e no app, com um escritor por vez |
| AMB-RF-12 | Todo relatório traz o aviso de uso didático e a lista de fatores com as fontes |
| AMB-RNF-01 | Modelo sem bloco ambiental dá o mesmo resultado de antes, byte a byte |
| AMB-RNF-02 | Mudança de coeficiente ou de fator atualiza a tela em menos de um segundo num modelo de sala de aula |

---

## 7. Riscos

| Risco | Cuidado |
|---|---|
| O número simulado ser lido como medido ou como oficial | origem sempre visível, aviso em todo relatório, selo "sem fonte" e "didático" |
| Aluno preencher coeficientes sem critério e acreditar no resultado | cartão "De onde veio", selo de balanço, missão sobre de onde tirar a potência de uma máquina (placa, catálogo, medição) |
| Rastro grande em modelo grande | cálculo numa só passada, com acumuladores por intervalo, e amostragem da reprodução |
| Rateio por produto virar discussão sem fim | regra declarada e trocável, total que nunca muda com a regra |
| Dependência do descongelamento das bibliotecas | arquivo lateral no MVP, mesma estrutura de dados |
| Escopo crescer para todas as categorias de impacto | clima e inventário primeiro, o resto só com fator do usuário |
| Sankey e mapa são desenho próprio em egui | módulos sem estado, com teste de geometria e exportação conferida como imagem |

---

## 8. Ideias de TCC

Cada ideia cabe num TCC e deixa algo no repositório ou um estudo de caso publicável. A coluna "tipo" diz se o aluno usa o app (estudo) ou o estende (extensão).

### 8.1 Engenharia ambiental

| Ideia | Tipo | Fase que toca |
|---|---|---|
| Inventário de entradas e saídas de uma fábrica real da região, modelada no UTSim, com balanço de massa fechado | estudo | AM5, AM6 |
| Avaliação de ciclo de vida simplificada de dois produtos ou duas rotas de fabricação, com análise do efeito da regra de rateio | estudo | AM7 |
| Módulo de efluentes: tratamento em etapas, eficiência por parâmetro e mistura | extensão | AM5 |
| Categorias de impacto além do clima, com fatores levantados e documentados pelo aluno | extensão | AM16 |
| Incerteza dos fatores de emissão somada à variabilidade da simulação | extensão | AM15 |
| Plano de gerenciamento de resíduos de uma linha, com cenários de destino e custo | estudo | AM5, AM8 |
| Simbiose entre duas fábricas: o resíduo de um modelo vira insumo de outro | extensão | AM5 |

### 8.2 Engenharia de alimentos

| Ideia | Tipo | Fase que toca |
|---|---|---|
| Água e energia por lote numa linha de laticínio, bebida ou conserva, com a limpeza entre lotes como setup com consumo próprio | estudo | AM5 |
| Efeito do tamanho de lote e da sequência de produtos sobre a água de limpeza | estudo | AM8 |
| Carga orgânica do efluente por etapa e cenários de segregação na fonte | estudo | AM5 |
| Perda de produto ao longo da linha vista como massa, energia e água embutidas | estudo | AM5, AM7 |
| Processos térmicos: energia de pasteurização, cozimento e refrigeração por kg de produto, com recuperação de calor como cenário | extensão | AM5, AM8 |
| Aproveitamento de subproduto (soro, bagaço, casca) como coproduto, com rateio por massa e por valor | extensão | AM7 |

### 8.3 Engenharia elétrica

| Ideia | Tipo | Fase que toca |
|---|---|---|
| Curva de carga simulada de uma planta e validação contra a medição real | estudo | AM4, AM14 |
| Deslocamento de carga e sequência de partida para reduzir a demanda no pico | estudo | AM4, AM8 |
| Geração própria e armazenamento como objetos do cenário, com despacho simples | extensão | AM8, AM15 |
| Importação de leituras de medidor e calibração automática das potências por estado | extensão | AM14 |
| Linha de base energética por regressão e verificação de economia depois de uma ação | estudo | AM11 |
| Modelo de consumo de motor com carga parcial e com inversor, no lugar da potência constante por estado | extensão | AM1 |
| Medidor de baixo custo numa bancada didática, alimentando o gêmeo | extensão | AM14 |

### 8.4 Engenharia de produção

| Ideia | Tipo | Fase que toca |
|---|---|---|
| Projeto DMAIC de redução de energia sem valor agregado, do termo ao controle, com o caso documentado | estudo | AM11 |
| Mapa de fluxo de valor com linha ambiental aplicado a uma empresa | estudo | AM11 |
| Compromisso entre custo e emissão com otimização de dois objetivos | extensão | AM15 |
| Efeito da política de inspeção sobre o impacto embutido no refugo | estudo | AM5, AM8 |
| Programação da produção com restrição de demanda de energia | extensão | AM8, AM15 |
| Avaliação do pacote "Fábrica verde" em sala: o aluno aprende mais com o mapa que com a planilha? | estudo | AM12 |
| Curva de custo por tonelada evitada de um conjunto de ações de uma fábrica | estudo | AM15 |

### 8.5 Outros cursos

| Curso | Ideia | Fase que toca |
|---|---|---|
| Computação | algoritmo de disposição do Sankey com menos cruzamentos e animação entre janelas de tempo | AM6 |
| Computação | camada ambiental na cena 3D | AM13 |
| Mecânica | modelo de ar comprimido com vazamento e de forno com perda pelas paredes | AM5 |
| Química | balanço de massa com reação em tanque, sobre a rede de fluidos | AM5 |
| Design | estudo de uso das telas com alunos e redesenho do primeiro uso | AM2 |

---

## 9. Decisões em aberto para o dono

Cada linha traz a proposta deste plano. Nada aqui está decidido.

### Gerais

| # | Pergunta | Proposta |
|---|---|---|
| A1 | Nome do produto e do binário | "UTSim Ambiental", binário `utsim-ambiental`, produto `ambiental` no launcher, crates `utsim-ambiental-*` (o nome `utsim-env` já é do aprendizado por reforço) |
| A2 | Cor de destaque e ícone do app | verde, com ícone de folha sobre engrenagem, no estilo plano da família |
| A3 | A conta ambiental mora onde? | numa crate pura do repositório do app, feita depois da rodada. A biblioteca do modelo só chama essa conta para publicar as métricas com nome, quando o campo for aceito. Alternativa: a conta dentro do núcleo, como a de qualidade |
| A4 | Enquanto as bibliotecas estão congeladas, o bloco fica onde? | num arquivo lateral `<modelo>.ambiental.ron` com a mesma estrutura, migrado para o nó na fase AM10. Alternativa: esperar o descongelamento antes de começar |
| A5 | Público do MVP | ensino primeiro, empresa piloto depois, como nos outros apps |
| A6 | Quem assume o repositório primeiro | definir o curso do primeiro aluno, porque isso ordena as fases depois do MVP (elétrica favorece AM4 e AM14, ambiental e alimentos favorecem AM5 a AM7, produção favorece AM8 e AM11) |

### Indicadores e formato

| # | Pergunta | Proposta |
|---|---|---|
| A7 | Estados em cinco grupos de potência ou potência por estado? | cinco grupos, com exceção por estado em `by_state` |
| A8 | Fatores de emissão: o app traz algum valor? | não. O catálogo nasce vazio, com fonte "a definir". O dono decide depois se quer uma biblioteca de fatores com fonte pública citada, e quem a mantém |
| A9 | Conjunto de fatores "didático" nas missões do Ensino | aceitar, sempre com o rótulo "didático" visível até o relatório |
| A10 | Tarifa de energia: até onde modelar? | pico e fora de pico, demanda contratada e ultrapassagem, com todos os preços informados pelo usuário. Bandeiras, tributos e energia reativa ficam fora |
| A11 | Intervalo de integração da demanda | 15 minutos como padrão, editável |
| A12 | Regra padrão de rateio para o produto | pelo tempo de processamento, com a troca a um clique |
| A13 | Sucata atribuída a quem, por padrão | ao nó onde o defeito nasceu, com a opção de ver pelo nó que rejeitou |
| A14 | Desligamento automático em espera calculado depois da rodada | aceitar como aproximação declarada na tela. Alternativa: só permitir quando modelado no UTSim |
| A15 | Metas moram no modelo ou na pasta de unidade? | no modelo quando são sobre o simulado, e as leituras reais na pasta de unidade |
| A16 | Massa do item: campo `mass_kg` no tipo de item | sim, combinado com o UTSim Produto, que já tem a estrutura do produto com quantidades |

### Telas e alcance

| # | Pergunta | Proposta |
|---|---|---|
| A17 | Categorias de impacto além do clima | fora até a fase AM16, e só com fatores cadastrados pelo usuário |
| A18 | Fronteira da avaliação de ciclo de vida | portão a portão e berço ao portão. Uso e fim de vida só como entrada manual, numa fase posterior |
| A19 | Preço de carbono na tela de cenários | campo opcional, vazio por padrão |
| A20 | Selos de conquista no painel de metas | ligados por padrão no modo ensino, com chave para o professor desligar |
| A21 | Limites de efluente e comparação com o limite | o usuário cadastra o limite e a fonte, o app compara e pinta, sem afirmar conformidade |
| A22 | Camada ambiental no 3D | depois da aba no nó (AM13). Alternativa: antes, se o dono quiser o efeito visual cedo |

### Integração

| # | Pergunta | Proposta |
|---|---|---|
| A23 | Ferramentas que o Ambiental atende por pedido | `mapa_impacto`, `sankey`, `pegada` e `cenario_ambiental`, com resultado no formato de troca |
| A24 | Linha ambiental no mapa de fluxo de valor do LSS | sim, alimentada pelo Ambiental, desenhada pelo LSS |
| A25 | Ganho ambiental no controle de ganhos do LSS | ao lado do financeiro, sem converter um no outro |
| A26 | Sankey e mapa de planta sobem para uma biblioteca comum de gráficos? | nascem no repositório do app, em módulos sem estado, e sobem quando um segundo app precisar |
| A27 | Pacote "Fábrica verde" mora no repositório do Ambiental ou no do Ensino? | no do Ambiental, como os pacotes de Qualidade e Segurança |
| A28 | Pacote de linha "ambiental" em `packs`, com predefinições de máquina já com potência típica? | predefinições com os campos, sem valores, pelo mesmo cuidado da A8 |
