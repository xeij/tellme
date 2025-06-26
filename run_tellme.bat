@echo off
REM run_tellme.bat - Helper script to run tellme on Windows
REM This script handles the Rust PATH issues

echo tellme - Random Knowledge from Wikipedia
echo =========================================
echo.

REM Add Rust to PATH for this session
set PATH=%PATH%;%USERPROFILE%\.cargo\bin

REM Check if data exists
if not exist "tellme_data\tellme.db" (
    echo No content database found!
    echo.
    echo Please run the data fetcher first:
    echo   run_fetch_data.bat
    echo.
    echo This will download Wikipedia content for all topics.
    pause
    exit /b 1
)

REM Run tellme
echo Starting tellme...
echo Press Ctrl+C to exit if needed.
echo.
cargo run --bin tellme

pause 