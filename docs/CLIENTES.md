# 🖥️ Clientes do System Monitor

## 📋 Opções Disponíveis

O System Monitor oferece **múltiplas formas** de visualizar e consumir as métricas:

```
                    ┌─────────────────┐
                    │   COLLECTOR     │
                    │   (Backend)     │
                    │   Port: 5253    │
                    └────────┬────────┘
                             │
                    ┌────────┴────────┐
                    │   API REST +    │
                    │   WebSocket     │
                    └────────┬────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
    ┌────▼────┐        ┌────▼────┐        ┌────▼────┐
    │ Web UI  │        │   TUI   │        │ Scripts │
    │ (React) │        │ (Term)  │        │ (curl)  │
    └─────────┘        └─────────┘        └─────────┘
```

---

## 1️⃣ Frontend Web (React)

### 📊 Características
- Interface gráfica rica
- Gráficos interativos (Recharts)
- Dashboard completo
- Múltiplos usuários simultâneos
- Acesso via browser

### 🚀 Como usar

**Desenvolvimento (Hot Reload)**:
```bash
cd web-frontend
npm install
npm run dev
# Acesse: http://localhost:5252
```

**Produção (Build)**:
```bash
cd web-frontend
npm run build
# Deploy da pasta dist/
```

**Docker (Isolamento)**:
```bash
docker compose up -d frontend
# Acesse: http://localhost:5252
```

### ✅ Quando usar
- ✅ Monitoramento visual rico
- ✅ Múltiplos usuários
- ✅ Comparação de gráficos históricos
- ✅ Análise detalhada
- ✅ Apresentações/demos

### ❌ Quando NÃO usar
- ❌ Servidor headless (sem GUI)
- ❌ Via SSH
- ❌ Baixíssimo uso de recursos
- ❌ Automação

---

## 2️⃣ TUI Client (Terminal)

### 📊 Características
- Interface de terminal (ASCII)
- Dashboard em texto
- Leve e rápido
- Ideal para SSH
- Controles via teclado

### 🚀 Como usar

**Básico**:
```bash
# Compilar (primeira vez)
cargo build --package tui-client --release

# Executar
./target/release/tui-client
```

**Avançado**:
```bash
# Help
./target/release/tui-client --help

# API remota
./target/release/tui-client --api-url http://192.168.1.100:5253

# Refresh customizado (5 segundos)
./target/release/tui-client --refresh 5

# WebSocket mode
./target/release/tui-client --websocket

# Combinado
./target/release/tui-client -u http://server:5253 -w -r 1
```

### ⌨️ Controles

| Tecla | Ação |
|-------|------|
| `q` | Sair |
| `Ctrl+C` | Sair |
| `↑` / `k` | Scroll up (anomalias) |
| `↓` / `j` | Scroll down (anomalias) |
| `Page Up` | Página anterior |
| `Page Down` | Próxima página |

### 📋 Opções

```
USAGE:
    tui-client [OPTIONS]

OPTIONS:
    -u, --api-url <URL>        API base URL (padrão: http://localhost:5253)
    -r, --refresh <SECONDS>    Taxa de refresh (padrão: 2)
    -w, --websocket            Usar WebSocket em vez de HTTP polling
    -h, --help                 Mostrar ajuda

EXAMPLES:
    tui-client
    tui-client --api-url http://192.168.1.100:5253 --refresh 5
    tui-client -w -r 1
```

### ✅ Quando usar
- ✅ SSH em servidor remoto
- ✅ Servidor sem GUI
- ✅ Monitoramento rápido
- ✅ Baixo uso de recursos (~5MB RAM)
- ✅ Terminal only
- ✅ Tmux/Screen sessions

### ❌ Quando NÃO usar
- ❌ Gráficos complexos necessários
- ❌ Análise histórica detalhada
- ❌ Múltiplos usuários simultâneos
- ❌ Apresentações visuais

---

## 3️⃣ API Direta (curl/scripts)

### 📊 Características
- Acesso direto à API REST
- Integração com scripts
- Automação
- Monitoring tools (Grafana, Prometheus)

### 🚀 Como usar

**Endpoints principais**:

```bash
# Health check
curl http://localhost:5253/health

# Métricas atuais
curl http://localhost:5253/api/v1/metrics/current | jq

# Histórico (últimos 60 registros)
curl "http://localhost:5253/api/v1/metrics/history?limit=60" | jq

# Anomalias
curl "http://localhost:5253/api/v1/anomalies?limit=50" | jq
```

**Exemplos práticos**:

```bash
# CPU atual
curl -s http://localhost:5253/api/v1/metrics/current | \
  jq '.data.cpu.global_usage'

# Memória disponível
curl -s http://localhost:5253/api/v1/metrics/current | \
  jq '.data.memory.available'

# Temperaturas críticas
curl -s http://localhost:5253/api/v1/metrics/current | \
  jq '.data.temperatures[] | select(.value > 80)'

# Monitoramento contínuo
watch -n 2 'curl -s http://localhost:5253/api/v1/metrics/current | \
  jq ".data.cpu.global_usage, .data.memory.usage_percent"'
```

**WebSocket (streaming)**:

```bash
# wscat (npm install -g wscat)
wscat -c ws://localhost:5253/ws

# Recebe updates em tempo real
```

### ✅ Quando usar
- ✅ Automação de scripts
- ✅ Integração com monitoring tools
- ✅ CI/CD pipelines
- ✅ Alertas personalizados
- ✅ Data collection
- ✅ APIs personalizadas

---

## 📊 Comparação dos Clientes

| Característica | Web Frontend | TUI Client | API Direta |
|----------------|--------------|------------|------------|
| **Interface** | Gráfica (Browser) | Terminal (ASCII) | JSON/programática |
| **Gráficos** | ✅ Ricos | ⚠️ ASCII básico | ❌ |
| **RAM** | ~100MB+ | ~5MB | ~0MB |
| **CPU** | ~2-5% | ~0.5% | ~0% |
| **SSH** | ❌ | ✅ | ✅ |
| **Múltiplos usuários** | ✅ | ❌ | ✅ |
| **Tempo real** | ✅ WebSocket | ✅ Polling/WS | ✅ WS |
| **Histórico** | ✅ Gráficos | ⚠️ Lista | ✅ JSON |
| **Filtros** | ✅ Interativos | ⚠️ Básicos | ✅ Query params |
| **Instalação** | npm/docker | cargo | nenhuma |
| **Automação** | ❌ | ❌ | ✅ |

---

## 🎯 Casos de Uso

### Cenário 1: Desenvolvimento Local
```bash
# Terminal 1: Collector
sudo systemctl start system-monitor-collector

# Terminal 2: Frontend
cd web-frontend && npm run dev

# Browser: http://localhost:5252
```

---

### Cenário 2: Servidor Remoto SSH
```bash
# SSH no servidor
ssh admin@server

# Instalar collector
sudo ./scripts/install.sh

# Monitorar via TUI
./target/release/tui-client
```

---

### Cenário 3: Monitoramento Headless
```bash
# Apenas collector rodando
sudo systemctl start system-monitor-collector

# Scripts de monitoramento
while true; do
  cpu=$(curl -s localhost:5253/api/v1/metrics/current | jq '.data.cpu.global_usage')
  echo "CPU: ${cpu}%"
  sleep 5
done
```

---

### Cenário 4: Dashboard + TUI + Scripts
```bash
# 1 Collector + múltiplos clientes

# Terminal 1: TUI
./target/release/tui-client

# Terminal 2: Script de alertas
./scripts/monitor-alerts.sh

# Browser: Frontend Web
http://localhost:5252

# Todos acessam o mesmo collector!
```

---

## 🚀 Quick Start

### Opção A: Web Only
```bash
sudo ./scripts/install.sh
cd web-frontend && npm run dev
# Acesse: http://localhost:5252
```

### Opção B: TUI Only
```bash
sudo ./scripts/install.sh
cargo run --package tui-client --release
```

### Opção C: Ambos
```bash
# Terminal 1
sudo ./scripts/install.sh

# Terminal 2
cd web-frontend && npm run dev

# Terminal 3
./target/release/tui-client
```

---

## 💡 Dicas

1. **Performance**: TUI usa menos recursos que Web Frontend
2. **SSH**: Sempre use TUI em conexões SSH
3. **Produção**: Use Web Frontend para dashboards permanentes
4. **Automação**: Use API direta com curl/scripts
5. **Debug**: TUI é ótimo para troubleshooting rápido
6. **Múltiplos usuários**: Web Frontend permite acesso simultâneo
7. **Offline**: TUI funciona mesmo sem browser

---

**Escolha o cliente certo para cada situação!** 🎯
