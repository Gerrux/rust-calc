#Requires -Version 5.1
<#
.SYNOPSIS
    Optimized build script for rust-calc (final size ~555KB)
.PARAMETER Profile
    Build profile: release (default), release-fast, or debug
.PARAMETER Clean
    Clean before building
.PARAMETER NoCompress
    Skip UPX compression
.PARAMETER NoOptimize
    Skip nightly + build-std optimization (use stable)
.PARAMETER Sign
    Sign the executable after building
.PARAMETER Install
    Install to user's bin directory
.EXAMPLE
    .\build.ps1              # Full optimized build (~555KB)
    .\build.ps1 -NoOptimize  # Stable build (~600KB)
    .\build.ps1 -Clean -Sign
#>

param(
    [ValidateSet("release", "release-fast", "debug")]
    [string]$Profile = "release",
    [switch]$Clean,
    [switch]$NoCompress,
    [switch]$NoOptimize,
    [switch]$Sign,
    [switch]$Install
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir
$DistDir = Join-Path $ProjectRoot "dist"
$Target = "x86_64-pc-windows-msvc"

Set-Location $ProjectRoot

# Find UPX
function Find-Upx {
    $upx = Get-Command upx -ErrorAction SilentlyContinue
    if ($upx) { return $upx.Source }

    $locations = @(
        "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\*upx*\*\upx.exe",
        "$env:ChocolateyInstall\bin\upx.exe",
        "$env:SCOOP\apps\upx\current\upx.exe"
    )

    foreach ($pattern in $locations) {
        $found = Get-Item $pattern -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($found) { return $found.FullName }
    }
    return $null
}

# Check nightly toolchain
function Test-Nightly {
    $result = rustup run nightly rustc --version 2>&1
    return $LASTEXITCODE -eq 0
}

# Install nightly if needed
function Install-Nightly {
    Write-Host "Installing nightly toolchain..." -ForegroundColor Yellow
    rustup install nightly 2>&1 | Out-Null
    rustup component add rust-src --toolchain nightly 2>&1 | Out-Null
}

# Clean
if ($Clean) {
    Write-Host "Cleaning..." -ForegroundColor Yellow
    cargo clean
    if (Test-Path $DistDir) {
        Remove-Item $DistDir -Recurse -Force
    }
}

# Determine build mode
$UseNightly = (-not $NoOptimize) -and ($Profile -ne "debug")

if ($UseNightly) {
    if (-not (Test-Nightly)) {
        Install-Nightly
    }
    Write-Host "Building with nightly + build-std (optimized)..." -ForegroundColor Cyan
} else {
    Write-Host "Building with stable toolchain..." -ForegroundColor Cyan
}

# Build
if ($Profile -eq "debug") {
    cargo build
    $SourceDir = Join-Path $ProjectRoot "target\debug"
} elseif ($UseNightly) {
    cargo +nightly build --release -Z build-std=std,panic_abort --target $Target
    $SourceDir = Join-Path $ProjectRoot "target\$Target\release"
} else {
    cargo build --profile $Profile
    $SourceDir = Join-Path $ProjectRoot "target\$Profile"
}

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Create dist directory
if (-not (Test-Path $DistDir)) {
    New-Item -ItemType Directory -Path $DistDir | Out-Null
}

$BinaryName = "rust-calc.exe"
$SourcePath = Join-Path $SourceDir $BinaryName
$DestPath = Join-Path $DistDir $BinaryName

if (-not (Test-Path $SourcePath)) {
    Write-Host "Binary not found: $SourcePath" -ForegroundColor Red
    exit 1
}

$OriginalSize = (Get-Item $SourcePath).Length

# UPX compression
if (-not $NoCompress -and $Profile -ne "debug") {
    Write-Host "Compressing with UPX..." -ForegroundColor Cyan

    $upxPath = Find-Upx

    if ($upxPath) {
        # Remove old file first
        if (Test-Path $DestPath) {
            Remove-Item $DestPath -Force
        }

        & $upxPath --best --lzma $SourcePath -o $DestPath 2>&1 | Out-Null

        if ($LASTEXITCODE -eq 0) {
            $CompressedSize = (Get-Item $DestPath).Length
            $Ratio = [math]::Round(($CompressedSize / $OriginalSize) * 100, 1)
            Write-Host "Compressed: $Ratio% of original" -ForegroundColor Green
        } else {
            Write-Host "UPX failed, copying uncompressed" -ForegroundColor Yellow
            Copy-Item $SourcePath $DestPath -Force
        }
    } else {
        Write-Host "UPX not found. Install: winget install upx.upx" -ForegroundColor Yellow
        Copy-Item $SourcePath $DestPath -Force
    }
} else {
    Copy-Item $SourcePath $DestPath -Force
}

# Sign if requested
if ($Sign) {
    Write-Host ""
    $signScript = Join-Path $ScriptDir "sign.ps1"
    if (Test-Path $signScript) {
        & $signScript -ExePath $DestPath
    } else {
        Write-Host "Sign script not found" -ForegroundColor Yellow
    }
}

# Install if requested
if ($Install) {
    $InstallDir = Join-Path $env:USERPROFILE "bin"
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir | Out-Null
    }
    Copy-Item $DestPath (Join-Path $InstallDir $BinaryName) -Force
    Write-Host "Installed to: $InstallDir\$BinaryName" -ForegroundColor Cyan
}

# Final output
$FinalSize = (Get-Item $DestPath).Length
$SizeKB = [math]::Round($FinalSize / 1KB)

Write-Host ""
Write-Host "========================================" -ForegroundColor DarkGray
Write-Host "  Output: $DestPath" -ForegroundColor White
Write-Host "  Size:   $SizeKB KB" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor DarkGray
Write-Host ""
