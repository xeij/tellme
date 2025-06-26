# install_rust.ps1 - Rust installation helper for Windows

Write-Host "tellme - Rust Installation Helper" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan
Write-Host ""

# Check if Rust is already installed
try {
    $rustVersion = rustc --version 2>$null
    if ($rustVersion) {
        Write-Host "✓ Rust is already installed: $rustVersion" -ForegroundColor Green
        Write-Host ""
        Write-Host "You can now run:" -ForegroundColor Yellow
        Write-Host "  cargo run --bin fetch_data" -ForegroundColor White
        Write-Host "  cargo run --bin tellme" -ForegroundColor White
        exit 0
    }
} catch {
    # Rust not found, continue with installation
}

Write-Host "Rust is not installed. Let us install it!" -ForegroundColor Yellow
Write-Host ""

# Download and run rustup-init.exe
Write-Host "1. Downloading Rust installer..." -ForegroundColor Blue
try {
    $url = "https://win.rustup.rs/x86_64"
    $output = "$env:TEMP\rustup-init.exe"
    
    Invoke-WebRequest -Uri $url -OutFile $output
    
    Write-Host "2. Running Rust installer..." -ForegroundColor Blue
    Write-Host "   Follow the prompts in the installer window." -ForegroundColor Gray
    Write-Host "   Choose option 1 (default installation) when prompted." -ForegroundColor Gray
    Write-Host ""
    
    # Run the installer
    Start-Process -FilePath $output -Wait
    
    Write-Host "3. Installation completed!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Please restart your PowerShell window and then run:" -ForegroundColor Yellow
    Write-Host "  cargo run --bin fetch_data" -ForegroundColor White
    Write-Host "  cargo run --bin tellme" -ForegroundColor White
    
} catch {
    Write-Host "Error downloading Rust installer: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install Rust manually:" -ForegroundColor Yellow
    Write-Host "1. Visit https://rustup.rs/" -ForegroundColor White
    Write-Host "2. Download and run rustup-init.exe" -ForegroundColor White
    Write-Host "3. Follow the installation instructions" -ForegroundColor White
}

Write-Host ""
Write-Host "For more help, see the README.md file." -ForegroundColor Gray 