# UTSim Ambiental (es). Mesmas chaves nas cinco línguas.

## app.rs

app-inicio = Inicio
app-grupo_ver = Ver
app-grupo_dados = Datos
app-grupo_decidir = Decidir
app-fabrica = Fábrica
app-fluxos = Flujos
app-produto = Producto
app-maquinas = Máquinas
app-fatores = Factores
app-cenarios = Escenarios
app-metas = Metas
app-relatorios = Informes
app-abra_um_modelo = Abra un modelo en Inicio.
app-nao_salvo = cambios sin guardar
app-subtitulo = Energía e impacto ambiental
app-recentes = Modelos recientes
app-acao_abrir = Abrir un modelo de UTSim
app-acao_abrir_desc = Elija un archivo .ron: la app simula y enciende la fábrica.
app-acao_exemplo = Abrir el ejemplo
app-acao_exemplo_desc = Una línea de pintura con la potencia de las máquinas ya informada.
app-onde_explicacao = El bloque ambiental queda en un archivo junto al modelo, <modelo>.ambiental.ron. El modelo no cambia.
app-aviso_didatico = Números simulados, para enseñanza: no son medición ni inventario oficial.
app-aberto = Modelo abierto: { $modelo }. Simulando…
app-simulado = Simulación lista.
app-erro_abrir = No pude abrir: { $e }
app-erro_simular = La simulación falló: { $e }
app-erro_config = No pude guardar la configuración: { $e }
app-salvar = Guardar
app-salvar_dica = Graba el bloque ambiental junto al modelo.
app-salvo = Guardado en { $arquivo }
app-erro_salvar = No pude guardar: { $e }

## unidades

un-kwh = kWh
un-kwh_peca = kWh/pieza
un-kg_co2e = kg CO₂e
un-kg_co2e_peca = kg CO₂e/pieza

## grupos de estado

grupo-produzindo = Produciendo
grupo-preparando = Preparando
grupo-esperando = Esperando
grupo-parada = Parada
grupo-fora_de_turno = Fuera de turno

## vilões

vilao-esperando = { $no } gasta { $pct } % de la energía esperando pieza ({ $pf } % de la fábrica sin valor).
vilao-parada = { $no } gasta { $pct } % de la energía parada ({ $pf } % de la fábrica sin valor).
vilao-fora_de_turno = { $no } gasta { $pct } % de la energía fuera de turno ({ $pf } % de la fábrica sin valor).
vilao-preparando = { $no } gasta { $pct } % de la energía preparando ({ $pf } % de la fábrica sin valor).
vilao-produzindo = { $no } gasta { $pct } % de la energía produciendo ({ $pf } % de la fábrica sin valor).

## fabrica.rs

fab-simulando = Simulando…
fab-origem = Simulado · { $n } réplicas · semilla { $semente }
fab-simular_de_novo = Simular de nuevo
fab-energia = Energía
fab-sem_valor = Energía sin valor
fab-co2e = CO₂ equivalente
fab-total = Total
fab-por_peca = Por pieza
fab-por_peca_boa = Por pieza buena
fab-pecas_boas = Piezas buenas
fab-esperando_simulacao = Esperando la simulación…
fab-a_fabrica = La fábrica
fab-falta_fator = falta factor
fab-falta_fator_dica = Sin factor de emisión para: { $fluxos }. Infórmelo en la pantalla Factores.
fab-sem_fonte = sin fuente: { $fluxo }
fab-didatico = didáctico: { $fluxo }
fab-clique_numa_maquina = Haga clic en una máquina del mapa para ver y cambiar su potencia.
fab-viloes = Los tres villanos
fab-sem_viloes = Ninguna máquina gasta energía sin agregar valor.
fab-ainda_nao_contado = Aún no contado en esta versión
fab-informe_potencia = Informe la potencia
fab-de_onde_veio = ¿De dónde viene?
fab-de_onde_veio_dica = Abre la cuenta de este número.
fab-de_onde_veio_titulo = De dónde viene: { $no }
fab-potencia_kw = Potencia en cada grupo de estado
fab-antes_e_depois = Antes y después
fab-base_explica = Guarde la situación actual como base, cambie una potencia y vea la diferencia.
fab-guardar_base = Guardar como base
fab-nova_base = Nueva base
fab-voltar_base = Volver a la base
fab-pareado_explica = Diferencia pareada: misma semilla y mismas réplicas, con intervalo del 95 %.
fab-formula_energia = E = Σ P(estado) × T(estado) + e(pieza) × piezas
fab-media_de = Media de { $n } réplicas.
fab-col_origem = Origen
fab-col_grupo = Grupo
fab-col_direcionador = Inductor
fab-col_coeficiente = Coeficiente
fab-col_quantidade = Cantidad
fab-estado = estado { $estado }
fab-pecas_que_entraram = piezas que entraron
fab-total_energia = Energía del nodo: { $kwh } kWh.
fab-conta_emissao = Emisión = energía × { $fator } kg CO₂e/kWh (alcance { $escopo }, fuente: { $fonte }) = { $kg } kg CO₂e.

## maquinas.rs

maq-titulo = Dar vida al modelo
maq-explica = Informe la potencia de cada máquina (kW) en cada grupo de estado: de la placa, del catálogo o de una medición. El tiempo en cada estado viene de la simulación.
maq-maquina = Máquina
maq-etapa = Etapa
maq-energia = Energía

## fatores.rs

fat-titulo = Factores de emisión
fat-explica = La app no trae números de factor: informe el valor y la fuente de cada uno. Un factor sin fuente aparece marcado en toda pantalla.
fat-unidade_tempo = Unidad de tiempo del modelo:
fat-unidade_tempo_dica = Las horas en cada estado dependen de ella.
fat-segundo = segundo
fat-minuto = minuto
fat-hora = hora
fat-fluxo = Flujo
fat-tipo = Tipo
fat-unidade = Unidad
fat-escopo = Alcance
fat-fator = Factor
fat-fonte = Fuente
fat-ano = Año
fat-didatico = Didáctico
fat-tem_valor = Tiene valor (desmarcado: a definir)
fat-kg_por = kg CO₂e por { $unidade }
fat-sem_fonte = sin fuente
fat-tirar = Quitar
fat-novo_fluxo = Nuevo flujo
fat-acrescentar = Agregar
fat-tipo_energia = energía
fat-tipo_combustivel = combustible
fat-tipo_agua = agua
fat-tipo_material = material
fat-tipo_residuo = residuo
fat-tipo_efluente = efluente
fat-tipo_emissao = emisión directa
fat-tipo_outro = otro

## pendências da conta (utsim-ambiental-core)

pend-setup = { $no }: energía o flujo por setup (necesita el rastro: simule de nuevo).
pend-partida = { $no }: energía o flujo por arranque (necesita el rastro: simule de nuevo).
pend-espera = { $no }: apagado en espera (necesita el rastro: simule de nuevo).
pend-refugo = { $no }: flujo por pieza rechazada (fase AM5).
pend-retrabalho = { $no }: flujo por retrabajo (fase AM5).
pend-distancia = { $no }: flujo por distancia (fase AM5).
pend-volume = { $no }: flujo por volumen (fase AM5).
pend-massa = { $no }: flujo por masa (fase AM5).

## validação (utsim-ambiental-core)

val-objeto_inexistente = El objeto { $no } no existe en el modelo.
val-fluxo_inexistente = { $no }: el flujo { $fluxo } no está en el catálogo.
val-fluxo_de_saida_como_entrada = { $no }: el flujo { $fluxo } es de salida y suma masa a la pieza.
val-negativo = { $no }: { $campo } no puede ser negativo.
val-agua_fracao = { $no }: las fracciones de agua van entre 0 y 1.
val-agua_soma = { $no }: las fracciones de agua suman más que 1.
val-reuso_inexistente = { $no }: el reúso va a { $alvo }, que no existe.
val-parametro_inexistente = { $no }: el parámetro de efluente { $parametro } no está en el catálogo.
val-pico_fora_do_dia = La ventana de punta debe quedar dentro de un día (HH:MM).
val-intervalo_zero = El intervalo de demanda debe ser mayor que cero.
val-massa_sem_tipo = { $no }: el flujo suma o quita masa, pero los tipos de ítem no tienen masa.
val-sem_fluxo_eletrico = El catálogo no tiene flujo de energía: la potencia no se convierte en emisión.
val-fluxo_repetido = El flujo { $fluxo } aparece dos veces en el catálogo.
val-escopo_invalido = El factor de { $fluxo } tiene alcance fuera de 1, 2 o 3.

## AM3: curva de carga, tocar o dia, detalhes, relatório

un-kw = kW
un-kwh_evento = kWh/evento
fab-curva_de_carga = Curva de carga
fab-tocar_o_dia = ▶ Reproducir el día
fab-pausar = ⏸ Pausar
fab-parar = Detener
fab-janela = Ventana de la cuenta: { $de } a { $ate }
fab-janela_inteira = Ventana entera
fab-arraste_dica = Arrastre en la curva para calcular solo un tramo; haga clic para ver un instante.
fab-sem_curva = Sin curva: simule de nuevo para grabar el rastro.
fab-no_pico = Punta a las { $hora }: { $kw } kW. Encendidas juntas: { $nos }.
fab-agora = { $hora }: { $kw } kW. Encendidas: { $nos }.
fab-pico = Punta de { $kw } kW a las { $hora }.
fab-setups = setups
fab-partidas = arranques
fab-estado_standby = apagada en espera
maq-detalhes = Detalles
maq-detalhes_de = Detalles: { $no }
maq-eventos = Energía por evento
maq-kwh_peca = Por pieza que entra
maq-kwh_setup = Por setup
maq-kwh_partida = Por arranque (encender tras una parada)
maq-desligar_em_espera = Apagar cuando espera mucho
maq-depois_de = Después de
maq-unidades_do_modelo = unidades de tiempo del modelo esperando
maq-espera_aproximacao = Aproximación: la simulación no retrasa la máquina para volver a encender. El tiempo de espera tras el límite cuenta con la potencia fuera de turno, y cada vuelta cuenta un arranque.
rel-tela = Informe ambiental
rel-explica = Genera el informe del modelo, con el aviso de uso didáctico y la lista de factores con sus fuentes. Queda junto al modelo.
rel-gerar_html = Generar HTML
rel-gerar_pdf = Generar PDF
rel-gravado = Informe guardado en { $arquivo }
rel-mostrar = Mostrar en la carpeta
rel-conteudo = Qué trae el informe:
rel-titulo = Informe ambiental: { $modelo }
rel-origem = Simulado: { $n } réplicas, semilla { $semente }, ventana de { $de } a { $ate }.
rel-aviso_titulo = Uso didáctico
rel-aviso = Números simulados, para enseñanza: no son medición, inventario oficial ni declaración de conformidad. Verifique los factores y las fuentes antes de usar cualquier número.
rel-resumo = Resumen
rel-por_maquina = Por máquina
rel-energia_por_grupo = Energía por grupo de estado (kWh)
rel-potencia_media = Potencia media de la fábrica (kW), primera réplica
rel-fatores = Factores y fuentes
rel-a_definir = a definir
rel-sem_fatores = Ningún factor registrado: ningún número de emisión en este informe.

## AM4: demanda, pico e tarifa

fat-tarifa = Tarifa de energía
fat-tarifa_explica = Horario punta, demanda contratada y precios: todos informados por usted, sin valores de fábrica. Recargos, impuestos y energía reactiva quedan fuera.
fat-usar_tarifa = Usar tarifa en este modelo
fat-horario_pico = Horario punta
fat-dias_uteis = solo días hábiles (el día 1 es lunes)
fat-intervalo_demanda = Intervalo de demanda
fat-contratada = Demanda contratada (kW)
fat-preco_pico = Precio del kWh en punta
fat-preco_fora = Precio del kWh fuera de punta
fat-preco_kw = Precio del kW de demanda
fat-preco_ultra = Precio del kW de excedente
fat-fonte_tarifa = Fuente de la tarifa
fab-demanda = Demanda y tarifa
fab-demanda_max = Demanda máxima
fab-demanda_pico = Demanda en punta
fab-sem_pico_na_janela = la ventana no pasa por la punta
fab-energia_pico = Energía en punta
fab-fator_de_carga = Factor de carga
fab-conta_energia = Cuenta de energía simulada
fab-conta_explica = Cuenta de la ventana simulada: energía en punta y fuera, la demanda (la contratada o la medida, la mayor) y el excedente, como si la ventana fuera un período de facturación.
fab-falta_preco = falta precio
fab-falta_preco_dica = Sin cuenta: faltan { $precos }. Infórmelos en la pantalla Factores.
fab-contratada = contratada { $kw } kW
rel-parcela_pico = Parte de la energía en punta
rel-parcela_fora = Parte de la energía fuera de punta
rel-parcela_demanda = Parte de la demanda
rel-parcela_ultra = Parte del excedente
rel-tarifa_fonte = Punta de { $de } a { $ate }, intervalo de { $min } minutos. Fuente de la tarifa: { $fonte }.

## AM5: massa, água, resíduos e efluentes

un-m3 = m³
un-m3_peca = m³/pieza
un-kg = kg
un-kg_peca = kg/pieza
fab-agua = Agua
fab-residuos = Residuos
fab-efluente = Efluente
fab-residuo_processo = Residuo de proceso
fab-sucata = Chatarra
fab-agua_e_efluentes = Agua y efluentes
fab-captacao = Captación
fab-consumo = Consumo (captación − efluente)
fab-reuso = Reúso
fab-tratamento = Energía del tratamiento
fab-lodo = Lodo del tratamiento
fab-carga = { $parametro }: { $antes } kg antes, { $depois } kg después del tratamiento; mezcla { $mgl } mg/L
fab-limite = Límite registrado: { $limite } (fuente: { $fonte }). La app compara, no afirma conformidad.
fab-desvio_aterro = Desviado de vertedero
fab-sem_destino = sin destino
fab-materiais = Materiales
fab-entrada_massa = Masa que entró
fab-massa_boa = Masa de producto bueno
fab-rendimento = Rendimiento de material
fab-sucata_pecas = { $kg } kg ({ $pecas } piezas)
fab-balanco = Balance de masa
fab-balanco_fecha = cierra
fab-balanco_erro = no cierra: { $pct } %
fab-mochila_kwh = Mochila del rechazo (energía)
fab-mochila_co2e = Mochila del rechazo (CO₂e)
fab-mochila_explica = La mochila es lo que las piezas rechazadas llevaron consigo: la energía media por pieza de cada máquina por la que pasaron y los flujos por pieza. Rechazar al final de la línea cuesta más que al principio.
fab-custo_fluxos = Costo de flujos y residuos
maq-fluxos = Flujos de la máquina
maq-fluxos_explica = Cada línea: cantidad × inductor (por pieza, por hora en un estado, por setup...). Agua en L o m³; residuo en kg.
maq-por = { $unidade } { $dir }
maq-dir_peca = por pieza que entra
maq-dir_peca_boa = por pieza que sale
maq-dir_refugo = por pieza rechazada aquí
maq-dir_setup = por setup
maq-dir_partida = por arranque
maq-dir_massa = por kg procesado
maq-dir_tempo_produzindo = por hora produciendo
maq-dir_tempo_ligada = por hora encendida
maq-dir_outro = otro (del archivo)
maq-soma_massa = suma a la pieza
maq-tira_massa = quita de la pieza
maq-novo_fluxo = + Flujo
maq-sem_fluxos = Registre flujos (agua, residuo, material) en la pantalla Factores.
maq-balanco_agua = Balance de agua de esta máquina
maq-incorporada = Fracción incorporada al producto
maq-evaporada = Fracción evaporada o perdida
maq-reuso_para = Reúso del efluente en
maq-reuso_fracao = Fracción del efluente reusada
maq-efluente = Efluente de esta máquina
maq-destino_efluente = Destino del efluente
maq-sem_parametros = Registre los parámetros del efluente en la pantalla Factores.
fat-preco = Precio
fat-destino = Destino
fat-massa = Masa de las piezas
fat-massa_explica = Con la masa, la app hace el balance de masa, la chatarra en kg y el rendimiento de material.
fat-qualquer_peca = Pieza sin tipo (todas)
fat-destinos = Destinos de residuo
fat-desviado = Desviado de vertedero
fat-fator_kg = Factor (kg CO₂e/kg)
fat-custo_kg = Costo por kg
fat-novo_destino = Nuevo destino
fat-destino_sucata = Destino de la chatarra:
fat-efluente = Efluente
fat-efluente_explica = Parámetros (con el límite y la fuente que registre) y destinos del efluente, con la remoción de cada parámetro, energía, lodo y costo por m³.
fat-parametros = Parámetros
fat-parametro = Parámetro
fat-limite = Límite
fat-fonte_limite = Fuente del límite
fat-novo_parametro = Nuevo parámetro
fat-destinos_efluente = Destinos del efluente
fat-kwh_m3 = kWh/m³
fat-lodo_m3 = lodo kg/m³
fat-custo_m3 = costo/m³
fat-remocao = remueve { $parametro }
fat-novo_destino_efluente = Nuevo destino de efluente
rel-cargas = Cargas del efluente
rel-antes_kg = Antes (kg)
rel-depois_kg = Después (kg)
rel-concentracao = Mezcla (mg/L)
rel-sem_conformidade = La comparación con el límite no es declaración de conformidad legal.
rel-desvio = Desviado de vertedero: { $pct } %.
val-destino_inexistente = { $no }: el destino { $destino } no está en el catálogo.
val-unidade_agua = El flujo de agua { $fluxo } debe estar en L o m3.

## AM6: Sankey

snk-titulo = Flujos
snk-massa = Masa
snk-energia = Energía
snk-agua = Agua
snk-carbono = Carbono
snk-por_etapa = Agrupar por etapa
snk-juntar = Ocultar flujos menores que
snk-sem_massa = Sin masa: informe la masa de las piezas en la pantalla Factores y simule de nuevo.
snk-vazio = Ninguna máquina tiene lo que muestra este modo.
snk-total = Total: { $valor } { $unidade }
snk-exportar_svg = Exportar SVG
snk-exportar_csv = Exportar inventario (CSV)
snk-dica = Haga clic en una franja para ver su camino; doble clic en una etapa la abre.
snk-balanco_dica = { $no }: entran { $entra } y salen { $sai }. Verifique si registró los materiales sumados a la pieza y los residuos quitados de ella en esta máquina.
snk-faltam = faltan { $pct } %
snk-sobram = sobran { $pct } %
snk-faixa = { $de } → { $para }: { $valor } { $unidade } ({ $pct } % del total)
snk-titulo_svg = { $modelo }: flujos de { $modo }
snk-agrega_valor = energía que agrega valor
snk-preparando = energía preparando
snk-sem_valor = energía sin valor (esperando, parada, fuera de turno)
snk-tratamento = tratamiento de efluente
snk-captacao = captación de agua
snk-evaporada = evaporada o perdida
snk-incorporada = incorporada al producto
snk-efluente_sem_tratamento = efluente sin tratamiento registrado
snk-produto = producto
snk-sucata_e_lodo = chatarra, lodo y tratamiento

## AM7: pegada por produto

prd-titulo = Huella por producto
prd-aviso = Estudio simplificado y didáctico: no es un análisis de ciclo de vida completo ni una declaración ambiental de producto.
prd-sem_rastro = La huella por producto necesita el rastro de la simulación: haga clic en Simular de nuevo en la Fábrica.
prd-sem_pecas = Ninguna pieza pasó por máquinas con bloque ambiental en la ventana.
prd-produto = Producto:
prd-sem_tipo = pieza (sin tipo)
prd-unidade = Unidad funcional:
prd-peca_boa = pieza buena
prd-kg_produto = kg de producto
prd-regra = Asignación:
prd-regra_tempo = por el tiempo en la máquina
prd-regra_pecas = por el número de piezas
prd-regra_massa = por la masa
prd-regra_estrutura = espera como estructura
prd-comparar = Comparar con:
prd-materias_primas = Materias primas
prd-transporte = Transporte
prd-fabricacao = Fabricación
prd-distribuicao = Distribución
prd-uso = Uso
prd-fim_de_vida = Fin de vida
prd-portao_a_portao = puerta a puerta (solo la fabricación)
prd-berco_ao_portao = cuna a puerta (materiales y fabricación)
prd-fronteira = Frontera: { $fronteira }. Haga clic en Materias primas para incluirla o quitarla.
prd-unidades = Unidades en la ventana
prd-co2_por = CO₂e por { $unidade }
prd-energia_por = Energía por { $unidade }
prd-agua_por = Agua por { $unidade }
prd-mochila_por = Energía en piezas rechazadas, por { $unidade }
prd-taxa_refugo = Tasa de rechazo:
prd-voltar_simulada = simulada
prd-com_refugo = Con { $taxa } % de rechazo (simulado: { $simulada } %), la huella sería { $valor } kg CO₂e por unidad.
prd-com_refugo_kwh = Con { $taxa } % de rechazo (simulado: { $simulada } %), la energía sería { $valor } kWh por unidad.
prd-contribuicao = Contribución por máquina
prd-kwh_por_unidade = kWh por unidad
prd-co2_por_unidade = kg CO₂e por unidad (fabricación)
prd-estrutura = Asignación { $regra }: productos { $produtos } kWh + estructura { $estrutura } kWh = fábrica { $fabrica } kWh. Cambiar la regla cambia los productos, nunca el total.
prd-relatorio = Informe del estudio
prd-rel_titulo = Estudio de huella: { $produto }
prd-rel_objetivo = Objetivo
prd-rel_objetivo_texto = Estimar, para enseñanza, la energía, el agua y el CO₂ equivalente de { $produto } por { $unidade } en la fábrica simulada.
prd-rel_fronteira = Unidad funcional y frontera
prd-rel_fora = Quedó fuera de la frontera:
prd-rel_inventario = Inventario por unidad funcional
prd-rel_impacto = Impacto: cambio climático
prd-rel_impacto_texto = { $valor } kg CO₂e por { $unidade }.
prd-rel_interpretacao = Interpretación
prd-rel_regras = Efecto de la regla de asignación
prd-rel_regras_texto = La regla de asignación es una elección: cambia la huella de cada producto, pero no el total de la fábrica.
prd-rel_limitacoes = Limitaciones
prd-lim_1 = Los números vienen de una simulación, no de una medición.
prd-lim_2 = Solo se evalúa el cambio climático; otras categorías de impacto quedan fuera.
prd-lim_3 = Los factores de emisión son los del catálogo del modelo, con las fuentes listadas abajo.

## AM8: cenários

cen-titulo = Escenarios
cen-explica = Cada escenario cambia algo y se compara con el Base, réplica a réplica.
cen-base = Base (el modelo tal como está)
cen-novo = + Nuevo escenario
cen-nome_padrao = Escenario { $n }
cen-parametros = Para la comparación
cen-horas_por_ano = Horas de operación por año
cen-preco_carbono = Precio del carbono (por kg CO₂e)
cen-salvar = Guardar escenarios
cen-apagar = Borrar escenario
cen-alavancas = Palancas
cen-nova_alavanca = + Palanca
cen-alv_potencia = Potencia de una máquina
cen-alv_desligamento = Apagar cuando espera
cen-alv_fator = Factor de emisión
cen-alv_destino = Destino de un residuo
cen-alv_reuso = Reúso de agua
cen-alv_remocao = Eficiencia del tratamiento
cen-alv_preco = Precio de la energía
cen-alv_contratada = Demanda contratada
cen-alv_pico = Ventana de punta
cen-alv_modelo = Cambio en el modelo (lote, máquinas, tiempo...)
cen-kg_por_unidade = kg CO₂e por unidad
cen-precisa_simular = necesita simular
cen-simular = Simular este escenario
cen-simulado = Escenario simulado con los mismos números aleatorios del Base.
cen-investimento = Inversión:
cen-vida = Vida útil (años):
cen-comparacao = Costo e impacto
cen-impacto = Impacto:
cen-cenario = Escenario
cen-custo = Costo
cen-diferenca = Diferencia con el Base
cen-melhor = mejor
cen-pior = peor
cen-nao_da = no se puede afirmar
cen-sem_custo = Sin costo: informe la tarifa o los precios en la pantalla Factores.
cen-sem_impacto = Sin este impacto: falta un factor o no hay el flujo en el modelo.
cen-dispersao = Costo × impacto
cen-fronteira = Los puntos unidos son los que no pierden en los dos criterios a la vez.
cen-investimentos = Inversión
cen-sem_horas = Informe las horas de operación por año para llevar la ventana simulada a un año.
cen-retorno = Retorno simple
cen-por_tonelada = Costo por tonelada de CO₂e evitada
cen-nao_paga = no se paga
cen-anos = { $anos } años

## AM9: metas

met-titulo = Metas
met-explica = Cada meta muestra la probabilidad de cumplirse en las réplicas: verde (80 % o más), amarillo, rojo (20 % o menos).
met-nova = + Nueva meta
met-orcamento = Presupuesto de carbono del período:
met-orcamento_usado = La ventana simulada usó { $usado } kg CO₂e ({ $pct } % del presupuesto).
met-selos = Sellos de logro
met-sem_selos = ninguno aún
met-selo_primeira_meta = primera meta cumplida
met-selo_balanco_fechado = balance de masa cerrado
met-selo_metade_desviada = más de la mitad del residuo fuera del vertedero
met-abrir = Abrir
met-sem_valor = Sin valor: el indicador falta en este alcance (factor, tarifa o flujo).
met-prazo = Plazo: { $prazo }
met-real = Real: { $valor } en { $periodo } ({ $origem })
met-detalhe = Detalle de la meta
met-apagar = Borrar meta
met-fechar = Cerrar
met-indicador = Indicador
met-alcance = Alcance
met-base = Base
met-linha_de_base = Línea de base
met-usar_atual = usar el valor actual
met-alvo = Objetivo
met-prazo_campo = Plazo
met-cenarios = Escenarios: qué hace cada uno con la meta (marque para vincularlo como acción)
met-sem_cenarios = Cree escenarios en la pantalla Escenarios para ver qué acerca la meta.
met-cenario_efeito = { $valor } (efecto { $efeito })
met-cascata = De la línea de base al objetivo
met-cascata_explica = Los efectos de los escenarios vinculados se suman como si fueran independientes: una aproximación.
met-leituras = Dato real
met-leituras_explica = Lecturas de medidor, facturas o pesajes, guardados junto al modelo con origen Real.
met-nova_leitura = + Lectura
met-origem_padrao = medidor
met-probabilidade = Cumplida en { $pct } % de las réplicas (95 %: { $de } a { $ate } %)
met-sem_alvo = Sin objetivo: infórmelo para ver el semáforo.
met-base_curta = base
met-alvo_curto = objetivo
met-simulado_curto = simulado
met-fabrica = Fábrica
met-etapa = Etapa { $etapa }
met-maquina = Máquina { $no }
met-base_total = total
met-base_peca = por pieza buena
met-base_hora = por hora
met-ind_co2e_kg = CO₂ equivalente (kg)
met-ind_energia_kwh = Energía (kWh)
met-ind_energia_sem_valor_kwh = Energía sin valor (kWh)
met-ind_agua_m3 = Captación de agua (m³)
met-ind_efluente_m3 = Efluente (m³)
met-ind_custo = Costo de flujos y residuos
met-ind_custo_energia = Cuenta de energía
met-ind_demanda_max_kw = Demanda máxima (kW)
met-ind_desvio_aterro_pct = Residuo desviado de vertedero (%)
met-ind_rendimento_material_pct = Rendimiento de material (%)
met-ind_reuso_pct = Reúso de agua (%)

## AM11: conversa com LSS e Estatística

cen-testar = Probar la reducción
cen-enviar_estatistica = Enviar a Estadística
cen-levar_lss = Llevar al LSS
cen-teste_reduz = { $metrica }: reduce { $dif } (95 %: { $de } a { $ate }), p = { $p } ({ $origem })
cen-teste_nao = { $metrica }: la reducción no se confirma ({ $dif }, 95 %: { $de } a { $ate }), p = { $p } ({ $origem })
cen-enviado_estatistica = Pedido y CSV guardados en { $pasta }; Estadística abrió con los datos.
cen-enviado_lss = Oportunidad guardada en { $pasta }; el LSS abrió. Registra la idea cuando atienda el pedido nova_oportunidade.
cen-pedido_gravado = Pedido guardado en { $pasta } (no se encontró la app: instálela con el launcher).
cen-resposta = Respuesta recibida para { $cenario }.
cen-origem_estatistica = UTSim Estatística

## AM10: bloco no modelo

fab-bloco_no_modelo = Bloque ambiental dentro del modelo (también se edita en la pestaña Ambiental de UTSim)
fab-bloco_ao_lado = Bloque ambiental en el archivo junto al modelo
fab-migrar = Mover dentro del modelo
fab-migrar_explica = Los bloques de { $nos } máquinas, el catálogo y { $massas } masas de tipo de ítem pasan dentro del modelo (el formato no cambia). UTSim pasa a mostrar la pestaña Ambiental con la potencia. El archivo al lado queda guardado como .bak.
fab-migrar_confirmar = Mover ahora
fab-cancelar = Cancelar
fab-migrado = Bloque ambiental grabado dentro de { $arquivo }.
