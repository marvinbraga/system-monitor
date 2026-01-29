#!/bin/bash
# Build all components of the system-monitor project

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "======================================"
echo "Building System Monitor - All Components"
echo "======================================"
echo ""

cd "$PROJECT_ROOT"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[*]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

# Check if Docker is available
if command -v docker &> /dev/null; then
    HAS_DOCKER=true
    print_success "Docker is available"
else
    HAS_DOCKER=false
    print_warning "Docker is not available - skipping Docker builds"
fi

# Check if Rust is available
if command -v cargo &> /dev/null; then
    HAS_RUST=true
    print_success "Rust is available"
else
    HAS_RUST=false
    print_warning "Rust is not available - skipping native builds"
fi

# Check if Node.js is available
if command -v npm &> /dev/null; then
    HAS_NODE=true
    print_success "Node.js is available"
else
    HAS_NODE=false
    print_warning "Node.js is not available - skipping web frontend build"
fi

echo ""

# Build Rust components (native)
if [ "$HAS_RUST" = true ]; then
    print_status "Building Rust workspace (native)..."
    cargo build --release
    print_success "Rust workspace built successfully"
    echo ""
fi

# Build web frontend (native)
if [ "$HAS_NODE" = true ]; then
    if [ -d "web-frontend" ] && [ -f "web-frontend/package.json" ]; then
        print_status "Building web frontend (native)..."
        cd web-frontend
        npm install
        npm run build
        cd ..
        print_success "Web frontend built successfully"
        echo ""
    else
        print_warning "Web frontend not found or incomplete - skipping"
        echo ""
    fi
fi

# Build Docker images
if [ "$HAS_DOCKER" = true ]; then
    print_status "Building Docker images..."

    # Build collector image
    print_status "Building collector Docker image..."
    docker build -t system-monitor-collector:latest -f collector/Dockerfile .
    print_success "Collector image built successfully"

    # Build web frontend image (if exists)
    if [ -d "web-frontend" ] && [ -f "web-frontend/Dockerfile" ]; then
        print_status "Building web frontend Docker image..."
        docker build -t system-monitor-web:latest -f web-frontend/Dockerfile .
        print_success "Web frontend image built successfully"
    fi

    echo ""
    print_success "All Docker images built successfully"

    # Show built images
    echo ""
    print_status "Docker images:"
    docker images | grep system-monitor
fi

echo ""
echo "======================================"
print_success "Build completed successfully!"
echo "======================================"
echo ""

# Show built artifacts
if [ "$HAS_RUST" = true ]; then
    echo "Native binaries:"
    echo "  - collector: target/release/collector"
    if [ -f "target/release/tui-client" ]; then
        echo "  - tui-client: target/release/tui-client"
    fi
    echo ""
fi

if [ "$HAS_NODE" = true ] && [ -d "web-frontend/dist" ]; then
    echo "Web frontend:"
    echo "  - dist: web-frontend/dist/"
    echo ""
fi

if [ "$HAS_DOCKER" = true ]; then
    echo "Docker images:"
    echo "  - system-monitor-collector:latest"
    if [ -d "web-frontend" ]; then
        echo "  - system-monitor-web:latest"
    fi
    echo ""
fi

echo "Next steps:"
echo "  - Run in development mode: ./scripts/run-dev.sh"
echo "  - Run in production mode: ./scripts/run-prod.sh"
echo "  - Run with Docker: docker-compose up -d"
