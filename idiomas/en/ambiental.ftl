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

## AM5: massa, água, resíduos e efluentes

un-m3 = m³
un-m3_peca = m³/part
un-kg = kg
un-kg_peca = kg/part
fab-agua = Water
fab-residuos = Waste
fab-efluente = Effluent
fab-residuo_processo = Process waste
fab-sucata = Scrap
fab-agua_e_efluentes = Water and effluents
fab-captacao = Water intake
fab-consumo = Consumption (intake − effluent)
fab-reuso = Reuse
fab-tratamento = Treatment energy
fab-lodo = Treatment sludge
fab-carga = { $parametro }: { $antes } kg before, { $depois } kg after treatment; mix { $mgl } mg/L
fab-limite = Registered limit: { $limite } (source: { $fonte }). The app compares; it does not state compliance.
fab-desvio_aterro = Diverted from landfill
fab-sem_destino = no destination
fab-materiais = Materials
fab-entrada_massa = Mass in
fab-massa_boa = Good product mass
fab-rendimento = Material yield
fab-sucata_pecas = { $kg } kg ({ $pecas } parts)
fab-balanco = Mass balance
fab-balanco_fecha = closes
fab-balanco_erro = does not close: { $pct } %
fab-mochila_kwh = Scrap backpack (energy)
fab-mochila_co2e = Scrap backpack (CO₂e)
fab-mochila_explica = The backpack is what the scrapped parts carried with them: the average energy per part of each machine they went through and the per-part flows. Scrapping at the end of the line costs more than at the start.
fab-custo_fluxos = Cost of flows and waste
maq-fluxos = Machine flows
maq-fluxos_explica = Each row: quantity × driver (per part, per hour in a state, per setup...). Water in L or m³; waste in kg.
maq-por = { $unidade } { $dir }
maq-dir_peca = per part in
maq-dir_peca_boa = per part out
maq-dir_refugo = per part scrapped here
maq-dir_setup = per setup
maq-dir_partida = per start-up
maq-dir_massa = per kg processed
maq-dir_tempo_produzindo = per hour producing
maq-dir_tempo_ligada = per hour switched on
maq-dir_outro = other (from the file)
maq-soma_massa = adds to the part
maq-tira_massa = removes from the part
maq-novo_fluxo = + Flow
maq-sem_fluxos = Register flows (water, waste, material) on the Factors screen.
maq-balanco_agua = Water balance of this machine
maq-incorporada = Fraction built into the product
maq-evaporada = Fraction evaporated or lost
maq-reuso_para = Effluent reused at
maq-reuso_fracao = Fraction of effluent reused
maq-efluente = Effluent of this machine
maq-destino_efluente = Effluent destination
maq-sem_parametros = Register the effluent parameters on the Factors screen.
fat-preco = Price
fat-destino = Destination
fat-massa = Part mass
fat-massa_explica = With the mass, the app makes the mass balance, scrap in kg and material yield.
fat-qualquer_peca = Part without type (all)
fat-destinos = Waste destinations
fat-desviado = Diverted from landfill
fat-fator_kg = Factor (kg CO₂e/kg)
fat-custo_kg = Cost per kg
fat-novo_destino = New destination
fat-destino_sucata = Scrap destination:
fat-efluente = Effluent
fat-efluente_explica = Parameters (with the limit and source you register) and effluent destinations, with the removal of each parameter, energy, sludge and cost per m³.
fat-parametros = Parameters
fat-parametro = Parameter
fat-limite = Limit
fat-fonte_limite = Limit source
fat-novo_parametro = New parameter
fat-destinos_efluente = Effluent destinations
fat-kwh_m3 = kWh/m³
fat-lodo_m3 = sludge kg/m³
fat-custo_m3 = cost/m³
fat-remocao = removes { $parametro }
fat-novo_destino_efluente = New effluent destination
rel-cargas = Effluent loads
rel-antes_kg = Before (kg)
rel-depois_kg = After (kg)
rel-concentracao = Mix (mg/L)
rel-sem_conformidade = The comparison with the limit is not a statement of legal compliance.
rel-desvio = Diverted from landfill: { $pct } %.
val-destino_inexistente = { $no }: destination { $destino } is not in the catalogue.
val-unidade_agua = Water flow { $fluxo } must be in L or m3.

## AM6: Sankey

snk-titulo = Flows
snk-massa = Mass
snk-energia = Energy
snk-agua = Water
snk-carbono = Carbon
snk-por_etapa = Group by stage
snk-juntar = Hide flows smaller than
snk-sem_massa = No mass: enter the part mass on the Factors screen and simulate again.
snk-vazio = No machine has what this mode shows.
snk-total = Total: { $valor } { $unidade }
snk-exportar_svg = Export SVG
snk-exportar_csv = Export inventory (CSV)
snk-dica = Click a band to see its path; double-click a stage to open it.
snk-balanco_dica = { $no }: { $entra } in and { $sai } out. Check that you registered the materials added to the part and the waste removed from it at this machine.
snk-faltam = { $pct } % missing
snk-sobram = { $pct } % extra
snk-faixa = { $de } → { $para }: { $valor } { $unidade } ({ $pct } % of the total)
snk-titulo_svg = { $modelo }: { $modo } flows
snk-agrega_valor = value-adding energy
snk-preparando = preparing energy
snk-sem_valor = non-value energy (waiting, stopped, off shift)
snk-tratamento = effluent treatment
snk-captacao = water intake
snk-evaporada = evaporated or lost
snk-incorporada = built into the product
snk-efluente_sem_tratamento = effluent with no treatment registered
snk-produto = product
snk-sucata_e_lodo = scrap, sludge and treatment

## AM7: pegada por produto

prd-titulo = Product footprint
prd-aviso = Simplified teaching study: not a full life cycle assessment nor an environmental product declaration.
prd-sem_rastro = The product footprint needs the simulation trace: click Simulate again on the Factory screen.
prd-sem_pecas = No part went through machines with an environmental block in the window.
prd-produto = Product:
prd-sem_tipo = part (no type)
prd-unidade = Functional unit:
prd-peca_boa = good part
prd-kg_produto = kg of product
prd-regra = Allocation:
prd-regra_tempo = by time at the machine
prd-regra_pecas = by number of parts
prd-regra_massa = by mass
prd-regra_estrutura = waiting as overhead
prd-comparar = Compare with:
prd-materias_primas = Raw materials
prd-transporte = Transport
prd-fabricacao = Manufacturing
prd-distribuicao = Distribution
prd-uso = Use
prd-fim_de_vida = End of life
prd-portao_a_portao = gate to gate (manufacturing only)
prd-berco_ao_portao = cradle to gate (materials and manufacturing)
prd-fronteira = Boundary: { $fronteira }. Click Raw materials to include or remove it.
prd-unidades = Units in the window
prd-co2_por = CO₂e per { $unidade }
prd-energia_por = Energy per { $unidade }
prd-agua_por = Water per { $unidade }
prd-mochila_por = Energy in scrapped parts, per { $unidade }
prd-taxa_refugo = Scrap rate:
prd-voltar_simulada = simulated
prd-com_refugo = With { $taxa } % scrap (simulated: { $simulada } %), the footprint would be { $valor } kg CO₂e per unit.
prd-com_refugo_kwh = With { $taxa } % scrap (simulated: { $simulada } %), the energy would be { $valor } kWh per unit.
prd-contribuicao = Contribution by machine
prd-kwh_por_unidade = kWh per unit
prd-co2_por_unidade = kg CO₂e per unit (manufacturing)
prd-estrutura = Allocation { $regra }: products { $produtos } kWh + overhead { $estrutura } kWh = factory { $fabrica } kWh. Changing the rule changes the products, never the total.
prd-relatorio = Study report
prd-rel_titulo = Footprint study: { $produto }
prd-rel_objetivo = Goal
prd-rel_objetivo_texto = Estimate, for teaching, the energy, water and CO₂ equivalent of { $produto } per { $unidade } in the simulated factory.
prd-rel_fronteira = Functional unit and boundary
prd-rel_fora = Left outside the boundary:
prd-rel_inventario = Inventory per functional unit
prd-rel_impacto = Impact: climate change
prd-rel_impacto_texto = { $valor } kg CO₂e per { $unidade }.
prd-rel_interpretacao = Interpretation
prd-rel_regras = Effect of the allocation rule
prd-rel_regras_texto = The allocation rule is a choice: it changes each product's footprint, but not the factory total.
prd-rel_limitacoes = Limitations
prd-lim_1 = The figures come from a simulation, not from measurement.
prd-lim_2 = Only climate change is assessed; other impact categories are left out.
prd-lim_3 = The emission factors are those of the model catalogue, with the sources listed below.
