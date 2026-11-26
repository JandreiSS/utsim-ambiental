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
app-fase_am6 = Arriva nella fase AM6 del piano.
app-fase_am7 = Arriva nella fase AM7 del piano.
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
