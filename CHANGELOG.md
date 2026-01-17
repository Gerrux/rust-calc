# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-01-17

### Added
- Initial release of Rust Calculator
- Scientific functions: sin, cos, tan, asin, acos, atan, log, ln, sqrt
- Mathematical constants: Pi, e
- Modern dark theme UI with custom frameless window
- Full keyboard input support
- Calculation history with click-to-reuse functionality
- Angle mode toggle (Degrees/Radians)
- Cross-platform support: Windows, macOS, Linux
- UPX compression support for smaller binaries
- Code signing support for Windows builds
- Build scripts for all platforms (PowerShell, Bash, Makefile)

### Technical
- Built with Rust and egui/eframe
- Minimal dependencies for fast startup
- Release binary ~1.2 MB (with UPX compression)
- No runtime dependencies

[Unreleased]: https://github.com/gerrux/rust-calc/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/gerrux/rust-calc/releases/tag/v1.0.0
