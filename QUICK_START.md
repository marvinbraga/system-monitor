# 🚀 Quick Start - System Monitor

Ultra-simplified guide to get started in 30 seconds.

---

## ⚡ Installation (1 command)

```bash
./setup.sh
```

Enter your sudo password when prompted.

**Done!** Open: http://localhost:5252

---

## 🛑 Stop Everything

```bash
docker compose down
sudo systemctl stop system-monitor
```

---

## 🗑️ Uninstall

```bash
./teardown.sh
```

---

## 📚 More Information

- **Complete Manual**: `USER_GUIDE.md`
- **README**: `README.md`
- **Technical Documentation**: `CLAUDE.md`

---

## 🆘 Problems?

### Dashboard won't open
```bash
docker compose restart frontend
```

### No metrics
```bash
sudo systemctl restart system-monitor
```

### View logs
```bash
sudo journalctl -u system-monitor -f
```

---

**Tip**: Always run `./setup.sh` from the project root!
