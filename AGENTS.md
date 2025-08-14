# OPNsense Config Faker - AI Coding Assistant Rules

This document outlines the core concepts, framework, and coding standards for the OPNsense Config Faker project. It serves as a comprehensive guide for AI coding assistants to ensure consistency, maintainability, and adherence to established best practices.

## 1. Core Philosophy

- **OPNsense Configuration Focus**: This project's primary purpose is generating valid OPNsense `config.xml` files with realistic faked data
- **Single Platform Target**: Specifically designed for OPNsense firewall configurations - not a general network configuration tool
- **CSV as Intermediate Format**: CSV generation is an internal implementation detail, not a primary feature
- **Operator-Centric Design**: Built for network operators and automation engineers who need realistic OPNsense test configurations
- **Ethical Development**: This is a complete rewrite with no remaining code from any previous implementation

## 2. Project Structure

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

## 3. Technology Stack

| Layer             | Technology                                | Notes                                            |
| ----------------- | ----------------------------------------- | ------------------------------------------------ |
| **Language**      | Rust 2021 Edition                         | Modern Rust with idiomatic patterns              |
| **CLI**           | Clap v4 with derive macros                | For clean, user-friendly command-line interfaces |
| **Serialization** | serde with CSV/XML support                | For data interchange and file I/O                |
| **XML**           | quick-xml or roxmltree                    | For OPNsense XML configuration handling          |
| **Testing**       | built-in test framework, criterion.rs     | For unit tests and performance benchmarking      |
| **CI/CD**         | GitHub Actions                            | For automated testing, linting, and releases     |
| **Tooling**       | `cargo` for deps, `just` for task running | `cargo clippy` and `cargo fmt` for quality       |

## 4. Coding Standards and Conventions

### Rust

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

### Commit Messages

- **Conventional Commits**: All commit messages must adhere to the [Conventional Commits](https://www.conventionalcommits.org) specification
  - **Types**: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`
  - **Scopes**: `(cli)`, `(generator)`, `(xml)`, `(validation)`, `(io)`, etc.
  - **Breaking Changes**: Indicated with `!` in the header or `BREAKING CHANGE:` in the footer

## 5. Development Guidelines

### Core Functionality

- **Primary Purpose**: Generate valid OPNsense `config.xml` files with realistic faked data
- **Main Binary**: `opnsense-config-faker` - Tool for OPNsense configuration generation
- **XML Generation**: Primary feature - creates complete OPNsense configurations with realistic test data
- **CSV as Intermediate**: CSV generation is an internal implementation detail for data processing
- **Single Platform**: OPNsense only - not designed for other platforms or output formats
- **CLI Interface**: Simple interface focused on OPNsense configuration generation
- **Internal Data Format**: CSV with columns: VLAN, IP Range, Beschreibung, WAN (used internally)

### Quality Assurance

1. **Code Quality**: All code must pass `cargo clippy -- -D warnings` with zero warnings
2. **Type Safety**: Comprehensive type safety through Rust's type system
3. **Testing**: Generate valid configurations that can be used in real testing scenarios
4. **Documentation**: Clear documentation for all user-facing functionality
5. **Performance**: Benchmark-driven development with regression detection

## 6. EvilBit Labs Standards Integration

### Brand Principles

- **Trust the Operator**: Full control, no black boxes
- **Polish Over Scale**: Quality over feature-bloat
- **Offline First**: Built for where the internet isn't
- **Sane Defaults**: Clean outputs, CLI help that's actually helpful
- **Ethical Constraints**: No dark patterns, spyware, or telemetry

### Security and Quality Requirements

- **No hardcoded secrets**: Use environment variables or secure vaults
- **Input validation**: Validate all inputs before processing
- **Offline-first**: No external dependencies or telemetry in production artifacts
- **Cross-platform**: Support macOS, Windows, and Linux
- **Airgap-ready**: All functionality must work offline

### CI/CD Integration

- **Conventional Commits**: All commit messages must follow specification
  - Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`
  - Scopes: `(cli)`, `(generator)`, `(xml)`, `(validation)`, `(io)`, etc.
  - Breaking Changes: Indicated with `!` in the header or `BREAKING CHANGE:` in the footer
- **Branch Protection**: Strict linting, testing, and security gates
- **Release Automation**: Release Please manages versioning and changelog

## 7. AI Assistant Behavior and Rules of Engagement

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

## 8. Rust Implementation Features

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

### Usage Examples

```bash
# Primary use case - Generate OPNsense XML configuration
cargo run -- xml --base-config config.xml --count 25

# Generate CSV for intermediate processing (rarely needed directly)
cargo run -- csv --count 25

# Use existing CSV file for XML generation (advanced usage)
cargo run -- xml --base-config config.xml --csv-file my-data.csv

# Run benchmarks
cargo bench

# Run quality checks
just qa
```

## 9. Future Roadmap Considerations

- Comprehensive test suite with property-based testing using proptest
- Enhanced OPNsense configuration elements (additional modules, more realistic data)
- Configuration validation and consistency checking for OPNsense configs
- Enhanced XML template system for different OPNsense versions
- Improved realistic data generation for OPNsense-specific features
- Performance optimization through benchmarking and profiling

**Note**: Supporting other platforms or output formats beyond OPNsense XML is explicitly NOT a feature goal
