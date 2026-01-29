# Monitor de Sistema em Rust

## Descrição

Aplicativo profissional de monitoramento de sistema desenvolvido em Rust para testar a estabilidade do Zorin OS após correções no sistema.

## Funcionalidades

✓ **Monitoramento de CPU**
- Uso global da CPU
- Uso individual de todos os 32 cores
- Indicadores visuais coloridos (verde/amarelo/vermelho)
- Barras de progresso em tempo real

✓ **Monitoramento de Memória**
- RAM total, usada e disponível
- SWAP total e usado
- Percentuais e gráficos visuais

✓ **Monitoramento de Temperaturas**
- Temperaturas da CPU (k10temp - Tctl e Tccd)
- Temperaturas dos discos NVMe
- Temperaturas da GPU (NVIDIA)
- Outros sensores do sistema
- Indicadores coloridos por faixa de temperatura
- Leitura direta de /sys/class/hwmon/

✓ **Informações do Sistema**
- Sistema Operacional 
- Versão do Kernel
- Hostname
- Tempo de uptime
- Número de CPUs

✓ **Status de Estabilidade**
- Indicadores de saúde do sistema
- Verificações automáticas de estabilidade
- Status visual: ESTÁVEL / CARGA MODERADA / ALTA CARGA

## Como Executar

### Opção 1: Script de execução (recomendado)
```bash
cd ~/dados/system-monitor
./run.sh
```

### Opção 2: Executável direto
```bash
cd ~/dados/system-monitor
./target/release/system-monitor
```

### Opção 3: Via cargo
```bash
cd ~/dados/system-monitor
cargo run --release
```

## Controles

- **Ctrl+C** - Sair do monitor

## Atualização

O monitor atualiza as informações **a cada 2 segundos** automaticamente.

## Interpretação dos Resultados

### Indicadores de Status
- **●** Verde: Uso abaixo de 50% (normal)
- **●** Amarelo: Uso entre 50-80% (moderado)
- **●** Vermelho: Uso acima de 80% (alto)

### Status Geral
- **SISTEMA ESTÁVEL ✓**: CPU < 70% e Memória < 80%
- **SISTEMA SOB CARGA MODERADA ⚠**: CPU < 90% e Memória < 90%
- **SISTEMA SOB ALTA CARGA ✗**: CPU ou Memória >= 90%

## Objetivo do Teste

Este monitor foi criado para verificar a estabilidade do sistema após:
1. Desabilitar o Elgato Wave:3 (que estava causando timeouts USB)
2. Identificar problemas com o NZXT Kraken X53

**Resultado esperado**: Sistema estável sem travamentos, ícones desaparecendo ou perda de acesso a disco.

## Tecnologias Utilizadas

- **Rust** (2021 edition)
- **sysinfo** 0.32 - Informações do sistema
- **colored** 2.1 - Saída colorida no terminal
- **chrono** 0.4 - Data e hora

## Compilação

Para recompilar o projeto:
```bash
cd ~/dados/system-monitor
cargo build --release
```

O binário compilado ficará em: `target/release/system-monitor`

---

**Desenvolvido para testar a estabilidade do sistema Zorin OS**
