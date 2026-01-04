#!/bin/bash

echo "========================================"
echo "  ASP CLI - Web-Version starten"
echo "========================================"
echo ""
echo "Stoppe alte trunk-Prozesse..."
cmd //c "taskkill /F /IM trunk.exe 2>nul" || true
sleep 2

echo ""
echo "Starte trunk Server..."
echo "Browser öffnet sich automatisch auf http://127.0.0.1:8080"
echo ""
echo "Drücke Ctrl+C zum Beenden"
echo ""

cd "$(dirname "$0")"
trunk serve

