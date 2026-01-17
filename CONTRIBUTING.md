# Contributing to Rust Calculator

Thank you for your interest in contributing! This document provides guidelines and information for contributors.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/gerrux/rust-calc/issues)
2. If not, create a new issue using the bug report template
3. Include as much detail as possible:
   - Steps to reproduce
   - Expected vs actual behavior
   - OS and version
   - Screenshots if applicable

### Suggesting Features

1. Check existing issues and discussions for similar ideas
2. Create a new issue using the feature request template
3. Explain the use case and benefits

### Pull Requests

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature-name`
3. Make your changes
4. Run tests and linting (see below)
5. Commit with clear messages
6. Push and create a Pull Request

## Development Setup

### Prerequisites

- [Rust](https://rustup.rs/) 1.70 or later
- Git

### Clone and Build

```bash
git clone https://github.com/gerrux/rust-calc.git
cd rust-calc
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Quality

Before submitting a PR, ensure your code passes all checks:

```bash
# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run tests
cargo test

# Full check (same as CI)
cargo fmt -- --check && cargo clippy -- -D warnings && cargo test
```

## Code Style

- Follow standard Rust conventions and idioms
- Use `cargo fmt` for formatting
- Keep functions focused and small
- Add doc comments for public APIs
- Prefer descriptive variable names

### Commit Messages

- Use present tense: "Add feature" not "Added feature"
- Use imperative mood: "Fix bug" not "Fixes bug"
- Keep the first line under 72 characters
- Reference issues when applicable: "Fix calculation error (#123)"

Examples:
```
Add keyboard shortcut for angle mode toggle
Fix decimal point validation in expressions
Update dependencies to latest versions
Refactor calculator module for clarity
```

## Project Structure

```
src/
├── main.rs          # Application entry point
├── app.rs           # UI rendering and event handling
└── calculator.rs    # Core calculation logic
```

### Key Components

- **Calculator struct**: Manages expression state and evaluation
- **App struct**: Handles UI rendering with egui
- **History**: Stores previous calculations

## Testing Guidelines

- Write tests for new calculation logic
- Test edge cases (empty input, invalid expressions, etc.)
- Keep tests focused and fast

Example test:
```rust
#[test]
fn test_basic_addition() {
    let mut calc = Calculator::default();
    calc.input_digit("2");
    calc.input_operator("+");
    calc.input_digit("3");
    let result = calc.calculate().unwrap();
    assert_eq!(result, 5.0);
}
```

## Questions?

Feel free to open an issue or start a discussion if you have questions about contributing.

Thank you for helping improve Rust Calculator!
