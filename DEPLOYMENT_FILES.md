# Deployment Infrastructure - Files Created

This document lists all deployment infrastructure files created for the system-monitor project.

## Files Created

### Docker and Container Configuration

1. **`/home/marvinbraga/dados/system-monitor/docker-compose.yml`** (2.0KB)
   - Full stack orchestration for Docker deployment
   - Defines collector and web services
   - Configures volumes, networks, environment variables
   - Includes health checks and security settings
   - Port mappings: 8080 (collector), 3000 (web)

2. **`/home/marvinbraga/dados/system-monitor/collector/Dockerfile`** (1.7KB)
   - Multi-stage build for collector service
   - Stage 1: Build with Rust 1.75
   - Stage 2: Minimal Debian runtime image
   - Non-root user (monitor)
   - Health check endpoint
   - Exposes port 8080

3. **`/home/marvinbraga/dados/system-monitor/web-frontend/Dockerfile`** (1.3KB)
   - Multi-stage build for web frontend
   - Stage 1: Build with Node.js 20
   - Stage 2: Nginx 1.25 Alpine runtime
   - Copies built static files to nginx
   - Non-root user (nginx)
   - Exposes port 80

4. **`/home/marvinbraga/dados/system-monitor/nginx.conf`** (2.5KB)
   - Nginx configuration for web frontend
   - SPA routing (serves index.html for all routes)
   - Reverse proxy to collector API
   - WebSocket proxy with long timeouts
   - Security headers
   - Gzip compression
   - Static file caching

5. **`/home/marvinbraga/dados/system-monitor/.dockerignore`** (606B)
   - Excludes unnecessary files from Docker builds
   - Reduces image size
   - Excludes: git, IDE files, docs, test files, build artifacts

### Systemd Service

6. **`/home/marvinbraga/dados/system-monitor/collector/systemd/system-monitor.service`** (1.6KB)
   - Systemd service unit file
   - Runs as non-root user (monitor)
   - Working directory: /opt/system-monitor
   - Binary path: /opt/system-monitor/collector
   - Restart policies (always, 10s delay)
   - Comprehensive security settings:
     - NoNewPrivileges
     - PrivateTmp
     - ProtectSystem=strict
     - ProtectHome
     - Limited capabilities
     - SystemCallFilter
     - Read-write paths for data/logs
   - Resource limits
   - Journal logging

### Build and Run Scripts

7. **`/home/marvinbraga/dados/system-monitor/scripts/build-all.sh`** (3.8KB, executable)
   - Builds all project components
   - Detects available tools (Docker, Rust, Node.js)
   - Builds native Rust workspace
   - Builds web frontend
   - Builds Docker images
   - Shows colored status output
   - Lists built artifacts

8. **`/home/marvinbraga/dados/system-monitor/scripts/run-dev.sh`** (3.7KB, executable)
   - Runs project in development mode
   - Supports Docker (--docker flag) or native mode
   - Sets RUST_LOG=debug
   - Starts collector service
   - Starts web frontend dev server (if available)
   - Shows service URLs
   - Handles graceful shutdown (Ctrl+C)
   - Cleanup function

9. **`/home/marvinbraga/dados/system-monitor/scripts/run-prod.sh`** (4.7KB, executable)
   - Runs project in production mode
   - Supports Docker (--docker flag) or native mode
   - Docker mode: starts in detached mode, shows status
   - Native mode: installs systemd service
   - Creates system user and directories
   - Sets proper permissions
   - Shows useful commands for monitoring

### Configuration and Documentation

10. **`/home/marvinbraga/dados/system-monitor/.env.example`** (519B)
    - Example environment configuration
    - Collector settings
    - Database configuration
    - Anomaly detection settings
    - Docker settings
    - Web frontend settings

11. **`/home/marvinbraga/dados/system-monitor/DEPLOYMENT.md`** (11KB)
    - Comprehensive deployment guide
    - Prerequisites
    - Quick start instructions
    - All deployment options:
      - Docker Compose (recommended)
      - Native installation
      - Systemd service
    - Configuration reference
    - Monitoring and maintenance
    - Troubleshooting guide
    - Security considerations
    - Production checklist

## File Structure

```
/home/marvinbraga/dados/system-monitor/
├── docker-compose.yml               # Docker orchestration
├── .dockerignore                    # Docker ignore patterns
├── .env.example                     # Environment template
├── nginx.conf                       # Nginx configuration
├── DEPLOYMENT.md                    # Deployment guide
├── DEPLOYMENT_FILES.md              # This file
├── collector/
│   ├── Dockerfile                   # Collector container
│   └── systemd/
│       └── system-monitor.service   # Systemd unit file
├── web-frontend/
│   └── Dockerfile                   # Web frontend container
└── scripts/
    ├── build-all.sh                 # Build script
    ├── run-dev.sh                   # Development runner
    └── run-prod.sh                  # Production runner
```

## Key Features

### Multi-Stage Docker Builds
- Minimal runtime images (Debian Slim, Alpine)
- Separate build and runtime stages
- Reduced image size
- Faster deployments

### Security Hardening
- Non-root users in containers
- Capability dropping
- Read-only file systems where possible
- Security headers
- Systemd security features

### Health Checks
- Collector: HTTP /health endpoint
- Web: HTTP root endpoint
- Automatic container restart on failure
- Systemd restart policies

### Logging
- Docker: container logs via docker-compose
- Systemd: journald integration
- Configurable log levels (RUST_LOG)
- Anomaly logs to separate file

### Deployment Flexibility
- Docker Compose for containerized deployment
- Native builds for development
- Systemd service for production servers
- All modes supported by helper scripts

## Usage Quick Reference

### Docker Deployment
```bash
# Build and run in production
./scripts/run-prod.sh --docker

# View logs
docker-compose logs -f

# Stop services
docker-compose down
```

### Native Deployment
```bash
# Build all
./scripts/build-all.sh

# Run in development
./scripts/run-dev.sh

# Install systemd service
sudo ./scripts/run-prod.sh
```

### Systemd Service
```bash
# Start service
sudo systemctl start system-monitor.service

# View logs
sudo journalctl -u system-monitor.service -f
```

## Environment Variables

All configurable via environment variables:
- `RUST_LOG`: Log level (debug, info, warn, error)
- `DATABASE_URL`: SQLite database path
- `HOST`: Bind address (0.0.0.0 for containers)
- `PORT`: HTTP server port (8080)
- `ANOMALY_DETECTION_ENABLED`: Enable/disable anomaly detection
- `COLLECTION_INTERVAL_SECS`: Metrics collection interval

## Ports

- **8080**: Collector REST API and WebSocket
- **3000**: Web frontend (proxies to collector)

## Next Steps

1. Copy `.env.example` to `.env` and customize
2. Run `./scripts/build-all.sh` to build all components
3. Choose deployment method:
   - Docker: `./scripts/run-prod.sh --docker`
   - Native: `./scripts/run-dev.sh`
   - Systemd: `sudo ./scripts/run-prod.sh`
4. Access web UI at http://localhost:3000
5. Access API at http://localhost:8080

## Notes

- All scripts are executable (chmod +x applied)
- Scripts include colored output for better visibility
- Docker images use multi-stage builds for efficiency
- Systemd service includes comprehensive security settings
- Web frontend proxies API requests through nginx
- Health checks ensure service availability
- Graceful shutdown handling in all modes

## References

- Docker Compose: https://docs.docker.com/compose/
- Systemd: https://www.freedesktop.org/software/systemd/man/systemd.service.html
- Nginx: https://nginx.org/en/docs/
- Rust: https://www.rust-lang.org/
- React: https://reactjs.org/
