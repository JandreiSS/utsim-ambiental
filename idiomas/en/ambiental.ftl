# UTSim Ambiental (en). Mesmas chaves nas cinco línguas.

## app.rs

app-inicio = Home
app-grupo_ver = See
app-grupo_dados = Data
app-grupo_decidir = Decide
app-fabrica = Factory
app-fluxos = Flows
app-produto = Product
app-maquinas = Machines
app-fatores = Factors
app-cenarios = Scenarios
app-metas = Targets
app-relatorios = Reports
app-abra_um_modelo = Open a model on the Home screen.
app-fase_am6 = Arrives in phase AM6 of the plan.
app-fase_am7 = Arrives in phase AM7 of the plan.
app-fase_am8 = Arrives in phase AM8 of the plan.
app-fase_am9 = Arrives in phase AM9 of the plan.
app-nao_salvo = unsaved changes
app-subtitulo = Energy and environmental impact
app-recentes = Recent models
app-acao_abrir = Open a UTSim model
app-acao_abrir_desc = Choose a .ron file: the app simulates and lights up the factory.
app-acao_exemplo = Open the example
app-acao_exemplo_desc = A painting line with the machine power already filled in.
app-onde_explicacao = The environmental block is stored in a file next to the model, <model>.ambiental.ron. The model does not change.
app-aviso_didatico = Simulated figures, for teaching: not a measurement nor an official inventory.
app-aberto = Model opened: { $modelo }. Simulating…
app-simulado = Simulation done.
app-erro_abrir = Could not open: { $e }
app-erro_simular = The simulation failed: { $e }
app-erro_config = Could not save the settings: { $e }
app-salvar = Save
app-salvar_dica = Writes the environmental block next to the model.
app-salvo = Saved to { $arquivo }
app-erro_salvar = Could not save: { $e }

## unidades

un-kwh = kWh
un-kwh_peca = kWh/part
un-kg_co2e = kg CO₂e
un-kg_co2e_peca = kg CO₂e/part

## grupos de estado

grupo-produzindo = Producing
grupo-preparando = Preparing
grupo-esperando = Waiting
grupo-parada = Stopped
grupo-fora_de_turno = Off shift

## vilões

vilao-esperando = { $no } spends { $pct } % of its energy waiting for parts ({ $pf } % of the factory, no value).
vilao-parada = { $no } spends { $pct } % of its energy while stopped ({ $pf } % of the factory, no value).
vilao-fora_de_turno = { $no } spends { $pct } % of its energy off shift ({ $pf } % of the factory, no value).
vilao-preparando = { $no } spends { $pct } % of its energy preparing ({ $pf } % of the factory, no value).
vilao-produzindo = { $no } spends { $pct } % of its energy producing ({ $pf } % of the factory, no value).

## fabrica.rs

fab-simulando = Simulating…
fab-origem = Simulated · { $n } replications · seed { $semente }
fab-simular_de_novo = Simulate again
fab-energia = Energy
fab-sem_valor = Non-value energy
fab-co2e = CO₂ equivalent
fab-total = Total
fab-por_peca = Per part
fab-por_peca_boa = Per good part
fab-pecas_boas = Good parts
fab-esperando_simulacao = Waiting for the simulation…
fab-a_fabrica = The factory
fab-falta_fator = factor missing
fab-falta_fator_dica = No emission factor for: { $fluxos }. Fill it in on the Factors screen.
fab-sem_fonte = no source: { $fluxo }
fab-didatico = teaching value: { $fluxo }
fab-clique_numa_maquina = Click a machine on the map to see and change its power.
fab-viloes = The three villains
fab-sem_viloes = No machine spends energy without adding value.
fab-ainda_nao_contado = Not counted yet in this version
fab-informe_potencia = Enter the power
fab-de_onde_veio = Where does it come from?
fab-de_onde_veio_dica = Opens the calculation of this number.
fab-de_onde_veio_titulo = Where it comes from: { $no }
fab-potencia_kw = Power in each state group
fab-antes_e_depois = Before and after
fab-base_explica = Keep the current situation as the baseline, change a power and see the difference.
fab-guardar_base = Keep as baseline
fab-nova_base = New baseline
fab-voltar_base = Back to baseline
fab-pareado_explica = Paired difference: same seed and same replications, with a 95 % interval.
fab-formula_energia = E = Σ P(state) × T(state) + e(part) × parts
fab-media_de = Mean of { $n } replications.
fab-col_origem = Source
fab-col_grupo = Group
fab-col_direcionador = Driver
fab-col_coeficiente = Coefficient
fab-col_quantidade = Quantity
fab-estado = state { $estado }
fab-pecas_que_entraram = parts that came in
fab-total_energia = Node energy: { $kwh } kWh.
fab-conta_emissao = Emission = energy × { $fator } kg CO₂e/kWh (scope { $escopo }, source: { $fonte }) = { $kg } kg CO₂e.

## maquinas.rs

maq-titulo = Bring the model to life
maq-explica = Enter each machine's power (kW) in each state group: from the nameplate, the catalogue or a measurement. The time in each state comes from the simulation.
maq-maquina = Machine
maq-etapa = Stage
maq-energia = Energy

## fatores.rs

fat-titulo = Emission factors
fat-explica = The app ships no factor values: enter each value and its source. A factor without a source is flagged on every screen.
fat-unidade_tempo = Model time unit:
fat-unidade_tempo_dica = The hours in each state depend on it.
fat-segundo = second
fat-minuto = minute
fat-hora = hour
fat-fluxo = Flow
fat-tipo = Kind
fat-unidade = Unit
fat-escopo = Scope
fat-fator = Factor
fat-fonte = Source
fat-ano = Year
fat-didatico = Teaching
fat-tem_valor = Has a value (unchecked: to be defined)
fat-kg_por = kg CO₂e per { $unidade }
fat-sem_fonte = no source
fat-tirar = Remove
fat-novo_fluxo = New flow
fat-acrescentar = Add
fat-tipo_energia = energy
fat-tipo_combustivel = fuel
fat-tipo_agua = water
fat-tipo_material = material
fat-tipo_residuo = waste
fat-tipo_efluente = effluent
fat-tipo_emissao = direct emission
fat-tipo_outro = other

## pendências da conta (utsim-ambiental-core)

pend-setup = { $no }: energy or flow per setup (needs the trace: simulate again).
pend-partida = { $no }: energy or flow per start-up (needs the trace: simulate again).
pend-espera = { $no }: standby shutdown (needs the trace: simulate again).
pend-refugo = { $no }: flow per scrapped part (phase AM5).
pend-retrabalho = { $no }: flow per rework (phase AM5).
pend-distancia = { $no }: flow per distance (phase AM5).
pend-volume = { $no }: flow per volume (phase AM5).
pend-massa = { $no }: flow per mass (phase AM5).

## validação (utsim-ambiental-core)

val-objeto_inexistente = Object { $no } does not exist in the model.
val-fluxo_inexistente = { $no }: flow { $fluxo } is not in the catalogue.
val-fluxo_de_saida_como_entrada = { $no }: flow { $fluxo } is an output but adds mass to the part.
val-negativo = { $no }: { $campo } cannot be negative.
val-agua_fracao = { $no }: water fractions must be between 0 and 1.
val-agua_soma = { $no }: water fractions add up to more than 1.
val-reuso_inexistente = { $no }: reuse goes to { $alvo }, which does not exist.
val-parametro_inexistente = { $no }: effluent parameter { $parametro } is not in the catalogue.
val-pico_fora_do_dia = The peak window must fit within one day (HH:MM).
val-intervalo_zero = The demand interval must be greater than zero.
val-massa_sem_tipo = { $no }: the flow adds or removes mass, but the item types have no mass.
val-sem_fluxo_eletrico = The catalogue has no energy flow: power does not turn into emission.
val-fluxo_repetido = Flow { $fluxo } appears twice in the catalogue.
val-escopo_invalido = The factor of { $fluxo } has a scope other than 1, 2 or 3.

## AM3: curva de carga, tocar o dia, detalhes, relatório

un-kw = kW
un-kwh_evento = kWh/event
fab-curva_de_carga = Load curve
fab-tocar_o_dia = ▶ Play the day
fab-pausar = ⏸ Pause
fab-parar = Stop
fab-janela = Calculation window: { $de } to { $ate }
fab-janela_inteira = Whole window
fab-arraste_dica = Drag on the curve to calculate only a stretch; click to see an instant.
fab-sem_curva = No curve: simulate again to record the trace.
fab-no_pico = Peak at { $hora }: { $kw } kW. On together: { $nos }.
fab-agora = { $hora }: { $kw } kW. On: { $nos }.
fab-pico = Peak of { $kw } kW at { $hora }.
fab-setups = setups
fab-partidas = start-ups
fab-estado_standby = switched off while waiting
maq-detalhes = Details
maq-detalhes_de = Details: { $no }
maq-eventos = Energy per event
maq-kwh_peca = Per part coming in
maq-kwh_setup = Per setup
maq-kwh_partida = Per start-up (switching on after a stop)
maq-desligar_em_espera = Switch off after a long wait
maq-depois_de = After
maq-unidades_do_modelo = model time units waiting
maq-espera_aproximacao = Approximation: the simulation does not delay the machine to switch back on. The waiting time past the limit counts at the off-shift power, and each return counts a start-up.
rel-tela = Environmental report
rel-explica = Creates the model report, with the teaching-use notice and the list of factors with their sources. It is saved next to the model.
rel-gerar_html = Create HTML
rel-gerar_pdf = Create PDF
rel-gravado = Report saved to { $arquivo }
rel-mostrar = Show in folder
rel-conteudo = What the report contains:
rel-titulo = Environmental report: { $modelo }
rel-origem = Simulated: { $n } replications, seed { $semente }, window from { $de } to { $ate }.
rel-aviso_titulo = Teaching use
rel-aviso = Simulated figures, for teaching: not a measurement, an official inventory or a statement of compliance. Check the factors and sources before using any number.
rel-resumo = Summary
rel-por_maquina = By machine
rel-energia_por_grupo = Energy by state group (kWh)
rel-potencia_media = Average factory power (kW), first replication
rel-fatores = Factors and sources
rel-a_definir = to be defined
rel-sem_fatores = No factor registered: no emission figure in this report.

## AM4: demanda, pico e tarifa

fat-tarifa = Energy tariff
fat-tarifa_explica = Peak hours, contracted demand and prices: all entered by you, with no built-in values. Surcharges, taxes and reactive energy are left out.
fat-usar_tarifa = Use a tariff in this model
fat-horario_pico = Peak hours
fat-dias_uteis = weekdays only (day 1 is Monday)
fat-intervalo_demanda = Demand interval
fat-contratada = Contracted demand (kW)
fat-preco_pico = Price per kWh at peak
fat-preco_fora = Price per kWh off peak
fat-preco_kw = Price per kW of demand
fat-preco_ultra = Price per kW of overrun
fat-fonte_tarifa = Tariff source
fab-demanda = Demand and tariff
fab-demanda_max = Maximum demand
fab-demanda_pico = Demand at peak
fab-sem_pico_na_janela = the window does not cross peak hours
fab-energia_pico = Energy at peak
fab-fator_de_carga = Load factor
fab-conta_energia = Simulated energy bill
fab-conta_explica = Bill of the simulated window: energy at and off peak, the demand (contracted or measured, the larger) and the overrun, as if the window were a billing period.
fab-falta_preco = price missing
fab-falta_preco_dica = No bill: missing { $precos }. Fill them in on the Factors screen.
fab-contratada = contracted { $kw } kW
rel-parcela_pico = Peak energy part
rel-parcela_fora = Off-peak energy part
rel-parcela_demanda = Demand part
rel-parcela_ultra = Overrun part
rel-tarifa_fonte = Peak from { $de } to { $ate }, { $min }-minute interval. Tariff source: { $fonte }.
