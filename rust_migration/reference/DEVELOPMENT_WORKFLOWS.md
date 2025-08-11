# Development Workflows for Rust Migration

## Recommended Justfile Tasks

Create or update your `justfile` with these Rust-specific tasks:

```make
# Set default shell and error handling
set shell := ["bash", "-c"]
set dotenv-load

# Default task - show help
default:
    @just --list

# Development tasks
dev-setup:
    @echo "Setting up Rust development environment..."
    rustup update stable
    rustup component add clippy rustfmt
    cargo install cargo-audit cargo-outdated

# Build tasks
build:
    cargo build --release

build-debug:
    cargo build

check:
    cargo check --all-targets

# Testing tasks
test:
    RUST_BACKTRACE=1 cargo test -- --nocapture

test-quiet:
    cargo test --quiet

test-integration:
    cargo test --test integration_tests

# Code quality tasks
clippy:
    cargo clippy --all-targets -- -D warnings

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

# Security and dependency tasks
audit:
    cargo audit

outdated:
    cargo outdated

# Performance tasks
bench:
    cargo bench -- --noplot --quiet

bench-full:
    cargo bench

# Sample runs for testing
run-sample:
    cargo run --release -- generate --count 10 --format csv --output sample_output.csv

run-xml-sample:
    cargo run --release -- generate --count 5 --format xml --output sample_config.xml

run-large-test:
    cargo run --release -- generate --count 1000 --format csv --output large_test.csv

# Validation tasks
validate-sample:
    cargo run --release -- validate sample_config.xml

# Development convenience tasks
clean:
    cargo clean
    rm -f *.csv *.xml

watch:
    cargo watch -x check -x test

watch-run:
    cargo watch -x "run -- generate --count 5 --format csv"

# Documentation tasks
doc:
    cargo doc --no-deps --open

doc-all:
    cargo doc --open

# Release preparation
pre-release: fmt clippy test bench audit
    @echo "All pre-release checks passed!"

# CI simulation
ci-check: fmt-check clippy test bench
    @echo "CI checks completed successfully!"
```

## CI/CD Configuration

### GitHub Actions Workflow

Create `.github/workflows/rust.yml`:

```yaml
name: Rust CI/CD

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test Suite
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable]

    steps:
      - name: Checkout code
        uses: actions/checkout@v3

      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: ${{ matrix.rust }}
          profile: minimal
          override: true
          components: rustfmt, clippy

      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ 
            hashFiles('**/Cargo.lock') }}

      - name: Check formatting
        run: cargo fmt --all -- --check

      - name: Run clippy
        run: cargo clippy --all-targets -- -D warnings

      - name: Run tests
        run: cargo test --locked --verbose --all-features
        env:
          RUST_BACKTRACE: 1

      - name: Run integration tests
        run: cargo test --test integration_tests --locked --verbose
        if: matrix.os == 'ubuntu-latest'

  benchmarks:
    name: Performance Benchmarks
    runs-on: ubuntu-latest
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'

    steps:
      - name: Checkout code
        uses: actions/checkout@v3

      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-bench-${{ hashFiles('**/Cargo.lock') }}

      - name: Run benchmarks
        run: cargo bench -- --noplot --quiet

  security:
    name: Security Audit
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v3

      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true

      - name: Install cargo-audit
        run: cargo install cargo-audit

      - name: Run security audit
        run: cargo audit

  release:
    name: Release Binaries
    runs-on: ${{ matrix.os }}
    if: github.event_name == 'push' && startsWith(github.ref, 'refs/tags/v')
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            name: linux
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            name: windows
          - os: macos-latest
            target: x86_64-apple-darwin
            name: macos

    steps:
      - name: Checkout code
        uses: actions/checkout@v3

      - name: Install Rust toolchain
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          override: true
          target: ${{ matrix.target }}

      - name: Build release binary
        run: cargo build --release --target ${{ matrix.target }}

      - name: Create release archive
        shell: bash
        run: |
          if [[ "${{ matrix.name }}" == "windows" ]]; then
            asset_name="opnsense-config-faker-${{ matrix.name }}.zip"
            zip -r "${asset_name}" target/${{ matrix.target }}/release/opnsense-config-faker.exe
          else
            asset_name="opnsense-config-faker-${{ matrix.name }}.tar.gz"
            tar -czf "${asset_name}" -C target/${{ matrix.target }}/release opnsense-config-faker
          fi
          echo "ASSET_NAME=${asset_name}" >> $GITHUB_ENV

      - name: Upload release asset
        uses: actions/upload-artifact@v3
        with:
          name: ${{ env.ASSET_NAME }}
          path: ${{ env.ASSET_NAME }}
```

## Development Environment Setup

### Prerequisites

```bash
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install required components
rustup component add clippy rustfmt
rustup target add x86_64-unknown-linux-gnu x86_64-pc-windows-gnu x86_64-apple-darwin

# Install development tools
cargo install cargo-watch cargo-audit cargo-outdated just
```

### IDE Configuration

#### VS Code Settings

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "rust-analyzer.checkOnSave.allTargets": false,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer",
    "editor.formatOnSave": true
  },
  "files.associations": {
    "justfile": "makefile"
  }
}
```

#### VS Code Extensions

- `rust-lang.rust-analyzer` - Core Rust support
- `serayuzgur.crates` - Cargo.toml management
- `vadimcn.vscode-lldb` - Debugging support
- `skellock.just` - Justfile syntax highlighting

## Quality Gates and Standards

### Pre-commit Checklist

- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo test` passes with no failures
- [ ] `cargo audit` shows no vulnerabilities
- [ ] Documentation updated if public API changed
- [ ] Benchmark performance within acceptable range

### Code Review Guidelines

- All public functions must have documentation
- Error handling must use `thiserror` for libraries, `anyhow` for applications
- No `unwrap()` or `expect()` in production code paths
- Performance-critical paths should be benchmarked
- Cross-platform compatibility verified

### Performance Benchmarking Standards

- All benchmarks must run in under 30 seconds
- Memory usage should be profiled for configurations > 1000 VLANs
- Performance regressions > 20% require investigation
- Benchmark results documented in commit messages

## Troubleshooting Common Issues

### Build Issues

```bash
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check for outdated dependencies
cargo outdated
```

### Test Issues

```bash
# Run single test with output
cargo test test_name -- --nocapture

# Run tests with backtrace
RUST_BACKTRACE=full cargo test
```

### Performance Issues

```bash
# Profile with flamegraph
cargo install flamegraph
cargo flamegraph --bin opnsense-config-faker -- generate --count 1000

# Check memory usage with heaptrack (Linux)
heaptrack target/release/opnsense-config-faker generate --count 1000
```

---

This workflow guide provides comprehensive development support for the Rust migration project, ensuring consistent quality and efficient development practices.
