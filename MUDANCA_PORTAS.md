# 🔧 Mudança de Portas - System Monitor

**Data**: 29 de Janeiro de 2026, 13:45 UTC-3
**Status**: ✅ CONCLUÍDO

---

## 📋 Alterações Realizadas

### Portas Antigas → Novas

| Serviço | Porta Antiga | Porta Nova |
|---------|--------------|------------|
| **Frontend (React + Vite)** | 3000 | **5252** |
| **Backend (Collector API)** | 8080 | **5253** |

---

## 📝 Arquivos Modificados

### 1. Backend Configuration
**Arquivo**: `collector/src/main.rs`

```rust
// Linha 24-26: Porta padrão alterada
let port = std::env::var("PORT")
    .unwrap_or_else(|_| "5253".to_string())  // Era 8080
    .parse::<u16>()
    .unwrap_or(5253);  // Era 8080

// Linha 44: CORS atualizado
cors_origins: vec!["http://localhost:5252".to_string()],  // Era 3000
```

**Rebuild necessário**: ✅ Executado (`cargo build --release`)

---

### 2. Frontend Configuration
**Arquivo**: `web-frontend/vite.config.ts`

```typescript
// Linha 8: Porta do servidor Vite
port: 5252,  // Era 3000

// Linha 11-12: Proxy para backend
target: 'http://localhost:5253',  // Era 8080

// Linha 15-16: WebSocket proxy
target: 'ws://localhost:5253',  // Era 8080
```

---

### 3. WebSocket Client
**Arquivo**: `web-frontend/src/api/websocket.ts`

```typescript
// Linha 33: URL dinâmica baseada no host
constructor(url: string = `ws://${window.location.host}/ws`) {
  // Antes era: 'ws://localhost:8080/ws'
  // Agora usa o host atual (localhost:5252) e o proxy do Vite redireciona
}
```

**Benefício**: Funciona automaticamente com qualquer porta configurada no Vite.

---

## ✅ Serviços Rodando

### Status Atual
```bash
$ ps aux | grep -E "(vite|collector)" | grep -v grep

# Frontend
marvinb+   92040  vite (Node.js) - Port 5252

# Backend
marvinb+   92495  collector - Port 5253
```

### Testes de Verificação
```bash
# Backend Health Check
$ curl http://localhost:5253/health
{"metrics_available":true,"status":"healthy","timestamp":"2026-01-29T16:45:27..."}

# Frontend
$ curl -I http://localhost:5252
HTTP/1.1 200 OK

# Métricas
$ curl -s http://localhost:5253/api/v1/metrics/current | jq '.data.cpu.global_usage'
3.92
```

---

## 🌐 Acessos

### Frontend (Dashboard Web)
- **URL**: http://localhost:5252
- **Tecnologia**: React 18.2 + Vite 5.4
- **Proxy**: Vite redireciona /api e /ws para o backend

### Backend (REST API)
- **URL Base**: http://localhost:5253
- **Endpoints**:
  - Health: http://localhost:5253/health
  - Métricas: http://localhost:5253/api/v1/metrics/current
  - Histórico: http://localhost:5253/api/v1/metrics/history
  - Anomalias: http://localhost:5253/api/v1/anomalies
  - WebSocket: ws://localhost:5253/ws

---

## 🔄 Como Reiniciar

### Parar Serviços
```bash
pkill -f "vite"
pkill -f "target/release/collector"
```

### Iniciar Backend
```bash
DATABASE_URL="sqlite://./data/system-monitor.db" \
RUST_LOG=info \
nohup ./target/release/collector > collector.log 2>&1 &
```

### Iniciar Frontend
```bash
cd web-frontend
nohup npm run dev > ../frontend.log 2>&1 &
```

---

## 📚 Documentação Atualizada

Os seguintes arquivos foram atualizados com as novas portas:
- ✅ `SERVICOS_RODANDO.md`
- ✅ `STATUS_ATUAL.md`
- ✅ `MUDANCA_PORTAS.md` (este arquivo)

---

## 🎯 Próximos Passos

Se quiser usar as portas antigas (3000 e 8080), basta reverter as mudanças:

```bash
# 1. Editar collector/src/main.rs (porta 5253 → 8080)
# 2. Editar web-frontend/vite.config.ts (porta 5252 → 3000, proxy 5253 → 8080)
# 3. Rebuild: cargo build --release --package collector
# 4. Reiniciar ambos os serviços
```

---

## 📊 Database

O database continua funcionando normalmente:
```bash
$ ls -lh ./data/system-monitor.db
-rw-r--r-- 1.2M jan 29 13:44 system-monitor.db
```

Métricas sendo persistidas a cada 2 segundos.

---

**Mudança de portas concluída com sucesso! ✅**
