@echo off
REM run_fetch_data.bat - Helper script to fetch Wikipedia data on Windows
REM This script handles the Rust PATH issues

echo tellme - Wikipedia Content Fetcher
echo ====================================
echo.
echo This will download and process Wikipedia articles for all topics.
echo The process will take several minutes...
echo.

REM Add Rust to PATH for this session
set PATH=%PATH%;%USERPROFILE%\.cargo\bin

REM Ask for confirmation
set /p confirm=Continue? (y/N): 
if /i not "%confirm%"=="y" (
    echo Cancelled.
    pause
    exit /b 0
)

echo.
echo Starting data fetch process...
echo Press Ctrl+C to cancel if needed.
echo.

REM Run the data fetcher
cargo run --bin fetch_data

echo.
echo Data fetching complete!
echo You can now run tellme with: run_tellme.bat
echo.
pause 