# GitHub Copilot Instructions for OPNsense Config Faker

## Project Purpose

This Rust project generates realistic OPNsense firewall `config.xml` files with faked but valid data for testing purposes. **Single-purpose tool** - OPNsense configuration generation only.

## Key Principles

- **Framework-First**: Prefer Clap and Serde built-ins over custom solutions
- **Operator-Centric**: Efficient, auditable, offline-capable workflows
- **Ethical Development**: This is a complete rewrite with no remaining code from any previous implementation
- **Quality First**: Zero warnings policy with `cargo clippy -- -D warnings`

## Critical Code Management Rules

## Coding Standards and Best Practices

### Rust Development Standards

- **Formatting**: `cargo fmt` using standard Rust formatting
- **Linting**: `cargo clippy -- -D warnings` to enforce strict zero-warning policy
- **Naming**: Follow standard Rust conventions - `snake_case` for variables/functions, `PascalCase` for types
- **Error Handling**: Use `Result<T, E>` types and `?` operator. Create custom error types when needed
- **Documentation**: Comprehensive `///` doc comments for all public APIs
- **Testing**: Unit tests co-located with code, integration tests in `tests/` directory

### Architecture Patterns

- **Command Pattern**: CLI commands encapsulated as discrete operations
- **Builder Pattern**: Used for complex configuration objects
- **Strategy Pattern**: Pluggable algorithms for different output formats
- **Error Chaining**: Comprehensive error context through the call stack
- **Resource Management**: RAII principles for file handles and system resources

### Data Generation Principles

- **Realistic Data**: Generate RFC-compliant network configurations
- **Unique Values**: Ensure no duplicate VLAN IDs or conflicting IP ranges
- **Configurable**: Allow users to specify count, output format, and other parameters
- **Consistent**: Maintain consistent data patterns across generated records

## Development Guidelines

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

## EvilBit Labs Standards Integration

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
  - Breaking Changes: Indicated with `!` in the header
- **Branch Protection**: Strict linting, testing, and security gates
- **Release Automation**: Release Please manages versioning and changelog

## AI Assistant Guidelines

### Development Rules of Engagement

- **Clippy Strict Enforcement**: Always use `cargo clippy -- -D warnings` to enforce zero-warning policy
- **OpenAPI Generator**: Use OpenAPI Generator for Rust client code generation where applicable
- **TERM=dumb Support**: Ensure terminal output respects `TERM="dumb"` environment variable for CI/automation
- **CodeRabbit.ai Integration**: Prefer coderabbit.ai for code review over GitHub Copilot auto-reviews
- **Single Maintainer Workflow**: Configure for single maintainer (UncleSp1d3r) with no second reviewer requirement
- **No Auto-commits**: Never commit code on behalf of maintainer without explicit permission
- **Pre-Commit CI Check**: **MANDATORY** - Always run `just ci-check` before committing any work to ensure all quality gates pass

### Assistant Behavior Rules

- **Clarity and Precision**: Be direct, professional, and context-aware in all interactions
- **Adherence to Standards**: Strictly follow the defined rules for code style and project structure
- **Tool Usage**: Use `cargo` for Rust development, `just` for task execution
- **Respect Legacy**: Don't modify legacy Python code unless specifically requested
- **Focus on Value**: Enhance the project's unique value proposition as a network configuration data generator
- **Ethical Boundaries**: Always respect the original project authors and encourage upstream usage where appropriate

### Code Generation Requirements

- Generated code must conform to all established patterns
- Include comprehensive type safety through Rust's type system
- Use proper error handling with context preservation
- Follow architectural patterns (Command, Builder, Strategy)
- Include appropriate documentation and testing

## Common Commands and Workflows

### Development Commands

- `cargo build` - Build the project
- `cargo test` - Run all tests
- `cargo bench` - Run benchmarks
- `just qa` - Run quality assurance checks
- `just clippy` - Run strict clippy lints

### Quality Assurance

- `cargo clippy -- -D warnings` - Strict linting (zero warnings)
- `cargo fmt --check` - Check code formatting
- `cargo audit` - Security vulnerability scan
- `cargo doc` - Generate documentation

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

## Performance Considerations

- **Memory Safety**: Leverage Rust's ownership system for safe memory management
- **Zero-Cost Abstractions**: Use Rust's zero-cost abstractions for performance
- **Benchmarking**: Use criterion.rs for performance regression detection
- **Streaming I/O**: Efficient processing of large configuration sets
- **Cross-platform compatibility**: Windows, macOS, Linux

## Security Considerations

- **No telemetry**: No external network calls or data collection
- **Offline operation**: Full functionality without internet connectivity
- **Input validation**: Comprehensive validation of all inputs
- **Memory safety**: Rust's memory safety guarantees prevent common vulnerabilities
- **Supply chain security**: Regular dependency auditing and SBOM generation

## Future Roadmap

- Comprehensive test suite with property-based testing using proptest
- Enhanced OPNsense configuration elements (additional modules, more realistic data)
- Configuration validation and consistency checking for OPNsense configs
- Enhanced XML template system for different OPNsense versions
- Improved realistic data generation for OPNsense-specific features
- Performance optimization through benchmarking and profiling

**Note**: Supporting other platforms or output formats beyond OPNsense XML is explicitly **NOT** a feature goal.

## Key Reminders

1. **Single Platform Focus**: This tool is exclusively for OPNsense firewall configurations
2. **Rust Implementation**: Core functionality is being migrated from Python to Rust
3. **Legacy Respect**: Preserve attribution to original work and don't modify generated legacy code
4. **Quality First**: Zero warnings policy, comprehensive testing, performance benchmarking
5. **Operator-Centric**: Build for network operators who need reliable, offline-capable tools
6. **Ethical Development**: Respect upstream project and maintain proper boundaries
