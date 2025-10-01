# Installation

## Prerequisites

- **Rust 1.70+**: Required for building from source
- **Git**: For cloning the repository
- **Just**: Task runner (optional but recommended)

## Installation Methods

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/EvilBit-Labs/OPNsense-config-faker.git
cd OPNsense-config-faker

# Build the release binary
cargo build --release

# The binary will be available at target/release/opnsense-config-faker
```

### Using Cargo Install

```bash
# Install directly from GitHub (when available)
cargo install --git https://github.com/EvilBit-Labs/OPNsense-config-faker.git

# Or install from crates.io (when published)
cargo install opnsense-config-faker
```

### Pre-built Binaries

Pre-built binaries are available in the [Releases](https://github.com/EvilBit-Labs/OPNsense-config-faker/releases) section for:

- Linux (x86_64)
- macOS (x86_64, ARM64)
- Windows (x86_64)

## Development Setup

For contributors and developers:

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install development tools
rustup component add clippy rustfmt
cargo install cargo-audit cargo-outdated just

# Clone and setup
git clone https://github.com/EvilBit-Labs/OPNsense-config-faker.git
cd OPNsense-config-faker

# Run development checks
just ci-check
```

## Verification

Verify your installation:

```bash
# Check version
cargo run --release -- --version

# Run help
cargo run --release -- --help

# Test with a small generation
cargo run --release -- generate vlan --count 5 --output test.xml
```

## Troubleshooting

### Common Issues

**Build fails with "no such file or directory":**

- Ensure you have Rust 1.70+ installed
- Run `rustup update` to get the latest toolchain

**Permission denied errors:**

- On Unix systems, you may need to add `~/.cargo/bin` to your PATH
- On Windows, ensure you have proper permissions for the installation directory

**Network connectivity issues:**

- The tool works offline, but initial setup requires internet for dependencies
- Use `cargo build --offline` if you have cached dependencies

### Getting Help

- Check the [Troubleshooting Guide](advanced/troubleshooting.md)
- Open an issue on [GitHub](https://github.com/EvilBit-Labs/OPNsense-config-faker/issues)
- Review the [Examples](user-guide/examples.md) for usage patterns
