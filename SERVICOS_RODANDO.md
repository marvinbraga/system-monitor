# ✅ System Monitor - Serviços em Execução

## Status: RODANDO

Data: 29 de Janeiro de 2026

---

## 🚀 Serviços Ativos

### 1. **Collector (Backend API)**
- **Status**: ✅ Ativo
- **Porta**: 5253
- **Processo**: `./target/release/collector`
- **Logs**: `collector.log`
- **Database**: `./data/system-monitor.db`

**Endpoints disponíveis:**
- Health: http://localhost:5253/health
- Métricas: http://localhost:5253/api/v1/metrics/current
- Histórico: http://localhost:5253/api/v1/metrics/history
- Anomalias: http://localhost:5253/api/v1/anomalies
- WebSocket: ws://localhost:5253/ws

### 2. **Web Frontend (React)**
- **Status**: ✅ Ativo
- **Porta**: 5252
- **Processo**: `vite` (development server)
- **URL**: http://localhost:5252

---

## 📊 Como Usar

### Acessar o Dashboard Web:
```bash
# Abrir no navegador
xdg-open http://localhost:5252

# Ou
google-chrome http://localhost:5252
firefox http://localhost:5252
```

### Testar a API:
```bash
# Health check
curl http://localhost:5253/health

# Métricas atuais
curl http://localhost:5253/api/v1/metrics/current | jq

# Anomalias
curl http://localhost:5253/api/v1/anomalies | jq
```

### Cliente TUI (Terminal):
```bash
cargo run --package tui-client
```

---

## 🛑 Parar os Serviços

```bash
# Parar collector
pkill -f "target/release/collector"

# Parar frontend
pkill -f "vite"

# Ou parar tudo
pkill -f "collector|vite"
```

---

## 🔄 Reiniciar os Serviços

```bash
# Collector
DATABASE_URL="sqlite://./data/system-monitor.db" RUST_LOG=info nohup ./target/release/collector > collector.log 2>&1 &

# Frontend
cd web-frontend && npm run dev &
```

---

## 📝 Logs

### Collector Logs:
```bash
tail -f collector.log
```

### Ver métricas sendo coletadas:
```bash
watch -n 2 "curl -s http://localhost:5253/api/v1/metrics/current | jq '.cpu.global_usage, .memory.usage_percent'"
```

---

## ✅ Checklist de Verificação

- [x] Collector compilado
- [x] Collector rodando
- [x] API respondendo (porta 8080)
- [x] Frontend compilado
- [x] Frontend rodando (porta 3000)
- [x] Database criado (./data/system-monitor.db)
- [x] Database migrations executadas
- [x] Métricas sendo persistidas no banco
- [x] Logs sendo gerados

---

## 🎯 Próximos Passos

1. **Acessar** http://localhost:3000 no navegador
2. **Ver** métricas em tempo real
3. **Testar** clientes (TUI, API)
4. **Opcional**: Instalar como serviço systemd com `sudo ./scripts/install.sh`

---

**Tudo funcionando! 🎉**
