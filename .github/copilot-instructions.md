# OPNsense Config Faker - Developer Instructions

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Prerequisites and Setup
- Install Python 3.10+ (3.13 recommended)
- Install uv package manager: `pip install uv`
- Install just task runner: `cargo install just` (requires Rust/Cargo)
- Add just to PATH: `export PATH="$HOME/.cargo/bin:$PATH"`

### Initial Setup (Complete Developer Environment)
- `export PATH="$HOME/.cargo/bin:$PATH"`  -- ensure just is in PATH
- `uv sync --extra dev`  -- installs all dependencies, takes ~120 seconds (includes Python 3.13 download). NEVER CANCEL. Set timeout to 180+ seconds.
- `uv run pre-commit install --hook-type commit-msg`  -- install git hooks, takes ~0.3 seconds
- `just verify-xsd`  -- verify XSD schema setup, takes ~0.5 seconds

### Build and Test Commands
- `just format-check`  -- check code formatting with ruff, takes ~0.05 seconds
- `just lint`  -- lint code with ruff, takes ~0.05 seconds  
- `just type-check`  -- type checking with basedpyright, takes ~4 seconds
- `just test`  -- run full test suite (22 tests), takes ~6 seconds. NEVER CANCEL. Set timeout to 30+ seconds.
- `just ci-check`  -- complete CI validation (format, lint, type-check, tests), takes ~9 seconds. NEVER CANCEL. Set timeout to 60+ seconds.

### Generate Models from XSD Schema
- `just generate-models`  -- regenerate Pydantic models from opnsense-config.xsd, takes ~40 seconds. NEVER CANCEL. Set timeout to 90+ seconds.

### Application Usage
- `uv run python main.py --help`  -- show main application help, takes ~0.5 seconds
- `uv run python main.py csv --count 5`  -- generate CSV with 5 VLAN configs, takes ~0.3 seconds
- `uv run python main.py xml --base-config legacy/opnsense/config-example.xml --count 3 --force`  -- generate OPNsense XML, takes ~0.4 seconds
- `just run 5`  -- generate CSV using just task (alternative method), takes ~0.3 seconds

### Cleanup and Maintenance
- `just clean`  -- clean __pycache__, build artifacts, takes ~0.4 seconds
- `just format`  -- auto-format code with ruff, takes ~0.05 seconds
- `just lint-fix`  -- auto-fix linting issues, takes ~0.1 seconds

## Validation

### Manual Testing Scenarios
Always manually validate any new code changes with these complete end-to-end scenarios:

1. **CSV Generation Workflow**:
   ```bash
   uv run python main.py csv --count 10 --output test-output.csv
   ```
   Verify: CSV file created with 10 unique VLAN records, proper headers, realistic data

2. **XML Generation Workflow**:
   ```bash
   uv run python main.py xml --base-config legacy/opnsense/config-example.xml --count 5 --force
   ```
   Verify: Generated XML files in output/, including complete OPNsense configuration

3. **Build and CI Validation**:
   ```bash
   just ci-check
   ```
   Verify: All checks pass (formatting, linting, type-checking, tests)

### Required Base Configuration Files
- Use `legacy/opnsense/config-example.xml` as the base configuration for XML generation
- This file contains a valid OPNsense configuration template
- XML generation requires lxml dependency (included in dev dependencies)

### Testing Requirements
- Run `just test` before committing changes - 22 tests must pass
- Test coverage focuses on core functionality (CSV generation, interface models)
- Generated models in `opnsense/models/` are auto-generated and excluded from coverage
- Warning about Pydantic deprecation is expected and safe to ignore

## Common Tasks

### Repository Structure
```
├── main.py                    # Main CLI application (Typer-based)
├── generate_csv.py            # Standalone CSV generator
├── pyproject.toml             # Project configuration and dependencies
├── justfile                   # Task runner configuration
├── opnsense/                  # Core Python package
│   ├── generators/            # XML generators for OPNsense components
│   ├── factories/             # Model factories for test data
│   └── models/                # Auto-generated Pydantic models (DO NOT EDIT)
├── legacy/                    # Original Python implementation (reference only)
├── tests/                     # Test suite
├── scripts/                   # Utility scripts
└── output/                    # Generated files directory
```

### Key Files You'll Work With
- `main.py` - Main application entry point with CLI commands
- `pyproject.toml` - Dependencies, tool configuration, project metadata
- `justfile` - All development task definitions
- `opnsense/generators/` - Core XML generation logic  
- `opnsense/factories/` - Data factories for realistic test data
- `tests/` - Unit tests for validation

### Generated Models Warning
⚠️ **CRITICAL**: The `opnsense/models/` directory contains auto-generated Python code from the XSD schema. DO NOT manually edit these files. Use `just generate-models` to regenerate them when the schema changes.

### Common Failure Cases
- **lxml not available**: XML generation requires lxml. Install with `uv sync --extra dev`
- **just not found**: Ensure `$HOME/.cargo/bin` is in PATH: `export PATH="$HOME/.cargo/bin:$PATH"`
- **Permission errors**: Ensure write access to `output/` directory
- **File exists prompts**: Use `--force` flag for XML generation to skip confirmation prompts

### Development Workflow Commands
```bash
# Complete setup for new developers
just dev-setup

# Daily development workflow  
just format && just lint && just test

# Before committing
just ci-check

# Generate sample data for testing
just run 10

# Clean up generated files
just clean
```

### Important Notes
- This tool generates OPNsense firewall configurations with realistic test data
- Primary use case is testing network automation and configuration management
- Uses modern Python practices: Typer CLI, Pydantic models, Rich output
- Legacy Python code preserved in `legacy/` for reference
- Future Rust migration planned (see `rust_migration/` directory)
- Based on original work by Stefan Reichhard (nett-media/opnsense-config-generator)

### Performance Expectations
- CSV generation: Sub-second for typical counts (5-50 records)
- XML generation: Sub-second for small configurations (3-10 VLANs)  
- Model generation: ~40 seconds (only needed when XSD schema changes)
- Full CI validation: ~9 seconds
- Test suite: ~6 seconds

Always run CI validation before submitting changes: `just ci-check`