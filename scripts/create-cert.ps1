#Requires -RunAsAdministrator
<#
.SYNOPSIS
    Creates a self-signed code signing certificate for development/testing
.DESCRIPTION
    This script creates a self-signed certificate and installs it to the certificate store.
    For production, purchase a certificate from a trusted CA (DigiCert, Sectigo, etc.)
.EXAMPLE
    .\create-cert.ps1
#>

$ErrorActionPreference = "Stop"

$CertName = "Rust Calculator Development"
$Publisher = "CN=gerrux, O=gerrux, C=RU"

Write-Host "Creating self-signed code signing certificate..." -ForegroundColor Cyan

# Check if cert already exists
$existingCert = Get-ChildItem Cert:\CurrentUser\My | Where-Object { $_.Subject -eq $Publisher }
if ($existingCert) {
    Write-Host "Certificate already exists: $($existingCert.Thumbprint)" -ForegroundColor Yellow
    Write-Host "To remove it: Remove-Item Cert:\CurrentUser\My\$($existingCert.Thumbprint)"
    exit 0
}

# Create certificate
$cert = New-SelfSignedCertificate `
    -Type CodeSigningCert `
    -Subject $Publisher `
    -FriendlyName $CertName `
    -CertStoreLocation Cert:\CurrentUser\My `
    -NotAfter (Get-Date).AddYears(3) `
    -KeyUsage DigitalSignature `
    -KeyAlgorithm RSA `
    -KeyLength 2048

Write-Host ""
Write-Host "Certificate created successfully!" -ForegroundColor Green
Write-Host "Thumbprint: $($cert.Thumbprint)" -ForegroundColor White
Write-Host "Subject: $($cert.Subject)" -ForegroundColor Gray
Write-Host "Expires: $($cert.NotAfter)" -ForegroundColor Gray

# Export to PFX (optional, for backup)
$pfxPath = Join-Path $PSScriptRoot "..\certs\dev-cert.pfx"
$certsDir = Split-Path $pfxPath -Parent

if (-not (Test-Path $certsDir)) {
    New-Item -ItemType Directory -Path $certsDir | Out-Null
}

$password = Read-Host "Enter password for PFX export (or press Enter to skip)" -AsSecureString
if ($password.Length -gt 0) {
    Export-PfxCertificate -Cert $cert -FilePath $pfxPath -Password $password | Out-Null
    Write-Host "Certificate exported to: $pfxPath" -ForegroundColor Green
}

Write-Host ""
Write-Host "To sign your application, run:" -ForegroundColor Cyan
Write-Host "  .\scripts\sign.ps1" -ForegroundColor White
