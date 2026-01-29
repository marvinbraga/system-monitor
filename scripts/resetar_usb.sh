#!/bin/bash
# Script para resetar dispositivos USB problemáticos

echo "=== Resetando controladores USB ==="
echo "Desconecte o Elgato Wave:3 fisicamente"
echo "Aguarde 10 segundos"
echo "Reconecte o dispositivo"
echo ""
echo "Se o problema persistir, execute como root:"
echo "sudo modprobe -r snd_usb_audio && sudo modprobe snd_usb_audio"
