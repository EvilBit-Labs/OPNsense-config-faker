# OPNsense Config Faker - AI Coding Assistant Rules

This document serves as the comprehensive guide for AI coding assistants working on the OPNsense Config Faker project. It consolidates all essential information for maintaining consistency, quality, and adherence to established best practices.

## 1. Core Philosophy and Purpose

- **OPNsense Configuration Focus**: This project's primary purpose is generating valid OPNsense `config.xml` files with realistic faked data
- **Single Platform Target**: Specifically designed for OPNsense firewall configurations - not a general network configuration tool
- **CSV as Intermediate Format**: CSV generation is an internal implementation detail, not a primary feature
- **Operator-Centric Design**: Built for network operators and automation engineers who need realistic OPNsense test configurations
- **Ethical Development**: This is a complete rewrite with no remaining code from any previous implementation
- **Offline-First**: No external dependencies or telemetry in production artifacts

## 2. Project Structure and Architecture

### Source Structure

```text
/
├── src/
│   ├── cli/               # Command line interface (Clap)
│   ├── generator/         # Core data generation logic
│   ├── io/               # Input/output handling (CSV, XML)
│   ├── model/            # Data models and structures
│   ├── validate/         # Validation framework
│   ├── xml/              # XML processing and generation
│   └── lib.rs            # Library entry point
├── tests/                # Integration tests
├── benches/              # Performance benchmarks
├── justfile              # Task runner configuration
├── Cargo.toml            # Project metadata and dependencies
└── opnsense-config.xsd   # OPNsense XML schema definition
```

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

## 3. Technology Stack

| Layer             | Technology                                | Notes                                            |
| ----------------- | ----------------------------------------- | ------------------------------------------------ |
| **Language**      | Rust 2024 Edition                         | Modern Rust with idiomatic patterns              |
| **CLI**           | Clap v4 with derive macros                | For clean, user-friendly command-line interfaces |
| **Serialization** | serde with CSV/XML support                | For data interchange and file I/O                |
| **XML**           | quick-xml or roxmltree                    | For OPNsense XML configuration handling          |
| **Testing**       | built-in test framework, criterion.rs     | For unit tests and performance benchmarking      |
| **CI/CD**         | GitHub Actions                            | For automated testing, linting, and releases     |
| **Tooling**       | `cargo` for deps, `just` for task running | `cargo clippy` and `cargo fmt` for quality       |

### Key Dependencies

- **CLI**: clap, clap_complete
- **Serialization**: serde, serde_json, csv, quick-xml, xmlwriter
- **Networking**: ipnet, ipnetwork
- **Data Generation**: rand, rand_chacha, fake, uuid
- **Errors**: anyhow, thiserror
- **Performance**: bumpalo, lru, rustc-hash, smallvec
- **Testing**: proptest, insta, criterion, pprof
- **UI**: indicatif, console

## 4. Essential Commands and Workflows

### Build Commands

```bash
cargo build --all-features
cargo build --release --all-features
```

### Quality Gates (Zero-Tolerance Policy)

```bash
# Formatting check
cargo fmt --check
just format-check

# Linting (MANDATORY - no warnings allowed)
cargo clippy --all-targets --all-features --benches -- -D warnings
just lint

# Comprehensive CI validation
just ci-check                # Full CI validation
just ci-check-fast          # Quick CI check (no coverage)
```

### Testing Strategy

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

# All tests
just test
just test-unit                               # Unit tests only
just test-integration                        # Integration tests only
just test-no-bench                           # Exclude benchmarks
```

### Coverage Requirements

```bash
just install-cov                             # One-time setup
just coverage                                # Local with threshold enforcement (≥79%)
just coverage-html                           # HTML report → target/llvm-cov/html/index.html
just coverage-report                         # Terminal output
just coverage-clean                          # Clean artifacts
```

### Performance and Benchmarks

```bash
cargo bench --all-features
just bench
```

### Local CI Testing

```bash
just act-ci                                  # Dry-run CI locally
just act-ci-dry-run                          # Show what would run
just act-ci-list                             # List available jobs
```

## 5. Usage Examples

### Basic XML Generation

```bash
# Generate OPNsense XML configs (primary use case)
cargo run --release -- xml --base-config config.xml --count 25

# Generate VLAN data
cargo run --release -- generate vlan --count 10 --output vlans.xml
cargo run --release -- generate vlan --count 50 --format csv --output network-data.csv

# Generate with firewall rules
cargo run --release -- generate --count 10 --format xml --base-config config.xml --include-firewall-rules --firewall-rules-per-vlan 3 --firewall-rule-complexity intermediate
```

## 6. Coding Standards and Conventions

### Rust Standards

- **Formatting**: `cargo fmt` using standard Rust formatting
- **Linting**: `cargo clippy -- -D warnings` to enforce strict zero-warning policy
- **Naming**: Follow standard Rust conventions - `snake_case` for variables/functions, `PascalCase` for types
- **Error Handling**: Use `Result<T, E>` types and `?` operator. Create custom error types when needed
- **Documentation**: Comprehensive `///` doc comments for all public APIs
- **Testing**: Unit tests co-located with code, integration tests in `tests/` directory

### Data Generation Principles

- **Realistic Data**: Generate RFC-compliant network configurations
- **Unique Values**: Ensure no duplicate VLAN IDs or conflicting IP ranges
- **Configurable**: Allow users to specify count, output format, and other parameters
- **Consistent**: Maintain consistent data patterns across generated records

### Architecture Patterns

- **Command Pattern**: CLI commands encapsulated as discrete operations
- **Builder Pattern**: Used for complex configuration objects
- **Strategy Pattern**: Pluggable algorithms for different output formats
- **Error Chaining**: Comprehensive error context through the call stack
- **Resource Management**: RAII principles for file handles and system resources

### Performance Considerations

- **Memory Safety**: Leverage Rust's ownership system for safe memory management
- **Zero-Cost Abstractions**: Use Rust's zero-cost abstractions for performance
- **Benchmarking**: Use criterion.rs for performance regression detection
- **Streaming I/O**: Efficient processing of large configuration sets
- **Parallelism**: Feature-gated with `--features rayon` for processing large sets
- **Optimization**: Streaming XML (xmlwriter) and allocation strategies (bumpalo) for large configurations

## 7. Quality Assurance and Standards

### Quality Standards

- **Lint policy**: `cargo clippy -- -D warnings` (no warnings allowed)
- **Formatting**: `cargo fmt --check` must pass
- **Tests**: Run with `TERM=dumb` for deterministic output when relevant
- **Coverage**: Target ≥79% locally (currently enforced); CI does not gate on coverage drops
- **Security**: `just audit` when changing dependencies
- **Offline-first**: No network calls in core build/test paths

### EvilBit Labs Standards

- **Trust the Operator**: Full control, no black boxes
- **Polish Over Scale**: Quality over feature-bloat
- **Offline First**: Built for where the internet isn't
- **Sane Defaults**: Clean outputs, CLI help that's actually helpful
- **Ethical Constraints**: No dark patterns, spyware, or telemetry
- **No hardcoded secrets**: Use environment variables or secure vaults
- **Input validation**: Validate all inputs before processing
- **Cross-platform**: Support macOS, Windows, and Linux
- **Airgap-ready**: All functionality must work offline

## 8. CI/CD Lessons Learned

### GitHub Actions Workflow Gotchas

- **mise in CI**: The `just install-tools` recipe depends on `mise`. CI runners must install it via `jdx/mise-action@v2` before calling any just recipes that use mise
- **CodeQL setup conflict**: Repository-level CodeQL "default setup" and a `.github/workflows/codeql.yml` (advanced setup) cannot coexist. Use one or the other — if default setup is enabled in repo settings, delete the workflow file
- **`just ci-check` exit code 255**: The `cargo dist plan` step returns 255 when `cargo-dist` version doesn't match the project config. This is a pre-existing environment issue, not a test failure
- **mdBook preprocessor compatibility**: Unused or deprecated preprocessors in `book.toml` (e.g. `mdbook-alerts`, the `multilingual` field) cause hard build failures. Only declare preprocessors that are installed and actively used

### Cross-Platform Snapshot Testing

Snapshot tests must produce identical output on Linux, macOS, and Windows. Key normalization patterns in `tests/common/mod.rs`:

- **Binary name normalization**: Replace `opnsense-config-faker.exe` with `opnsense-config-faker` so Windows snapshots match Unix
- **Temp path normalization**: Windows temp paths like `C:\Users\...\AppData\Local\Temp\file.xml` must be normalized to `<TEMP_FILE>`. Use `[^:\s]*` (not `[^\\]*`) in regex patterns to match the full multi-segment Windows path including file extension
- **Temp directory normalization**: Temp directories are normalized to `<TEMP_DIR>`. The temp file patterns must be checked *before* temp directory patterns to avoid greedy matching
- **Environment variables**: Use `TERM=dumb`, `NO_COLOR=1`, `CARGO_TERM_COLOR=never` for deterministic output

### Dependency Management

- **Exact pinning for breaking deprecations**: `assert_cmd` is pinned to `=2.0.17` because `2.1.x` deprecates `Command::cargo_bin` with `#[deprecated]`, which becomes a hard error under `-D warnings`. Migration to the 2.1+ API is tracked separately
- **Cargo.lock conflicts during rebase**: When rebasing across Cargo.lock changes, accept theirs and regenerate: `git checkout --theirs Cargo.lock && cargo generate-lockfile && git add Cargo.lock && git rebase --continue`
- **Semver-compatible bumps can break clippy**: A dep resolving to a newer semver-compatible version may introduce new deprecation warnings. With `-D warnings`, this is a build failure. Pin with `=` when needed

### Snapshot Workflow

```bash
cargo insta review                                        # Interactive review (preferred)
INSTA_UPDATE=always cargo test --test snapshot_tests       # Force-accept all (use sparingly)
```

After accepting snapshots, delete any leftover `.snap.new` files before committing.

## 9. AI Assistant Behavior and Rules of Engagement

### Core Development Rules

- **Clippy Strict Enforcement**: Always use `cargo clippy -- -D warnings` to enforce zero-warning policy
- **OpenAPI Generator**: Use OpenAPI Generator for Rust client code generation where applicable
- **TERM=dumb Support**: Ensure terminal output respects `TERM="dumb"` environment variable for CI/automation
- **CodeRabbit.ai Integration**: Prefer coderabbit.ai for code review over GitHub Copilot auto-reviews
- **Single Maintainer Workflow**: Configure for single maintainer (UncleSp1d3r) with no second reviewer requirement
- **No Auto-commits**: Never commit code on behalf of maintainer without explicit permission
- **Pre-Commit CI Check**: **MANDATORY** - Always run `just ci-check` before committing any work to ensure all quality gates pass

### Assistant Guidelines

- **Clarity and Precision**: Be direct, professional, and context-aware in all interactions
- **Adherence to Standards**: Strictly follow the defined rules for code style and project structure
- **Tool Usage**: Use `cargo` for Rust development, `just` for task execution
- **Focus on Value**: Enhance the project's unique value proposition as a network configuration data generator
- **Ethical Boundaries**: This is a complete rewrite with no remaining code from the original project

### Review Process

- Use coderabbit.ai for code review
- Single maintainer workflow (@UncleSp1d3r)
- Never auto-commit code

## 10. Commit Messages and Version Control

### Conventional Commits

All commit messages must adhere to the [Conventional Commits](https://www.conventionalcommits.org) specification:

- **Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`
- **Scopes**: `(cli)`, `(generator)`, `(xml)`, `(validation)`, `(io)`, etc.
- **Breaking Changes**: Indicated with `!` in the header or `BREAKING CHANGE:` in the footer

### Branch Protection

- Strict linting, testing, and security gates
- Release automation via Release Please manages versioning and changelog

## 11. Testing Strategy

### Test Types

- **Unit Tests**: Test individual functions and data structures
- **Integration Tests**: Test CLI commands and file generation end-to-end
- **Property Tests**: Validate network configuration properties with proptest
- **Performance Tests**: Benchmark data generation speed and memory usage
- **Snapshot Tests**: Ensure output consistency with insta

### Deterministic Output Tips

- Use `TERM=dumb`, `NO_COLOR=1`, `CARGO_TERM_COLOR=never`
- Keep slow-tests behind feature flags
- Seeded randomness for reproducibility in tests where applicable

### Snapshot Management

```bash
cargo insta review                           # Review changes
INSTA_UPDATE=always cargo test --test snapshot_tests   # Force accept (use sparingly)
```

## 12. Platform Scope and Constraints

### Platform Scope

- **OPNsense-only**: CSV is internal, not a user-facing deliverable
- **Single-purpose**: Do not extend to other firewall platforms
- **Cross-platform**: Support macOS, Windows, and Linux

### Future Roadmap Considerations

- Comprehensive test suite with property-based testing using proptest
- Enhanced OPNsense configuration elements (additional modules, more realistic data)
- Configuration validation and consistency checking for OPNsense configs
- Enhanced XML template system for different OPNsense versions
- Improved realistic data generation for OPNsense-specific features
- Performance optimization through benchmarking and profiling

**Note**: Supporting other platforms or output formats beyond OPNsense XML is explicitly NOT a feature goal

## 13. Performance and Benchmarks

### Benchmarking Framework

- **Benchmarks**: criterion framework; HTML reports under `target/criterion/reports/index.html`
- **Profiling**: Optional pprof + flamegraph support
- **Parallelism**: Feature-gated with `--features rayon` for processing large sets
- **Optimization**: Streaming XML (xmlwriter) and allocation strategies (bumpalo) for large configurations

### Performance Characteristics

Generation performance scales linearly with count:

- Small datasets (\<100 VLANs): 10-50ms
- Medium datasets (100-1000 VLANs): 50-500ms
- Large datasets (>1000 VLANs): 500ms-2s

Memory usage is approximately 500 bytes per VLAN configuration.

## 14. Final Reminders

### Critical Requirements

1. **Network Validity First**: Every change must maintain realistic, valid network configurations
2. **Zero Warnings**: `cargo clippy -- -D warnings` must pass
3. **CLI Usability**: Commands must be intuitive with clear help and error messages
4. **Testing Coverage**: Comprehensive unit, integration, and property-based tests
5. **Quality Focus**: Build for network professionals who need reliable test data
6. **Documentation**: Clear docs for all public APIs and CLI usage

### Common Commands Summary

```bash
# Development workflow
just ci-check                                # MANDATORY before any commit
just format && just lint && just test        # Basic quality checks
just coverage                               # Coverage validation
just bench                                  # Performance testing

# Primary use cases
cargo run --release -- xml --base-config config.xml --count 25
cargo run --release -- generate vlan --count 10 --output vlans.xml
```

This document serves as the single source of truth for AI coding assistants working on the OPNsense Config Faker project.
