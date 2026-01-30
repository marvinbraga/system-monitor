#!/bin/bash
#
# System Monitor - Simplified Installation
# This script installs and starts everything automatically
#

set -e

# Output colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   System Monitor - Quick Setup        ║${NC}"
echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo ""

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo -e "${RED}✗ Do not run this script as root (without sudo)${NC}"
   echo -e "${YELLOW}  Use: ./setup.sh${NC}"
   exit 1
fi

# Check if in correct directory
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}✗ Run this script from the project directory${NC}"
    exit 1
fi

# Step 1: Compile the project
echo -e "${BLUE}[1/3]${NC} Compiling collector..."
if cargo build --release --package collector > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Collector compiled successfully"
else
    echo -e "${RED}✗${NC} Compilation error. Check if Rust is installed."
    exit 1
fi

# Step 2: Install the service
echo ""
echo -e "${BLUE}[2/3]${NC} Installing systemd service..."
echo -e "${YELLOW}→${NC} You will need to enter sudo password"

if sudo ./scripts/install.sh; then
    echo -e "${GREEN}✓${NC} Service installed and started"
else
    echo -e "${RED}✗${NC} Error installing service"
    exit 1
fi

# Wait for service to start
sleep 3

# Check if service is running
if sudo systemctl is-active --quiet system-monitor; then
    echo -e "${GREEN}✓${NC} Collector running correctly"
else
    echo -e "${RED}✗${NC} Service did not start. Check logs with:"
    echo -e "${YELLOW}  sudo journalctl -u system-monitor -n 50${NC}"
    exit 1
fi

# Step 3: Start the frontend
echo ""
echo -e "${BLUE}[3/3]${NC} Starting web dashboard..."

if docker compose up -d frontend > /dev/null 2>&1; then
    echo -e "${GREEN}✓${NC} Dashboard started"
else
    echo -e "${RED}✗${NC} Error starting dashboard"
    echo -e "${YELLOW}  Check if Docker is installed and running${NC}"
    exit 1
fi

# Wait for container to start
sleep 5

# Check if container is running
if docker compose ps | grep -q "Up"; then
    echo -e "${GREEN}✓${NC} Frontend running correctly"
else
    echo -e "${RED}✗${NC} Container did not start. Check with:"
    echo -e "${YELLOW}  docker compose logs frontend${NC}"
    exit 1
fi

# Success!
echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║     ✓ Installation Complete!          ║${NC}"
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo ""
echo -e "${YELLOW}📊 Dashboard:${NC}     http://localhost:5252"
echo -e "${YELLOW}🔌 API:${NC}           http://localhost:5253"
echo ""
echo -e "${BLUE}Useful commands:${NC}"
echo -e "  • View status:      ${YELLOW}sudo systemctl status system-monitor${NC}"
echo -e "  • View logs:        ${YELLOW}sudo journalctl -u system-monitor -f${NC}"
echo -e "  • Stop everything:  ${YELLOW}docker compose down && sudo systemctl stop system-monitor${NC}"
echo -e "  • View manual:      ${YELLOW}cat USER_GUIDE.md${NC}"
echo ""
echo -e "${GREEN}Opening dashboard in browser...${NC}"

# Try to open browser
sleep 2
if command -v xdg-open > /dev/null; then
    xdg-open http://localhost:5252 2>/dev/null &
elif command -v firefox > /dev/null; then
    firefox http://localhost:5252 2>/dev/null &
elif command -v google-chrome > /dev/null; then
    google-chrome http://localhost:5252 2>/dev/null &
else
    echo -e "${YELLOW}ℹ${NC}  Open manually: http://localhost:5252"
fi

echo ""
echo -e "${GREEN}✓ Installation completed successfully!${NC}"
echo ""
