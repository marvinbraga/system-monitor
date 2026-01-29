# 🐳 Docker Compose - Guia Rápido

## Sistema Completo com Backend + Frontend

O `docker-compose.yml` já está configurado e pronto para rodar todo o sistema System Monitor com apenas um comando.

---

## 🚀 Início Rápido (3 passos)

### 1. Build das imagens
```bash
cd /home/marvinbraga/dados/system-monitor
docker-compose build
```

### 2. Iniciar os serviços
```bash
docker-compose up -d
```

### 3. Acessar
- **Frontend Web**: http://localhost:3000
- **API Backend**: http://localhost:8080
- **Health Check**: http://localhost:8080/health
- **WebSocket**: ws://localhost:8080/ws

---

## 📊 Arquitetura do Docker Compose

```
┌─────────────────────────────────────────────┐
│            Docker Network                   │
│         (monitor-network)                   │
│                                             │
│  ┌─────────────────┐    ┌────────────────┐ │
│  │   COLLECTOR     │◄───┤   WEB (React)  │ │
│  │   (Backend)     │    │   (Frontend)   │ │
│  │                 │    │                │ │
│  │ - Rust Service  │    │ - Nginx        │ │
│  │ - REST API      │    │ - React App    │ │
│  │ - WebSocket     │    │                │ │
│  │ - SQLite DB     │    │                │ │
│  │                 │    │                │ │
│  │ Port: 8080      │    │ Port: 3000     │ │
│  └─────────────────┘    └────────────────┘ │
│         │                                   │
│         ▼                                   │
│  ┌─────────────────┐                       │
│  │   Volume        │                       │
│  │ collector-data  │                       │
│  │ (SQLite + logs) │                       │
│  └─────────────────┘                       │
└─────────────────────────────────────────────┘
```

---

## 🛠️ Comandos Úteis

### Gerenciamento dos Serviços

```bash
# Iniciar (em background)
docker-compose up -d

# Iniciar (com logs visíveis)
docker-compose up

# Parar
docker-compose stop

# Parar e remover containers
docker-compose down

# Parar e remover containers + volumes
docker-compose down -v

# Reiniciar
docker-compose restart

# Reiniciar apenas um serviço
docker-compose restart collector
docker-compose restart web
```

### Logs e Monitoramento

```bash
# Ver logs de todos os serviços
docker-compose logs -f

# Ver logs apenas do collector
docker-compose logs -f collector

# Ver logs apenas do web
docker-compose logs -f web

# Ver últimas 50 linhas
docker-compose logs --tail=50

# Status dos serviços
docker-compose ps

# Verificar health checks
docker-compose ps --format json | jq -r '.[] | "\(.Name): \(.Health)"'
```

### Build e Rebuild

```bash
# Build inicial
docker-compose build

# Rebuild forçado (sem cache)
docker-compose build --no-cache

# Rebuild apenas um serviço
docker-compose build collector
docker-compose build web

# Build e iniciar
docker-compose up -d --build
```

---

## 🔧 Configuração

### Variáveis de Ambiente

O `docker-compose.yml` usa variáveis de ambiente configuráveis:

**Collector (Backend):**
- `RUST_LOG` - Nível de log (debug, info, warn, error)
- `DATABASE_URL` - Caminho do banco SQLite
- `HOST` - Host do servidor (0.0.0.0 para Docker)
- `PORT` - Porta da API (8080)
- `COLLECTION_INTERVAL_SECS` - Intervalo de coleta (5s)

**Web (Frontend):**
- `REACT_APP_API_URL` - URL da API backend
- `REACT_APP_WS_URL` - URL do WebSocket

### Personalizar Configuração

Crie um arquivo `.env` no diretório raiz:

```bash
# .env
RUST_LOG=debug
COLLECTION_INTERVAL_SECS=2
DATA_DIR=./data
```

Depois reinicie:
```bash
docker-compose down
docker-compose up -d
```

---

## 📁 Volumes e Dados

### Volume Persistente

O banco de dados SQLite e logs são armazenados no volume `collector-data`:

```bash
# Ver volumes
docker volume ls | grep system-monitor

# Inspecionar volume
docker volume inspect system-monitor_collector-data

# Backup do banco de dados
docker cp system-monitor-collector:/data/system-monitor.db ./backup.db

# Restaurar backup
docker cp ./backup.db system-monitor-collector:/data/system-monitor.db
```

### Localização dos Dados

Por padrão, os dados ficam em:
- **Local**: `./data/` (bind mount)
- **Container**: `/data/`

Para mudar a localização, edite no `docker-compose.yml` ou use variável de ambiente:
```bash
DATA_DIR=/caminho/personalizado docker-compose up -d
```

---

## 🌐 Acessando os Serviços

### Frontend Web (React)
```bash
# Abrir no navegador
xdg-open http://localhost:3000

# Ou
google-chrome http://localhost:3000
firefox http://localhost:3000
```

### API Backend
```bash
# Health check
curl http://localhost:8080/health

# Métricas atuais
curl http://localhost:8080/api/v1/metrics/current | jq

# Anomalias
curl http://localhost:8080/api/v1/anomalies | jq

# Histórico
curl "http://localhost:8080/api/v1/metrics/history?limit=10" | jq

# Informações do sistema
curl http://localhost:8080/api/v1/system/info | jq
```

### WebSocket
```bash
# Instalar websocat se não tiver
cargo install websocat

# Conectar ao WebSocket
websocat ws://localhost:8080/ws
```

---

## 🔍 Debug e Troubleshooting

### Container não inicia

```bash
# Ver logs detalhados
docker-compose logs collector

# Verificar status
docker-compose ps

# Entrar no container
docker-compose exec collector /bin/sh
```

### Porta já em uso

```bash
# Verificar quem está usando a porta 8080
sudo lsof -i :8080

# Mudar porta no docker-compose.yml
# De:
#   - "8080:8080"
# Para:
#   - "8081:8080"
```

### Rebuild completo

```bash
# Parar tudo
docker-compose down -v

# Limpar imagens antigas
docker image prune -a

# Rebuild do zero
docker-compose build --no-cache

# Iniciar
docker-compose up -d
```

### Ver recursos consumidos

```bash
# CPU e memória por container
docker stats

# Apenas system-monitor
docker stats system-monitor-collector system-monitor-web
```

---

## 🔒 Segurança

O `docker-compose.yml` inclui hardening de segurança:

- ✅ **no-new-privileges**: Previne escalação de privilégios
- ✅ **cap_drop: ALL**: Remove todas as capabilities
- ✅ **cap_add**: Adiciona apenas as necessárias
- ✅ **read_only**: Filesystem readonly no web
- ✅ **tmpfs**: Diretórios temporários em memória
- ✅ **Health checks**: Monitora saúde dos containers
- ✅ **Restart policies**: Reinicia automaticamente em falhas

---

## 📊 Monitoramento em Produção

### Verificar Health

```bash
# Status de saúde
docker inspect system-monitor-collector | jq '.[0].State.Health'

# Loop de monitoramento
watch -n 2 'docker-compose ps'
```

### Métricas do Docker

```bash
# Estatísticas em tempo real
docker stats --no-stream

# Uso de disco
docker system df

# Logs com timestamp
docker-compose logs -f --timestamps
```

---

## 🚀 Produção

### Deploy em servidor remoto

```bash
# 1. Clonar repositório no servidor
git clone <repo> /opt/system-monitor
cd /opt/system-monitor

# 2. Build
docker-compose build

# 3. Iniciar como daemon
docker-compose up -d

# 4. Verificar
docker-compose ps
curl http://localhost:8080/health
```

### Proxy reverso (Nginx/Caddy)

Se quiser expor na porta 80/443:

```nginx
# /etc/nginx/sites-available/system-monitor
server {
    listen 80;
    server_name monitor.example.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    location /api/ {
        proxy_pass http://localhost:8080/api/;
    }

    location /ws {
        proxy_pass http://localhost:8080/ws;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
    }
}
```

---

## 🔄 Atualização

```bash
# 1. Pull mudanças
git pull

# 2. Rebuild
docker-compose build

# 3. Restart (zero downtime)
docker-compose up -d

# Ou com recreate forçado
docker-compose up -d --force-recreate
```

---

## 📝 Estrutura do Projeto Docker

```
system-monitor/
├── docker-compose.yml          # ← Orquestração principal
├── collector/
│   └── Dockerfile             # ← Build do backend
├── web-frontend/
│   └── Dockerfile             # ← Build do frontend
├── nginx.conf                 # ← Config do Nginx
├── .dockerignore              # ← Arquivos ignorados
└── data/                      # ← Volume de dados
    ├── system-monitor.db
    └── anomalies.log
```

---

## ✅ Checklist de Verificação

Após `docker-compose up -d`, verifique:

- [ ] Containers rodando: `docker-compose ps`
- [ ] Health checks OK: `docker inspect system-monitor-collector | jq '.[0].State.Health.Status'`
- [ ] Logs sem erros: `docker-compose logs --tail=50`
- [ ] API respondendo: `curl http://localhost:8080/health`
- [ ] Frontend acessível: `curl http://localhost:3000`
- [ ] WebSocket funcionando: `websocat ws://localhost:8080/ws`
- [ ] Dados persistindo: `ls -lh ./data/`

---

## 🎯 Resumo dos Comandos Mais Usados

```bash
# Iniciar tudo
docker-compose up -d

# Ver logs
docker-compose logs -f

# Parar tudo
docker-compose down

# Status
docker-compose ps

# Reiniciar
docker-compose restart

# Rebuild
docker-compose build --no-cache && docker-compose up -d
```

---

## 🆘 Precisa de Ajuda?

1. Verifique os logs: `docker-compose logs -f`
2. Verifique health: `docker-compose ps`
3. Teste a API: `curl http://localhost:8080/health`
4. Veja a documentação completa: `DEPLOYMENT.md`

---

**Pronto para uso! 🚀**

Execute `docker-compose up -d` e acesse http://localhost:3000
