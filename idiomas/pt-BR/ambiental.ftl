# UTSim Ambiental (pt-BR). Mesmas chaves nas cinco línguas.

## app.rs

app-inicio = Início
app-grupo_ver = Ver
app-grupo_dados = Dados
app-grupo_decidir = Decidir
app-fabrica = Fábrica
app-fluxos = Fluxos
app-produto = Produto
app-maquinas = Máquinas
app-fatores = Fatores
app-cenarios = Cenários
app-metas = Metas
app-relatorios = Relatórios
app-abra_um_modelo = Abra um modelo no Início.
app-fase_am6 = Chega na fase AM6 do plano.
app-fase_am7 = Chega na fase AM7 do plano.
app-fase_am8 = Chega na fase AM8 do plano.
app-fase_am9 = Chega na fase AM9 do plano.
app-nao_salvo = mudanças não salvas
app-subtitulo = Energia e impacto ambiental
app-recentes = Modelos recentes
app-acao_abrir = Abrir um modelo do UTSim
app-acao_abrir_desc = Escolha um arquivo .ron: o app simula e acende a fábrica.
app-acao_exemplo = Abrir o exemplo
app-acao_exemplo_desc = Uma linha de pintura com a potência das máquinas já informada.
app-onde_explicacao = O bloco ambiental fica num arquivo ao lado do modelo, <modelo>.ambiental.ron. O modelo não muda.
app-aviso_didatico = Números simulados, para ensino: não são medição nem inventário oficial.
app-aberto = Modelo aberto: { $modelo }. Simulando…
app-simulado = Simulação pronta.
app-erro_abrir = Não consegui abrir: { $e }
app-erro_simular = A simulação falhou: { $e }
app-erro_config = Não consegui lembrar a configuração: { $e }
app-salvar = Salvar
app-salvar_dica = Grava o bloco ambiental ao lado do modelo.
app-salvo = Salvo em { $arquivo }
app-erro_salvar = Não consegui salvar: { $e }

## unidades

un-kwh = kWh
un-kwh_peca = kWh/peça
un-kg_co2e = kg CO₂e
un-kg_co2e_peca = kg CO₂e/peça

## grupos de estado

grupo-produzindo = Produzindo
grupo-preparando = Preparando
grupo-esperando = Esperando
grupo-parada = Parada
grupo-fora_de_turno = Fora de turno

## vilões

vilao-esperando = { $no } gasta { $pct } % da energia esperando peça ({ $pf } % da fábrica sem valor).
vilao-parada = { $no } gasta { $pct } % da energia parada ({ $pf } % da fábrica sem valor).
vilao-fora_de_turno = { $no } gasta { $pct } % da energia fora de turno ({ $pf } % da fábrica sem valor).
vilao-preparando = { $no } gasta { $pct } % da energia preparando ({ $pf } % da fábrica sem valor).
vilao-produzindo = { $no } gasta { $pct } % da energia produzindo ({ $pf } % da fábrica sem valor).

## fabrica.rs

fab-simulando = Simulando…
fab-origem = Simulado · { $n } replicações · semente { $semente }
fab-simular_de_novo = Simular de novo
fab-energia = Energia
fab-sem_valor = Energia sem valor
fab-co2e = CO₂ equivalente
fab-total = Total
fab-por_peca = Por peça
fab-por_peca_boa = Por peça boa
fab-pecas_boas = Peças boas
fab-esperando_simulacao = Esperando a simulação…
fab-a_fabrica = A fábrica
fab-falta_fator = falta fator
fab-falta_fator_dica = Sem fator de emissão para: { $fluxos }. Informe na tela Fatores.
fab-sem_fonte = sem fonte: { $fluxo }
fab-didatico = didático: { $fluxo }
fab-clique_numa_maquina = Clique numa máquina do mapa para ver e mudar a potência dela.
fab-viloes = Os três vilões
fab-sem_viloes = Nenhuma máquina gasta energia sem agregar valor.
fab-ainda_nao_contado = Ainda não contado nesta versão
fab-informe_potencia = Informe a potência
fab-de_onde_veio = De onde veio?
fab-de_onde_veio_dica = Abre a conta deste número.
fab-de_onde_veio_titulo = De onde veio: { $no }
fab-potencia_kw = Potência em cada grupo de estado
fab-antes_e_depois = Antes e depois
fab-base_explica = Guarde a situação de agora como base, mude uma potência e veja a diferença.
fab-guardar_base = Guardar como base
fab-nova_base = Nova base
fab-voltar_base = Voltar à base
fab-pareado_explica = Diferença pareada: mesma semente e mesmas replicações, com intervalo de 95 %.
fab-formula_energia = E = Σ P(estado) × T(estado) + e(peça) × peças
fab-media_de = Média de { $n } replicações.
fab-col_origem = Origem
fab-col_grupo = Grupo
fab-col_direcionador = Direcionador
fab-col_coeficiente = Coeficiente
fab-col_quantidade = Quantidade
fab-estado = estado { $estado }
fab-pecas_que_entraram = peças que entraram
fab-total_energia = Energia do nó: { $kwh } kWh.
fab-conta_emissao = Emissão = energia × { $fator } kg CO₂e/kWh (escopo { $escopo }, fonte: { $fonte }) = { $kg } kg CO₂e.

## maquinas.rs

maq-titulo = Dar vida ao modelo
maq-explica = Informe a potência de cada máquina (kW) em cada grupo de estado: da placa, do catálogo ou de uma medição. O tempo em cada estado vem da simulação.
maq-maquina = Máquina
maq-etapa = Etapa
maq-energia = Energia

## fatores.rs

fat-titulo = Fatores de emissão
fat-explica = O app não traz número de fator: informe o valor e a fonte de cada um. Fator sem fonte aparece marcado em toda tela.
fat-unidade_tempo = Unidade de tempo do modelo:
fat-unidade_tempo_dica = As horas em cada estado dependem dela.
fat-segundo = segundo
fat-minuto = minuto
fat-hora = hora
fat-fluxo = Fluxo
fat-tipo = Tipo
fat-unidade = Unidade
fat-escopo = Escopo
fat-fator = Fator
fat-fonte = Fonte
fat-ano = Ano
fat-didatico = Didático
fat-tem_valor = Tem valor (desmarcado: a definir)
fat-kg_por = kg CO₂e por { $unidade }
fat-sem_fonte = sem fonte
fat-tirar = Tirar
fat-novo_fluxo = Novo fluxo
fat-acrescentar = Acrescentar
fat-tipo_energia = energia
fat-tipo_combustivel = combustível
fat-tipo_agua = água
fat-tipo_material = material
fat-tipo_residuo = resíduo
fat-tipo_efluente = efluente
fat-tipo_emissao = emissão direta
fat-tipo_outro = outro

## pendências da conta (utsim-ambiental-core)

pend-setup = { $no }: energia ou fluxo por setup (precisa do rastro: simule de novo).
pend-partida = { $no }: energia ou fluxo por partida (precisa do rastro: simule de novo).
pend-espera = { $no }: desligamento em espera (precisa do rastro: simule de novo).
pend-refugo = { $no }: fluxo por peça refugada (fase AM5).
pend-retrabalho = { $no }: fluxo por retrabalho (fase AM5).
pend-distancia = { $no }: fluxo por distância (fase AM5).
pend-volume = { $no }: fluxo por volume (fase AM5).
pend-massa = { $no }: fluxo por massa (fase AM5).

## validação (utsim-ambiental-core)

val-objeto_inexistente = O objeto { $no } não existe no modelo.
val-fluxo_inexistente = { $no }: o fluxo { $fluxo } não está no catálogo.
val-fluxo_de_saida_como_entrada = { $no }: o fluxo { $fluxo } é de saída e soma massa à peça.
val-negativo = { $no }: { $campo } não pode ser negativo.
val-agua_fracao = { $no }: as frações de água ficam entre 0 e 1.
val-agua_soma = { $no }: as frações de água somam mais que 1.
val-reuso_inexistente = { $no }: o reuso vai para { $alvo }, que não existe.
val-parametro_inexistente = { $no }: o parâmetro de efluente { $parametro } não está no catálogo.
val-pico_fora_do_dia = A janela de pico precisa ficar dentro de um dia (HH:MM).
val-intervalo_zero = O intervalo de demanda precisa ser maior que zero.
val-massa_sem_tipo = { $no }: o fluxo soma ou tira massa, mas os tipos de item não têm massa.
val-sem_fluxo_eletrico = O catálogo não tem fluxo de energia: a potência não vira emissão.
val-fluxo_repetido = O fluxo { $fluxo } aparece duas vezes no catálogo.
val-escopo_invalido = O fator de { $fluxo } tem escopo fora de 1, 2 ou 3.

## AM3: curva de carga, tocar o dia, detalhes, relatório

un-kw = kW
un-kwh_evento = kWh/evento
fab-curva_de_carga = Curva de carga
fab-tocar_o_dia = ▶ Tocar o dia
fab-pausar = ⏸ Pausar
fab-parar = Parar
fab-janela = Janela da conta: { $de } a { $ate }
fab-janela_inteira = Janela inteira
fab-arraste_dica = Arraste na curva para fazer a conta só num trecho; clique para ver um instante.
fab-sem_curva = Sem curva: simule de novo para gravar o rastro.
fab-no_pico = Pico às { $hora }: { $kw } kW. Ligadas juntas: { $nos }.
fab-agora = { $hora }: { $kw } kW. Ligadas: { $nos }.
fab-pico = Pico de { $kw } kW às { $hora }.
fab-setups = setups
fab-partidas = partidas
fab-estado_standby = desligada em espera
maq-detalhes = Detalhes
maq-detalhes_de = Detalhes: { $no }
maq-eventos = Energia por evento
maq-kwh_peca = Por peça que entra
maq-kwh_setup = Por setup
maq-kwh_partida = Por partida (ligar depois de parada)
maq-desligar_em_espera = Desligar quando espera muito
maq-depois_de = Depois de
maq-unidades_do_modelo = unidades de tempo do modelo esperando
maq-espera_aproximacao = Aproximação: a simulação não atrasa a máquina para religar. O tempo de espera depois do limite conta com a potência de fora de turno, e cada volta conta uma partida.
rel-tela = Relatório ambiental
rel-explica = Gera o relatório do modelo, com o aviso de uso didático e a lista de fatores com as fontes. Fica ao lado do modelo.
rel-gerar_html = Gerar HTML
rel-gerar_pdf = Gerar PDF
rel-gravado = Relatório gravado em { $arquivo }
rel-mostrar = Mostrar na pasta
rel-conteudo = O que o relatório traz:
rel-titulo = Relatório ambiental: { $modelo }
rel-origem = Simulado: { $n } replicações, semente { $semente }, janela de { $de } a { $ate }.
rel-aviso_titulo = Uso didático
rel-aviso = Números simulados, para ensino: não são medição, inventário oficial nem declaração de conformidade. Confira os fatores e as fontes antes de usar qualquer número.
rel-resumo = Resumo
rel-por_maquina = Por máquina
rel-energia_por_grupo = Energia por grupo de estado (kWh)
rel-potencia_media = Potência média da fábrica (kW), primeira replicação
rel-fatores = Fatores e fontes
rel-a_definir = a definir
rel-sem_fatores = Nenhum fator cadastrado: nenhum número de emissão neste relatório.
