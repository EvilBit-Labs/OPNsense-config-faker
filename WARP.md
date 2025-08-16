# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Purpose and Scope

Single-purpose Rust tool that generates realistic OPNsense `config.xml` files for testing, training, and development.

- **OPNsense-only**: Exclusively generates OPNsense firewall configurations; other platforms are out of scope
- **CSV is internal**: CSV generation is an implementation detail, not a first-class user-facing feature
- **Operator-centric**: Offline-first, no telemetry, deterministic-friendly CLI designed for network operators

## Essential Commands

### Build

```bash
cargo build --all-features
cargo build --release --all-features
```

### Lint and Format (zero-tolerance policy)

```bash
cargo fmt --check
cargo clippy --all-targets --all-features --benches -- -D warnings
just format-check
just lint
```

### Tests

```bash
cargo test --all-features
TERM=dumb cargo test --all-features          # For deterministic output
just test
just test-unit                               # Unit tests only
just test-integration                        # Integration tests only
just test-no-bench                           # Exclude benchmarks
```

### Coverage

```bash
just install-cov                             # One-time setup
just coverage                                # Local with threshold enforcement
just coverage-html                           # HTML report → target/llvm-cov/html/index.html
just coverage-report                         # Terminal output
just coverage-clean                          # Clean artifacts
```

### Benchmarks (excluded from coverage)

```bash
cargo bench --all-features
just bench
```

### CI/QA Shortcuts

```bash
just qa                                      # Format check, lint, test
just qa-cov                                  # QA + coverage
just ci-qa                                   # CI-friendly QA
just ci-check                                # Full CI validation
just ci-check-fast                           # Quick CI check (no coverage)
just act-ci                                  # Test CI locally with act
just act-ci-list                             # List available CI jobs
```

### Run the Generator

```bash
# Generate OPNsense XML configs (primary use case)
cargo run --release -- xml --base-config config.xml --count 25

# Generate VLAN data
cargo run --release -- generate vlan --count 10 --output vlans.xml
cargo run --release -- generate vlan --count 50 --format csv --output network-data.csv

# Generate with firewall rules
cargo run --release -- generate --count 10 --format xml --base-config config.xml --include-firewall-rules --firewall-rules-per-vlan 3 --firewall-rule-complexity intermediate
```

## Architecture and Module Boundaries

### Data Flow

`CLI args → generator → model → validate → xml writer → config.xml on disk`

CSV may be used internally for processing, but is not a public contract.

### Module Responsibilities

- **cli**: Clap v4 derived CLI, subcommands (xml, generate), argument parsing, completions, progress/info output respecting `TERM=dumb`/`NO_COLOR`
- **generator**: Produces realistic VLANs, interfaces, DHCP, NAT, firewall policies, CARP VIPs, RADIUS users with uniqueness and RFC-compliant ranges
- **model**: Strongly-typed serde models for config elements; central invariants and serde derive
- **validate**: Cross-object consistency checks (unique VLAN IDs, no IP conflicts, RFC ranges)
- **io**: CSV utilities (internal), XML I/O helpers; streaming-friendly operations for large sets
- **xml**: Constructs OPNsense config.xml via quick-xml/xmlwriter; integrates base-config and adheres to `opnsense-config.xsd`
- **Cross-cutting**: Robust errors (thiserror/anyhow), performance helpers (bumpalo, lru, rustc-hash, smallvec), optional parallelism via feature `rayon`

## Development Workflow and Quality Gates

**Before proposing any changes, always run**: `just ci-check`

### Quality Standards

- **Lint policy**: `cargo clippy -- -D warnings` (no warnings allowed)
- **Formatting**: `cargo fmt --check` must pass
- **Tests**: Run with `TERM=dumb` for deterministic output when relevant
- **Coverage**: Target ≥79% locally (currently enforced); CI does not gate on coverage drops
- **Security**: `just audit` when changing dependencies
- **Offline-first**: No network calls in core build/test paths

### Review Process

- Use coderabbit.ai for code review
- Single maintainer workflow (@UncleSp1d3r)
- Never auto-commit code

## Testing Strategy

### Test Types

```bash
# Unit tests
cargo test --lib --all-features

# Integration tests (CLI)
cargo test --tests --all-features
TERM=dumb cargo test --tests --all-features

# Property-based tests (proptest)
cargo test proptest --all-features
cargo test proptest --all-features --features slow-tests
PROPTEST_CASES=1000 cargo test proptest

# Snapshot tests (insta)
cargo test --test snapshot_tests
TERM=dumb cargo test --test snapshot_tests
```

### Snapshot Management

```bash
cargo insta review                           # Review changes
INSTA_UPDATE=auto cargo test --test snapshot_tests    # Force accept (use sparingly)
```

### Deterministic Output Tips

- Use `TERM=dumb`, `NO_COLOR=1`, `CARGO_TERM_COLOR=never`
- Keep slow-tests behind feature flags
- Seeded randomness for reproducibility in tests where applicable

## Coverage and CI

### Local Coverage

- `just coverage` enforces 79% threshold (docs mention 80% target - known discrepancy)
- `just coverage-html` → `target/llvm-cov/html/index.html`
- `just coverage-report` for terminal-only output

### CI Coverage

- `just coverage-ci` generates an lcov report without failing the job
- CI respects `TERM=dumb` and `NO_COLOR`; console/indicatif auto-adapt

### Local CI Testing

```bash
just act-ci                                  # Dry-run CI locally
just act-ci-dry-run                          # Show what would run
just act-ci-list                             # List available jobs
```

## Conventions and Practices

### Platform Scope

- **OPNsense-only**: CSV is internal, not a user-facing deliverable
- **Single-purpose**: Do not extend to other firewall platforms

### Commit Messages

- **Conventional Commits** required
- **Common scopes**: `(cli)`, `(generator)`, `(xml)`, `(validation)`, `(io)`
- **Breaking changes**: Mark with `!` or footer `BREAKING CHANGE:`

### EvilBit Labs Standards

- Trust the operator, offline-first, no telemetry, sane defaults
- Cross-platform (macOS, Windows, Linux)
- No hardcoded secrets; validate inputs; airgap-ready

### Assistant Rules

- Run `just ci-check` before any proposed commit
- Prefer coderabbit.ai for review; single maintainer (@UncleSp1d3r)
- Never auto-commit
- Use OpenAPI Generator for Rust clients when applicable (rare in this project)

## Performance and Benchmarks

- **Benchmarks**: criterion framework; HTML reports under `target/criterion/reports/index.html`
- **Profiling**: Optional pprof + flamegraph support
- **Parallelism**: Feature-gated with `--features rayon` for processing large sets
- **Optimization**: Streaming XML (xmlwriter) and allocation strategies (bumpalo) for large configurations

## Project Layout

### Source Structure

```text
src/cli          # Command-line interface
src/generator    # Data generation logic  
src/io           # I/O utilities (CSV internal, XML)
src/model        # Typed data models
src/validate     # Consistency validation
src/xml          # OPNsense XML construction
tests/           # Integration tests
benches/         # Performance benchmarks
opnsense-config.xsd    # XML schema reference
```

### Key Dependencies

- **CLI**: clap, clap_complete
- **Serialization**: serde, serde_json, csv, quick-xml, xmlwriter
- **Networking**: ipnet, ipnetwork
- **Data Generation**: rand, rand_chacha, fake, uuid
- **Errors**: anyhow, thiserror
- **Performance**: bumpalo, lru, rustc-hash, smallvec
- **Testing**: proptest, insta, criterion, pprof
- **UI**: indicatif, console

### Toolchain

- **Rust**: Stable channel
- **Components**: rustfmt, clippy, llvm-tools-preview
- **Targets**: Multi-platform support (x86_64-unknown-linux-gnu, x86_64-apple-darwin, aarch64-apple-darwin, x86_64-pc-windows-msvc)

## Usage Examples

### Basic XML Generation

```bash
cargo run --release -- xml --base-config legacy/opnsense/config-example.xml --count 25
```

### VLAN Generation

```bash
cargo run --release -- generate vlan --count 10 --output vlans.xml
cargo run --release -- generate vlan --count 50 --format csv --output network-data.csv
```

### Firewall Rules

```bash
cargo run --release -- generate --count 10 --format xml --base-config config.xml --include-firewall-rules --firewall-rules-per-vlan 3 --firewall-rule-complexity intermediate
```

**Note**: `--firewall-rules-per-vlan` overrides default rule count for the chosen complexity and reassigns priorities per VLAN starting at 1.
