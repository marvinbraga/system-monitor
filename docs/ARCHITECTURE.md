# System Architecture

## Table of Contents
- [Overview](#overview)
- [Component Architecture](#component-architecture)
- [Collector Service](#collector-service)
- [Client Applications](#client-applications)
- [Data Flow](#data-flow)
- [Use Cases](#use-cases)
- [FAQ](#faq)

---

## Overview

System Monitor employs a **decoupled three-tier architecture** that separates concerns between data collection, persistence, and presentation. This design enables flexible deployment scenarios, from standalone headless monitoring to multi-user dashboard deployments.

### Architectural Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      PHYSICAL HOST                          │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │          COLLECTOR SERVICE (Backend)                 │  │
│  │  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │  │
│  │  • Native Rust binary (systemd service)             │  │
│  │  • Port: 5253                                        │  │
│  │  • Database: SQLite (./data/system-monitor.db)     │  │
│  │  • Direct kernel access: /proc, /sys, /dev         │  │
│  │  • REST API + WebSocket streaming                   │  │
│  │  • Anomaly detection engine                         │  │
│  │  • Collection interval: 2 seconds                   │  │
│  └──────────────────────────────────────────────────────┘  │
│                           ↑                                 │
│                           │ HTTP/WebSocket                  │
│                           │                                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │          CLIENT APPLICATIONS (Optional)             │  │
│  │  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │  │
│  │                                                      │  │
│  │  1. Web Dashboard (React/TypeScript)               │  │
│  │     • Port: 5252                                    │  │
│  │     • Interactive browser-based UI                 │  │
│  │     • Real-time charts and graphs                  │  │
│  │     • Multi-user support                           │  │
│  │                                                      │  │
│  │  2. TUI Client (Terminal Interface)                │  │
│  │     • Terminal-based dashboard                      │  │
│  │     • Keyboard-driven navigation                   │  │
│  │     • SSH-friendly, minimal resources              │  │
│  │     • ASCII-based visualizations                   │  │
│  │                                                      │  │
│  │  3. HTTP/WebSocket Clients                         │  │
│  │     • curl, wget, custom scripts                   │  │
│  │     • Third-party integrations                     │  │
│  │     • Monitoring tools (Grafana, Prometheus)       │  │
│  │                                                      │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Separation of Concerns**: Collector handles metrics, clients handle presentation
2. **Component Independence**: Collector operates standalone without any clients
3. **API-First Design**: All functionality exposed via REST + WebSocket
4. **Native Performance**: Direct kernel access for accurate, low-latency metrics
5. **Flexible Deployment**: Support for headless, GUI, and hybrid configurations

---

## Component Architecture

### 1. Collector Service (Core)

**Language**: Rust
**Type**: Native binary / systemd service
**Port**: 5253
**Dependencies**: None (fully self-contained)

#### Why Native Deployment?

The collector **MUST run natively** on the host, not in Docker containers.

❌ **Docker Container Limitations:**
```
Docker Container Environment
├─ Isolated namespace
├─ Accesses only container's /proc
├─ Sees container's CPU/memory limits
├─ Cannot read physical temperature sensors
├─ Limited USB device visibility
└─ Inaccurate or incomplete metrics
```

✅ **Native Systemd Service Advantages:**
```
Native Systemd Service
├─ Direct access to host /proc, /sys, /dev
├─ 100% accurate metric collection
├─ All temperature sensors visible
├─ Complete USB device enumeration
├─ Boot-time startup via systemd
└─ Standard service management (systemctl)
```

#### Collector Responsibilities

**1. Metric Collection (every 2 seconds)**

| Category | Metrics Collected |
|----------|------------------|
| **CPU** | Global usage, per-core utilization, load averages (1m, 5m, 15m) |
| **Memory** | Total, used, available RAM; swap usage and availability |
| **Temperature** | All hwmon sensors (CPU: k10temp, NVMe, GPU, chipset) |
| **Disk** | Usage, available space, I/O statistics, mount points |
| **USB** | Connected devices, manufacturer/product info, timeout detection |
| **Network** | RX/TX bytes and packets per interface |

**2. Anomaly Detection**

Intelligent rule-based detection with configurable thresholds:

| Anomaly Type | Warning | Critical | Action |
|--------------|---------|----------|--------|
| CPU Spike | >70% | >90% | Log + Store + Alert |
| Memory Pressure | >80% | >95% | Log + Store + Alert |
| High Temperature | >75°C | >85°C | Log + Store + Alert |
| Disk Full | >80% | >90% | Log + Store + Alert |
| Load Average | >cores×1.5 | >cores×2.0 | Log + Store + Alert |

**3. Data Persistence**

- **Database**: SQLite (embedded, no external server required)
- **Location**: `./data/system-monitor.db` (dev) or `/var/lib/system-monitor/metrics.db` (production)
- **Schema**:
  - `metrics` table: Time-series metric snapshots
  - `anomalies` table: Detected anomalies with severity and context
- **Retention**: Configurable (default: 30 days)

**4. API Exposure**

RESTful API with WebSocket support:

```
GET  /health                      # Service health check
GET  /api/v1/metrics/current      # Latest metric snapshot
GET  /api/v1/metrics/history      # Historical metrics (paginated)
GET  /api/v1/anomalies            # Detected anomalies (filterable)
WS   /ws                          # Real-time metric streaming
```

**API Response Format:**
```json
{
  "status": "success",
  "timestamp": "2026-01-29T14:00:00Z",
  "data": {
    "cpu": { "global_usage": 45.2, "per_core": [...], ... },
    "memory": { "total": 33554432000, "used": 16777216000, ... },
    "temperatures": [...],
    "disks": [...],
    "usb_devices": [...],
    "network": { "rx_bytes": 1234567890, ... }
  }
}
```

#### Collector Installation

```bash
# Install as systemd service (creates /opt/system-monitor/)
sudo ./scripts/install.sh

# Verify installation
sudo systemctl status system-monitor-collector
curl http://localhost:5253/health

# View logs
sudo journalctl -u system-monitor-collector -f
```

#### Collector Uninstallation

```bash
# Complete removal (service, binaries, database)
sudo ./scripts/uninstall.sh
```

---

### 2. Client Applications (Optional)

All clients consume the same collector API. Multiple clients can connect simultaneously.

#### 2.1 Web Dashboard (React/TypeScript)

**Port**: 5252
**Type**: Single Page Application (SPA)
**Deployment**: Native (npm) or Docker

**Dependencies**: Requires collector running on `localhost:5253`

**Architecture**:
```
Web Frontend
    ├─ API Client (Axios)
    │   └─ HTTP requests to collector
    ├─ WebSocket Client
    │   └─ Real-time metric streaming
    ├─ React Components
    │   ├─ Dashboard layout
    │   ├─ Metric cards
    │   ├─ Charts (Recharts)
    │   └─ Anomaly list
    └─ State Management
        └─ React hooks + Context
```

**Features**:
- Interactive real-time charts (CPU, memory, temperature, disk)
- Live anomaly feed with filtering
- Historical metric visualization
- Responsive design (desktop/tablet/mobile)
- WebSocket streaming for sub-second updates
- Multi-user support

**Deployment Options**:

```bash
# Option 1: Native development server
cd web-frontend
npm install
npm run dev              # http://localhost:5252

# Option 2: Production build
npm run build
# Deploy dist/ to nginx/apache

# Option 3: Docker container
docker compose up -d frontend
```

**Environment Configuration**:
- `VITE_API_URL`: Collector API endpoint (default: `http://localhost:5253`)

---

#### 2.2 TUI Client (Terminal Interface)

**Type**: Terminal User Interface
**Language**: Rust + Ratatui
**Deployment**: Native binary

**Dependencies**: Requires collector running (local or remote)

**Architecture**:
```
TUI Client
    ├─ HTTP Client (reqwest)
    │   └─ Polling or WebSocket connection
    ├─ Terminal Rendering (Ratatui)
    │   ├─ Dashboard layout
    │   ├─ ASCII charts
    │   ├─ Metric tables
    │   └─ Anomaly list
    └─ Event Loop
        ├─ Keyboard input
        ├─ Metric refresh
        └─ Screen updates
```

**Features**:
- ASCII-based dashboard with real-time updates
- Keyboard navigation (vim-style + arrow keys)
- Minimal resource footprint (~5MB RAM)
- SSH-friendly (works over any terminal)
- Two modes: HTTP polling or WebSocket streaming

**Usage**:

```bash
# Basic usage (localhost:5253)
./target/release/tui-client

# Remote server
./target/release/tui-client --api-url http://192.168.1.100:5253

# Custom refresh rate (5 seconds)
./target/release/tui-client --refresh 5

# WebSocket mode for real-time updates
./target/release/tui-client --websocket

# Combined options
./target/release/tui-client -u http://server:5253 -w -r 1
```

**Command-line Options**:
| Flag | Description | Default |
|------|-------------|---------|
| `-u, --api-url <URL>` | Collector API endpoint | `http://localhost:5253` |
| `-r, --refresh <SEC>` | Metric refresh interval | `2` |
| `-w, --websocket` | Use WebSocket instead of polling | `false` |
| `-h, --help` | Show help message | - |

**Keyboard Controls**:
| Key | Action |
|-----|--------|
| `q`, `Ctrl+C` | Exit application |
| `↑`, `k` | Scroll up (anomaly list) |
| `↓`, `j` | Scroll down (anomaly list) |
| `Page Up` | Previous page |
| `Page Down` | Next page |

**When to Use TUI vs Web**:

| Scenario | TUI Client | Web Dashboard |
|----------|-----------|---------------|
| Remote SSH access | ✅ Ideal | ❌ Not available |
| Headless server | ✅ Perfect | ❌ Requires browser |
| Low resource usage | ✅ ~5MB | ⚠️ ~100MB+ |
| Rich visualizations | ⚠️ ASCII only | ✅ Full charts |
| Multi-user access | ❌ Single user | ✅ Multiple users |
| Quick diagnostics | ✅ Fast startup | ⚠️ Slower |
| Long-running dashboard | ⚠️ Terminal-bound | ✅ Ideal |

---

#### 2.3 Direct API Access

**Protocol**: REST + WebSocket
**Authentication**: None (configure firewall/reverse proxy as needed)
**Format**: JSON

**Use Cases**:
- Custom scripts and automation
- Integration with monitoring tools (Grafana, Prometheus, Datadog)
- CI/CD pipeline health checks
- Custom alerting systems
- Data export and analysis

**Example Usage**:

```bash
# Health check
curl http://localhost:5253/health

# Current metrics
curl http://localhost:5253/api/v1/metrics/current | jq

# Historical data (last 60 records)
curl "http://localhost:5253/api/v1/metrics/history?limit=60" | jq

# Anomalies (critical only)
curl "http://localhost:5253/api/v1/anomalies?severity=critical" | jq

# Extract specific metrics
curl -s http://localhost:5253/api/v1/metrics/current | \
  jq '.data.cpu.global_usage, .data.memory.usage_percent'

# Monitor temperature sensors
curl -s http://localhost:5253/api/v1/metrics/current | \
  jq '.data.temperatures[] | select(.value > 80)'

# WebSocket streaming (using wscat)
npm install -g wscat
wscat -c ws://localhost:5253/ws
```

**Integration Example (Bash Script)**:

```bash
#!/bin/bash
# Simple alerting script

ALERT_THRESHOLD=90

while true; do
  cpu=$(curl -s localhost:5253/api/v1/metrics/current | \
        jq -r '.data.cpu.global_usage')

  if (( $(echo "$cpu > $ALERT_THRESHOLD" | bc -l) )); then
    echo "ALERT: CPU usage ${cpu}% exceeds threshold!"
    # Send notification (email, Slack, PagerDuty, etc.)
  fi

  sleep 10
done
```

---

## Data Flow

### End-to-End Metric Pipeline

```
┌──────────────┐
│  Linux       │
│  Kernel      │
│  (/proc)     │  Direct system calls
│  (/sys)      │  via sysinfo crate
│  (/dev)      │
└──────┬───────┘
       │
       │ Native reads (no overhead)
       ↓
┌──────────────┐
│  Collector   │
│  Service     │◄────┐
│  (Rust)      │     │
└──────┬───────┘     │
       │             │
       ├─ Collect ◄──┘ Every 2 seconds
       │
       ├─ Detect anomalies (rule engine)
       │
       ├─ Persist ──► SQLite database
       │              (time-series + anomalies)
       │
       └─ Expose ──► REST API (port 5253)
                     WebSocket (port 5253)
                     │
        ┌────────────┼────────────┐
        │            │            │
        ▼            ▼            ▼
  ┌─────────┐  ┌─────────┐  ┌─────────┐
  │   Web   │  │   TUI   │  │ Scripts │
  │Dashboard│  │ Client  │  │  curl   │
  └─────────┘  └─────────┘  └─────────┘
      │            │            │
      ▼            ▼            ▼
  Browser      Terminal      Automation
```

### Metric Collection Flow

1. **Kernel Interfaces** → System stats exposed via `/proc`, `/sys`, `/dev`
2. **Collector reads** → `sysinfo` crate provides abstraction over kernel interfaces
3. **Aggregation** → Metrics combined into `SystemMetrics` struct
4. **Anomaly detection** → Rules engine evaluates thresholds
5. **Persistence** → Metrics + anomalies written to SQLite
6. **API exposure** → Data available via REST endpoints + WebSocket
7. **Client consumption** → Web/TUI/Scripts query or stream data
8. **Visualization** → Clients render data appropriately

---

## Use Cases

### Use Case 1: API-Only Monitoring

**Scenario**: Headless server with no GUI, metrics consumed programmatically

```bash
# Collector runs as systemd service
sudo systemctl start system-monitor-collector

# Query metrics via API
curl http://localhost:5253/api/v1/metrics/current | jq

# Integrate with external tools
# - Grafana dashboards
# - Prometheus scraping
# - Custom alerting systems
```

**Clients Required**: None ✅
**Benefits**: Minimal resource usage, programmatic access, integration-friendly

---

### Use Case 2: Local Web Dashboard

**Scenario**: Developer workstation with browser-based monitoring

```bash
# Start collector service
sudo systemctl start system-monitor-collector

# Start web frontend
cd web-frontend && npm run dev

# Access dashboard
# http://localhost:5252
```

**Clients Required**: Web Dashboard
**Benefits**: Rich visualizations, real-time charts, user-friendly

---

### Use Case 3: Headless Server Monitoring

**Scenario**: Production server with no GUI, monitored via logs

```bash
# Collector runs as service
sudo systemctl start system-monitor-collector

# Monitor via logs (anomalies auto-logged)
sudo journalctl -u system-monitor-collector -f

# Or query database directly
sqlite3 /var/lib/system-monitor/metrics.db \
  "SELECT * FROM anomalies WHERE severity='Critical' ORDER BY timestamp DESC LIMIT 10"
```

**Clients Required**: None ✅
**Benefits**: Fully automated, log-based monitoring, minimal overhead

---

### Use Case 4: SSH Terminal Monitoring

**Scenario**: Remote server accessed via SSH

```bash
# SSH into server
ssh admin@remote-server

# Install collector (one-time)
sudo ./scripts/install.sh

# Monitor via TUI client
./target/release/tui-client

# Or monitor remote server from local machine
./target/release/tui-client --api-url http://remote-server:5253
```

**Clients Required**: TUI Client
**Benefits**: Terminal-based, low bandwidth, SSH-friendly

---

### Use Case 5: Multi-Client Deployment

**Scenario**: Single collector serving multiple client types simultaneously

```bash
# 1 Collector (backend)
sudo systemctl start system-monitor-collector

# Multiple clients accessing simultaneously:
# 1. Web dashboard (browser)
#    http://localhost:5252
#
# 2. TUI client (terminal)
#    ./target/release/tui-client
#
# 3. Automation scripts
#    curl http://localhost:5253/api/v1/metrics/current
#
# 4. External integrations
#    Grafana, Prometheus, Datadog, etc.
```

**All clients share the same collector!** ✅
**Benefits**: Flexibility, multi-user support, diverse use cases

---

## FAQ

### Does the collector require a client to function?

**No.** The collector is fully independent and operates standalone. Clients are optional visualization/access layers.

### Can the frontend run without the collector?

**No.** The frontend is a client application that consumes the collector's API. Without a running collector, the frontend will display connection errors.

### Why can't the collector run in Docker?

Docker containers have **namespace isolation** which prevents accurate metric collection:
- `/proc` shows container's processes, not host's
- Temperature sensors (`/sys/class/hwmon/`) are inaccessible
- USB devices are not visible
- CPU/memory metrics reflect container limits, not host usage

**Solution**: Always run the collector natively using the provided installation script.

### Can I access the collector from another machine?

**Yes.** By default, the collector binds to `127.0.0.1:5253` (localhost only). To allow remote access:

1. Edit `/etc/system-monitor/config.toml`
2. Change `api_host = "0.0.0.0"`
3. Restart service: `sudo systemctl restart system-monitor-collector`
4. Configure firewall to allow port 5253

**Security Note**: Implement authentication via reverse proxy (nginx, Traefik) if exposing publicly.

### What are the resource requirements?

| Component | CPU | RAM | Disk I/O |
|-----------|-----|-----|----------|
| Collector | ~1-2% | ~10-15 MB | ~1 MB/hour (database growth) |
| Web Frontend | ~2-5% | ~100+ MB | Minimal |
| TUI Client | ~0.5% | ~5 MB | Minimal |

### How is data retention managed?

Retention is configurable in `/etc/system-monitor/config.toml`:

```toml
retention_days = 30  # Keep metrics for 30 days
```

The collector automatically purges old records during startup and periodically during runtime.

### Can I customize anomaly thresholds?

Yes, edit `/etc/system-monitor/config.toml`:

```toml
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

Restart the service after changes:
```bash
sudo systemctl restart system-monitor-collector
```

---

## Summary

The System Monitor architecture prioritizes:

1. **Component Independence**: Collector operates standalone
2. **Flexible Deployment**: Multiple client options for diverse scenarios
3. **Native Performance**: Direct kernel access for accurate metrics
4. **API-First Design**: All functionality exposed via REST + WebSocket
5. **Production-Ready**: Systemd integration, persistence, logging, health checks

**Architecture Principle**: The collector is the authoritative source of truth. Clients are interchangeable presentation layers.
