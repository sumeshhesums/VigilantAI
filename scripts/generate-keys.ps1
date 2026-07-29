param(
    [string]$OutputDir = (Join-Path (Get-Location) "keys")
)

Write-Host "=== VigilantAI JWT Key Generation ===" -ForegroundColor Cyan
Write-Host ""

# Create output directory
if (-not (Test-Path -LiteralPath $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
    Write-Host "Created directory: $OutputDir" -ForegroundColor Green
}

$privateKeyPath = Join-Path $OutputDir "private_key.pem"
$publicKeyPath = Join-Path $OutputDir "public_key.pem"

# Generate RSA private key (2048-bit)
Write-Host "Generating RSA 2048-bit private key..." -ForegroundColor Yellow
# Use .NET's RSACryptoServiceProvider for cross-platform compatibility
$rsa = New-Object System.Security.Cryptography.RSACryptoServiceProvider(2048)

# Export private key in PEM format
$privateKeyBytes = $rsa.ExportRSAPrivateKey()
$privateKeyBase64 = [Convert]::ToBase64String($privateKeyBytes, [System.Base64FormattingOptions]::InsertLineBreaks)
@"
-----BEGIN RSA PRIVATE KEY-----
$privateKeyBase64
-----END RSA PRIVATE KEY-----
"@ | Out-File -FilePath $privateKeyPath -Encoding ascii -Force

# Export public key in PEM format
$publicKeyBytes = $rsa.ExportRSAPublicKey()
$publicKeyBase64 = [Convert]::ToBase64String($publicKeyBytes, [System.Base64FormattingOptions]::InsertLineBreaks)
@"
-----BEGIN RSA PUBLIC KEY-----
$publicKeyBase64
-----END RSA PUBLIC KEY-----
"@ | Out-File -FilePath $publicKeyPath -Encoding ascii -Force

Write-Host "Private key: $privateKeyPath" -ForegroundColor Green
Write-Host "Public key:  $publicKeyPath" -ForegroundColor Green
Write-Host ""

# Generate .env compatible format (single-line for env vars)
$privateKeySingleLine = (Get-Content $privateKeyPath -Raw) -replace "`r`n", '\n' -replace "`n", '\n'
$publicKeySingleLine = (Get-Content $publicKeyPath -Raw) -replace "`r`n", '\n' -replace "`n", '\n'

Write-Host "=== Environment Variables for .env ===" -ForegroundColor Cyan
Write-Host "JWT_PRIVATE_KEY=$privateKeySingleLine"
Write-Host "JWT_PUBLIC_KEY=$publicKeySingleLine"

Write-Host ""
Write-Host "✅ Key generation complete!" -ForegroundColor Green
Write-Host "Copy the JWT_PRIVATE_KEY and JWT_PUBLIC_KEY values above into your .env file." -ForegroundColor Yellow
