<# .SYNOPSIS
    VigilantAI Project Initialization Script (Windows)
.DESCRIPTION
    Initializes the VigilantAI project for local development:
    1. Generates JWT RSA key pair
    2. Creates .env from .env.example if not exists
    3. Sets up the Python AI service virtual environment
    4. Provides instructions for Docker deployment
#>

param(
    [switch]$SkipDocker
)

$ErrorActionPreference = "Stop"
$ProjectRoot = Split-Path -Parent $PSScriptRoot
$KeysDir = Join-Path $ProjectRoot "keys"
$EnvFile = Join-Path $ProjectRoot ".env"
$EnvExample = Join-Path $ProjectRoot ".env.example"

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  VigilantAI Project Initialization" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""

# ── Step 1: Generate JWT Keys ──
Write-Host "[1/4] Generating JWT RSA key pair..." -ForegroundColor Yellow
& "$PSScriptRoot\generate-keys.ps1" -OutputDir $KeysDir

# Read generated keys
$privateKey = (Get-Content (Join-Path $KeysDir "private_key.pem") -Raw).Trim()
$publicKey = (Get-Content (Join-Path $KeysDir "public_key.pem") -Raw).Trim()

Write-Host ""

# ── Step 2: Create .env file ──
Write-Host "[2/4] Creating .env file..." -ForegroundColor Yellow
if (Test-Path -LiteralPath $EnvFile) {
    Write-Host "  .env already exists at $EnvFile — skipping." -ForegroundColor Green
} else {
    if (Test-Path -LiteralPath $EnvExample) {
        $envContent = Get-Content $EnvExample -Raw
        # Replace JWT key placeholders with generated keys
        $envContent = $envContent -replace 'JWT_PRIVATE_KEY=$', "JWT_PRIVATE_KEY=$privateKey"
        $envContent = $envContent -replace 'JWT_PUBLIC_KEY=$', "JWT_PUBLIC_KEY=$publicKey"
        # Replace placeholder passwords
        $envContent = $envContent -replace 'CHANGE_ME_STRONG_PASSWORD', 'vigilant_dev_password_2026'
        Set-Content -Path $EnvFile -Value $envContent -Encoding ascii
        Write-Host "  Created .env from .env.example" -ForegroundColor Green
    } else {
        Write-Host "  ERROR: .env.example not found at $EnvExample" -ForegroundColor Red
        exit 1
    }
}
Write-Host ""

# ── Step 3: Python virtual environment ──
Write-Host "[3/4] Setting up Python AI Service virtual environment..." -ForegroundColor Yellow
$AiServiceDir = Join-Path $ProjectRoot "ai-service"
$VenvDir = Join-Path $AiServiceDir "venv"
if (Test-Path -LiteralPath $VenvDir) {
    Write-Host "  Virtual environment already exists at $VenvDir — skipping." -ForegroundColor Green
} else {
    Write-Host "  Creating virtual environment..."
    python -m venv $VenvDir
    if ($?) {
        & "$VenvDir\Scripts\pip" install -r (Join-Path $AiServiceDir "requirements.txt") 2>&1 | Out-Null
        Write-Host "  Installed Python dependencies." -ForegroundColor Green
    } else {
        Write-Host "  WARNING: Could not create Python virtual environment. Install Python 3.11+ and run manually." -ForegroundColor Yellow
    }
}
Write-Host ""

# ── Step 4: Docker setup ──
Write-Host "[4/4] Docker setup..." -ForegroundColor Yellow
if (-not $SkipDocker) {
    Write-Host "  To start all services:"
    Write-Host "    docker compose up -d" -ForegroundColor White
    Write-Host ""
    Write-Host "  To start only infrastructure (PostgreSQL + Redis):"
    Write-Host "    docker compose up -d postgres redis" -ForegroundColor White
    Write-Host ""
    Write-Host "  To view logs:"
    Write-Host "    docker compose logs -f" -ForegroundColor White
} else {
    Write-Host "  Skipped Docker setup (use -SkipDocker to include)." -ForegroundColor Yellow
}
Write-Host ""

Write-Host "============================================" -ForegroundColor Cyan
Write-Host "  Initialization Complete!" -ForegroundColor Cyan
Write-Host "============================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Dashboard:  http://localhost:3000"
Write-Host "  API:        http://localhost:8080"
Write-Host "  Grafana:    http://localhost:3001"
Write-Host "  Prometheus: http://localhost:9090"
Write-Host ""
Write-Host "  Default login credentials (after seeding):"
Write-Host "    Email:    admin@vigilantai.local"
Write-Host "    Password: admin123"
Write-Host ""
Write-Host "  IMPORTANT: Change default credentials in production!"
Write-Host ""
