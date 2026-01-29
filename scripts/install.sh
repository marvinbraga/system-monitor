#!/bin/bash
#
# System Monitor - Installation Script
# Installs the collector service as a systemd daemon
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

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Print functions
print_header() {
    echo -e "${BLUE}============================================${NC}"
    echo -e "${BLUE}  System Monitor - Installation Script${NC}"
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

# Check if running as root
check_root() {
    if [[ $EUID -ne 0 ]]; then
        print_error "This script must be run as root (use sudo)"
        exit 1
    fi
}

# Check if service is already installed
check_existing() {
    if systemctl is-active --quiet "$SERVICE_NAME"; then
        print_info "Service is currently running. Stopping..."
        systemctl stop "$SERVICE_NAME"
    fi

    if systemctl is-enabled --quiet "$SERVICE_NAME" 2>/dev/null; then
        print_info "Service is enabled. Disabling..."
        systemctl disable "$SERVICE_NAME"
    fi
}

# Create system user and group
create_user() {
    if ! getent group "$SERVICE_GROUP" > /dev/null 2>&1; then
        print_step "Creating group: $SERVICE_GROUP"
        groupadd --system "$SERVICE_GROUP"
    else
        print_info "Group $SERVICE_GROUP already exists"
    fi

    if ! getent passwd "$SERVICE_USER" > /dev/null 2>&1; then
        print_step "Creating user: $SERVICE_USER"
        useradd --system \
            --gid "$SERVICE_GROUP" \
            --home-dir "$DATA_DIR" \
            --shell /usr/sbin/nologin \
            --comment "System Monitor Service" \
            "$SERVICE_USER"
    else
        print_info "User $SERVICE_USER already exists"
    fi
}

# Create directories
create_directories() {
    print_step "Creating directories"

    mkdir -p "$INSTALL_DIR"
    mkdir -p "$DATA_DIR"
    mkdir -p "$LOG_DIR"
    mkdir -p "$CONFIG_DIR"

    print_step "Setting directory permissions"
    chown -R "$SERVICE_USER:$SERVICE_GROUP" "$DATA_DIR"
    chown -R "$SERVICE_USER:$SERVICE_GROUP" "$LOG_DIR"
    chmod 755 "$INSTALL_DIR"
    chmod 750 "$DATA_DIR"
    chmod 750 "$LOG_DIR"
}

# Build the project
build_project() {
    print_step "Building collector service (release mode)"
    cd "$PROJECT_ROOT"

    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed. Please install Rust from https://rustup.rs/"
        exit 1
    fi

    cargo build --release --package collector

    if [ ! -f "$PROJECT_ROOT/target/release/collector" ]; then
        print_error "Build failed. Binary not found at target/release/collector"
        exit 1
    fi

    print_step "Build completed successfully"
}

# Install binary
install_binary() {
    print_step "Installing collector binary"

    cp "$PROJECT_ROOT/target/release/collector" "$INSTALL_DIR/collector"
    chmod 755 "$INSTALL_DIR/collector"
    chown root:root "$INSTALL_DIR/collector"
}

# Create configuration file
create_config() {
    print_step "Creating configuration file"

    cat > "$CONFIG_DIR/config.toml" << 'EOF'
# System Monitor Configuration

# Database settings
database_url = "/var/lib/system-monitor/metrics.db"

# Collection settings
collection_interval = 2  # seconds

# API settings
api_host = "127.0.0.1"
api_port = 8080

# Retention policy
retention_days = 30  # Keep data for 30 days

# Logging
log_level = "info"  # trace, debug, info, warn, error

# Anomaly detection thresholds
[thresholds]
cpu_critical = 90.0
cpu_warning = 70.0
memory_critical = 95.0
memory_warning = 80.0
temperature_critical = 85.0
temperature_warning = 75.0
disk_critical = 90.0
disk_warning = 80.0
EOF

    chmod 644 "$CONFIG_DIR/config.toml"
    chown root:root "$CONFIG_DIR/config.toml"
}

# Create systemd service file
create_service() {
    print_step "Creating systemd service file"

    cat > "$SYSTEMD_DIR/$SERVICE_NAME.service" << EOF
[Unit]
Description=System Monitor Collector Service
Documentation=https://github.com/yourusername/system-monitor
After=network.target
Wants=network-online.target

[Service]
Type=simple
User=$SERVICE_USER
Group=$SERVICE_GROUP
WorkingDirectory=$DATA_DIR

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=$DATA_DIR $LOG_DIR

# Capabilities (needed for system metrics collection)
CapabilityBoundingSet=
AmbientCapabilities=

# Environment
Environment="RUST_LOG=info"
Environment="CONFIG_PATH=$CONFIG_DIR/config.toml"
Environment="DATABASE_PATH=$DATA_DIR/metrics.db"

# Start the service
ExecStart=$INSTALL_DIR/collector

# Restart policy
Restart=on-failure
RestartSec=10s
StartLimitInterval=5min
StartLimitBurst=3

# Logging
StandardOutput=journal
StandardError=journal
SyslogIdentifier=$SERVICE_NAME

[Install]
WantedBy=multi-user.target
EOF

    chmod 644 "$SYSTEMD_DIR/$SERVICE_NAME.service"
}

# Enable and start service
enable_service() {
    print_step "Reloading systemd daemon"
    systemctl daemon-reload

    print_step "Enabling $SERVICE_NAME service"
    systemctl enable "$SERVICE_NAME"

    print_step "Starting $SERVICE_NAME service"
    systemctl start "$SERVICE_NAME"

    # Wait a moment and check status
    sleep 2

    if systemctl is-active --quiet "$SERVICE_NAME"; then
        print_step "Service started successfully"
    else
        print_error "Service failed to start. Check logs with: journalctl -u $SERVICE_NAME"
        exit 1
    fi
}

# Create helper scripts
create_helpers() {
    print_step "Creating helper scripts"

    # Status script
    cat > "$INSTALL_DIR/status.sh" << 'EOF'
#!/bin/bash
systemctl status system-monitor
EOF
    chmod +x "$INSTALL_DIR/status.sh"

    # Logs script
    cat > "$INSTALL_DIR/logs.sh" << 'EOF'
#!/bin/bash
journalctl -u system-monitor -f
EOF
    chmod +x "$INSTALL_DIR/logs.sh"

    # Restart script
    cat > "$INSTALL_DIR/restart.sh" << 'EOF'
#!/bin/bash
echo "Restarting system-monitor service..."
sudo systemctl restart system-monitor
echo "Done. Check status with: sudo systemctl status system-monitor"
EOF
    chmod +x "$INSTALL_DIR/restart.sh"
}

# Print final information
print_summary() {
    echo ""
    echo -e "${GREEN}============================================${NC}"
    echo -e "${GREEN}  Installation Complete!${NC}"
    echo -e "${GREEN}============================================${NC}"
    echo ""
    echo -e "${YELLOW}Service Information:${NC}"
    echo "  Name:      $SERVICE_NAME"
    echo "  User:      $SERVICE_USER"
    echo "  Install:   $INSTALL_DIR"
    echo "  Data:      $DATA_DIR"
    echo "  Config:    $CONFIG_DIR/config.toml"
    echo "  Logs:      journalctl -u $SERVICE_NAME"
    echo ""
    echo -e "${YELLOW}Useful Commands:${NC}"
    echo "  Status:    systemctl status $SERVICE_NAME"
    echo "  Stop:      systemctl stop $SERVICE_NAME"
    echo "  Start:     systemctl start $SERVICE_NAME"
    echo "  Restart:   systemctl restart $SERVICE_NAME"
    echo "  Logs:      journalctl -u $SERVICE_NAME -f"
    echo ""
    echo -e "${YELLOW}Helper Scripts:${NC}"
    echo "  $INSTALL_DIR/status.sh    - Check service status"
    echo "  $INSTALL_DIR/logs.sh      - View live logs"
    echo "  $INSTALL_DIR/restart.sh   - Restart service"
    echo ""
    echo -e "${YELLOW}API Endpoints:${NC}"
    echo "  http://127.0.0.1:8080/health"
    echo "  http://127.0.0.1:8080/api/v1/metrics/current"
    echo "  http://127.0.0.1:8080/api/v1/anomalies"
    echo "  ws://127.0.0.1:8080/ws"
    echo ""
    echo -e "${YELLOW}Configuration:${NC}"
    echo "  Edit: $CONFIG_DIR/config.toml"
    echo "  Then: systemctl restart $SERVICE_NAME"
    echo ""
    echo -e "${GREEN}Service is now running!${NC}"
    echo ""
}

# Main installation flow
main() {
    print_header

    check_root
    check_existing
    create_user
    create_directories
    build_project
    install_binary
    create_config
    create_service
    create_helpers
    enable_service
    print_summary
}

# Run main
main "$@"
