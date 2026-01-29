# System Monitor

<div align="center">

![Rust](https://img.shields.io/badge/rust-2021-orange?logo=rust)
![License](https://img.shields.io/badge/license-GPL--2.0-blue)
![Platform](https://img.shields.io/badge/platform-Linux-lightgrey?logo=linux)
![Status](https://img.shields.io/badge/status-production-green)

**Enterprise-grade system monitoring solution built with Rust**

Real-time metrics collection • Anomaly detection • Multi-client architecture • SQLite persistence

[Architecture](docs/ARCHITECTURE.md) • [Clients](docs/CLIENTS.md) • [Contributing](CONTRIBUTING.md)

</div>

---

## Overview

System Monitor is a high-performance, production-ready system monitoring platform designed for precision metric collection and intelligent anomaly detection. Built entirely in Rust for maximum performance and memory safety, it features a decoupled architecture that separates data collection from visualization, enabling flexible deployment scenarios.

### Key Highlights

- **Native Performance**: Direct access to kernel interfaces (`/proc`, `/sys`, `/dev`) for accurate, low-latency metrics
- **Intelligent Monitoring**: Rule-based anomaly detection with configurable thresholds
- **Flexible Architecture**: Standalone collector service with multiple client options
- **Real-time Streaming**: WebSocket support for live metric updates
- **Persistent Storage**: SQLite-backed historical data with automatic retention management
- **Production Ready**: Systemd integration, comprehensive logging, and health checks

---

## Architecture

System Monitor employs a **three-tier architecture** with complete component independence:

```
┌─────────────────────────────────────────────────────┐
│                   PHYSICAL HOST                     │
│                                                     │
│  ┌──────────────────────────────────────────────┐  │
│  │          COLLECTOR (Core Service)            │  │
│  │  • Native Rust binary                        │  │
│  │  • Systemd service                           │  │
│  │  • Port: 5253                                │  │
│  │  • SQLite persistence                        │  │
│  │  • REST API + WebSocket                      │  │
│  └────────────────┬─────────────────────────────┘  │
│                   │ HTTP/WebSocket                 │
│                   ↓                                 │
│  ┌──────────────────────────────────────────────┐  │
│  │          CLIENTS (Optional)                  │  │
│  │  • Web Dashboard (React/TypeScript)         │  │
│  │  • TUI Client (Ratatui)                     │  │
│  │  • Direct API access (curl, scripts)        │  │
│  └──────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────┘
```

**For detailed architecture documentation, see [Architecture Guide](docs/ARCHITECTURE.md)**

---

## Features

### Comprehensive Metrics Collection

<table>
<tr>
<td width="50%">

**CPU Monitoring**
- Global CPU utilization
- Per-core statistics (all cores)
- Load averages (1m, 5m, 15m)
- Real-time usage tracking

**Memory Monitoring**
- Total, used, and available RAM
- Swap utilization
- Usage percentages
- Memory pressure indicators

</td>
<td width="50%">

**Temperature Sensors**
- CPU temperatures (k10temp: Tctl, Tccd)
- NVMe drive temperatures
- GPU sensors (NVIDIA)
- Direct `/sys/class/hwmon/` reads
- Color-coded thermal zones

**System Information**
- OS details and kernel version
- Hostname and uptime
- CPU topology
- Process statistics

</td>
</tr>
<tr>
<td width="50%">

**Disk Monitoring**
- Storage usage and availability
- I/O statistics (read/write)
- Mount point tracking
- Per-disk metrics

**Network Metrics**
- Bytes transmitted/received
- Packet counters
- Interface statistics
- Network throughput

</td>
<td width="50%">

**USB Device Tracking**
- Connected device enumeration
- Manufacturer and product info
- Timeout detection
- Device health monitoring

**Anomaly Detection**
- Configurable threshold rules
- Severity classification
- Automatic alerting
- Historical trend analysis

</td>
</tr>
</table>

---

## Quick Start

### Prerequisites

- **Linux** (tested on Ubuntu/Zorin OS)
- **Rust** 1.70+ (2021 edition)
- **Node.js** 18+ (for web frontend only)
- Read access to `/proc`, `/sys`, `/dev`

### Installation

#### 1. Install Collector Service (Required)

The collector is the core component and must be installed natively (not in Docker) for accurate metrics:

```bash
# Clone repository
git clone <repository-url>
cd system-monitor

# Install as systemd service
sudo ./scripts/install.sh
```

This will:
- Build the collector in release mode
- Install it to `/opt/system-monitor/`
- Create systemd service configuration
- Start the service automatically
- Configure database at `/var/lib/system-monitor/`

#### 2. Verify Installation

```bash
# Check service status
sudo systemctl status system-monitor-collector

# Test API endpoint
curl http://localhost:5253/health

# View live logs
sudo journalctl -u system-monitor-collector -f
```

#### 3. Choose Your Client (Optional)

**Option A: Web Dashboard**
```bash
cd web-frontend
npm install
npm run dev
# Access: http://localhost:5252
```

**Option B: Terminal UI**
```bash
cargo run --package tui-client --release
# Or: ./target/release/tui-client
```

**Option C: Direct API Access**
```bash
# Current metrics
curl http://localhost:5253/api/v1/metrics/current | jq

# Anomalies
curl http://localhost:5253/api/v1/anomalies | jq
```

---

## API Endpoints

The collector exposes a RESTful API and WebSocket interface:

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check and service status |
| `/api/v1/metrics/current` | GET | Current system metrics snapshot |
| `/api/v1/metrics/history` | GET | Historical metrics (query params: `limit`, `offset`) |
| `/api/v1/anomalies` | GET | Detected anomalies (query params: `limit`, `severity`) |
| `/ws` | WebSocket | Real-time metric streaming |

**Example API Response:**
```json
{
  "status": "success",
  "data": {
    "timestamp": "2026-01-29T14:00:00Z",
    "cpu": {
      "global_usage": 45.2,
      "per_core": [42.1, 48.3, ...],
      "load_avg_1": 2.15,
      "load_avg_5": 1.98,
      "load_avg_15": 1.76
    },
    "memory": {
      "total": 33554432000,
      "used": 16777216000,
      "available": 16777216000,
      "usage_percent": 50.0,
      "swap_total": 8589934592,
      "swap_used": 0
    },
    "temperatures": [...],
    "disks": [...],
    "network": {...}
  }
}
```

---

## Client Options

System Monitor supports multiple client interfaces, all consuming the same collector API:

### 1. Web Dashboard (React)
- **Port**: 5252
- **Features**: Interactive charts, real-time updates, multi-user support
- **Best for**: Rich visualization, dashboards, presentations
- **Resource usage**: ~100MB RAM

### 2. TUI Client (Terminal)
- **Interface**: Terminal-based ASCII dashboard
- **Features**: Keyboard controls, low resource usage, SSH-friendly
- **Best for**: Headless servers, SSH sessions, minimal overhead
- **Resource usage**: ~5MB RAM

### 3. Direct API
- **Protocol**: REST + WebSocket
- **Features**: Programmatic access, automation, integrations
- **Best for**: Scripts, monitoring tools (Grafana, Prometheus), CI/CD
- **Resource usage**: Negligible

**For detailed client comparison, see [Client Guide](docs/CLIENTS.md)**

---

## Development

### Project Structure

This is a Cargo workspace with three packages:

```
system-monitor/
├── collector/          # Core monitoring service (Rust)
│   ├── src/
│   │   ├── api/       # REST & WebSocket handlers
│   │   ├── collectors/# Metric collection modules
│   │   ├── detector/  # Anomaly detection engine
│   │   └── storage/   # SQLite persistence layer
├── tui-client/        # Terminal UI client (Rust + Ratatui)
├── shared/            # Common types and constants
├── web-frontend/      # Web dashboard (React + TypeScript)
│   └── src/
│       ├── api/       # API client
│       ├── components/# React components
│       └── hooks/     # Custom hooks
└── scripts/           # Installation and utility scripts
```

### Building from Source

```bash
# Build all workspace packages
cargo build --release

# Build specific package
cargo build --release --package collector
cargo build --release --package tui-client

# Build web frontend
cd web-frontend
npm run build
```

### Running in Development

```bash
# Collector (standalone)
cargo run --package collector --release

# Web frontend (requires collector running)
cd web-frontend && npm run dev

# TUI client (requires collector running)
cargo run --package tui-client --release
```

### Running Tests

```bash
# Rust tests
cargo test --workspace

# Frontend tests
cd web-frontend && npm test

# Frontend linting
cd web-frontend && npm run lint
```

---

## Configuration

### Environment Variables (Collector)

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite://./data/system-monitor.db` | SQLite database path |
| `COLLECTION_INTERVAL_SECS` | `2` | Metric collection frequency |
| `HOST` | `127.0.0.1` | API bind address |
| `PORT` | `5253` | API port |
| `RUST_LOG` | `info` | Log level (trace, debug, info, warn, error) |

### Systemd Service Configuration

When installed via `./scripts/install.sh`, configuration is stored at:
- **Config file**: `/etc/system-monitor/config.toml`
- **Database**: `/var/lib/system-monitor/metrics.db`
- **Logs**: `journalctl -u system-monitor-collector`

Edit `/etc/system-monitor/config.toml` and restart the service:
```bash
sudo systemctl restart system-monitor-collector
```

---

## Anomaly Detection

The collector includes intelligent anomaly detection with configurable thresholds:

| Category | Warning Threshold | Critical Threshold |
|----------|-------------------|-------------------|
| CPU Usage | 70% | 90% |
| Memory Usage | 80% | 95% |
| Temperature | 75°C | 85°C |
| Disk Usage | 80% | 90% |

Anomalies are:
- Logged to systemd journal
- Stored in SQLite database
- Available via `/api/v1/anomalies` endpoint
- Broadcast to WebSocket clients
- Classified by severity (Info, Warning, Critical)

---

## Management Commands

```bash
# Service control
sudo systemctl start system-monitor-collector
sudo systemctl stop system-monitor-collector
sudo systemctl restart system-monitor-collector
sudo systemctl status system-monitor-collector

# View logs
sudo journalctl -u system-monitor-collector -f
sudo journalctl -u system-monitor-collector -n 100

# Uninstall
sudo ./scripts/uninstall.sh
```

---

## Docker Considerations

⚠️ **IMPORTANT**: The collector should **NOT** be run in Docker.

**Why?** Docker containers have isolated namespaces and cannot access:
- Host's `/proc` filesystem accurately
- Physical temperature sensors (`/sys/class/hwmon/`)
- Actual USB devices
- True CPU/memory metrics (sees container limits, not host)

**Solution**: Always install the collector natively using `./scripts/install.sh`

The web frontend CAN optionally run in Docker:
```bash
docker compose up -d frontend
```

---

## Use Cases

### Production Monitoring
Deploy collector as a systemd service for continuous monitoring with persistent metrics and automatic anomaly detection.

### Development Diagnostics
Run collector locally and use the TUI client for quick system health checks during development.

### Integration & Automation
Consume the REST API programmatically for custom monitoring solutions, alerting systems, or CI/CD pipelines.

### Remote Server Management
Install collector on remote servers and monitor via TUI client over SSH or web dashboard over network.

---

## Technology Stack

### Collector (Rust)
- `sysinfo 0.32` - Cross-platform system information
- `axum 0.7` - Web framework and WebSocket
- `sqlx 0.7` - Async SQLite ORM
- `tokio 1.35` - Async runtime
- `tracing` - Structured logging

### Web Frontend (TypeScript)
- `react 18.2` - UI framework
- `recharts 2.10` - Data visualization
- `axios 1.6` - HTTP client
- `vite 5.0` - Build tool
- `tailwindcss 3.4` - Styling

### TUI Client (Rust)
- `ratatui 0.26` - Terminal UI framework
- `crossterm 0.27` - Terminal manipulation
- `reqwest 0.11` - HTTP client

---

## Performance Characteristics

| Metric | Collector | Web Frontend | TUI Client |
|--------|-----------|--------------|------------|
| CPU Usage | ~1-2% | ~2-5% | ~0.5% |
| Memory (RAM) | ~10-15 MB | ~100+ MB | ~5 MB |
| Disk I/O | ~1 MB/hour | Minimal | Minimal |
| Network | ~10 KB/s | ~50 KB/s | ~20 KB/s |

---

## Troubleshooting

### Collector won't start
```bash
# Check if port is already in use
sudo lsof -i :5253

# Verify database permissions
ls -la /var/lib/system-monitor/

# Check logs for errors
sudo journalctl -u system-monitor-collector -n 50
```

### Frontend connection issues
```bash
# Ensure collector is running
curl http://localhost:5253/health

# Check CORS configuration
# Verify VITE_API_URL in web-frontend/.env
```

### Inaccurate metrics
- Confirm collector is running natively (not in Docker)
- Verify access to `/sys/class/hwmon/`
- Check for permission errors in logs

---

## Roadmap

- [ ] Support for additional platforms (macOS, BSD)
- [ ] Prometheus exporter format
- [ ] Configurable alert webhooks
- [ ] GPU utilization tracking (AMD, Intel)
- [ ] Process-level monitoring
- [ ] Network connection tracking
- [ ] Custom metric plugins

---

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## License

This project is licensed under the GNU General Public License v2.0 - see the [LICENSE](LICENSE) file for details.

This program is free software; you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation; either version 2 of the License, or (at your option) any later version.

---

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Metrics provided by [sysinfo](https://github.com/GuillaumeGomez/sysinfo)
- Web framework powered by [Axum](https://github.com/tokio-rs/axum)
- Terminal UI via [Ratatui](https://github.com/ratatui-org/ratatui)

---

<div align="center">

**[Documentation](docs/)** • **[Issues](issues/)** • **[Discussions](discussions/)**

Made with ❤️ using Rust

</div>
