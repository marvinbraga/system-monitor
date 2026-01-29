#!/bin/bash
echo "=== Verificação de Processos Zumbis ==="
echo ""
ZOMBIES=$(ps -eo stat | grep -c "^Z")
echo "Processos zumbis: $ZOMBIES"
echo ""
if [ $ZOMBIES -gt 0 ]; then
    echo "⚠️  Zumbis detectados:"
    ps -eo pid,ppid,stat,cmd | grep "^[0-9].*Z"
    echo ""
    echo "Para eliminar, reinicie o processo pai (PPID)"
else
    echo "✅ Sistema limpo - nenhum zumbi detectado"
fi
echo ""
echo "=== Carga do Sistema ==="
uptime
