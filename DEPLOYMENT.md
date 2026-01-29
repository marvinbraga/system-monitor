# System Monitor - Deployment Guide

This guide covers all deployment options for the System Monitor application.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Deployment Options](#deployment-options)
  - [Docker Compose (Recommended)](#docker-compose-recommended)
  - [Native Installation](#native-installation)
  - [Systemd Service](#systemd-service)
- [Configuration](#configuration)
- [Monitoring and Maintenance](#monitoring-and-maintenance)
- [Troubleshooting](#troubleshooting)

## Prerequisites

### For Docker Deployment
- Docker Engine 20.10+
- Docker Compose 2.0+
- 2GB RAM minimum
- 1GB disk space

### For Native Deployment
- Rust 1.75+
- Node.js 18+ (for web frontend)
- SQLite3
- Linux kernel with /proc and /sys support

## Quick Start

### Using Docker (Recommended)

```bash
# Build all components
./scripts/build-all.sh

# Run in production mode with Docker
./scripts/run-prod.sh --docker

# Access the application
# - Collector API: http://localhost:8080
# - Web Dashboard: http://localhost:3000
```

### Using Native Build

```bash
# Build all components
./scripts/build-all.sh

# Run in development mode
./scripts/run-dev.sh

# Run in production mode
./scripts/run-prod.sh
```

## Deployment Options

### Docker Compose (Recommended)

Docker Compose provides the easiest and most reliable deployment method.

#### Configuration

1. **Create data directory:**
```bash
mkdir -p data
```

2. **Configure environment (optional):**
```bash
cp .env.example .env
# Edit .env with your settings
```

3. **Start services:**
```bash
# Development mode (with hot reload)
docker-compose up --build

# Production mode (detached)
docker-compose up -d
```

#### Docker Compose Commands

```bash
# View logs
docker-compose logs -f

# View specific service logs
docker-compose logs -f collector
docker-compose logs -f web

# Stop services
docker-compose down

# Restart services
docker-compose restart

# Check service status
docker-compose ps

# Remove volumes (warning: deletes data)
docker-compose down -v
```

#### Docker Compose Configuration

The `docker-compose.yml` file includes:

- **Collector Service:**
  - Builds from `collector/Dockerfile`
  - Exposes port 8080
  - Persistent data volume
  - Health checks
  - Security hardening

- **Web Service:**
  - Builds from `web-frontend/Dockerfile`
  - Exposes port 3000 (Nginx on port 80 internally)
  - Proxies API requests to collector
  - Health checks
  - Security hardening

### Native Installation

For development or custom deployments.

#### Build from Source

```bash
# Clone repository
git clone https://github.com/marvinbraga/system-monitor.git
cd system-monitor

# Build all components
./scripts/build-all.sh
```

#### Manual Build Steps

```bash
# Build Rust workspace
cargo build --release

# Build web frontend (if available)
cd web-frontend
npm install
npm run build
cd ..
```

#### Running Native

```bash
# Development mode
./scripts/run-dev.sh

# Production mode
./scripts/run-prod.sh
```

### Systemd Service

For production deployments on Linux servers.

#### Installation

```bash
# Build the project
cargo build --release

# Run the production script (installs systemd service)
sudo ./scripts/run-prod.sh
```

#### Manual Installation

```bash
# Create system user
sudo useradd -r -s /bin/false monitor

# Create directories
sudo mkdir -p /opt/system-monitor
sudo mkdir -p /var/lib/system-monitor
sudo mkdir -p /var/log/system-monitor

# Copy binary
sudo cp target/release/collector /opt/system-monitor/
sudo chown monitor:monitor /opt/system-monitor/collector
sudo chmod +x /opt/system-monitor/collector

# Install service
sudo cp collector/systemd/system-monitor.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable system-monitor.service
sudo systemctl start system-monitor.service
```

#### Systemd Commands

```bash
# Start service
sudo systemctl start system-monitor.service

# Stop service
sudo systemctl stop system-monitor.service

# Restart service
sudo systemctl restart system-monitor.service

# Check status
sudo systemctl status system-monitor.service

# View logs
sudo journalctl -u system-monitor.service -f

# Enable at boot
sudo systemctl enable system-monitor.service

# Disable at boot
sudo systemctl disable system-monitor.service
```

## Configuration

### Environment Variables

#### Collector Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level (trace, debug, info, warn, error) |
| `DATABASE_URL` | `/var/lib/system-monitor/system-monitor.db` | SQLite database path |
| `HOST` | `0.0.0.0` | Bind address |
| `PORT` | `8080` | HTTP server port |
| `ANOMALY_DETECTION_ENABLED` | `true` | Enable anomaly detection |
| `ANOMALY_LOG_PATH` | `/var/log/system-monitor/anomalies.log` | Anomaly log file path |
| `COLLECTION_INTERVAL_SECS` | `5` | Metrics collection interval in seconds |

#### Web Frontend Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `REACT_APP_API_URL` | `http://localhost:8080` | Collector API URL |
| `REACT_APP_WS_URL` | `ws://localhost:8080` | WebSocket URL |

### Configuration Files

1. **Docker Compose**: `docker-compose.yml`
2. **Nginx**: `nginx.conf`
3. **Systemd**: `collector/systemd/system-monitor.service`
4. **Environment**: `.env` (copy from `.env.example`)

### Network Configuration

#### Ports

- **8080**: Collector REST API and WebSocket
- **3000**: Web frontend (Nginx)

#### Firewall Rules

```bash
# Allow collector API
sudo ufw allow 8080/tcp

# Allow web frontend
sudo ufw allow 3000/tcp
```

## Monitoring and Maintenance

### Health Checks

#### Collector
```bash
curl http://localhost:8080/health
```

#### Web Frontend
```bash
curl http://localhost:3000/
```

### Logs

#### Docker Logs
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f collector
```

#### Systemd Logs
```bash
# Follow logs
sudo journalctl -u system-monitor.service -f

# Last 100 lines
sudo journalctl -u system-monitor.service -n 100

# Today's logs
sudo journalctl -u system-monitor.service --since today
```

#### Log Files
- Collector logs: `/var/log/system-monitor/` (systemd) or `data/` (Docker)
- Anomaly logs: Configured by `ANOMALY_LOG_PATH`

### Database Maintenance

#### Backup Database
```bash
# Docker
docker exec system-monitor-collector sqlite3 /data/system-monitor.db ".backup '/data/backup.db'"

# Native
sqlite3 /var/lib/system-monitor/system-monitor.db ".backup '/var/lib/system-monitor/backup.db'"
```

#### Restore Database
```bash
# Stop service first
docker-compose down
# Or: sudo systemctl stop system-monitor.service

# Restore
cp backup.db system-monitor.db

# Start service
docker-compose up -d
# Or: sudo systemctl start system-monitor.service
```

### Performance Tuning

#### Collection Interval
Adjust `COLLECTION_INTERVAL_SECS` based on your needs:
- Lower values (1-3s): More granular data, higher CPU usage
- Higher values (10-30s): Less data, lower resource usage

#### Database Optimization
```bash
# Vacuum database (Docker)
docker exec system-monitor-collector sqlite3 /data/system-monitor.db "VACUUM;"

# Vacuum database (Native)
sqlite3 /var/lib/system-monitor/system-monitor.db "VACUUM;"
```

## Troubleshooting

### Common Issues

#### Port Already in Use
```bash
# Find process using port
sudo lsof -i :8080

# Kill process
sudo kill -9 <PID>
```

#### Permission Denied
```bash
# Fix data directory permissions (Docker)
sudo chown -R $USER:$USER data/

# Fix systemd permissions
sudo chown -R monitor:monitor /var/lib/system-monitor
sudo chown -R monitor:monitor /var/log/system-monitor
```

#### Database Locked
```bash
# Stop all instances
docker-compose down
sudo systemctl stop system-monitor.service

# Remove lock files
rm -f data/*.db-shm data/*.db-wal

# Restart
docker-compose up -d
```

#### Container Won't Start
```bash
# Check logs
docker-compose logs collector

# Check container status
docker-compose ps

# Rebuild images
docker-compose up --build --force-recreate
```

#### High CPU Usage
- Increase `COLLECTION_INTERVAL_SECS`
- Disable anomaly detection: `ANOMALY_DETECTION_ENABLED=false`
- Check for resource leaks in logs

### Debug Mode

Enable debug logging:
```bash
# Docker
export RUST_LOG=debug
docker-compose up

# Native
RUST_LOG=debug ./target/release/collector

# Systemd
sudo systemctl edit system-monitor.service
# Add: Environment="RUST_LOG=debug"
sudo systemctl daemon-reload
sudo systemctl restart system-monitor.service
```

### Verification Steps

1. **Check if services are running:**
```bash
# Docker
docker-compose ps

# Systemd
sudo systemctl status system-monitor.service
```

2. **Test API endpoints:**
```bash
# Health check
curl http://localhost:8080/health

# System info
curl http://localhost:8080/api/system-info

# Recent metrics
curl http://localhost:8080/api/metrics/recent
```

3. **Test WebSocket:**
```bash
# Using wscat (install: npm install -g wscat)
wscat -c ws://localhost:8080/ws
```

4. **Check web frontend:**
```bash
curl http://localhost:3000/
```

## Security Considerations

### Docker Security
- Services run as non-root users
- Capabilities are dropped and only necessary ones are added
- Read-only file systems where possible
- Security options enabled (no-new-privileges)

### Systemd Security
- Dedicated user account (`monitor`)
- Restricted file system access
- Limited capabilities
- Hardened security settings

### Network Security
- Use reverse proxy (nginx/traefik) for HTTPS
- Configure firewall rules
- Consider network isolation
- Use authentication for production

### Data Security
- Regular database backups
- Secure storage of sensitive data
- Log rotation and retention policies
- Encrypt data at rest if needed

## Production Checklist

- [ ] Configure environment variables
- [ ] Set up data persistence
- [ ] Configure log rotation
- [ ] Set up monitoring and alerts
- [ ] Configure backups
- [ ] Secure network access
- [ ] Set up HTTPS (reverse proxy)
- [ ] Test health checks
- [ ] Document runbook procedures
- [ ] Set up alerting
- [ ] Test disaster recovery

## Support

For issues, questions, or contributions:
- GitHub Issues: https://github.com/marvinbraga/system-monitor/issues
- Documentation: See `README.md` and `API_DOCUMENTATION.md`
