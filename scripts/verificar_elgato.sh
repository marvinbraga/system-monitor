#!/bin/bash
echo "=== Verificando Elgato Wave:3 ==="
lsusb | grep -i elgato
echo ""
echo "=== Dispositivos de áudio ==="
arecord -l 2>/dev/null | grep -i elgato
echo ""
echo "=== Erros USB recentes ==="
journalctl -b --since "5 minutes ago" | grep -i "usb.*error\|usb.*fail" | tail -5
