#!/bin/bash
# Scripts de emergência pós-desligamento forçado

echo "=== Reiniciando Nautilus ==="
killall nautilus 2>/dev/null
sleep 2
nautilus &

echo "=== Limpando caches ==="
rm -rf ~/.cache/thumbnails/*
rm -rf ~/.cache/nautilus/*
rm -rf ~/.local/share/gvfs-metadata/*

echo "=== Verificando sistema ==="
df -h
journalctl -b -p err --no-pager | tail -10

echo "✅ Procedimento concluído!"
