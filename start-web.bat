@echo off
echo ========================================
echo   convrs-web - Web-Version starten
echo ========================================
echo.
echo Stoppe alte trunk-Prozesse...
taskkill /F /IM trunk.exe 2>nul
timeout /t 2 >nul

echo.
echo Starte trunk Server...
echo Browser oeffnet sich automatisch auf http://127.0.0.1:8080
echo.
echo Druecke Ctrl+C zum Beenden
echo.

cd %~dp0
trunk serve --address 127.0.0.1

