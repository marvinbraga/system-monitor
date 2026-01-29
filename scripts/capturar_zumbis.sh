#!/bin/bash
echo "Monitorando processos zumbis por 30 segundos..."
echo "Timestamp | PID | PPID | Status | Comando"
echo "=========================================="

for i in {1..30}; do
    ps -eo pid,ppid,stat,comm | grep -E " Z[+ ]" | while read line; do
        echo "$(date '+%H:%M:%S') | $line"
    done
    sleep 1
done

echo ""
echo "Resumo dos processos pai (PPID) dos zumbis:"
ps -eo pid,ppid,stat,comm | grep -E " Z[+ ]" | awk '{print $2}' | sort | uniq -c
