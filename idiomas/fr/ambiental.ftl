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

pend-setup = { $no } : énergie ou flux par réglage (il faut la trace : simulez à nouveau).
pend-partida = { $no } : énergie ou flux par démarrage (il faut la trace : simulez à nouveau).
pend-espera = { $no } : arrêt en attente (il faut la trace : simulez à nouveau).
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

## AM3: curva de carga, tocar o dia, detalhes, relatório

un-kw = kW
un-kwh_evento = kWh/événement
fab-curva_de_carga = Courbe de charge
fab-tocar_o_dia = ▶ Jouer la journée
fab-pausar = ⏸ Pause
fab-parar = Arrêter
fab-janela = Fenêtre du calcul : { $de } à { $ate }
fab-janela_inteira = Fenêtre entière
fab-arraste_dica = Faites glisser sur la courbe pour ne calculer qu'une plage ; cliquez pour voir un instant.
fab-sem_curva = Pas de courbe : simulez à nouveau pour enregistrer la trace.
fab-no_pico = Pointe à { $hora } : { $kw } kW. Allumées ensemble : { $nos }.
fab-agora = { $hora } : { $kw } kW. Allumées : { $nos }.
fab-pico = Pointe de { $kw } kW à { $hora }.
fab-setups = réglages
fab-partidas = démarrages
fab-estado_standby = éteinte en attente
maq-detalhes = Détails
maq-detalhes_de = Détails : { $no }
maq-eventos = Énergie par événement
maq-kwh_peca = Par pièce entrante
maq-kwh_setup = Par réglage
maq-kwh_partida = Par démarrage (rallumer après un arrêt)
maq-desligar_em_espera = Éteindre après une longue attente
maq-depois_de = Après
maq-unidades_do_modelo = unités de temps du modèle en attente
maq-espera_aproximacao = Approximation : la simulation ne retarde pas la machine pour la rallumer. L'attente au-delà de la limite compte à la puissance hors poste, et chaque retour compte un démarrage.
rel-tela = Rapport environnemental
rel-explica = Crée le rapport du modèle, avec l'avertissement d'usage pédagogique et la liste des facteurs avec leurs sources. Il est enregistré à côté du modèle.
rel-gerar_html = Créer le HTML
rel-gerar_pdf = Créer le PDF
rel-gravado = Rapport enregistré dans { $arquivo }
rel-mostrar = Afficher dans le dossier
rel-conteudo = Contenu du rapport :
rel-titulo = Rapport environnemental : { $modelo }
rel-origem = Simulé : { $n } réplications, graine { $semente }, fenêtre de { $de } à { $ate }.
rel-aviso_titulo = Usage pédagogique
rel-aviso = Chiffres simulés, pour l'enseignement : ni mesure, ni inventaire officiel, ni déclaration de conformité. Vérifiez les facteurs et les sources avant d'utiliser un chiffre.
rel-resumo = Résumé
rel-por_maquina = Par machine
rel-energia_por_grupo = Énergie par groupe d'états (kWh)
rel-potencia_media = Puissance moyenne de l'usine (kW), première réplication
rel-fatores = Facteurs et sources
rel-a_definir = à définir
rel-sem_fatores = Aucun facteur enregistré : aucun chiffre d'émission dans ce rapport.

## AM4: demanda, pico e tarifa

fat-tarifa = Tarif d'énergie
fat-tarifa_explica = Heures de pointe, puissance souscrite et prix : tous saisis par vous, sans valeur par défaut. Surcoûts, taxes et énergie réactive sont exclus.
fat-usar_tarifa = Utiliser un tarif dans ce modèle
fat-horario_pico = Heures de pointe
fat-dias_uteis = jours ouvrés seulement (le 1er jour est lundi)
fat-intervalo_demanda = Intervalle de demande
fat-contratada = Puissance souscrite (kW)
fat-preco_pico = Prix du kWh en pointe
fat-preco_fora = Prix du kWh hors pointe
fat-preco_kw = Prix du kW de puissance
fat-preco_ultra = Prix du kW de dépassement
fat-fonte_tarifa = Source du tarif
fab-demanda = Puissance et tarif
fab-demanda_max = Puissance maximale
fab-demanda_pico = Puissance en pointe
fab-sem_pico_na_janela = la fenêtre ne passe pas par la pointe
fab-energia_pico = Énergie en pointe
fab-fator_de_carga = Facteur de charge
fab-conta_energia = Facture d'énergie simulée
fab-conta_explica = Facture de la fenêtre simulée : énergie en pointe et hors pointe, la puissance (souscrite ou mesurée, la plus grande) et le dépassement, comme si la fenêtre était une période de facturation.
fab-falta_preco = prix manquant
fab-falta_preco_dica = Pas de facture : il manque { $precos }. Renseignez-les dans l'écran Facteurs.
fab-contratada = souscrite { $kw } kW
rel-parcela_pico = Part de l'énergie en pointe
rel-parcela_fora = Part de l'énergie hors pointe
rel-parcela_demanda = Part de la puissance
rel-parcela_ultra = Part du dépassement
rel-tarifa_fonte = Pointe de { $de } à { $ate }, intervalle de { $min } minutes. Source du tarif : { $fonte }.

## AM5: massa, água, resíduos e efluentes

un-m3 = m³
un-m3_peca = m³/pièce
un-kg = kg
un-kg_peca = kg/pièce
fab-agua = Eau
fab-residuos = Déchets
fab-efluente = Effluent
fab-residuo_processo = Déchet de procédé
fab-sucata = Rebut
fab-agua_e_efluentes = Eau et effluents
fab-captacao = Prélèvement
fab-consumo = Consommation (prélèvement − effluent)
fab-reuso = Réutilisation
fab-tratamento = Énergie du traitement
fab-lodo = Boues du traitement
fab-carga = { $parametro } : { $antes } kg avant, { $depois } kg après traitement ; mélange { $mgl } mg/L
fab-limite = Limite enregistrée : { $limite } (source : { $fonte }). L'app compare, elle n'affirme pas la conformité.
fab-desvio_aterro = Détourné de l'enfouissement
fab-sem_destino = sans destination
fab-materiais = Matières
fab-entrada_massa = Masse entrée
fab-massa_boa = Masse de produit bon
fab-rendimento = Rendement matière
fab-sucata_pecas = { $kg } kg ({ $pecas } pièces)
fab-balanco = Bilan matière
fab-balanco_fecha = équilibré
fab-balanco_erro = ne s'équilibre pas : { $pct } %
fab-mochila_kwh = Sac à dos du rebut (énergie)
fab-mochila_co2e = Sac à dos du rebut (CO₂e)
fab-mochila_explica = Le sac à dos, c'est ce que les pièces rebutées ont emporté : l'énergie moyenne par pièce de chaque machine traversée et les flux par pièce. Rebuter en fin de ligne coûte plus qu'au début.
fab-custo_fluxos = Coût des flux et déchets
maq-fluxos = Flux de la machine
maq-fluxos_explica = Chaque ligne : quantité × inducteur (par pièce, par heure dans un état, par réglage...). Eau en L ou m³ ; déchet en kg.
maq-por = { $unidade } { $dir }
maq-dir_peca = par pièce entrante
maq-dir_peca_boa = par pièce sortante
maq-dir_refugo = par pièce rebutée ici
maq-dir_setup = par réglage
maq-dir_partida = par démarrage
maq-dir_massa = par kg traité
maq-dir_tempo_produzindo = par heure de production
maq-dir_tempo_ligada = par heure allumée
maq-dir_outro = autre (du fichier)
maq-soma_massa = ajoute à la pièce
maq-tira_massa = retire de la pièce
maq-novo_fluxo = + Flux
maq-sem_fluxos = Enregistrez des flux (eau, déchet, matière) dans l'écran Facteurs.
maq-balanco_agua = Bilan d'eau de cette machine
maq-incorporada = Fraction incorporée au produit
maq-evaporada = Fraction évaporée ou perdue
maq-reuso_para = Réutilisation de l'effluent à
maq-reuso_fracao = Fraction de l'effluent réutilisée
maq-efluente = Effluent de cette machine
maq-destino_efluente = Destination de l'effluent
maq-sem_parametros = Enregistrez les paramètres de l'effluent dans l'écran Facteurs.
fat-preco = Prix
fat-destino = Destination
fat-massa = Masse des pièces
fat-massa_explica = Avec la masse, l'app fait le bilan matière, le rebut en kg et le rendement matière.
fat-qualquer_peca = Pièce sans type (toutes)
fat-destinos = Destinations des déchets
fat-desviado = Détourné de l'enfouissement
fat-fator_kg = Facteur (kg CO₂e/kg)
fat-custo_kg = Coût par kg
fat-novo_destino = Nouvelle destination
fat-destino_sucata = Destination du rebut :
fat-efluente = Effluent
fat-efluente_explica = Paramètres (avec la limite et la source que vous enregistrez) et destinations de l'effluent, avec l'abattement de chaque paramètre, l'énergie, les boues et le coût par m³.
fat-parametros = Paramètres
fat-parametro = Paramètre
fat-limite = Limite
fat-fonte_limite = Source de la limite
fat-novo_parametro = Nouveau paramètre
fat-destinos_efluente = Destinations de l'effluent
fat-kwh_m3 = kWh/m³
fat-lodo_m3 = boues kg/m³
fat-custo_m3 = coût/m³
fat-remocao = abat { $parametro }
fat-novo_destino_efluente = Nouvelle destination d'effluent
rel-cargas = Charges de l'effluent
rel-antes_kg = Avant (kg)
rel-depois_kg = Après (kg)
rel-concentracao = Mélange (mg/L)
rel-sem_conformidade = La comparaison avec la limite n'est pas une déclaration de conformité légale.
rel-desvio = Détourné de l'enfouissement : { $pct } %.
val-destino_inexistente = { $no } : la destination { $destino } n'est pas au catalogue.
val-unidade_agua = Le flux d'eau { $fluxo } doit être en L ou m3.

## AM6: Sankey

snk-titulo = Flux
snk-massa = Masse
snk-energia = Énergie
snk-agua = Eau
snk-carbono = Carbone
snk-por_etapa = Regrouper par étape
snk-juntar = Masquer les flux inférieurs à
snk-sem_massa = Pas de masse : renseignez la masse des pièces dans l'écran Facteurs et simulez à nouveau.
snk-vazio = Aucune machine n'a ce que ce mode montre.
snk-total = Total : { $valor } { $unidade }
snk-exportar_svg = Exporter en SVG
snk-exportar_csv = Exporter l'inventaire (CSV)
snk-dica = Cliquez sur une bande pour voir son chemin ; double-cliquez sur une étape pour l'ouvrir.
snk-balanco_dica = { $no } : { $entra } entrent et { $sai } sortent. Vérifiez que vous avez enregistré les matières ajoutées à la pièce et les déchets retirés sur cette machine.
snk-faltam = il manque { $pct } %
snk-sobram = { $pct } % en trop
snk-faixa = { $de } → { $para } : { $valor } { $unidade } ({ $pct } % du total)
snk-titulo_svg = { $modelo } : flux de { $modo }
snk-agrega_valor = énergie à valeur ajoutée
snk-preparando = énergie de préparation
snk-sem_valor = énergie sans valeur (attente, arrêt, hors poste)
snk-tratamento = traitement de l'effluent
snk-captacao = prélèvement d'eau
snk-evaporada = évaporée ou perdue
snk-incorporada = incorporée au produit
snk-efluente_sem_tratamento = effluent sans traitement enregistré
snk-produto = produit
snk-sucata_e_lodo = rebut, boues et traitement

## AM7: pegada por produto

prd-titulo = Empreinte par produit
prd-aviso = Étude simplifiée et pédagogique : ni analyse de cycle de vie complète ni déclaration environnementale de produit.
prd-sem_rastro = L'empreinte par produit demande la trace de la simulation : cliquez sur Simuler à nouveau dans l'Usine.
prd-sem_pecas = Aucune pièce n'est passée par des machines avec bloc environnemental dans la fenêtre.
prd-produto = Produit :
prd-sem_tipo = pièce (sans type)
prd-unidade = Unité fonctionnelle :
prd-peca_boa = pièce bonne
prd-kg_produto = kg de produit
prd-regra = Allocation :
prd-regra_tempo = par le temps sur la machine
prd-regra_pecas = par le nombre de pièces
prd-regra_massa = par la masse
prd-regra_estrutura = attente en structure
prd-comparar = Comparer avec :
prd-materias_primas = Matières premières
prd-transporte = Transport
prd-fabricacao = Fabrication
prd-distribuicao = Distribution
prd-uso = Utilisation
prd-fim_de_vida = Fin de vie
prd-portao_a_portao = porte à porte (fabrication seule)
prd-berco_ao_portao = berceau à porte (matières et fabrication)
prd-fronteira = Frontière : { $fronteira }. Cliquez sur Matières premières pour l'inclure ou la retirer.
prd-unidades = Unités dans la fenêtre
prd-co2_por = CO₂e par { $unidade }
prd-energia_por = Énergie par { $unidade }
prd-agua_por = Eau par { $unidade }
prd-mochila_por = Énergie des pièces rebutées, par { $unidade }
prd-taxa_refugo = Taux de rebut :
prd-voltar_simulada = simulé
prd-com_refugo = Avec { $taxa } % de rebut (simulé : { $simulada } %), l'empreinte serait de { $valor } kg CO₂e par unité.
prd-com_refugo_kwh = Avec { $taxa } % de rebut (simulé : { $simulada } %), l'énergie serait de { $valor } kWh par unité.
prd-contribuicao = Contribution par machine
prd-kwh_por_unidade = kWh par unité
prd-co2_por_unidade = kg CO₂e par unité (fabrication)
prd-estrutura = Allocation { $regra } : produits { $produtos } kWh + structure { $estrutura } kWh = usine { $fabrica } kWh. Changer la règle change les produits, jamais le total.
prd-relatorio = Rapport de l'étude
prd-rel_titulo = Étude d'empreinte : { $produto }
prd-rel_objetivo = Objectif
prd-rel_objetivo_texto = Estimer, pour l'enseignement, l'énergie, l'eau et le CO₂ équivalent de { $produto } par { $unidade } dans l'usine simulée.
prd-rel_fronteira = Unité fonctionnelle et frontière
prd-rel_fora = Hors de la frontière :
prd-rel_inventario = Inventaire par unité fonctionnelle
prd-rel_impacto = Impact : changement climatique
prd-rel_impacto_texto = { $valor } kg CO₂e par { $unidade }.
prd-rel_interpretacao = Interprétation
prd-rel_regras = Effet de la règle d'allocation
prd-rel_regras_texto = La règle d'allocation est un choix : elle change l'empreinte de chaque produit, mais pas le total de l'usine.
prd-rel_limitacoes = Limites
prd-lim_1 = Les chiffres viennent d'une simulation, pas d'une mesure.
prd-lim_2 = Seul le changement climatique est évalué ; les autres catégories d'impact sont exclues.
prd-lim_3 = Les facteurs d'émission sont ceux du catalogue du modèle, avec les sources listées ci-dessous.
