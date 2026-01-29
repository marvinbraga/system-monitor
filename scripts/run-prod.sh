#!/bin/bash
# Run system-monitor in production mode

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "======================================"
echo "System Monitor - Production Mode"
echo "======================================"
echo ""

cd "$PROJECT_ROOT"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[*]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

# Check if running in Docker mode (recommended for production)
if [ "$1" == "--docker" ]; then
    print_status "Starting with Docker Compose in production mode..."

    # Create data directory if it doesn't exist
    mkdir -p data

    # Export environment for docker-compose
    export DATA_DIR="$(pwd)/data"
    export RUST_LOG=info

    # Pull latest images (if they exist)
    print_status "Pulling latest images..."
    docker-compose pull 2>/dev/null || true

    # Start services in detached mode
    print_status "Starting services..."
    docker-compose up -d

    echo ""
    print_success "Services started successfully"
    echo ""

    # Show service status
    print_status "Service status:"
    docker-compose ps

    echo ""
    print_status "Collector API: http://localhost:8080"
    print_status "Web UI: http://localhost:3000"
    echo ""

    echo "Useful commands:"
    echo "  - View logs: docker-compose logs -f"
    echo "  - Stop services: docker-compose down"
    echo "  - Restart services: docker-compose restart"
    echo "  - View status: docker-compose ps"

    exit 0
fi

# Native production mode
print_warning "Running in native production mode"
print_warning "For production deployments, consider using:"
print_warning "  - Docker: $0 --docker"
print_warning "  - Systemd: See collector/systemd/system-monitor.service"
echo ""

# Check if release build exists
if [ ! -f "target/release/collector" ]; then
    print_error "Release build not found. Please run: cargo build --release"
    exit 1
fi

# Set production environment variables
export RUST_LOG="${RUST_LOG:-info}"
export DATABASE_URL="${DATABASE_URL:-/var/lib/system-monitor/system-monitor.db}"
export HOST="${HOST:-0.0.0.0}"
export PORT="${PORT:-8080}"
export ANOMALY_DETECTION_ENABLED="${ANOMALY_DETECTION_ENABLED:-true}"
export ANOMALY_LOG_PATH="${ANOMALY_LOG_PATH:-/var/log/system-monitor/anomalies.log}"
export COLLECTION_INTERVAL_SECS="${COLLECTION_INTERVAL_SECS:-5}"

# Create required directories
print_status "Setting up directories..."
sudo mkdir -p /var/lib/system-monitor
sudo mkdir -p /var/log/system-monitor

# Create user if doesn't exist
if ! id -u monitor &>/dev/null; then
    print_status "Creating monitor user..."
    sudo useradd -r -s /bin/false monitor
fi

# Set permissions
sudo chown -R monitor:monitor /var/lib/system-monitor
sudo chown -R monitor:monitor /var/log/system-monitor

print_status "Environment Configuration:"
echo "  RUST_LOG: $RUST_LOG"
echo "  DATABASE_URL: $DATABASE_URL"
echo "  HOST: $HOST"
echo "  PORT: $PORT"
echo "  ANOMALY_DETECTION_ENABLED: $ANOMALY_DETECTION_ENABLED"
echo "  COLLECTION_INTERVAL_SECS: $COLLECTION_INTERVAL_SECS"
echo ""

# Install systemd service
if [ -f "collector/systemd/system-monitor.service" ]; then
    print_status "Installing systemd service..."

    # Copy binary
    sudo mkdir -p /opt/system-monitor
    sudo cp target/release/collector /opt/system-monitor/
    sudo chown monitor:monitor /opt/system-monitor/collector
    sudo chmod +x /opt/system-monitor/collector

    # Install service file
    sudo cp collector/systemd/system-monitor.service /etc/systemd/system/

    # Reload systemd
    sudo systemctl daemon-reload

    # Enable service
    sudo systemctl enable system-monitor.service

    # Start service
    sudo systemctl start system-monitor.service

    print_success "Systemd service installed and started"
    echo ""

    # Show service status
    print_status "Service status:"
    sudo systemctl status system-monitor.service --no-pager

    echo ""
    print_status "Collector API: http://$HOST:$PORT"
    echo ""

    echo "Useful commands:"
    echo "  - View logs: sudo journalctl -u system-monitor.service -f"
    echo "  - Stop service: sudo systemctl stop system-monitor.service"
    echo "  - Restart service: sudo systemctl restart system-monitor.service"
    echo "  - Check status: sudo systemctl status system-monitor.service"

else
    print_error "Systemd service file not found"
    print_status "Running collector directly..."

    # Run as current user (not recommended for production)
    ./target/release/collector
fi
