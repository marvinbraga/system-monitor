# Docker Quick Start Guide

⚠️ **IMPORTANT NOTICE ABOUT DOCKER USAGE** ⚠️

This guide explains **ONLY** how to run the **Web Frontend** in Docker. The **Collector service MUST NOT be run in Docker** due to architectural constraints.

---

## Why Docker Limitations Exist

### ❌ Collector CANNOT Run in Docker

**Docker containers have namespace isolation that prevents accurate metric collection:**

```
Docker Container Issues:
├─ Isolated namespace
├─ Only sees container's /proc (not host's)
├─ CPU/memory metrics reflect container limits, not host
├─ Temperature sensors (/sys/class/hwmon/) inaccessible
├─ USB devices not visible
└─ Results in inaccurate or incomplete metrics
```

### ✅ Correct Architecture

```
┌─────────────────────────────────────────────────────┐
│                   PHYSICAL HOST                     │
│                                                     │
│  ┌──────────────────────────────────────────────┐  │
│  │  COLLECTOR (Native Systemd Service)          │  │
│  │  • Native Rust binary                        │  │
│  │  • Direct access to /proc, /sys, /dev        │  │
│  │  • Port: 5253                                │  │
│  │  • Installation: sudo ./scripts/install.sh   │  │
│  └──────────────────────────────────────────────┘  │
│                        ↑                            │
│                        │ HTTP/WebSocket             │
│                        │                            │
│  ┌──────────────────────────────────────────────┐  │
│  │  WEB FRONTEND (Optional - CAN use Docker)   │  │
│  │  • React application                         │  │
│  │  • Nginx web server                          │  │
│  │  • Port: 5252                                │  │
│  │  • Deployment: Docker OR native npm          │  │
│  └──────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

---

## Quick Start (3 Steps)

### Step 1: Install Collector Natively (Required)

```bash
# Navigate to project directory
cd /home/marvinbraga/dados/system-monitor

# Install collector as systemd service
sudo ./scripts/install.sh

# Verify installation
sudo systemctl status system-monitor-collector
curl http://localhost:5253/health
```

### Step 2: Choose Frontend Deployment

**Option A: Docker (Isolated)**

```bash
# Build and start frontend container
docker compose up -d frontend

# Access dashboard
# http://localhost:5252
```

**Option B: Native (Development)**

```bash
# Install dependencies
cd web-frontend
npm install

# Start development server
npm run dev

# Access dashboard
# http://localhost:5252
```

### Step 3: Verify System

```bash
# Check collector service
sudo systemctl status system-monitor-collector

# Test API
curl http://localhost:5253/api/v1/metrics/current | jq

# Check frontend (browser or curl)
curl http://localhost:5252
```

---

## Docker Compose Configuration

The `docker-compose.yml` is configured **ONLY for the frontend**:

```yaml
services:
  frontend:
    build:
      context: ./web-frontend
      dockerfile: Dockerfile
    container_name: system-monitor-frontend
    ports:
      - "5252:80"
    environment:
      # Frontend connects to collector on HOST (not in container)
      - VITE_API_URL=http://host.docker.internal:5253
    extra_hosts:
      # Allows access to host via host.docker.internal
      - "host.docker.internal:host-gateway"
    restart: unless-stopped
```

**Key Configuration:**
- Frontend runs in Docker on port **5252**
- Collector runs natively on host on port **5253**
- Frontend accesses collector via `host.docker.internal:5253`

---

## Docker Commands (Frontend Only)

### Build and Run

```bash
# Build frontend image
docker compose build frontend

# Start frontend container (detached)
docker compose up -d frontend

# Start with logs visible
docker compose up frontend

# Rebuild without cache
docker compose build --no-cache frontend
```

### Service Management

```bash
# Stop frontend
docker compose stop frontend

# Stop and remove container
docker compose down

# Restart frontend
docker compose restart frontend

# View logs
docker compose logs -f frontend

# Container status
docker compose ps
```

### Maintenance

```bash
# Enter frontend container
docker compose exec frontend /bin/sh

# View resource usage
docker stats system-monitor-frontend

# Remove everything (container + volumes)
docker compose down -v
```

---

## Environment Configuration

### Frontend Environment Variables

Configure in `web-frontend/.env` or `docker-compose.yml`:

```bash
# Collector API endpoint
VITE_API_URL=http://host.docker.internal:5253

# WebSocket endpoint (auto-derived from API_URL if not set)
VITE_WS_URL=ws://host.docker.internal:5253/ws

# Refresh interval (milliseconds)
VITE_REFRESH_INTERVAL=2000
```

### Collector Configuration

Collector is configured via `/etc/system-monitor/config.toml` (created by install script):

```toml
# Database settings
database_url = "/var/lib/system-monitor/metrics.db"

# Collection settings
collection_interval = 2  # seconds

# API settings
api_host = "127.0.0.1"
api_port = 5253

# Retention policy
retention_days = 30

# Logging
log_level = "info"

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

---

## Accessing Services

### Web Dashboard (Frontend)

```bash
# Browser
xdg-open http://localhost:5252

# Or specific browser
google-chrome http://localhost:5252
firefox http://localhost:5252

# Test with curl
curl http://localhost:5252
```

### API Backend (Collector)

```bash
# Health check
curl http://localhost:5253/health

# Current metrics
curl http://localhost:5253/api/v1/metrics/current | jq

# Historical metrics
curl "http://localhost:5253/api/v1/metrics/history?limit=100" | jq

# Anomalies
curl http://localhost:5253/api/v1/anomalies | jq

# Filter anomalies by severity
curl "http://localhost:5253/api/v1/anomalies?severity=critical" | jq
```

### WebSocket

```bash
# Using wscat (npm install -g wscat)
wscat -c ws://localhost:5253/ws

# Using websocat (cargo install websocat)
websocat ws://localhost:5253/ws
```

---

## Troubleshooting

### Frontend Container Won't Start

```bash
# View detailed logs
docker compose logs frontend

# Check container status
docker compose ps

# Inspect container
docker inspect system-monitor-frontend

# Rebuild from scratch
docker compose down
docker compose build --no-cache frontend
docker compose up -d frontend
```

### Frontend Can't Connect to Collector

**Symptoms**: Dashboard shows "Connection Error" or "API Unavailable"

**Solutions**:

```bash
# 1. Verify collector is running
sudo systemctl status system-monitor-collector
curl http://localhost:5253/health

# 2. Check collector logs
sudo journalctl -u system-monitor-collector -n 50

# 3. Restart collector if needed
sudo systemctl restart system-monitor-collector

# 4. Verify network configuration in docker-compose.yml
grep -A 5 "extra_hosts" docker-compose.yml

# 5. Test connectivity from frontend container
docker compose exec frontend ping host.docker.internal
docker compose exec frontend wget -O- http://host.docker.internal:5253/health
```

### Port Already in Use

```bash
# Check what's using port 5252
sudo lsof -i :5252

# Kill process (if safe)
sudo kill -9 <PID>

# Or change port in docker-compose.yml:
# ports:
#   - "5253:80"  # Changed from 5252 to 5253
```

### Permission Issues

```bash
# If collector can't access system files
sudo systemctl status system-monitor-collector

# Check service user permissions
ls -l /var/lib/system-monitor/

# Reinstall collector with proper permissions
sudo ./scripts/uninstall.sh
sudo ./scripts/install.sh
```

---

## Production Deployment

### 1. Install Collector on Production Server

```bash
# SSH into production server
ssh admin@production-server

# Clone repository
git clone <repository-url> /opt/system-monitor
cd /opt/system-monitor

# Install collector as systemd service
sudo ./scripts/install.sh

# Verify
sudo systemctl status system-monitor-collector
curl http://localhost:5253/health
```

### 2. Deploy Frontend

**Option A: Docker on Production**

```bash
# Build and start
docker compose up -d frontend

# Verify
docker compose ps
curl http://localhost:5252
```

**Option B: Native with Nginx**

```bash
# Build frontend
cd web-frontend
npm run build

# Copy to web root
sudo cp -r dist/* /var/www/system-monitor/

# Configure nginx (see example below)
```

### 3. Configure Reverse Proxy

**Nginx Configuration** (`/etc/nginx/sites-available/system-monitor`):

```nginx
server {
    listen 80;
    server_name monitor.example.com;

    # Frontend
    location / {
        proxy_pass http://localhost:5252;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }

    # API
    location /api/ {
        proxy_pass http://localhost:5253/api/;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }

    # WebSocket
    location /ws {
        proxy_pass http://localhost:5253/ws;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "Upgrade";
        proxy_set_header Host $host;
    }
}
```

Enable site:
```bash
sudo ln -s /etc/nginx/sites-available/system-monitor /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 4. Enable HTTPS (Optional)

```bash
# Using Let's Encrypt with Certbot
sudo certbot --nginx -d monitor.example.com

# Or manually configure SSL certificates
```

---

## Remote Access Configuration

By default, the collector binds to `127.0.0.1` (localhost only). To allow remote access:

### 1. Edit Collector Configuration

```bash
# Edit config file
sudo nano /etc/system-monitor/config.toml

# Change:
# api_host = "127.0.0.1"
# To:
# api_host = "0.0.0.0"
```

### 2. Restart Collector

```bash
sudo systemctl restart system-monitor-collector
```

### 3. Configure Firewall

```bash
# UFW
sudo ufw allow 5253/tcp

# iptables
sudo iptables -A INPUT -p tcp --dport 5253 -j ACCEPT
```

### 4. Update Frontend Configuration

If frontend is on a different machine:

```bash
# In docker-compose.yml or .env
VITE_API_URL=http://production-server-ip:5253
VITE_WS_URL=ws://production-server-ip:5253/ws
```

⚠️ **Security Warning**: Exposing the API publicly without authentication is not recommended. Use a reverse proxy with authentication or VPN for production.

---

## Monitoring and Health Checks

### Collector Health

```bash
# Systemd status
sudo systemctl status system-monitor-collector

# Check if responding
curl http://localhost:5253/health

# View logs
sudo journalctl -u system-monitor-collector -f

# Check resource usage
ps aux | grep collector
```

### Frontend Health (Docker)

```bash
# Container status
docker compose ps

# Health check (if configured)
docker inspect system-monitor-frontend | jq '.[0].State.Health'

# Resource usage
docker stats system-monitor-frontend

# Logs
docker compose logs -f frontend
```

### Automated Monitoring

```bash
# Add to crontab for periodic health checks
*/5 * * * * curl -f http://localhost:5253/health || systemctl restart system-monitor-collector

# Or use systemd timer for health checks
```

---

## Updating the System

### Update Collector

```bash
# Pull latest changes
cd /opt/system-monitor
git pull

# Reinstall
sudo ./scripts/uninstall.sh
sudo ./scripts/install.sh

# Or just rebuild and restart
cargo build --release --package collector
sudo systemctl restart system-monitor-collector
```

### Update Frontend (Docker)

```bash
# Pull latest changes
git pull

# Rebuild and restart
docker compose build --no-cache frontend
docker compose up -d frontend
```

### Update Frontend (Native)

```bash
# Pull latest changes
git pull

# Rebuild
cd web-frontend
npm install
npm run build

# Copy to web root (if using nginx)
sudo cp -r dist/* /var/www/system-monitor/
```

---

## Summary of Deployment Options

### Recommended Production Setup

| Component | Deployment Method | Port | Notes |
|-----------|------------------|------|-------|
| **Collector** | Native systemd service | 5253 | **REQUIRED** - Must be native |
| **Frontend** | Docker or Native nginx | 5252 | Optional - Choose based on preference |
| **Reverse Proxy** | Nginx on host | 80/443 | Recommended for production |

### Development Setup

| Component | Deployment Method | Port | Notes |
|-----------|------------------|------|-------|
| **Collector** | Native (cargo run) | 5253 | Direct execution for testing |
| **Frontend** | npm dev server | 5252 | Hot reload for development |

---

## Quick Reference Commands

```bash
# === COLLECTOR (Native) ===
# Install
sudo ./scripts/install.sh

# Status
sudo systemctl status system-monitor-collector

# Logs
sudo journalctl -u system-monitor-collector -f

# Restart
sudo systemctl restart system-monitor-collector

# Test
curl http://localhost:5253/health


# === FRONTEND (Docker) ===
# Start
docker compose up -d frontend

# Logs
docker compose logs -f frontend

# Restart
docker compose restart frontend

# Stop
docker compose down

# Test
curl http://localhost:5252


# === FRONTEND (Native) ===
# Install & run
cd web-frontend
npm install
npm run dev

# Build
npm run build

# Test
curl http://localhost:5252
```

---

## Need Help?

1. **Collector Issues**: Check systemd logs
   ```bash
   sudo journalctl -u system-monitor-collector -n 100
   ```

2. **Frontend Issues**: Check Docker logs
   ```bash
   docker compose logs frontend
   ```

3. **Connection Issues**: Verify both services are running
   ```bash
   curl http://localhost:5253/health
   curl http://localhost:5252
   ```

4. **Documentation**: See comprehensive guides
   - [Architecture](docs/ARCHITECTURE.md)
   - [Clients](docs/CLIENTS.md)
   - [README](README.md)

---

## Important Reminders

✅ **DO:**
- Install collector natively using `./scripts/install.sh`
- Run frontend in Docker if desired (optional)
- Use reverse proxy for production deployments
- Configure firewall for remote access
- Monitor collector via systemd logs

❌ **DON'T:**
- Run collector in Docker (metrics will be inaccurate)
- Expose API publicly without authentication
- Skip systemd installation for production
- Use development mode in production

---

**Ready to Deploy!** 🚀

Follow the 3-step quick start above to get System Monitor running correctly with native collector and optional Docker frontend.
