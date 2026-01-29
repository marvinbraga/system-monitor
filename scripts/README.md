# System Monitor - Installation Scripts

Scripts para instalação e desinstalação do serviço System Monitor como daemon systemd.

## 📦 Scripts Disponíveis

### 1. `install.sh` - Instalação do Serviço

Script completo que automatiza toda a instalação do System Monitor como serviço systemd.

**O que o script faz:**

1. ✅ Verifica permissões (requer sudo)
2. ✅ Cria usuário/grupo do sistema (`monitor`)
3. ✅ Cria diretórios necessários:
   - `/opt/system-monitor` - Binários
   - `/var/lib/system-monitor` - Dados e database
   - `/var/log/system-monitor` - Logs
   - `/etc/system-monitor` - Configuração
4. ✅ Compila o projeto em modo release
5. ✅ Instala o binário do collector
6. ✅ Cria arquivo de configuração (`config.toml`)
7. ✅ Cria serviço systemd
8. ✅ Habilita e inicia o serviço
9. ✅ Cria scripts auxiliares (status, logs, restart)

**Uso:**
```bash
cd ~/dados/system-monitor
sudo ./scripts/install.sh
```

**Após a instalação:**
```bash
# Verificar status
systemctl status system-monitor

# Ver logs em tempo real
journalctl -u system-monitor -f

# Testar API
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8080/api/v1/metrics/current
```

### 2. `uninstall.sh` - Desinstalação do Serviço

Script completo que remove o System Monitor do sistema.

**O que o script faz:**

1. ✅ Solicita confirmação
2. ✅ Pergunta se deve remover dados e logs
3. ✅ Cria backup opcional dos dados
4. ✅ Para o serviço
5. ✅ Desabilita o serviço
6. ✅ Remove arquivo systemd
7. ✅ Remove diretórios de instalação
8. ✅ Remove configuração
9. ✅ Remove dados e logs (opcional)
10. ✅ Remove usuário e grupo do sistema
11. ✅ Limpa logs do systemd journal

**Uso:**
```bash
cd ~/dados/system-monitor
sudo ./scripts/uninstall.sh
```

**Opções:**
```bash
# Desinstalar sem confirmações (remove tudo)
sudo ./scripts/uninstall.sh --yes

# Desinstalar interativamente (pergunta sobre dados)
sudo ./scripts/uninstall.sh
```

**Preservar dados:**
- Durante a desinstalação, você pode escolher manter os dados
- Se escolher "N" quando perguntado, os diretórios `/var/lib/system-monitor` e `/var/log/system-monitor` serão preservados
- Para remover manualmente depois: `sudo rm -rf /var/lib/system-monitor /var/log/system-monitor`

---

## 🔧 Configuração

### Arquivo de Configuração

Localização: `/etc/system-monitor/config.toml`

```toml
# Database settings
database_url = "/var/lib/system-monitor/metrics.db"

# Collection settings
collection_interval = 2  # seconds

# API settings
api_host = "127.0.0.1"
api_port = 8080

# Retention policy
retention_days = 30  # Keep data for 30 days

# Logging
log_level = "info"  # trace, debug, info, warn, error

# Anomaly detection thresholds
[thresholds]
cpu_critical = 90.0
cpu_warning = 70.0
memory_critical = 95.0
memory_warning = 80.0
temperature_critical = 85.0
temperature_warning = 75.0
disk_critical = 90.0
disk_warning = 80.0
```

**Aplicar mudanças:**
```bash
sudo systemctl restart system-monitor
```

---

## 📂 Estrutura Instalada

```
/opt/system-monitor/
├── collector              # Binário principal
├── status.sh             # Script auxiliar - status
├── logs.sh               # Script auxiliar - logs
└── restart.sh            # Script auxiliar - restart

/etc/system-monitor/
└── config.toml           # Configuração

/var/lib/system-monitor/
└── metrics.db            # Database SQLite

/var/log/system-monitor/
└── (logs gerenciados pelo systemd journal)

/etc/systemd/system/
└── system-monitor.service  # Serviço systemd
```

---

## 🚀 Comandos Úteis

### Gerenciamento do Serviço

```bash
# Status
systemctl status system-monitor

# Parar
sudo systemctl stop system-monitor

# Iniciar
sudo systemctl start system-monitor

# Reiniciar
sudo systemctl restart system-monitor

# Desabilitar (não inicia no boot)
sudo systemctl disable system-monitor

# Habilitar (inicia no boot)
sudo systemctl enable system-monitor
```

### Logs

```bash
# Ver logs em tempo real
journalctl -u system-monitor -f

# Ver últimas 100 linhas
journalctl -u system-monitor -n 100

# Ver logs desde hoje
journalctl -u system-monitor --since today

# Ver logs de erro
journalctl -u system-monitor -p err

# Exportar logs
journalctl -u system-monitor > system-monitor.log
```

### Scripts Auxiliares

Os scripts auxiliares são instalados em `/opt/system-monitor/`:

```bash
# Ver status
/opt/system-monitor/status.sh

# Ver logs em tempo real
/opt/system-monitor/logs.sh

# Reiniciar serviço
sudo /opt/system-monitor/restart.sh
```

---

## 🔒 Segurança

O serviço systemd é configurado com várias proteções:

- **Usuário dedicado:** Roda como `monitor` (não root)
- **NoNewPrivileges:** Impede escalação de privilégios
- **ProtectSystem:** Protege sistema de arquivos
- **ProtectHome:** Protege diretórios home
- **PrivateTmp:** Diretório /tmp isolado
- **ReadWritePaths:** Apenas data e log writeable

---

## 🌐 API Endpoints

Após instalação, a API estará disponível em:

```bash
# Health check
curl http://127.0.0.1:8080/health

# Métricas atuais
curl http://127.0.0.1:8080/api/v1/metrics/current

# Histórico (últimos 10)
curl "http://127.0.0.1:8080/api/v1/metrics/history?limit=10"

# Anomalias
curl http://127.0.0.1:8080/api/v1/anomalies

# Informações do sistema
curl http://127.0.0.1:8080/api/v1/system/info

# WebSocket (requer websocat ou similar)
websocat ws://127.0.0.1:8080/ws
```

---

## 🔄 Atualização

Para atualizar para uma nova versão:

```bash
# 1. Parar o serviço
sudo systemctl stop system-monitor

# 2. Fazer pull das mudanças
cd ~/dados/system-monitor
git pull

# 3. Recompilar
cargo build --release --package collector

# 4. Substituir binário
sudo cp target/release/collector /opt/system-monitor/collector

# 5. Reiniciar serviço
sudo systemctl start system-monitor

# 6. Verificar
systemctl status system-monitor
```

Ou simplesmente rode o install.sh novamente:
```bash
sudo ./scripts/install.sh
```

---

## 🐛 Troubleshooting

### Serviço não inicia

```bash
# Ver logs detalhados
journalctl -u system-monitor -n 50

# Verificar permissões
ls -la /var/lib/system-monitor
ls -la /opt/system-monitor

# Verificar configuração
cat /etc/system-monitor/config.toml

# Testar binário manualmente
sudo -u monitor /opt/system-monitor/collector
```

### Porta 8080 já em uso

```bash
# Verificar o que está usando a porta
sudo lsof -i :8080

# Mudar porta na configuração
sudo nano /etc/system-monitor/config.toml
# Alterar api_port = 8081

# Reiniciar
sudo systemctl restart system-monitor
```

### Database corrompido

```bash
# Backup do database atual
sudo cp /var/lib/system-monitor/metrics.db /tmp/metrics.db.backup

# Remover database
sudo rm /var/lib/system-monitor/metrics.db

# Reiniciar (cria novo database)
sudo systemctl restart system-monitor
```

### Permissões incorretas

```bash
# Corrigir permissões
sudo chown -R monitor:monitor /var/lib/system-monitor
sudo chown -R monitor:monitor /var/log/system-monitor
sudo chmod 750 /var/lib/system-monitor
sudo chmod 750 /var/log/system-monitor
```

---

## 📊 Monitoramento

### Consumo de recursos

```bash
# Ver uso de CPU/memória do serviço
systemctl status system-monitor

# Detalhado
top -p $(pgrep -f system-monitor)

# Ou com htop
htop -p $(pgrep -f system-monitor)
```

### Tamanho do database

```bash
# Ver tamanho
du -h /var/lib/system-monitor/metrics.db

# Ver registros
sqlite3 /var/lib/system-monitor/metrics.db "SELECT COUNT(*) FROM metrics;"
sqlite3 /var/lib/system-monitor/metrics.db "SELECT COUNT(*) FROM anomalies;"
```

---

## 📝 Notas

- **Requer Rust:** O script de instalação precisa do Rust/Cargo instalado
- **Requer sudo:** Ambos scripts precisam de permissões de root
- **Systemd:** Requer sistema com systemd (Linux moderno)
- **Backup:** Sempre faça backup antes de desinstalar se tiver dados importantes

---

## 🆘 Suporte

Se encontrar problemas:

1. Verifique os logs: `journalctl -u system-monitor -f`
2. Verifique a configuração: `/etc/system-monitor/config.toml`
3. Teste a API: `curl http://127.0.0.1:8080/health`
4. Verifique permissões: `ls -la /var/lib/system-monitor`

---

**Desenvolvido para Zorin OS / Ubuntu / Debian**
