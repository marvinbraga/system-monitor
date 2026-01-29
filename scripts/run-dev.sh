#!/bin/bash
# Run system-monitor in development mode

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "======================================"
echo "System Monitor - Development Mode"
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

# Check if running in Docker mode
if [ "$1" == "--docker" ]; then
    print_status "Starting with Docker Compose in development mode..."

    # Create data directory if it doesn't exist
    mkdir -p data

    # Export environment for docker-compose
    export DATA_DIR="$(pwd)/data"
    export RUST_LOG=debug

    # Start services
    docker-compose up --build

    exit 0
fi

# Native development mode
print_status "Starting in native development mode..."
echo ""

# Set development environment variables
export RUST_LOG=debug
export DATABASE_URL="${DATABASE_URL:-./data/system-monitor.db}"
export HOST="${HOST:-127.0.0.1}"
export PORT="${PORT:-8080}"
export ANOMALY_DETECTION_ENABLED="${ANOMALY_DETECTION_ENABLED:-true}"
export ANOMALY_LOG_PATH="${ANOMALY_LOG_PATH:-./data/anomalies.log}"
export COLLECTION_INTERVAL_SECS="${COLLECTION_INTERVAL_SECS:-5}"

# Create data directory
mkdir -p data

print_status "Environment Configuration:"
echo "  RUST_LOG: $RUST_LOG"
echo "  DATABASE_URL: $DATABASE_URL"
echo "  HOST: $HOST"
echo "  PORT: $PORT"
echo "  ANOMALY_DETECTION_ENABLED: $ANOMALY_DETECTION_ENABLED"
echo "  COLLECTION_INTERVAL_SECS: $COLLECTION_INTERVAL_SECS"
echo ""

# Check for web frontend
if [ -d "web-frontend" ] && [ -f "web-frontend/package.json" ]; then
    HAS_WEB=true
else
    HAS_WEB=false
fi

# Start collector in background
print_status "Starting collector service..."
cargo run --package collector &
COLLECTOR_PID=$!

# Wait for collector to start
sleep 3

if kill -0 $COLLECTOR_PID 2>/dev/null; then
    print_success "Collector started (PID: $COLLECTOR_PID)"
else
    print_error "Failed to start collector"
    exit 1
fi

echo ""
print_status "Collector API: http://$HOST:$PORT"
print_status "Health check: http://$HOST:$PORT/health"
print_status "WebSocket: ws://$HOST:$PORT/ws"
echo ""

# Start web frontend if available
if [ "$HAS_WEB" = true ]; then
    print_status "Starting web frontend development server..."
    cd web-frontend

    # Set frontend environment
    export REACT_APP_API_URL="http://$HOST:$PORT"
    export REACT_APP_WS_URL="ws://$HOST:$PORT"

    npm run dev &
    WEB_PID=$!
    cd ..

    sleep 3

    if kill -0 $WEB_PID 2>/dev/null; then
        print_success "Web frontend started (PID: $WEB_PID)"
        echo ""
        print_status "Web UI: http://localhost:3000"
    else
        print_error "Failed to start web frontend"
        WEB_PID=""
    fi
fi

echo ""
print_success "System Monitor is running in development mode"
echo ""
echo "Press Ctrl+C to stop all services"
echo ""

# Cleanup function
cleanup() {
    echo ""
    print_status "Shutting down services..."

    if [ -n "$COLLECTOR_PID" ]; then
        kill $COLLECTOR_PID 2>/dev/null || true
        print_success "Collector stopped"
    fi

    if [ -n "$WEB_PID" ]; then
        kill $WEB_PID 2>/dev/null || true
        print_success "Web frontend stopped"
    fi

    echo ""
    print_success "All services stopped"
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Wait for processes
if [ -n "$WEB_PID" ]; then
    wait $COLLECTOR_PID $WEB_PID
else
    wait $COLLECTOR_PID
fi
