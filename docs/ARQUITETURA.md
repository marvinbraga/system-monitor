# 🏗️ Arquitetura do System Monitor

## 📊 Visão Geral

O System Monitor é composto por **três componentes independentes**:

```
┌─────────────────────────────────────────────────────────────┐
│                      HOST FÍSICO                            │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │          COLLECTOR (Backend)                         │  │
│  │  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │  │
│  │  • Binário nativo Rust                               │  │
│  │  • Serviço systemd                                   │  │
│  │  • Porta: 5253                                       │  │
│  │  • Database: SQLite (./data/system-monitor.db)      │  │
│  │  • Acesso direto: /proc, /sys, /dev                 │  │
│  │  • API REST + WebSocket                             │  │
│  │  • Detecção de anomalias                            │  │
│  │  • Coleta a cada 2 segundos                         │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↑                                 │
│                           │ HTTP/WS                         │
│                           │                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │          CLIENTES (Opcionais)                       │  │
│  │  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │  │
│  │                                                      │  │
│  │  1. Frontend Web (React)                           │  │
│  │     • Porta: 5252                                   │  │
│  │     • Dashboard interativo no browser              │  │
│  │     • Gráficos em tempo real                       │  │
│  │                                                      │  │
│  │  2. TUI Client (Terminal)                          │  │
│  │     • Interface de terminal                         │  │
│  │     • Dashboard no terminal                         │  │
│  │     • Controles via teclado                        │  │
│  │                                                      │  │
│  │  3. Qualquer cliente HTTP/WS                       │  │
│  │     • curl, wget, scripts                          │  │
│  │     • Integrações personalizadas                   │  │
│  │                                                      │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 🎯 Componente 1: Collector (Backend)

### Características

- **Linguagem**: Rust (performance e segurança)
- **Tipo**: Binário standalone
- **Porta**: 5253
- **Independente**: Funciona sem o frontend

### Por que NÃO deve rodar em Docker?

❌ **Problemas com Docker:**
```
Container Docker
├─ Namespace isolado
├─ Acessa apenas /proc do container
├─ Vê apenas CPU/memória do container
├─ Temperaturas inacessíveis ou incorretas
└─ USB devices não visíveis
```

✅ **Vantagens do serviço nativo:**
```
Serviço Systemd
├─ Acesso direto ao /proc, /sys, /dev do host
├─ Métricas 100% precisas
├─ Todas as temperaturas visíveis
├─ USB devices completos
├─ Inicia com o sistema
└─ Gerenciamento via systemctl
```

### O que o Collector faz?

1. **Coleta métricas a cada 2 segundos**:
   - CPU: uso global e per-core, load average
   - Memória: RAM e Swap
   - Temperaturas: todos os sensores (hwmon)
   - Discos: uso, I/O, montagens
   - USB: dispositivos conectados
   - Rede: RX/TX bytes e pacotes

2. **Detecta anomalias**:
   - CPU spike (>80%)
   - Memória crítica (>95%)
   - Temperaturas altas (>80°C)
   - Discos cheios (>90%)
   - E mais...

3. **Persiste dados**:
   - SQLite: `./data/system-monitor.db`
   - Histórico de métricas
   - Log de anomalias

4. **Fornece API REST**:
   ```
   GET  /health                      - Health check
   GET  /api/v1/metrics/current      - Métricas atuais
   GET  /api/v1/metrics/history      - Histórico
   GET  /api/v1/anomalies            - Anomalias detectadas
   WS   /ws                          - Stream em tempo real
   ```

### Dependências

**Resposta: NENHUMA!**

O collector é **100% independente**:
- ✅ Roda sozinho como serviço
- ✅ Não precisa de frontend
- ✅ Não precisa de servidor web externo
- ✅ Não precisa de banco de dados externo (usa SQLite embarcado)
- ✅ Pode ser acessado por qualquer cliente HTTP

### Instalação

```bash
# Instalar como serviço systemd
sudo ./scripts/install.sh

# Verificar status
sudo systemctl status system-monitor-collector

# Ver logs
sudo journalctl -u system-monitor-collector -f

# Testar API
curl http://localhost:5253/health
```

### Desinstalação

```bash
sudo ./scripts/uninstall.sh
```

---

## 🎨 Componente 2: Clientes (Opcionais)

### 2.1. Frontend Web (React)

### Características

- **Linguagem**: TypeScript + React
- **Tipo**: Single Page Application (SPA)
- **Porta**: 5252
- **Totalmente OPCIONAL**

### Dependências

**Depende de**: Collector (API em localhost:5253)

O frontend é apenas um **cliente** que consome a API do collector:
```
Frontend → HTTP → Collector API
         ← JSON ←
```

### O que o Frontend faz?

1. **Dashboard interativo**:
   - Gráficos de CPU, memória, temperatura
   - Uso de discos em tempo real
   - Lista de dispositivos USB
   - Anomalias com filtros

2. **Atualização em tempo real**:
   - WebSocket para streaming
   - Auto-refresh de métricas
   - Alertas visuais

3. **Visualizações**:
   - Recharts para gráficos
   - Layout responsivo
   - Filtros e buscas

### Formas de Rodar

**Opção 1 - Nativo (Desenvolvimento)**:
```bash
cd web-frontend
npm install
npm run dev
# Acesse: http://localhost:5252
```

**Opção 2 - Docker (Isolamento)**:
```bash
docker compose up -d frontend
# Acesse: http://localhost:5252
```

**Opção 3 - Build de Produção**:
```bash
cd web-frontend
npm run build
# Deploy do diretório dist/ em nginx/apache
```

---

### 2.2. TUI Client (Terminal)

#### Características

- **Linguagem**: Rust + Ratatui
- **Tipo**: Terminal User Interface (TUI)
- **Interface**: Dashboard no terminal
- **Totalmente OPCIONAL**

#### Dependências

**Depende de**: Collector (API em localhost:5253)

O TUI é um cliente terminal que consome a mesma API do frontend web:
```
TUI Client → HTTP → Collector API
           ← JSON ←
```

#### O que o TUI faz?

1. **Dashboard no terminal**:
   - Visualização em ASCII art
   - Métricas de CPU, memória, discos
   - Lista de anomalias
   - Atualização em tempo real

2. **Controles de teclado**:
   - `q` ou `Ctrl+C` - Sair
   - `↑/↓` ou `k/j` - Scroll em anomalias
   - `Page Up/Down` - Scroll por página

3. **Modos de operação**:
   - HTTP polling (padrão)
   - WebSocket streaming (flag `-w`)

#### Como usar

**Uso básico**:
```bash
# Com valores padrão (localhost:5253, refresh 2s)
cargo run --package tui-client --release

# Ou usando o binário diretamente
./target/release/tui-client
```

**Opções avançadas**:
```bash
# Help
./target/release/tui-client --help

# API remota com refresh customizado
./target/release/tui-client --api-url http://192.168.1.100:5253 --refresh 5

# Usar WebSocket com refresh rápido
./target/release/tui-client -w -r 1

# Servidor remoto
./target/release/tui-client -u http://server:5253
```

**Argumentos**:
- `-u, --api-url <URL>` - URL da API (padrão: http://localhost:5253)
- `-r, --refresh <SEC>` - Taxa de refresh em segundos (padrão: 2)
- `-w, --websocket` - Usar WebSocket em vez de HTTP polling
- `-h, --help` - Mostrar ajuda

#### Quando usar TUI vs Web?

| Cenário | TUI Client | Web Frontend |
|---------|-----------|--------------|
| Servidor remoto sem GUI | ✅ Ideal | ❌ |
| SSH em servidor | ✅ Perfeito | ❌ |
| Monitoramento rápido local | ✅ Rápido | ⚠️ Mais pesado |
| Dashboard rico com gráficos | ⚠️ ASCII básico | ✅ Ideal |
| Múltiplos usuários | ❌ | ✅ |
| Automação/scripts | ⚠️ | ❌ |
| Baixo consumo de recursos | ✅ ~5MB RAM | ⚠️ ~100MB+ |

---

## 🔄 Fluxo de Dados

```
┌──────────────┐
│  Kernel      │
│  (/proc)     │
│  (/sys)      │
│  (/dev)      │
└──────┬───────┘
       │
       │ leitura direta
       ↓
┌──────────────┐
│  Collector   │
│  (Rust)      │────┐
└──────┬───────┘    │
       │            │ persiste
       │ fornece    ↓
       │ API     ┌──────────────┐
       │         │  SQLite      │
       │         │  (database)  │
       │         └──────────────┘
       ↓
┌──────────────┐
│  Frontend    │
│  (React)     │
└──────────────┘
       │
       ↓
┌──────────────┐
│  Browser     │
│  (Usuário)   │
└──────────────┘
```

---

## 📝 Casos de Uso

### Uso 1: Monitoramento via API

```bash
# Coletor roda como serviço
sudo systemctl start system-monitor-collector

# Consultar métricas via curl
curl http://localhost:5253/api/v1/metrics/current | jq

# Integrar com Grafana, Prometheus, etc.
```

**Frontend**: Não necessário ✅

---

### Uso 2: Dashboard Web Local

```bash
# Coletor como serviço
sudo systemctl start system-monitor-collector

# Frontend em dev mode
cd web-frontend && npm run dev

# Acessar: http://localhost:5252
```

**Frontend**: Necessário ✅

---

### Uso 3: Monitoramento Headless

```bash
# Apenas coletor rodando
sudo systemctl start system-monitor-collector

# Logs automáticos de anomalias
sudo journalctl -u system-monitor-collector -f
```

**Frontend**: Não necessário ✅

---

### Uso 4: Cliente TUI (Terminal)

```bash
# Coletor como serviço
sudo systemctl start system-monitor-collector

# TUI no terminal
./target/release/tui-client

# Ou via SSH em servidor remoto
ssh user@server
./tui-client --api-url http://localhost:5253
```

**Frontend**: Não necessário ✅

---

### Uso 5: Múltiplos Clientes Simultâneos

```bash
# 1 Collector (backend)
sudo systemctl start system-monitor-collector

# N Clientes simultâneos:
1. Frontend Web:    http://localhost:5252
2. TUI Client:      ./target/release/tui-client
3. Scripts curl:    curl http://localhost:5253/api/v1/metrics/current
4. Integrações:     Grafana, Prometheus, etc.
```

Todos acessam o mesmo collector! ✅

---

## ❓ FAQ

### O collector precisa do frontend para funcionar?

**Não!** O collector é independente. O frontend é apenas um cliente visual opcional.

### O frontend funciona sem o collector?

**Não!** O frontend é um cliente que consome a API do collector. Sem o collector rodando em localhost:5253, o frontend mostrará erros de conexão.

### Posso rodar o collector em Docker?

**Não é recomendado!** Você perderá precisão nas métricas. Use a instalação nativa com systemd.

### Posso acessar o collector de outra máquina?

**Sim!** Por padrão ele escuta em `127.0.0.1:5253`, mas você pode alterar em `/etc/system-monitor/config.toml` para `0.0.0.0:5253` e acessar de qualquer lugar da rede.

### Quantos recursos o collector consome?

Muito pouco:
- **CPU**: ~1-2%
- **RAM**: ~10-15 MB
- **Disco**: Database cresce ~1MB por hora

---

## 🚀 Resumo da Instalação

```bash
# 1. Instalar collector (OBRIGATÓRIO)
sudo ./scripts/install.sh

# 2. Verificar
curl http://localhost:5253/health

# 3. Cliente (OPCIONAL - escolha um ou mais):

#    a) Frontend Web - Nativo
cd web-frontend && npm run dev
# Acesse: http://localhost:5252

#    b) Frontend Web - Docker
docker compose up -d frontend
# Acesse: http://localhost:5252

#    c) TUI Terminal
cargo run --package tui-client --release
# Ou: ./target/release/tui-client

#    d) API direta
curl http://localhost:5253/api/v1/metrics/current | jq

#    e) Nenhum (collector standalone)
# Apenas logs e persistência em database
```

---

**Conclusão**: O collector é a peça central e independente. O frontend é um cliente visual opcional.
