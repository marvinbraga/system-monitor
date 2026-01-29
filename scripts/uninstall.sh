#!/bin/bash
#
# System Monitor - Uninstallation Script
# Removes the collector service and all related files
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SERVICE_NAME="system-monitor"
SERVICE_USER="monitor"
SERVICE_GROUP="monitor"
INSTALL_DIR="/opt/system-monitor"
DATA_DIR="/var/lib/system-monitor"
LOG_DIR="/var/log/system-monitor"
CONFIG_DIR="/etc/system-monitor"
SYSTEMD_DIR="/etc/systemd/system"

# Print functions
print_header() {
    echo -e "${BLUE}============================================${NC}"
    echo -e "${BLUE}  System Monitor - Uninstallation Script${NC}"
    echo -e "${BLUE}============================================${NC}"
    echo ""
}

print_step() {
    echo -e "${GREEN}[✓]${NC} $1"
}

print_info() {
    echo -e "${YELLOW}[i]${NC} $1"
}

print_error() {
    echo -e "${RED}[✗]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[!]${NC} $1"
}

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        print_error "This script must be run as root (use sudo)"
        exit 1
    fi
}

# Confirm uninstallation
confirm_uninstall() {
    echo -e "${YELLOW}WARNING: This will completely remove System Monitor service${NC}"
    echo ""
    echo "The following will be removed:"
    echo "  - Service: $SERVICE_NAME"
    echo "  - User: $SERVICE_USER"
    echo "  - Install directory: $INSTALL_DIR"
    echo "  - Config directory: $CONFIG_DIR"
    echo ""

    if [ "$1" != "--yes" ] && [ "$1" != "-y" ]; then
        read -p "Do you want to remove data and logs? [y/N]: " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            REMOVE_DATA=true
        else
            REMOVE_DATA=false
        fi

        echo ""
        read -p "Continue with uninstallation? [y/N]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "Uninstallation cancelled"
            exit 0
        fi
    else
        REMOVE_DATA=true
    fi
}

# Stop service
stop_service() {
    if systemctl is-active --quiet "$SERVICE_NAME"; then
        print_step "Stopping $SERVICE_NAME service"
        systemctl stop "$SERVICE_NAME"
    else
        print_info "Service is not running"
    fi
}

# Disable service
disable_service() {
    if systemctl is-enabled --quiet "$SERVICE_NAME" 2>/dev/null; then
        print_step "Disabling $SERVICE_NAME service"
        systemctl disable "$SERVICE_NAME"
    else
        print_info "Service is not enabled"
    fi
}

# Remove systemd service file
remove_service_file() {
    if [ -f "$SYSTEMD_DIR/$SERVICE_NAME.service" ]; then
        print_step "Removing systemd service file"
        rm -f "$SYSTEMD_DIR/$SERVICE_NAME.service"
        systemctl daemon-reload
    else
        print_info "Service file not found"
    fi
}

# Remove installation directory
remove_install_dir() {
    if [ -d "$INSTALL_DIR" ]; then
        print_step "Removing installation directory: $INSTALL_DIR"
        rm -rf "$INSTALL_DIR"
    else
        print_info "Installation directory not found"
    fi
}

# Remove configuration directory
remove_config_dir() {
    if [ -d "$CONFIG_DIR" ]; then
        print_step "Removing configuration directory: $CONFIG_DIR"
        rm -rf "$CONFIG_DIR"
    else
        print_info "Configuration directory not found"
    fi
}

# Remove data directory
remove_data_dir() {
    if [ "$REMOVE_DATA" = true ]; then
        if [ -d "$DATA_DIR" ]; then
            print_step "Removing data directory: $DATA_DIR"
            rm -rf "$DATA_DIR"
        else
            print_info "Data directory not found"
        fi
    else
        print_warning "Keeping data directory: $DATA_DIR"
        print_info "To manually remove: sudo rm -rf $DATA_DIR"
    fi
}

# Remove log directory
remove_log_dir() {
    if [ "$REMOVE_DATA" = true ]; then
        if [ -d "$LOG_DIR" ]; then
            print_step "Removing log directory: $LOG_DIR"
            rm -rf "$LOG_DIR"
        else
            print_info "Log directory not found"
        fi
    else
        print_warning "Keeping log directory: $LOG_DIR"
        print_info "To manually remove: sudo rm -rf $LOG_DIR"
    fi
}

# Remove system user
remove_user() {
    if getent passwd "$SERVICE_USER" > /dev/null 2>&1; then
        print_step "Removing user: $SERVICE_USER"
        userdel "$SERVICE_USER" 2>/dev/null || true
    else
        print_info "User $SERVICE_USER not found"
    fi
}

# Remove system group
remove_group() {
    if getent group "$SERVICE_GROUP" > /dev/null 2>&1; then
        print_step "Removing group: $SERVICE_GROUP"
        groupdel "$SERVICE_GROUP" 2>/dev/null || true
    else
        print_info "Group $SERVICE_GROUP not found"
    fi
}

# Clean up systemd journal logs
cleanup_journal() {
    print_step "Cleaning up systemd journal logs"
    journalctl --vacuum-time=1s --identifier="$SERVICE_NAME" 2>/dev/null || true
}

# Print final information
print_summary() {
    echo ""
    echo -e "${GREEN}============================================${NC}"
    echo -e "${GREEN}  Uninstallation Complete!${NC}"
    echo -e "${GREEN}============================================${NC}"
    echo ""

    if [ "$REMOVE_DATA" != true ]; then
        echo -e "${YELLOW}Preserved directories:${NC}"
        [ -d "$DATA_DIR" ] && echo "  Data: $DATA_DIR"
        [ -d "$LOG_DIR" ] && echo "  Logs: $LOG_DIR"
        echo ""
        echo -e "${YELLOW}To manually remove:${NC}"
        echo "  sudo rm -rf $DATA_DIR"
        echo "  sudo rm -rf $LOG_DIR"
        echo ""
    fi

    echo -e "${GREEN}System Monitor has been completely removed.${NC}"
    echo ""
}

# Backup data before removal
backup_data() {
    if [ -d "$DATA_DIR" ] && [ "$REMOVE_DATA" = true ]; then
        BACKUP_FILE="/tmp/system-monitor-backup-$(date +%Y%m%d-%H%M%S).tar.gz"

        read -p "Do you want to create a backup before removal? [Y/n]: " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Nn]$ ]]; then
            print_step "Creating backup: $BACKUP_FILE"
            tar -czf "$BACKUP_FILE" -C "$(dirname "$DATA_DIR")" "$(basename "$DATA_DIR")" 2>/dev/null || true

            if [ -f "$BACKUP_FILE" ]; then
                print_step "Backup created successfully"
                echo "  Location: $BACKUP_FILE"
            else
                print_warning "Backup creation failed"
            fi
        fi
    fi
}

# Main uninstallation flow
main() {
    print_header

    check_root
    confirm_uninstall "$@"

    echo ""
    print_info "Starting uninstallation..."
    echo ""

    backup_data
    stop_service
    disable_service
    remove_service_file
    remove_install_dir
    remove_config_dir
    remove_data_dir
    remove_log_dir
    remove_user
    remove_group
    cleanup_journal

    print_summary
}

# Run main
main "$@"
