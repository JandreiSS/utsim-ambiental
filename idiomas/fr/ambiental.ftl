# UTSim Ambiental (fr). Mesmas chaves nas cinco línguas.

## app.rs

app-inicio = Accueil
app-grupo_ver = Voir
app-grupo_dados = Données
app-grupo_decidir = Décider
app-fabrica = Usine
app-fluxos = Flux
app-produto = Produit
app-maquinas = Machines
app-fatores = Facteurs
app-cenarios = Scénarios
app-metas = Objectifs
app-relatorios = Rapports
app-abra_um_modelo = Ouvrez un modèle dans l'Accueil.
app-fase_am3 = Arrive à la phase AM3 du plan.
app-fase_am6 = Arrive à la phase AM6 du plan.
app-fase_am7 = Arrive à la phase AM7 du plan.
app-fase_am8 = Arrive à la phase AM8 du plan.
app-fase_am9 = Arrive à la phase AM9 du plan.
app-nao_salvo = modifications non enregistrées
app-subtitulo = Énergie et impact environnemental
app-recentes = Modèles récents
app-acao_abrir = Ouvrir un modèle UTSim
app-acao_abrir_desc = Choisissez un fichier .ron : l'app simule et allume l'usine.
app-acao_exemplo = Ouvrir l'exemple
app-acao_exemplo_desc = Une ligne de peinture avec la puissance des machines déjà renseignée.
app-onde_explicacao = Le bloc environnemental est dans un fichier à côté du modèle, <modèle>.ambiental.ron. Le modèle ne change pas.
app-aviso_didatico = Chiffres simulés, pour l'enseignement : ni mesure ni inventaire officiel.
app-aberto = Modèle ouvert : { $modelo }. Simulation…
app-simulado = Simulation terminée.
app-erro_abrir = Impossible d'ouvrir : { $e }
app-erro_simular = La simulation a échoué : { $e }
app-erro_config = Impossible d'enregistrer la configuration : { $e }
app-salvar = Enregistrer
app-salvar_dica = Écrit le bloc environnemental à côté du modèle.
app-salvo = Enregistré dans { $arquivo }
app-erro_salvar = Impossible d'enregistrer : { $e }

## unidades

un-kwh = kWh
un-kwh_peca = kWh/pièce
un-kg_co2e = kg CO₂e
un-kg_co2e_peca = kg CO₂e/pièce

## grupos de estado

grupo-produzindo = En production
grupo-preparando = En préparation
grupo-esperando = En attente
grupo-parada = À l'arrêt
grupo-fora_de_turno = Hors poste

## vilões

vilao-esperando = { $no } dépense { $pct } % de son énergie à attendre des pièces ({ $pf } % de l'usine, sans valeur).
vilao-parada = { $no } dépense { $pct } % de son énergie à l'arrêt ({ $pf } % de l'usine, sans valeur).
vilao-fora_de_turno = { $no } dépense { $pct } % de son énergie hors poste ({ $pf } % de l'usine, sans valeur).
vilao-preparando = { $no } dépense { $pct } % de son énergie en préparation ({ $pf } % de l'usine, sans valeur).
vilao-produzindo = { $no } dépense { $pct } % de son énergie en production ({ $pf } % de l'usine, sans valeur).

## fabrica.rs

fab-simulando = Simulation…
fab-origem = Simulé · { $n } réplications · graine { $semente }
fab-simular_de_novo = Simuler à nouveau
fab-energia = Énergie
fab-sem_valor = Énergie sans valeur
fab-co2e = CO₂ équivalent
fab-total = Total
fab-por_peca = Par pièce
fab-por_peca_boa = Par pièce bonne
fab-pecas_boas = Pièces bonnes
fab-esperando_simulacao = En attente de la simulation…
fab-a_fabrica = L'usine
fab-falta_fator = facteur manquant
fab-falta_fator_dica = Pas de facteur d'émission pour : { $fluxos }. Renseignez-le dans l'écran Facteurs.
fab-sem_fonte = sans source : { $fluxo }
fab-didatico = didactique : { $fluxo }
fab-clique_numa_maquina = Cliquez sur une machine de la carte pour voir et changer sa puissance.
fab-viloes = Les trois coupables
fab-sem_viloes = Aucune machine ne dépense d'énergie sans valeur ajoutée.
fab-ainda_nao_contado = Pas encore compté dans cette version
fab-informe_potencia = Renseignez la puissance
fab-de_onde_veio = D'où ça vient ?
fab-de_onde_veio_dica = Ouvre le calcul de ce nombre.
fab-de_onde_veio_titulo = D'où ça vient : { $no }
fab-potencia_kw = Puissance dans chaque groupe d'états
fab-antes_e_depois = Avant et après
fab-base_explica = Gardez la situation actuelle comme référence, changez une puissance et voyez la différence.
fab-guardar_base = Garder comme référence
fab-nova_base = Nouvelle référence
fab-voltar_base = Revenir à la référence
fab-pareado_explica = Différence appariée : même graine et mêmes réplications, avec un intervalle de 95 %.
fab-formula_energia = E = Σ P(état) × T(état) + e(pièce) × pièces
fab-media_de = Moyenne de { $n } réplications.
fab-col_origem = Origine
fab-col_grupo = Groupe
fab-col_direcionador = Inducteur
fab-col_coeficiente = Coefficient
fab-col_quantidade = Quantité
fab-estado = état { $estado }
fab-pecas_que_entraram = pièces entrées
fab-total_energia = Énergie du nœud : { $kwh } kWh.
fab-conta_emissao = Émission = énergie × { $fator } kg CO₂e/kWh (scope { $escopo }, source : { $fonte }) = { $kg } kg CO₂e.

## maquinas.rs

maq-titulo = Donner vie au modèle
maq-explica = Renseignez la puissance de chaque machine (kW) dans chaque groupe d'états : plaque, catalogue ou mesure. Le temps dans chaque état vient de la simulation.
maq-maquina = Machine
maq-etapa = Étape
maq-energia = Énergie

## fatores.rs

fat-titulo = Facteurs d'émission
fat-explica = L'app ne fournit aucune valeur de facteur : renseignez la valeur et la source de chacun. Un facteur sans source est signalé sur chaque écran.
fat-unidade_tempo = Unité de temps du modèle :
fat-unidade_tempo_dica = Les heures dans chaque état en dépendent.
fat-segundo = seconde
fat-minuto = minute
fat-hora = heure
fat-fluxo = Flux
fat-tipo = Type
fat-unidade = Unité
fat-escopo = Scope
fat-fator = Facteur
fat-fonte = Source
fat-ano = Année
fat-didatico = Didactique
fat-tem_valor = A une valeur (décoché : à définir)
fat-kg_por = kg CO₂e par { $unidade }
fat-sem_fonte = sans source
fat-tirar = Retirer
fat-novo_fluxo = Nouveau flux
fat-acrescentar = Ajouter
fat-tipo_energia = énergie
fat-tipo_combustivel = combustible
fat-tipo_agua = eau
fat-tipo_material = matière
fat-tipo_residuo = déchet
fat-tipo_efluente = effluent
fat-tipo_emissao = émission directe
fat-tipo_outro = autre

## pendências da conta (utsim-ambiental-core)

pend-setup = { $no } : énergie ou flux par réglage (phase AM3, par la trace).
pend-partida = { $no } : énergie ou flux par démarrage (phase AM3, par la trace).
pend-espera = { $no } : arrêt en attente (phase AM3, par la trace).
pend-refugo = { $no } : flux par pièce rebutée (phase AM5).
pend-retrabalho = { $no } : flux par retouche (phase AM5).
pend-distancia = { $no } : flux par distance (phase AM5).
pend-volume = { $no } : flux par volume (phase AM5).
pend-massa = { $no } : flux par masse (phase AM5).

## validação (utsim-ambiental-core)

val-objeto_inexistente = L'objet { $no } n'existe pas dans le modèle.
val-fluxo_inexistente = { $no } : le flux { $fluxo } n'est pas au catalogue.
val-fluxo_de_saida_como_entrada = { $no } : le flux { $fluxo } est une sortie mais ajoute de la masse à la pièce.
val-negativo = { $no } : { $campo } ne peut pas être négatif.
val-agua_fracao = { $no } : les fractions d'eau sont entre 0 et 1.
val-agua_soma = { $no } : les fractions d'eau dépassent 1.
val-reuso_inexistente = { $no } : la réutilisation va vers { $alvo }, qui n'existe pas.
val-parametro_inexistente = { $no } : le paramètre d'effluent { $parametro } n'est pas au catalogue.
val-pico_fora_do_dia = La plage de pointe doit tenir dans une journée (HH:MM).
val-intervalo_zero = L'intervalle de demande doit être supérieur à zéro.
val-massa_sem_tipo = { $no } : le flux ajoute ou retire de la masse, mais les types d'article n'ont pas de masse.
val-sem_fluxo_eletrico = Le catalogue n'a pas de flux d'énergie : la puissance ne devient pas émission.
val-fluxo_repetido = Le flux { $fluxo } apparaît deux fois au catalogue.
val-escopo_invalido = Le facteur de { $fluxo } a un scope hors de 1, 2 ou 3.
