# Rust Calculator

[![CI](https://github.com/gerrux/rust-calc/actions/workflows/ci.yml/badge.svg)](https://github.com/gerrux/rust-calc/actions/workflows/ci.yml)
[![Release](https://github.com/gerrux/rust-calc/actions/workflows/release.yml/badge.svg)](https://github.com/gerrux/rust-calc/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A lightweight, cross-platform scientific calculator with a modern dark UI built in Rust.

**Final size: ~1.2 MB** (with UPX compression)

<!--
TODO: Add screenshot here
![Screenshot](assets/screenshot.png)
-->

## Features

- **Scientific Functions**: sin, cos, tan, asin, acos, atan, log, ln, sqrt, power
- **Constants**: Pi (π), Euler's number (e)
- **Modern UI**: Dark theme with custom frameless window
- **Keyboard Support**: Full keyboard input support
- **History**: Calculation history with click-to-reuse
- **Angle Modes**: Degrees/Radians toggle
- **Cross-platform**: Windows, macOS, Linux
- **Standalone**: Single executable, no runtime dependencies

## Installation

### Download Binary

Download the latest release for your platform from the [Releases](https://github.com/gerrux/rust-calc/releases) page:

| Platform | Download |
|----------|----------|
| Windows (x64) | [rust-calc-windows-x64.zip](https://github.com/gerrux/rust-calc/releases/latest) |
| macOS (Intel) | [rust-calc-macos-x64.tar.gz](https://github.com/gerrux/rust-calc/releases/latest) |
| macOS (Apple Silicon) | [rust-calc-macos-arm64.tar.gz](https://github.com/gerrux/rust-calc/releases/latest) |
| Linux (x64) | [rust-calc-linux-x64.tar.gz](https://github.com/gerrux/rust-calc/releases/latest) |

### Build from Source

#### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- (Optional) [UPX](https://upx.github.io/) for compression

#### Quick Build

```bash
git clone https://github.com/gerrux/rust-calc.git
cd rust-calc
cargo build --release
```

The binary will be at `target/release/rust-calc` (or `rust-calc.exe` on Windows).

#### Build Scripts

```powershell
# Windows PowerShell - full build with compression
.\scripts\build.ps1

# With code signing
.\scripts\build.ps1 -Sign

# Install to ~/bin
.\scripts\build.ps1 -Install
```

```bash
# Linux/macOS
./scripts/build.sh

# With installation
./scripts/build.sh --install
```

```bash
# Using Make (Linux/macOS)
make release
make install
```

## Usage

### Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `0-9` | Input digits |
| `+ - * /` | Operators |
| `.` `,` | Decimal point |
| `^` | Power |
| `( )` | Parentheses |
| `Enter` | Calculate |
| `Backspace` | Delete last |
| `Escape` | Clear all |
| `r` | Toggle Radians/Degrees |

### Examples

```
sin(45)     → 0.7071067812  (in Degrees mode)
2^10        → 1024
sqrt(144)   → 12
ln(e)       → 1
log(1000)   → 3
π * 2       → 6.2831853072
```

## Project Structure

```
rust-calc/
├── src/
│   ├── main.rs          # Entry point
│   ├── app.rs           # UI and rendering
│   └── calculator.rs    # Calculation logic
├── assets/
│   ├── icon.ico         # Windows icon
│   └── icon.png         # Cross-platform icon
├── scripts/
│   ├── build.ps1        # Windows build script
│   ├── build.sh         # Unix build script
│   ├── create-cert.ps1  # Create code signing cert
│   └── sign.ps1         # Sign Windows executable
├── .github/
│   └── workflows/       # CI/CD pipelines
├── build.rs             # Windows resource embedding
├── Cargo.toml
├── Makefile
├── CHANGELOG.md
├── CONTRIBUTING.md
└── LICENSE
```

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## Code Signing (Windows)

For development/testing with self-signed certificate:

```powershell
# 1. Create certificate (run as Admin)
.\scripts\create-cert.ps1

# 2. Build and sign
.\scripts\build.ps1 -Sign
```

For production releases, use a certificate from a trusted CA (DigiCert, Sectigo, etc.).

## License

MIT License - see [LICENSE](LICENSE) for details.

## Acknowledgments

- [egui](https://github.com/emilk/egui) - Immediate mode GUI library
- [meval](https://github.com/rekka/meval-rs) - Math expression evaluator
