# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

System Monitor is a professional system monitoring application written in Rust, designed to test Zorin OS stability. It consists of three independent components:

1. **Collector** (Backend) - Core service that collects system metrics
2. **Web Frontend** (React/TypeScript) - Optional web-based dashboard
3. **TUI Client** (Rust/Ratatui) - Optional terminal-based dashboard

**Critical Architecture Note**: The collector MUST run natively on the host (as a systemd service), NOT in Docker. It requires direct access to `/proc`, `/sys`, and `/dev` to collect accurate metrics. Running in Docker results in namespace isolation and inaccurate readings.

## Project Structure

This is a Cargo workspace with three packages:

```
/
├── collector/          # Backend service (main component)
│   └── src/
│       ├── api/       # REST API and WebSocket handlers
│       ├── collectors/ # Metric collection modules (cpu, memory, temperature, etc.)
│       ├── detector/   # Anomaly detection rules and analysis
│       └── storage/    # SQLite database layer
├── tui-client/        # Terminal UI client (optional)
├── shared/            # Common types shared across packages
│   └── src/
│       ├── types.rs   # SystemMetrics, Anomaly, and other core types
│       └── constants.rs
├── web-frontend/      # React web dashboard (optional)
│   └── src/
│       ├── api/       # API client
│       ├── components/ # React components
│       └── hooks/     # Custom React hooks
└── scripts/           # Installation and utility scripts
```

## Common Commands

### Building

```bash
# Build all workspace packages (release mode)
cargo build --release

# Build only collector
cargo build --release --package collector

# Build only TUI client
cargo build --release --package tui-client

# Quick build script (builds all + installs collector)
./scripts/build-all.sh
```

### Running

**Collector (Backend)**:
```bash
# Option 1: Install as systemd service (RECOMMENDED)
sudo ./scripts/install.sh

# Option 2: Run directly (development)
cargo run --package collector --release

# Option 3: Via compiled binary
./target/release/collector
```

**Web Frontend**:
```bash
cd web-frontend
npm install
npm run dev          # Development (http://localhost:5252)
npm run build        # Production build
npm run lint         # Run ESLint
```

**TUI Client**:
```bash
# Basic usage (connects to localhost:5253)
cargo run --package tui-client --release

# Or run binary directly
./target/release/tui-client

# With options
./target/release/tui-client --api-url http://192.168.1.100:5253 --refresh 5 --websocket
```

### Testing and Monitoring

```bash
# Check collector service status
sudo systemctl status system-monitor-collector

# View collector logs
sudo journalctl -u system-monitor-collector -f

# Test API endpoint
curl http://localhost:5253/health
curl http://localhost:5253/api/v1/metrics/current | jq

# Quick development run (collector + frontend + TUI)
./scripts/run-dev.sh

# Production run (installs collector, builds frontend)
sudo ./scripts/run-prod.sh
```

### Installation Management

```bash
# Install collector as systemd service
sudo ./scripts/install.sh

# Uninstall collector service
sudo ./scripts/uninstall.sh
```

## High-Level Architecture

### Data Flow

```
Kernel (/proc, /sys, /dev)
    ↓
Collector (collects every 2 seconds)
    ├→ SQLite Database (persistence)
    └→ REST API + WebSocket (port 5253)
         ↓
    Clients (multiple simultaneous)
    ├→ Web Frontend (port 5252)
    ├→ TUI Client (terminal)
    └→ Direct API calls (curl, scripts, integrations)
```

### Component Independence

- **Collector**: Fully independent, runs as standalone service
  - Can run without any clients
  - Provides REST API + WebSocket
  - Stores data in SQLite (`./data/system-monitor.db`)
  - Logs anomalies to systemd journal

- **Web Frontend**: Optional client
  - Requires collector running on localhost:5253
  - Can be run natively or in Docker
  - Multiple users can connect simultaneously

- **TUI Client**: Optional client
  - Requires collector running (local or remote)
  - Ideal for SSH/headless servers
  - Single-user per instance

### Metrics Collection Architecture

The collector uses a modular design with separate collectors for each metric type:

- **`collectors/cpu.rs`**: Global CPU usage, per-core usage, load averages (via sysinfo)
- **`collectors/memory.rs`**: RAM and swap usage (via sysinfo)
- **`collectors/temperature.rs`**: Direct reads from `/sys/class/hwmon/` (k10temp, nvme, GPU)
- **`collectors/disk.rs`**: Disk usage, I/O statistics, mount points (via sysinfo)
- **`collectors/usb.rs`**: USB device enumeration, timeout detection (via sysinfo)
- **`collectors/network.rs`**: Network bytes/packets RX/TX (via sysinfo)
- **`collectors/gpu.rs`**: NVIDIA GPU metrics via `nvidia-smi` (usage, temperature, VRAM, power, fan speed)

All collectors implement consistent error handling and are called by `MetricsCollector::collect_all()` in the main loop.

### Anomaly Detection

The `detector/` module implements rule-based anomaly detection:

- **`rules.rs`**: `AnomalyRules` checks metrics against configurable thresholds
- **`analyzer.rs`**: Trend analysis and pattern recognition
- Thresholds: CPU (70/90%), Memory (80/95%), Temperature (75/85°C), Disk (80/90%), GPU (90°C temp, 95% usage/VRAM)
- Generates `Anomaly` objects with severity (Info/Warning/Critical), category, and detailed messages
- Categories: Cpu, Memory, Temperature, Disk, Usb, Network, Gpu, System
- Anomalies are stored in database and kept in memory (last 100)

### API Layer

The `api/` module provides REST and WebSocket access:

- **`rest.rs`**: REST API handlers (health, metrics, history, anomalies)
- **`websocket.rs`**: WebSocket handlers for real-time metric streaming
- **`routes.rs`**: Axum route definitions
- CORS enabled for web frontend (localhost:5252)
- JSON responses with standard envelope format

### Storage Layer

SQLite-based persistence in `storage/`:

- **`repository.rs`**: `MetricsRepository` handles all database operations
- **`models.rs`**: Database models and conversions
- Migrations embedded in code
- Tables: `metrics`, `anomalies`
- Automatic cleanup of old data (configurable retention)

## Development Guidelines

### When Making Changes to Metrics

1. Update the shared type in `shared/src/types.rs` first
2. Update the collector module in `collector/src/collectors/`
3. Update database schema if needed (add migration in `storage/repository.rs`)
4. Update API response if structure changed
5. Update frontend types in `web-frontend/src/types/`

### Configuration

Collector configuration is via environment variables:

- `DATABASE_URL`: SQLite database path (default: `sqlite://./data/system-monitor.db`)
- `COLLECTION_INTERVAL_SECS`: Collection frequency (default: 2)
- `HOST`: API bind address (default: 127.0.0.1)
- `PORT`: API port (default: 5253)

Systemd service uses `/etc/system-monitor/config.toml` (created by install script).

### Database Location

- **Development**: `./data/system-monitor.db` (relative to project root)
- **Production**: `/var/lib/system-monitor/metrics.db` (systemd service)

### Port Usage

- **5253**: Collector API (REST + WebSocket)
- **5252**: Web Frontend (React dev server or nginx)

### Testing Collector Without Clients

```bash
# Start collector
cargo run --package collector --release

# In another terminal, query metrics
watch -n 2 'curl -s localhost:5253/api/v1/metrics/current | jq ".data.cpu.global_usage, .data.memory.usage_percent"'
```

## Important Notes

### Docker Considerations

- **DO NOT** run the collector in Docker - metrics will be inaccurate
- The web frontend CAN run in Docker (optional)
- Use `docker compose up -d frontend` if desired
- Frontend in Docker accesses collector via `host.docker.internal:5253`

### System Requirements

- **Rust**: 1.70+ (2021 edition)
- **Node.js**: 18+ (for web frontend)
- **OS**: Linux (tested on Zorin OS/Ubuntu)
- **Access**: Read access to `/proc`, `/sys`, `/dev`

### Why This Architecture?

The project originally ran the collector as a terminal app (`run.sh` in legacy code). It was refactored to:

1. Provide persistent system monitoring (systemd service)
2. Enable multiple clients (web, TUI, API) to access the same data
3. Store historical metrics for trend analysis
4. Detect anomalies automatically
5. Allow headless operation with API-only mode

### Client Selection Guide

- **Web Frontend**: Rich visualization, multiple users, dashboard displays
- **TUI Client**: SSH sessions, headless servers, low resource usage (~5MB RAM)
- **Direct API**: Automation, scripts, integrations (Grafana, Prometheus, etc.)

All clients can run simultaneously against one collector instance.

## Key Dependencies

### Collector (Rust)
- `sysinfo 0.32` - Cross-platform system metrics
- `axum 0.7` - Web framework (REST + WebSocket)
- `sqlx 0.7` - Async SQLite driver
- `tokio 1.35` - Async runtime
- `tracing` - Structured logging

### Web Frontend (TypeScript)
- `react 18.2` - UI framework
- `recharts 2.10` - Charts and graphs
- `axios 1.6` - HTTP client
- `vite 5.0` - Build tool and dev server
- `tailwindcss 3.4` - Styling

### TUI Client (Rust)
- `ratatui 0.26` - Terminal UI framework
- `crossterm 0.27` - Cross-platform terminal manipulation
- `reqwest 0.11` - HTTP client

## Troubleshooting

### Collector won't start
```bash
# Check if port is in use
sudo lsof -i :5253

# Check database permissions
ls -la ./data/

# View full logs
sudo journalctl -u system-monitor-collector -n 100 --no-pager
```

### Frontend can't connect
- Ensure collector is running: `curl http://localhost:5253/health`
- Check CORS configuration in `collector/src/main.rs` (line 49)
- Verify frontend env: `VITE_API_URL` should be `http://localhost:5253`

### Inaccurate metrics
- Confirm collector is running natively (not in Docker)
- Check `/sys/class/hwmon/` accessibility
- Verify no permission errors in logs

### TUI Client issues
- Ensure API URL is correct: `./target/release/tui-client --api-url http://localhost:5253`
- Check terminal size (minimum 80x24)
- Try with `--websocket` flag for real-time updates

## Graceful Shutdown

The collector implements graceful shutdown using a `CancellationToken` pattern:

1. **Signal Reception**: Main task listens for SIGTERM (systemd) or SIGINT (Ctrl+C)
2. **Token Cancellation**: Shutdown signal cancels shared token
3. **Task Coordination**: All async tasks check token and exit gracefully
4. **Resource Cleanup**: Database connections closed explicitly
5. **Clean Exit**: Process exits with code 0

**Expected shutdown time:** < 5 seconds

### Shutdown Sequence

```
SIGTERM received
  ├─> CancellationToken cancelled
  ├─> API Server stops accepting connections
  │   └─> Completes active requests
  ├─> Collection Loop exits current iteration
  │   └─> Stops collecting metrics
  ├─> Database pool closed
  │   └─> Active connections drained
  └─> Process exits with code 0
```

### Monitoring Shutdown

```bash
# Watch shutdown logs
sudo journalctl -u system-monitor -f

# Expected log sequence:
# "Received SIGTERM, initiating graceful shutdown..."
# "Collection loop received shutdown signal"
# "Collection loop stopped"
# "API server task completed"
# "Collection loop task completed"
# "Closing database connection..."
# "Database connection pool closed"
# "Shutdown complete!"
```
