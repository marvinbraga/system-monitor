#!/bin/bash
echo "Remova qualquer tampa da webcam e pressione ENTER..."
read
echo "Capturando em 3 segundos..."
sleep 3
ffmpeg -f v4l2 -video_size 1280x720 -i /dev/video0 -frames:v 1 ~/dados/webcam_teste.jpg -y 2>&1 | tail -3
echo ""
echo "✅ Imagem salva em: ~/dados/webcam_teste.jpg"
