# 📖 User Guide - System Monitor

Quick and practical guide to install and use System Monitor.

---

## 🚀 Quick Installation (3 Steps)

### 1. Install the Collector (Backend)

```bash
cd /path/to/system-monitor
sudo ./scripts/install.sh
```

Enter your sudo password when prompted.

✅ This installs and starts the service that collects system metrics.

### 2. Start the Dashboard (Frontend)

```bash
docker compose up -d frontend
```

✅ This starts the web interface in a Docker container.

### 3. Access the Dashboard

Open your browser at: **http://localhost:5252**

🎉 Done! The system is running.

---

## 📊 What You'll See on the Dashboard

- **CPU Usage**: CPU and per-core usage
- **Memory Usage**: RAM and Swap consumption
- **Temperature**: Sensor and component temperatures
- **Disk Usage**: Disk space used/available
- **USB Devices**: Connected USB devices
- **Anomalies**: Detected problem alerts
- **Network**: Network traffic (RX/TX)

The dashboard updates **automatically every 2 seconds**.

---

## 🔧 Useful Commands

### Check Service Status

```bash
sudo systemctl status system-monitor
```

### View Collector Logs

```bash
sudo journalctl -u system-monitor -f
```

### Restart Collector

```bash
sudo systemctl restart system-monitor
```

### Stop Frontend

```bash
docker compose down
```

### Restart Frontend

```bash
docker compose restart frontend
```

### View Frontend Logs

```bash
docker logs -f system-monitor-frontend
```

---

## 🛑 How to Stop Everything

### Stop Frontend

```bash
docker compose down
```

### Stop Collector

```bash
sudo systemctl stop system-monitor
```

### Uninstall Completely

```bash
sudo ./scripts/uninstall.sh
docker compose down
```

---

## ⚙️ Advanced Configuration

### Change Collector Port

Edit the file:
```bash
sudo nano /etc/systemd/system/system-monitor.service
```

Change the line:
```
Environment="PORT=5253"
```

Then restart:
```bash
sudo systemctl daemon-reload
sudo systemctl restart system-monitor
```

### Adjust Anomaly Thresholds

Edit:
```bash
sudo nano /etc/system-monitor/config.toml
```

Example:
```toml
[thresholds]
cpu_critical = 90.0      # Critical CPU alert
cpu_warning = 70.0       # Warning CPU alert
memory_critical = 95.0   # Critical memory alert
temperature_critical = 85.0  # Critical temperature in °C
```

---

## 🔍 Troubleshooting

### Dashboard won't open (localhost:5252)

1. Check if frontend is running:
   ```bash
   docker compose ps
   ```

2. If not, start it:
   ```bash
   docker compose up -d frontend
   ```

### "Error loading metrics" on Dashboard

1. Check if collector is running:
   ```bash
   sudo systemctl status system-monitor
   ```

2. If not, start it:
   ```bash
   sudo systemctl start system-monitor
   ```

3. Test API directly:
   ```bash
   curl http://localhost:5253/health
   ```

### Service won't start

1. View logs:
   ```bash
   sudo journalctl -u system-monitor -n 50
   ```

2. Check if port is free:
   ```bash
   sudo lsof -i :5253
   ```

3. Reinstall:
   ```bash
   sudo ./scripts/uninstall.sh
   sudo ./scripts/install.sh
   ```

---

## 📁 File Structure

```
/opt/system-monitor/           # Collector binary
/var/lib/system-monitor/       # SQLite database
/var/log/system-monitor/       # System logs
/etc/system-monitor/           # Configuration files
```

---

## 🔄 System Update

When there are code updates:

```bash
# 1. Stop everything
sudo systemctl stop system-monitor
docker compose down

# 2. Recompile
cargo build --release

# 3. Reinstall
sudo ./scripts/install.sh
docker compose up -d --build frontend
```

---

## ℹ️ Technical Information

- **Collector**: Runs natively on host (not in Docker)
- **Frontend**: Runs in Docker container
- **Collector Port**: 5253 (REST API + WebSocket)
- **Frontend Port**: 5252 (HTTP)
- **Collection Interval**: 2 seconds
- **Data Retention**: 30 days (configurable)

---

## 📞 Support

- **Technical Documentation**: See `CLAUDE.md`
- **Scripts**: See `scripts/` directory
- **System Logs**: `sudo journalctl -u system-monitor -f`

---

## 🎯 Quick Command Summary

```bash
# INSTALLATION
sudo ./scripts/install.sh
docker compose up -d frontend

# ACCESS
firefox http://localhost:5252

# STATUS
sudo systemctl status system-monitor
docker compose ps

# LOGS
sudo journalctl -u system-monitor -f
docker logs -f system-monitor-frontend

# STOP
docker compose down
sudo systemctl stop system-monitor

# UNINSTALL
sudo ./scripts/uninstall.sh
docker compose down
```

---

**Version**: 1.0
**Last Updated**: January 29, 2026
