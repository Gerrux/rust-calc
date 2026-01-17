<#
.SYNOPSIS
    Signs the application executable with a code signing certificate
.PARAMETER ExePath
    Path to the executable to sign
.PARAMETER Thumbprint
    Certificate thumbprint (optional, auto-detects if not specified)
.EXAMPLE
    .\sign.ps1
    .\sign.ps1 -ExePath ".\dist\rust-calc.exe"
#>

param(
    [string]$ExePath,
    [string]$Thumbprint
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

# Default exe path
if (-not $ExePath) {
    $ExePath = Join-Path $ProjectRoot "dist\rust-calc.exe"
}

if (-not (Test-Path $ExePath)) {
    Write-Host "Executable not found: $ExePath" -ForegroundColor Red
    Write-Host "Run build script first: .\scripts\build.ps1" -ForegroundColor Yellow
    exit 1
}

# Find certificate
if (-not $Thumbprint) {
    $Publisher = "CN=gerrux"
    $cert = Get-ChildItem Cert:\CurrentUser\My -CodeSigningCert |
            Where-Object { $_.Subject -like "*$Publisher*" } |
            Select-Object -First 1

    if (-not $cert) {
        Write-Host "No code signing certificate found!" -ForegroundColor Red
        Write-Host "Run .\scripts\create-cert.ps1 first (as Administrator)" -ForegroundColor Yellow
        exit 1
    }
    $Thumbprint = $cert.Thumbprint
}

Write-Host "Signing: $ExePath" -ForegroundColor Cyan
Write-Host "Certificate: $Thumbprint" -ForegroundColor Gray

# Find signtool
$signtoolPaths = @(
    "${env:ProgramFiles(x86)}\Windows Kits\10\bin\*\x64\signtool.exe",
    "${env:ProgramFiles}\Windows Kits\10\bin\*\x64\signtool.exe"
)

$signtool = $null
foreach ($pattern in $signtoolPaths) {
    $found = Get-Item $pattern -ErrorAction SilentlyContinue | Sort-Object -Descending | Select-Object -First 1
    if ($found) {
        $signtool = $found.FullName
        break
    }
}

if (-not $signtool) {
    Write-Host "signtool.exe not found!" -ForegroundColor Red
    Write-Host "Install Windows SDK from Visual Studio Installer" -ForegroundColor Yellow
    exit 1
}

Write-Host "Using: $signtool" -ForegroundColor Gray

# Sign the executable
& $signtool sign /sha1 $Thumbprint /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 /v $ExePath

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "Successfully signed!" -ForegroundColor Green

    # Verify
    Write-Host ""
    Write-Host "Verifying signature..." -ForegroundColor Cyan
    & $signtool verify /pa /v $ExePath
} else {
    Write-Host "Signing failed!" -ForegroundColor Red
    exit 1
}
