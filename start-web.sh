#!/bin/bash

echo "========================================"
echo "  ASP CLI - Web-Version starten"
echo "========================================"
echo ""
echo "Stoppe alte trunk-Prozesse..."
pkill -f trunk 2>/dev/null || true
sleep 2

echo ""
echo "Starte trunk Server..."
echo "Browser öffnet sich automatisch auf http://127.0.0.1:8080"
echo ""
echo "Drücke Ctrl+C zum Beenden"
echo ""

cd "$(dirname "$0")"
trunk serve

