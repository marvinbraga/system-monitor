# Client Applications

## Table of Contents
- [Overview](#overview)
- [Web Dashboard](#web-dashboard)
- [TUI Client](#tui-client)
- [Direct API Access](#direct-api-access)
- [Client Comparison](#client-comparison)
- [Deployment Scenarios](#deployment-scenarios)
- [Best Practices](#best-practices)

---

## Overview

System Monitor supports **multiple client interfaces**, all consuming the same collector API. This architecture enables flexible deployment scenarios tailored to specific use cases, resource constraints, and user preferences.

### Client Architecture

```
                    ┌─────────────────┐
                    │   COLLECTOR     │
                    │   (Backend)     │
                    │   Port: 5253    │
                    └────────┬────────┘
                             │
                    ┌────────┴────────┐
                    │   API Layer     │
                    │  REST + WebSocket│
                    └────────┬────────┘
                             │
         ┌───────────────────┼───────────────────┐
         │                   │                   │
    ┌────▼────┐        ┌────▼────┐        ┌────▼────┐
    │   Web   │        │   TUI   │        │  Direct │
    │Dashboard│        │ Client  │        │   API   │
    └─────────┘        └─────────┘        └─────────┘
        │                   │                   │
        ▼                   ▼                   ▼
    Browser             Terminal            Scripts
                                           Integrations
```

### Design Principles

- **API-First**: All clients consume standardized REST + WebSocket APIs
- **Stateless Clients**: No local data storage; collector is source of truth
- **Multiple Simultaneous**: Several clients can connect to one collector
- **Interchangeable**: Choose the right client for each scenario

---

## Web Dashboard

### Overview

**Technology**: React 18 + TypeScript + Vite
**Port**: 5252
**Deployment**: Native (npm) or Docker
**Resource Usage**: ~100MB RAM, ~2-5% CPU

### Features

#### Rich Visualization
- **Interactive Charts**: Real-time line charts for CPU, memory, temperature
- **Responsive Design**: Optimized for desktop, tablet, and mobile
- **Color-Coded Metrics**: Visual indicators for health status (green/yellow/red)
- **Historical Trends**: Time-series graphs with customizable ranges

#### Real-Time Updates
- **WebSocket Streaming**: Sub-second metric updates
- **Live Anomaly Feed**: Instant notifications of detected issues
- **Auto-Reconnect**: Resilient connection handling
- **Bandwidth Efficient**: Delta updates only

#### User Experience
- **Multi-User Support**: Concurrent access from multiple browsers
- **Filtering & Search**: Advanced anomaly filtering by severity, category, time
- **Exportable Data**: Download metrics as CSV or JSON
- **Dark/Light Modes**: Theme customization (if implemented)

### Installation & Usage

#### Development Mode (Hot Reload)

```bash
cd web-frontend

# Install dependencies (first time)
npm install

# Start development server
npm run dev

# Access dashboard
# http://localhost:5252
```

**Features**:
- Hot module replacement (HMR)
- Source maps for debugging
- Fast refresh on code changes

#### Production Build

```bash
cd web-frontend

# Build optimized production bundle
npm run build

# Output: dist/ directory
```

**Deployment Options**:

1. **Nginx** (recommended for production):
```nginx
server {
    listen 80;
    server_name monitor.example.com;

    location / {
        root /var/www/system-monitor;
        try_files $uri $uri/ /index.html;
    }

    # Proxy API requests to collector
    location /api/ {
        proxy_pass http://localhost:5253;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

2. **Apache**:
```apache
<VirtualHost *:80>
    ServerName monitor.example.com
    DocumentRoot /var/www/system-monitor

    <Directory /var/www/system-monitor>
        Options -Indexes +FollowSymLinks
        AllowOverride All
        Require all granted
    </Directory>

    ProxyPass /api/ http://localhost:5253/api/
    ProxyPassReverse /api/ http://localhost:5253/api/
</VirtualHost>
```

3. **Static File Hosting**: Deploy `dist/` to Netlify, Vercel, or GitHub Pages

#### Docker Deployment

```bash
# Build and start frontend container
docker compose up -d frontend

# Access dashboard
# http://localhost:5252
```

**Docker Configuration** (`docker-compose.yml`):
```yaml
frontend:
  build: ./web-frontend
  container_name: system-monitor-frontend
  ports:
    - "5252:80"
  environment:
    - VITE_API_URL=http://host.docker.internal:5253
  extra_hosts:
    - "host.docker.internal:host-gateway"
```

### Configuration

**Environment Variables** (`.env` file):

```bash
# Collector API endpoint
VITE_API_URL=http://localhost:5253

# WebSocket endpoint (optional, auto-derived from API_URL)
VITE_WS_URL=ws://localhost:5253/ws

# Refresh interval (milliseconds)
VITE_REFRESH_INTERVAL=2000

# Enable debug logging
VITE_DEBUG=false
```

### Use Cases

✅ **Ideal For**:
- Rich visual monitoring with charts and graphs
- Multi-user dashboards (operations centers, team monitoring)
- Historical trend analysis
- Presentations and demos
- Long-running monitoring displays
- Remote monitoring via web browser

❌ **Not Ideal For**:
- Headless servers without browser access
- SSH-only environments
- Extremely resource-constrained systems
- Automation and scripting
- Offline environments

### Development

#### Project Structure

```
web-frontend/
├── src/
│   ├── api/           # API client (Axios)
│   │   └── client.ts  # HTTP + WebSocket client
│   ├── components/    # React components
│   │   ├── Dashboard.tsx
│   │   ├── CpuChart.tsx
│   │   ├── MemoryChart.tsx
│   │   ├── TemperatureChart.tsx
│   │   └── AnomalyList.tsx
│   ├── hooks/         # Custom React hooks
│   │   ├── useMetrics.ts
│   │   ├── useWebSocket.ts
│   │   └── useAnomalies.ts
│   ├── types/         # TypeScript types
│   │   └── metrics.ts
│   ├── utils/         # Utility functions
│   ├── App.tsx        # Main component
│   └── main.tsx       # Entry point
├── package.json
├── vite.config.ts
├── tsconfig.json
└── tailwind.config.js
```

#### Available Scripts

```bash
npm run dev      # Development server with HMR
npm run build    # Production build
npm run preview  # Preview production build locally
npm run lint     # Run ESLint
npm run type-check # TypeScript type checking
```

---

## TUI Client

### Overview

**Technology**: Rust + Ratatui + Crossterm
**Interface**: Terminal-based ASCII dashboard
**Deployment**: Native binary
**Resource Usage**: ~5MB RAM, ~0.5% CPU

### Features

#### Lightweight Performance
- **Minimal Footprint**: ~5MB RAM (20x less than web dashboard)
- **Low CPU Usage**: ~0.5% CPU overhead
- **Fast Startup**: Instant launch, no build step
- **Efficient Rendering**: Terminal updates only changed regions

#### SSH-Friendly
- **Works Over SSH**: Full functionality via SSH tunnel
- **Tmux/Screen Compatible**: Runs in terminal multiplexers
- **No Display Server**: No X11/Wayland required
- **Low Bandwidth**: Minimal network traffic (terminal escape codes)

#### Keyboard-Driven Navigation
- **Vim Bindings**: `j/k` for up/down navigation
- **Arrow Keys**: Standard navigation support
- **Page Navigation**: `Page Up/Down` for fast scrolling
- **Quick Exit**: `q` or `Ctrl+C` to quit

### Installation & Usage

#### Building

```bash
# Build in release mode (optimized)
cargo build --package tui-client --release

# Binary location
./target/release/tui-client
```

#### Basic Usage

```bash
# Default configuration (localhost:5253, 2-second refresh)
./target/release/tui-client

# Show help
./target/release/tui-client --help
```

#### Advanced Usage

**Remote Server Monitoring**:
```bash
# Connect to remote collector
./target/release/tui-client --api-url http://192.168.1.100:5253

# Use short flag
./target/release/tui-client -u http://remote-server:5253
```

**Custom Refresh Rate**:
```bash
# Refresh every 5 seconds (reduce load)
./target/release/tui-client --refresh 5

# Refresh every 1 second (high-frequency monitoring)
./target/release/tui-client -r 1
```

**WebSocket Mode**:
```bash
# Use WebSocket for real-time streaming
./target/release/tui-client --websocket

# Short flag
./target/release/tui-client -w
```

**Combined Options**:
```bash
# Remote server + WebSocket + 1-second refresh
./target/release/tui-client \
  --api-url http://server:5253 \
  --websocket \
  --refresh 1

# Short flags
./target/release/tui-client -u http://server:5253 -w -r 1
```

### Command-Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--api-url <URL>` | `-u` | Collector API endpoint | `http://localhost:5253` |
| `--refresh <SEC>` | `-r` | Metric refresh interval (seconds) | `2` |
| `--websocket` | `-w` | Use WebSocket instead of HTTP polling | `false` |
| `--help` | `-h` | Show help message | - |

### Keyboard Controls

| Key | Action |
|-----|--------|
| `q` | Quit application |
| `Ctrl+C` | Force quit |
| `↑` or `k` | Scroll up (anomaly list) |
| `↓` or `j` | Scroll down (anomaly list) |
| `Page Up` | Previous page (anomalies) |
| `Page Down` | Next page (anomalies) |
| `Home` | Jump to top |
| `End` | Jump to bottom |

### Use Cases

✅ **Ideal For**:
- SSH access to remote servers
- Headless servers without GUI
- Quick system health checks
- Minimal resource overhead scenarios
- Terminal multiplexer sessions (tmux, screen)
- Offline environments (no browser)
- Rapid troubleshooting

❌ **Not Ideal For**:
- Rich graphical visualizations
- Historical trend analysis with charts
- Multi-user concurrent access
- Long-term monitoring displays
- Presentations requiring visual polish

### Dashboard Layout

```
┌─────────────────────────────────────────────────────────────┐
│ System Monitor - TUI Client                     [Connected] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ CPU Usage:  ████████████░░░░░░░░ 62.3%                     │
│ Memory:     ████████████████░░░░ 78.5%                     │
│ Swap:       ███░░░░░░░░░░░░░░░░░ 15.2%                     │
│                                                             │
│ Temperatures:                                               │
│   CPU (k10temp):  65.0°C  [███████░░░]                     │
│   NVMe SSD:       45.0°C  [████░░░░░░]                     │
│   GPU:            72.0°C  [████████░░]                     │
│                                                             │
│ Disks:                                                      │
│   /dev/nvme0n1p1  ████████████████░░ 82.1% (1.2TB / 1.5TB)│
│   /dev/sda1       ████████░░░░░░░░░░ 45.3% (500GB / 1.0TB)│
│                                                             │
│ Recent Anomalies:                                           │
│ [CRITICAL] CPU spike: 95.2% (cores: 28-31)                │
│ [WARNING]  Memory pressure: 85.0%                          │
│ [INFO]     Network spike: eth0 RX 150MB/s                  │
│                                                             │
│ q: Quit  ↑↓: Scroll  PgUp/PgDn: Page                       │
└─────────────────────────────────────────────────────────────┘
```

---

## Direct API Access

### Overview

**Protocol**: REST (JSON) + WebSocket
**Authentication**: None (configure via reverse proxy if needed)
**Rate Limiting**: None (client-side responsibility)

### REST API Endpoints

#### Health Check

```bash
GET /health

Response:
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime_seconds": 86400
}
```

#### Current Metrics

```bash
GET /api/v1/metrics/current

Response:
{
  "status": "success",
  "timestamp": "2026-01-29T14:00:00Z",
  "data": {
    "cpu": { ... },
    "memory": { ... },
    "temperatures": [ ... ],
    "disks": [ ... ],
    "usb_devices": [ ... ],
    "network": { ... }
  }
}
```

#### Historical Metrics

```bash
GET /api/v1/metrics/history?limit=100&offset=0

Query Parameters:
  - limit: Number of records (default: 100, max: 1000)
  - offset: Skip records (default: 0)

Response:
{
  "status": "success",
  "count": 100,
  "data": [ ... ]
}
```

#### Anomalies

```bash
GET /api/v1/anomalies?limit=50&severity=critical

Query Parameters:
  - limit: Number of records (default: 50)
  - severity: Filter by severity (info, warning, critical)
  - category: Filter by category (cpu, memory, temperature, disk, usb, network)
  - since: ISO timestamp (e.g., 2026-01-29T00:00:00Z)

Response:
{
  "status": "success",
  "count": 50,
  "data": [
    {
      "id": "uuid",
      "timestamp": "2026-01-29T14:00:00Z",
      "severity": "critical",
      "category": "cpu",
      "message": "CPU spike detected: 95.2%",
      "metrics": { ... }
    }
  ]
}
```

### WebSocket API

#### Connection

```bash
ws://localhost:5253/ws
```

#### Message Format

**Subscription** (client → server):
```json
{
  "type": "subscribe",
  "channels": ["metrics", "anomalies"]
}
```

**Metric Update** (server → client):
```json
{
  "type": "metrics",
  "timestamp": "2026-01-29T14:00:00Z",
  "data": { ... }
}
```

**Anomaly Alert** (server → client):
```json
{
  "type": "anomaly",
  "data": {
    "severity": "critical",
    "category": "cpu",
    "message": "CPU spike detected"
  }
}
```

### Usage Examples

#### Shell Scripts

**Monitor CPU Usage**:
```bash
#!/bin/bash

while true; do
  cpu=$(curl -s localhost:5253/api/v1/metrics/current | \
        jq -r '.data.cpu.global_usage')
  echo "CPU: ${cpu}%"
  sleep 5
done
```

**Alert on High Temperature**:
```bash
#!/bin/bash

TEMP_THRESHOLD=80

temps=$(curl -s localhost:5253/api/v1/metrics/current | \
        jq -r '.data.temperatures[] | select(.value > '$TEMP_THRESHOLD') | "\(.sensor): \(.value)°C"')

if [ -n "$temps" ]; then
  echo "ALERT: High temperatures detected!"
  echo "$temps"
  # Send notification (email, Slack, etc.)
fi
```

**Export to CSV**:
```bash
#!/bin/bash

# Fetch last 1000 metrics
curl -s "localhost:5253/api/v1/metrics/history?limit=1000" | \
  jq -r '.data[] | [.timestamp, .cpu.global_usage, .memory.usage_percent] | @csv' \
  > metrics.csv
```

#### Python Integration

```python
import requests
import json

# Fetch current metrics
response = requests.get('http://localhost:5253/api/v1/metrics/current')
metrics = response.json()

cpu_usage = metrics['data']['cpu']['global_usage']
mem_usage = metrics['data']['memory']['usage_percent']

print(f"CPU: {cpu_usage}%")
print(f"Memory: {mem_usage}%")

# Fetch anomalies
response = requests.get('http://localhost:5253/api/v1/anomalies?severity=critical')
anomalies = response.json()

for anomaly in anomalies['data']:
    print(f"[{anomaly['severity'].upper()}] {anomaly['message']}")
```

#### WebSocket Client (Python)

```python
import asyncio
import websockets
import json

async def monitor():
    uri = "ws://localhost:5253/ws"
    async with websockets.connect(uri) as websocket:
        # Subscribe to metrics
        await websocket.send(json.dumps({
            "type": "subscribe",
            "channels": ["metrics", "anomalies"]
        }))

        # Receive updates
        while True:
            message = await websocket.recv()
            data = json.loads(message)

            if data['type'] == 'metrics':
                cpu = data['data']['cpu']['global_usage']
                print(f"CPU: {cpu}%")
            elif data['type'] == 'anomaly':
                print(f"ALERT: {data['data']['message']}")

asyncio.run(monitor())
```

### Use Cases

✅ **Ideal For**:
- Automation and scripting
- CI/CD pipeline health checks
- Custom monitoring dashboards
- Integration with third-party tools (Grafana, Prometheus, Datadog)
- Alert systems and notifications
- Data export and analysis
- Programmatic access from any language

❌ **Not Ideal For**:
- Manual interactive monitoring (use Web or TUI)
- Non-technical users
- Scenarios requiring visual dashboards

---

## Client Comparison

### Feature Matrix

| Feature | Web Dashboard | TUI Client | Direct API |
|---------|--------------|------------|------------|
| **Interface** | Graphical (Browser) | Terminal (ASCII) | Programmatic (JSON) |
| **Deployment** | npm / Docker | Native binary | N/A |
| **Resource Usage (RAM)** | ~100MB | ~5MB | ~0MB |
| **Resource Usage (CPU)** | ~2-5% | ~0.5% | ~0% |
| **SSH Compatible** | ❌ | ✅ | ✅ |
| **Multi-User** | ✅ | ❌ | ✅ |
| **Real-Time Updates** | ✅ WebSocket | ✅ Polling/WS | ✅ WebSocket |
| **Historical Charts** | ✅ Rich graphs | ⚠️ ASCII only | ✅ Data available |
| **Filtering** | ✅ Interactive | ⚠️ Basic | ✅ Query params |
| **Installation** | npm / Docker | cargo | None |
| **Automation** | ❌ | ❌ | ✅ |
| **Visual Polish** | ✅ High | ⚠️ Terminal | ❌ |
| **Bandwidth** | Medium | Low | Minimal |

### Performance Comparison

| Metric | Web Dashboard | TUI Client | Direct API |
|--------|--------------|------------|------------|
| Startup Time | ~2-3 seconds | <100ms | N/A |
| Memory Footprint | ~100-150MB | ~5-8MB | Negligible |
| CPU Usage (Idle) | ~1-2% | ~0.2% | ~0% |
| CPU Usage (Active) | ~3-5% | ~0.5-1% | ~0% |
| Network (per update) | ~5-10KB | ~2-5KB | ~2-5KB |

---

## Deployment Scenarios

### Scenario 1: Local Development Workstation

**Requirements**: Visual monitoring with rich charts, easy access

**Recommended Client**: Web Dashboard

```bash
# Terminal 1: Start collector
sudo systemctl start system-monitor-collector

# Terminal 2: Start web dashboard
cd web-frontend && npm run dev

# Access: http://localhost:5252
```

**Benefits**:
- Interactive visual interface
- Real-time charts and graphs
- Easy to use during development

---

### Scenario 2: Remote SSH Server

**Requirements**: Headless server, SSH-only access, low resources

**Recommended Client**: TUI Client

```bash
# SSH into server
ssh admin@remote-server

# Install collector (one-time)
sudo ./scripts/install.sh

# Monitor via TUI
./target/release/tui-client
```

**Benefits**:
- Works over SSH
- Minimal resource usage
- No GUI required

---

### Scenario 3: Production Monitoring

**Requirements**: Multi-user dashboard, 24/7 availability

**Recommended Client**: Web Dashboard (production build)

```bash
# Build and deploy frontend
cd web-frontend
npm run build
sudo cp -r dist/* /var/www/system-monitor/

# Configure nginx reverse proxy
# Access: https://monitor.company.com
```

**Benefits**:
- Multiple users can access simultaneously
- Professional visual interface
- Integration with existing infrastructure

---

### Scenario 4: Automation & Alerting

**Requirements**: Programmatic access, custom logic, integrations

**Recommended Client**: Direct API

```bash
# Custom alert script
./scripts/alert-on-high-cpu.sh

# Integration with monitoring tools
# Grafana, Prometheus, Datadog, etc.
```

**Benefits**:
- Full programmatic control
- Easy integration with existing tools
- Minimal overhead

---

### Scenario 5: Hybrid Multi-Client

**Requirements**: Diverse use cases, multiple user types

**Recommended Clients**: All simultaneously

```bash
# 1. Collector (backend)
sudo systemctl start system-monitor-collector

# 2. Web dashboard for team
cd web-frontend && npm run dev

# 3. TUI for SSH access
./target/release/tui-client

# 4. API for automation
curl http://localhost:5253/api/v1/metrics/current | jq
```

**Benefits**:
- Flexibility for different use cases
- Single collector serves all clients
- Cost-effective resource utilization

---

## Best Practices

### Client Selection

**Choose Web Dashboard when**:
- Rich visual monitoring is required
- Multiple users need concurrent access
- Historical trend analysis with charts
- Presenting data to stakeholders

**Choose TUI Client when**:
- SSH is the only access method
- Resource usage must be minimal
- No GUI/browser available
- Quick diagnostics needed

**Choose Direct API when**:
- Building custom integrations
- Automation and scripting
- Integrating with existing monitoring tools
- Programmatic data access required

### Performance Optimization

**Web Dashboard**:
- Use production build (`npm run build`) for deployment
- Enable gzip compression in web server
- Configure caching headers for static assets
- Use WebSocket for real-time updates (reduces HTTP polling overhead)

**TUI Client**:
- Use WebSocket mode (`-w`) for real-time monitoring
- Increase refresh interval (`-r 5`) if high frequency not needed
- Run in tmux/screen for persistent sessions

**Direct API**:
- Use query parameters to limit response size (`?limit=100`)
- Implement client-side caching
- Use WebSocket for streaming vs polling

### Security Considerations

**All Clients**:
- Run collector behind reverse proxy for authentication
- Use HTTPS/WSS in production
- Implement rate limiting to prevent abuse
- Restrict network access via firewall

**Example nginx configuration**:
```nginx
server {
    listen 443 ssl;
    server_name monitor.example.com;

    # SSL configuration
    ssl_certificate /etc/ssl/certs/monitor.crt;
    ssl_certificate_key /etc/ssl/private/monitor.key;

    # Basic auth
    auth_basic "System Monitor";
    auth_basic_user_file /etc/nginx/.htpasswd;

    # Proxy to collector
    location /api/ {
        proxy_pass http://localhost:5253;
    }

    # WebSocket
    location /ws {
        proxy_pass http://localhost:5253;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

---

## Summary

System Monitor provides **three client options** to suit different monitoring needs:

1. **Web Dashboard**: Rich visual interface for interactive monitoring
2. **TUI Client**: Lightweight terminal interface for SSH and headless servers
3. **Direct API**: Programmatic access for automation and integrations

**All clients consume the same collector API**, enabling flexible deployment scenarios and seamless switching between clients based on context.

**Choose the right client for your use case** to maximize efficiency and user experience.
