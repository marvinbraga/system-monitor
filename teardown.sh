#!/bin/bash
#
# System Monitor - Simplified Uninstallation
# Removes the service and stops the frontend
#

set -e

# Output colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  System Monitor - Uninstall           ║${NC}"
echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo ""

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo -e "${RED}✗ Do not run this script as root (without sudo)${NC}"
   echo -e "${YELLOW}  Use: ./teardown.sh${NC}"
   exit 1
fi

# Ask for confirmation
echo -e "${YELLOW}⚠${NC}  This will:"
echo "  • Stop and remove the system-monitor service"
echo "  • Stop and remove the frontend container"
echo "  • Keep data in /var/lib/system-monitor (optional to delete)"
echo ""
read -p "Continue? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Cancelled.${NC}"
    exit 0
fi

# Step 1: Stop and remove frontend
echo ""
echo -e "${BLUE}[1/2]${NC} Stopping frontend..."
if docker compose down 2>/dev/null; then
    echo -e "${GREEN}✓${NC} Frontend stopped and removed"
else
    echo -e "${YELLOW}⚠${NC} Frontend was already stopped or not found"
fi

# Step 2: Uninstall service
echo ""
echo -e "${BLUE}[2/2]${NC} Uninstalling service..."
echo -e "${YELLOW}→${NC} You will need to enter sudo password"

if [ -f "./scripts/uninstall.sh" ]; then
    if sudo ./scripts/uninstall.sh; then
        echo -e "${GREEN}✓${NC} Service uninstalled"
    else
        echo -e "${RED}✗${NC} Error uninstalling service"
        exit 1
    fi
else
    echo -e "${YELLOW}⚠${NC} Uninstall script not found"
fi

# Ask about removing data
echo ""
read -p "Do you want to remove stored data? (y/N): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}→${NC} Removing data..."
    sudo rm -rf /var/lib/system-monitor
    sudo rm -rf /var/log/system-monitor
    sudo rm -rf /etc/system-monitor
    echo -e "${GREEN}✓${NC} Data removed"
else
    echo -e "${YELLOW}ℹ${NC}  Data kept in:"
    echo "  • /var/lib/system-monitor/"
    echo "  • /var/log/system-monitor/"
    echo "  • /etc/system-monitor/"
fi

echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   ✓ Uninstallation Complete!          ║${NC}"
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo ""
echo -e "${BLUE}To reinstall:${NC}"
echo -e "  ${YELLOW}./setup.sh${NC}"
echo ""
