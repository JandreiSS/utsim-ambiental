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
app-fase_am6 = Llega en la fase AM6 del plan.
app-fase_am7 = Llega en la fase AM7 del plan.
app-fase_am8 = Llega en la fase AM8 del plan.
app-fase_am9 = Llega en la fase AM9 del plan.
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
