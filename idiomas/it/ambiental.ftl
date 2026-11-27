# UTSim Ambiental (it). Mesmas chaves nas cinco línguas.

## app.rs

app-inicio = Home
app-grupo_ver = Vedere
app-grupo_dados = Dati
app-grupo_decidir = Decidere
app-fabrica = Fabbrica
app-fluxos = Flussi
app-produto = Prodotto
app-maquinas = Macchine
app-fatores = Fattori
app-cenarios = Scenari
app-metas = Obiettivi
app-relatorios = Report
app-abra_um_modelo = Apri un modello nella Home.
app-fase_am8 = Arriva nella fase AM8 del piano.
app-fase_am9 = Arriva nella fase AM9 del piano.
app-nao_salvo = modifiche non salvate
app-subtitulo = Energia e impatto ambientale
app-recentes = Modelli recenti
app-acao_abrir = Apri un modello UTSim
app-acao_abrir_desc = Scegli un file .ron: l'app simula e accende la fabbrica.
app-acao_exemplo = Apri l'esempio
app-acao_exemplo_desc = Una linea di verniciatura con la potenza delle macchine già inserita.
app-onde_explicacao = Il blocco ambientale sta in un file accanto al modello, <modello>.ambiental.ron. Il modello non cambia.
app-aviso_didatico = Numeri simulati, per la didattica: non sono misure né inventario ufficiale.
app-aberto = Modello aperto: { $modelo }. Simulazione…
app-simulado = Simulazione pronta.
app-erro_abrir = Impossibile aprire: { $e }
app-erro_simular = La simulazione è fallita: { $e }
app-erro_config = Impossibile salvare la configurazione: { $e }
app-salvar = Salva
app-salvar_dica = Scrive il blocco ambientale accanto al modello.
app-salvo = Salvato in { $arquivo }
app-erro_salvar = Impossibile salvare: { $e }

## unidades

un-kwh = kWh
un-kwh_peca = kWh/pezzo
un-kg_co2e = kg CO₂e
un-kg_co2e_peca = kg CO₂e/pezzo

## grupos de estado

grupo-produzindo = In produzione
grupo-preparando = In preparazione
grupo-esperando = In attesa
grupo-parada = Ferma
grupo-fora_de_turno = Fuori turno

## vilões

vilao-esperando = { $no } spende il { $pct } % dell'energia in attesa di pezzi ({ $pf } % della fabbrica senza valore).
vilao-parada = { $no } spende il { $pct } % dell'energia da ferma ({ $pf } % della fabbrica senza valore).
vilao-fora_de_turno = { $no } spende il { $pct } % dell'energia fuori turno ({ $pf } % della fabbrica senza valore).
vilao-preparando = { $no } spende il { $pct } % dell'energia in preparazione ({ $pf } % della fabbrica senza valore).
vilao-produzindo = { $no } spende il { $pct } % dell'energia in produzione ({ $pf } % della fabbrica senza valore).

## fabrica.rs

fab-simulando = Simulazione…
fab-origem = Simulato · { $n } repliche · seme { $semente }
fab-simular_de_novo = Simula di nuovo
fab-energia = Energia
fab-sem_valor = Energia senza valore
fab-co2e = CO₂ equivalente
fab-total = Totale
fab-por_peca = Per pezzo
fab-por_peca_boa = Per pezzo buono
fab-pecas_boas = Pezzi buoni
fab-esperando_simulacao = In attesa della simulazione…
fab-a_fabrica = La fabbrica
fab-falta_fator = manca il fattore
fab-falta_fator_dica = Nessun fattore di emissione per: { $fluxos }. Inseriscilo nella schermata Fattori.
fab-sem_fonte = senza fonte: { $fluxo }
fab-didatico = didattico: { $fluxo }
fab-clique_numa_maquina = Fai clic su una macchina della mappa per vederne e cambiarne la potenza.
fab-viloes = I tre colpevoli
fab-sem_viloes = Nessuna macchina spende energia senza aggiungere valore.
fab-ainda_nao_contado = Non ancora conteggiato in questa versione
fab-informe_potencia = Inserisci la potenza
fab-de_onde_veio = Da dove viene?
fab-de_onde_veio_dica = Apre il calcolo di questo numero.
fab-de_onde_veio_titulo = Da dove viene: { $no }
fab-potencia_kw = Potenza in ogni gruppo di stati
fab-antes_e_depois = Prima e dopo
fab-base_explica = Salva la situazione attuale come base, cambia una potenza e guarda la differenza.
fab-guardar_base = Salva come base
fab-nova_base = Nuova base
fab-voltar_base = Torna alla base
fab-pareado_explica = Differenza appaiata: stesso seme e stesse repliche, con intervallo al 95 %.
fab-formula_energia = E = Σ P(stato) × T(stato) + e(pezzo) × pezzi
fab-media_de = Media di { $n } repliche.
fab-col_origem = Origine
fab-col_grupo = Gruppo
fab-col_direcionador = Driver
fab-col_coeficiente = Coefficiente
fab-col_quantidade = Quantità
fab-estado = stato { $estado }
fab-pecas_que_entraram = pezzi entrati
fab-total_energia = Energia del nodo: { $kwh } kWh.
fab-conta_emissao = Emissione = energia × { $fator } kg CO₂e/kWh (scope { $escopo }, fonte: { $fonte }) = { $kg } kg CO₂e.

## maquinas.rs

maq-titulo = Dare vita al modello
maq-explica = Inserisci la potenza di ogni macchina (kW) in ogni gruppo di stati: dalla targa, dal catalogo o da una misura. Il tempo in ogni stato viene dalla simulazione.
maq-maquina = Macchina
maq-etapa = Fase
maq-energia = Energia

## fatores.rs

fat-titulo = Fattori di emissione
fat-explica = L'app non fornisce valori dei fattori: inserisci il valore e la fonte di ciascuno. Un fattore senza fonte è segnalato in ogni schermata.
fat-unidade_tempo = Unità di tempo del modello:
fat-unidade_tempo_dica = Le ore in ogni stato dipendono da essa.
fat-segundo = secondo
fat-minuto = minuto
fat-hora = ora
fat-fluxo = Flusso
fat-tipo = Tipo
fat-unidade = Unità
fat-escopo = Scope
fat-fator = Fattore
fat-fonte = Fonte
fat-ano = Anno
fat-didatico = Didattico
fat-tem_valor = Ha un valore (deselezionato: da definire)
fat-kg_por = kg CO₂e per { $unidade }
fat-sem_fonte = senza fonte
fat-tirar = Rimuovi
fat-novo_fluxo = Nuovo flusso
fat-acrescentar = Aggiungi
fat-tipo_energia = energia
fat-tipo_combustivel = combustibile
fat-tipo_agua = acqua
fat-tipo_material = materiale
fat-tipo_residuo = rifiuto
fat-tipo_efluente = effluente
fat-tipo_emissao = emissione diretta
fat-tipo_outro = altro

## pendências da conta (utsim-ambiental-core)

pend-setup = { $no }: energia o flusso per setup (serve la traccia: simula di nuovo).
pend-partida = { $no }: energia o flusso per avvio (serve la traccia: simula di nuovo).
pend-espera = { $no }: spegnimento in attesa (serve la traccia: simula di nuovo).
pend-refugo = { $no }: flusso per pezzo scartato (fase AM5).
pend-retrabalho = { $no }: flusso per rilavorazione (fase AM5).
pend-distancia = { $no }: flusso per distanza (fase AM5).
pend-volume = { $no }: flusso per volume (fase AM5).
pend-massa = { $no }: flusso per massa (fase AM5).

## validação (utsim-ambiental-core)

val-objeto_inexistente = L'oggetto { $no } non esiste nel modello.
val-fluxo_inexistente = { $no }: il flusso { $fluxo } non è nel catalogo.
val-fluxo_de_saida_como_entrada = { $no }: il flusso { $fluxo } è in uscita ma aggiunge massa al pezzo.
val-negativo = { $no }: { $campo } non può essere negativo.
val-agua_fracao = { $no }: le frazioni d'acqua sono tra 0 e 1.
val-agua_soma = { $no }: le frazioni d'acqua superano 1.
val-reuso_inexistente = { $no }: il riuso va a { $alvo }, che non esiste.
val-parametro_inexistente = { $no }: il parametro di effluente { $parametro } non è nel catalogo.
val-pico_fora_do_dia = La fascia di punta deve stare in un giorno (HH:MM).
val-intervalo_zero = L'intervallo di domanda deve essere maggiore di zero.
val-massa_sem_tipo = { $no }: il flusso aggiunge o toglie massa, ma i tipi di articolo non hanno massa.
val-sem_fluxo_eletrico = Il catalogo non ha un flusso di energia: la potenza non diventa emissione.
val-fluxo_repetido = Il flusso { $fluxo } compare due volte nel catalogo.
val-escopo_invalido = Il fattore di { $fluxo } ha uno scope diverso da 1, 2 o 3.

## AM3: curva de carga, tocar o dia, detalhes, relatório

un-kw = kW
un-kwh_evento = kWh/evento
fab-curva_de_carga = Curva di carico
fab-tocar_o_dia = ▶ Riproduci la giornata
fab-pausar = ⏸ Pausa
fab-parar = Ferma
fab-janela = Finestra del calcolo: da { $de } a { $ate }
fab-janela_inteira = Finestra intera
fab-arraste_dica = Trascina sulla curva per calcolare solo un tratto; fai clic per vedere un istante.
fab-sem_curva = Nessuna curva: simula di nuovo per registrare la traccia.
fab-no_pico = Picco alle { $hora }: { $kw } kW. Accese insieme: { $nos }.
fab-agora = { $hora }: { $kw } kW. Accese: { $nos }.
fab-pico = Picco di { $kw } kW alle { $hora }.
fab-setups = setup
fab-partidas = avvii
fab-estado_standby = spenta in attesa
maq-detalhes = Dettagli
maq-detalhes_de = Dettagli: { $no }
maq-eventos = Energia per evento
maq-kwh_peca = Per pezzo in entrata
maq-kwh_setup = Per setup
maq-kwh_partida = Per avvio (riaccendere dopo una fermata)
maq-desligar_em_espera = Spegnere dopo una lunga attesa
maq-depois_de = Dopo
maq-unidades_do_modelo = unità di tempo del modello in attesa
maq-espera_aproximacao = Approssimazione: la simulazione non ritarda la macchina per riaccenderla. L'attesa oltre il limite conta alla potenza fuori turno, e ogni ritorno conta un avvio.
rel-tela = Report ambientale
rel-explica = Crea il report del modello, con l'avviso di uso didattico e l'elenco dei fattori con le fonti. Viene salvato accanto al modello.
rel-gerar_html = Crea HTML
rel-gerar_pdf = Crea PDF
rel-gravado = Report salvato in { $arquivo }
rel-mostrar = Mostra nella cartella
rel-conteudo = Cosa contiene il report:
rel-titulo = Report ambientale: { $modelo }
rel-origem = Simulato: { $n } repliche, seme { $semente }, finestra da { $de } a { $ate }.
rel-aviso_titulo = Uso didattico
rel-aviso = Numeri simulati, per la didattica: non sono misure, inventario ufficiale né dichiarazione di conformità. Verifica fattori e fonti prima di usare qualsiasi numero.
rel-resumo = Sintesi
rel-por_maquina = Per macchina
rel-energia_por_grupo = Energia per gruppo di stati (kWh)
rel-potencia_media = Potenza media della fabbrica (kW), prima replica
rel-fatores = Fattori e fonti
rel-a_definir = da definire
rel-sem_fatores = Nessun fattore registrato: nessun numero di emissione in questo report.

## AM4: demanda, pico e tarifa

fat-tarifa = Tariffa dell'energia
fat-tarifa_explica = Fascia di punta, potenza impegnata e prezzi: tutti inseriti da te, senza valori predefiniti. Maggiorazioni, imposte ed energia reattiva sono esclusi.
fat-usar_tarifa = Usa una tariffa in questo modello
fat-horario_pico = Fascia di punta
fat-dias_uteis = solo feriali (il 1º giorno è lunedì)
fat-intervalo_demanda = Intervallo di domanda
fat-contratada = Potenza impegnata (kW)
fat-preco_pico = Prezzo del kWh in punta
fat-preco_fora = Prezzo del kWh fuori punta
fat-preco_kw = Prezzo del kW di potenza
fat-preco_ultra = Prezzo del kW di superamento
fat-fonte_tarifa = Fonte della tariffa
fab-demanda = Potenza e tariffa
fab-demanda_max = Potenza massima
fab-demanda_pico = Potenza in punta
fab-sem_pico_na_janela = la finestra non attraversa la punta
fab-energia_pico = Energia in punta
fab-fator_de_carga = Fattore di carico
fab-conta_energia = Bolletta energetica simulata
fab-conta_explica = Bolletta della finestra simulata: energia in punta e fuori punta, la potenza (impegnata o misurata, la maggiore) e il superamento, come se la finestra fosse un periodo di fatturazione.
fab-falta_preco = manca il prezzo
fab-falta_preco_dica = Nessuna bolletta: mancano { $precos }. Inseriscili nella schermata Fattori.
fab-contratada = impegnata { $kw } kW
rel-parcela_pico = Quota dell'energia in punta
rel-parcela_fora = Quota dell'energia fuori punta
rel-parcela_demanda = Quota della potenza
rel-parcela_ultra = Quota del superamento
rel-tarifa_fonte = Punta da { $de } a { $ate }, intervallo di { $min } minuti. Fonte della tariffa: { $fonte }.

## AM5: massa, água, resíduos e efluentes

un-m3 = m³
un-m3_peca = m³/pezzo
un-kg = kg
un-kg_peca = kg/pezzo
fab-agua = Acqua
fab-residuos = Rifiuti
fab-efluente = Effluente
fab-residuo_processo = Rifiuto di processo
fab-sucata = Scarto
fab-agua_e_efluentes = Acqua ed effluenti
fab-captacao = Prelievo
fab-consumo = Consumo (prelievo − effluente)
fab-reuso = Riuso
fab-tratamento = Energia del trattamento
fab-lodo = Fanghi del trattamento
fab-carga = { $parametro }: { $antes } kg prima, { $depois } kg dopo il trattamento; miscela { $mgl } mg/L
fab-limite = Limite registrato: { $limite } (fonte: { $fonte }). L'app confronta, non afferma la conformità.
fab-desvio_aterro = Deviato dalla discarica
fab-sem_destino = senza destinazione
fab-materiais = Materiali
fab-entrada_massa = Massa entrata
fab-massa_boa = Massa di prodotto buono
fab-rendimento = Resa del materiale
fab-sucata_pecas = { $kg } kg ({ $pecas } pezzi)
fab-balanco = Bilancio di massa
fab-balanco_fecha = chiude
fab-balanco_erro = non chiude: { $pct } %
fab-mochila_kwh = Zaino dello scarto (energia)
fab-mochila_co2e = Zaino dello scarto (CO₂e)
fab-mochila_explica = Lo zaino è ciò che i pezzi scartati hanno portato con sé: l'energia media per pezzo di ogni macchina attraversata e i flussi per pezzo. Scartare a fine linea costa più che all'inizio.
fab-custo_fluxos = Costo di flussi e rifiuti
maq-fluxos = Flussi della macchina
maq-fluxos_explica = Ogni riga: quantità × driver (per pezzo, per ora in uno stato, per setup...). Acqua in L o m³; rifiuto in kg.
maq-por = { $unidade } { $dir }
maq-dir_peca = per pezzo in entrata
maq-dir_peca_boa = per pezzo in uscita
maq-dir_refugo = per pezzo scartato qui
maq-dir_setup = per setup
maq-dir_partida = per avvio
maq-dir_massa = per kg lavorato
maq-dir_tempo_produzindo = per ora in produzione
maq-dir_tempo_ligada = per ora accesa
maq-dir_outro = altro (dal file)
maq-soma_massa = aggiunge al pezzo
maq-tira_massa = toglie dal pezzo
maq-novo_fluxo = + Flusso
maq-sem_fluxos = Registra i flussi (acqua, rifiuto, materiale) nella schermata Fattori.
maq-balanco_agua = Bilancio idrico di questa macchina
maq-incorporada = Frazione incorporata nel prodotto
maq-evaporada = Frazione evaporata o persa
maq-reuso_para = Riuso dell'effluente in
maq-reuso_fracao = Frazione di effluente riusata
maq-efluente = Effluente di questa macchina
maq-destino_efluente = Destinazione dell'effluente
maq-sem_parametros = Registra i parametri dell'effluente nella schermata Fattori.
fat-preco = Prezzo
fat-destino = Destinazione
fat-massa = Massa dei pezzi
fat-massa_explica = Con la massa, l'app fa il bilancio di massa, lo scarto in kg e la resa del materiale.
fat-qualquer_peca = Pezzo senza tipo (tutti)
fat-destinos = Destinazioni dei rifiuti
fat-desviado = Deviato dalla discarica
fat-fator_kg = Fattore (kg CO₂e/kg)
fat-custo_kg = Costo per kg
fat-novo_destino = Nuova destinazione
fat-destino_sucata = Destinazione dello scarto:
fat-efluente = Effluente
fat-efluente_explica = Parametri (con il limite e la fonte che registri) e destinazioni dell'effluente, con la rimozione di ogni parametro, energia, fanghi e costo per m³.
fat-parametros = Parametri
fat-parametro = Parametro
fat-limite = Limite
fat-fonte_limite = Fonte del limite
fat-novo_parametro = Nuovo parametro
fat-destinos_efluente = Destinazioni dell'effluente
fat-kwh_m3 = kWh/m³
fat-lodo_m3 = fanghi kg/m³
fat-custo_m3 = costo/m³
fat-remocao = rimuove { $parametro }
fat-novo_destino_efluente = Nuova destinazione dell'effluente
rel-cargas = Carichi dell'effluente
rel-antes_kg = Prima (kg)
rel-depois_kg = Dopo (kg)
rel-concentracao = Miscela (mg/L)
rel-sem_conformidade = Il confronto con il limite non è una dichiarazione di conformità legale.
rel-desvio = Deviato dalla discarica: { $pct } %.
val-destino_inexistente = { $no }: la destinazione { $destino } non è nel catalogo.
val-unidade_agua = Il flusso d'acqua { $fluxo } deve essere in L o m3.

## AM6: Sankey

snk-titulo = Flussi
snk-massa = Massa
snk-energia = Energia
snk-agua = Acqua
snk-carbono = Carbonio
snk-por_etapa = Raggruppa per fase
snk-juntar = Nascondi i flussi minori di
snk-sem_massa = Nessuna massa: inserisci la massa dei pezzi nella schermata Fattori e simula di nuovo.
snk-vazio = Nessuna macchina ha ciò che mostra questa modalità.
snk-total = Totale: { $valor } { $unidade }
snk-exportar_svg = Esporta SVG
snk-exportar_csv = Esporta inventario (CSV)
snk-dica = Fai clic su una banda per vederne il percorso; doppio clic su una fase la apre.
snk-balanco_dica = { $no }: entrano { $entra } ed escono { $sai }. Controlla di aver registrato i materiali aggiunti al pezzo e i rifiuti tolti in questa macchina.
snk-faltam = mancano { $pct } %
snk-sobram = avanzano { $pct } %
snk-faixa = { $de } → { $para }: { $valor } { $unidade } ({ $pct } % del totale)
snk-titulo_svg = { $modelo }: flussi di { $modo }
snk-agrega_valor = energia che aggiunge valore
snk-preparando = energia di preparazione
snk-sem_valor = energia senza valore (attesa, ferma, fuori turno)
snk-tratamento = trattamento dell'effluente
snk-captacao = prelievo d'acqua
snk-evaporada = evaporata o persa
snk-incorporada = incorporata nel prodotto
snk-efluente_sem_tratamento = effluente senza trattamento registrato
snk-produto = prodotto
snk-sucata_e_lodo = scarto, fanghi e trattamento

## AM7: pegada por produto

prd-titulo = Impronta per prodotto
prd-aviso = Studio semplificato e didattico: non è una valutazione del ciclo di vita completa né una dichiarazione ambientale di prodotto.
prd-sem_rastro = L'impronta per prodotto richiede la traccia della simulazione: fai clic su Simula di nuovo nella Fabbrica.
prd-sem_pecas = Nessun pezzo è passato per macchine con blocco ambientale nella finestra.
prd-produto = Prodotto:
prd-sem_tipo = pezzo (senza tipo)
prd-unidade = Unità funzionale:
prd-peca_boa = pezzo buono
prd-kg_produto = kg di prodotto
prd-regra = Allocazione:
prd-regra_tempo = per tempo sulla macchina
prd-regra_pecas = per numero di pezzi
prd-regra_massa = per massa
prd-regra_estrutura = attesa come struttura
prd-comparar = Confronta con:
prd-materias_primas = Materie prime
prd-transporte = Trasporto
prd-fabricacao = Produzione
prd-distribuicao = Distribuzione
prd-uso = Uso
prd-fim_de_vida = Fine vita
prd-portao_a_portao = da cancello a cancello (solo produzione)
prd-berco_ao_portao = dalla culla al cancello (materiali e produzione)
prd-fronteira = Confine: { $fronteira }. Fai clic su Materie prime per includerle o toglierle.
prd-unidades = Unità nella finestra
prd-co2_por = CO₂e per { $unidade }
prd-energia_por = Energia per { $unidade }
prd-agua_por = Acqua per { $unidade }
prd-mochila_por = Energia nei pezzi scartati, per { $unidade }
prd-taxa_refugo = Tasso di scarto:
prd-voltar_simulada = simulato
prd-com_refugo = Con { $taxa } % di scarto (simulato: { $simulada } %), l'impronta sarebbe { $valor } kg CO₂e per unità.
prd-com_refugo_kwh = Con { $taxa } % di scarto (simulato: { $simulada } %), l'energia sarebbe { $valor } kWh per unità.
prd-contribuicao = Contributo per macchina
prd-kwh_por_unidade = kWh per unità
prd-co2_por_unidade = kg CO₂e per unità (produzione)
prd-estrutura = Allocazione { $regra }: prodotti { $produtos } kWh + struttura { $estrutura } kWh = fabbrica { $fabrica } kWh. Cambiare la regola cambia i prodotti, mai il totale.
prd-relatorio = Report dello studio
prd-rel_titulo = Studio di impronta: { $produto }
prd-rel_objetivo = Obiettivo
prd-rel_objetivo_texto = Stimare, per la didattica, energia, acqua e CO₂ equivalente di { $produto } per { $unidade } nella fabbrica simulata.
prd-rel_fronteira = Unità funzionale e confine
prd-rel_fora = Fuori dal confine:
prd-rel_inventario = Inventario per unità funzionale
prd-rel_impacto = Impatto: cambiamento climatico
prd-rel_impacto_texto = { $valor } kg CO₂e per { $unidade }.
prd-rel_interpretacao = Interpretazione
prd-rel_regras = Effetto della regola di allocazione
prd-rel_regras_texto = La regola di allocazione è una scelta: cambia l'impronta di ogni prodotto, ma non il totale della fabbrica.
prd-rel_limitacoes = Limitazioni
prd-lim_1 = I numeri vengono da una simulazione, non da una misura.
prd-lim_2 = Si valuta solo il cambiamento climatico; le altre categorie di impatto sono escluse.
prd-lim_3 = I fattori di emissione sono quelli del catalogo del modello, con le fonti elencate sotto.
