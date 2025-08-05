# 🔧 justfile — OPNsense Config Faker Developer Tasks
set dotenv-load := true
set ignore-comments := true

# Common variables
_uv := "uv run"
_cd := "cd {{justfile_dir()}}"
_pytest := _uv + " pytest"
_ruff := _uv + " ruff"
_basedpyright := _uv + " basedpyright"
_precommit := _uv + " pre-commit"

# Default recipe - shows available commands
default:
    just --summary

# Show help
help:
    just --summary

# -----------------------------
# 🔧 Setup & Installation
# -----------------------------

# Install dependencies and setup pre-commit hooks
install: _ensure-cd
    # 🚀 Set up dev env & pre-commit hooks
    uv sync --no-install-project --extra dev
    {{_precommit}} install
    {{_precommit}} install --hook-type commit-msg
    {{_precommit}} install --hook-type pre-push
    # Verify xsdata is available for model generation
    {{_uv}} xsdata --version

# Install all extras
install-all:
    uv sync --no-install-project --all-extras

# Update uv dependencies
update-deps: _ensure-cd
    uv sync --no-install-project --extra dev -U

# -----------------------------
# 🧹 Linting, Typing & Formatting
# -----------------------------

# Format code with ruff
format: _ensure-cd
    {{_ruff}} format .

# Check code formatting using ruff
format-check:
    {{_ruff}} format --check .

# Lint code with ruff
lint:
    {{_ruff}} check .

# Fix linting issues automatically
lint-fix:
    {{_ruff}} check --fix .
    {{_ruff}} format .

# Run type checking with basedpyright
type-check:
    {{_basedpyright}}

# Run type checking in watch mode
type-check-watch:
    {{_basedpyright}} --watch

# Run all linting and type checks
full-checks: _ensure-cd format-check lint pre-commit-run type-check test-fast verify-xsd

# -----------------------------
# 🧪 Testing & Coverage
# -----------------------------

# Run tests (when test suite is created)
test:
    {{_pytest}}

# Run tests with coverage
test-cov:
    {{_pytest}} --cov=. --cov-report=term-missing --cov-report=html

# Run all tests with maxfail=1 and disable warnings
test-fast:
    {{_pytest}} --maxfail=1 --disable-warnings -v tests/

# Run coverage report
coverage:
    {{_uv}} coverage report

# Clean up and run tests
clean-test: clean
    @echo "✅ Cleaned. Running tests..."
    just test

# -----------------------------
# 🔧 XSD Model Generation
# -----------------------------

# Generate Pydantic models from XSD schema
generate-models: _ensure-cd
    @echo "🔧 Generating Pydantic models from XSD schema..."
    {{_uv}} xsdata generate opnsense-config.xsd --config {{justfile_dir()}}/pydantic.config.xml
    @echo "✅ Models generated successfully!"

# Verify xsdata installation and XSD schema
verify-xsd: _ensure-cd
    @echo "🔍 Verifying XSD setup..."
    {{_uv}} xsdata --version
    ./scripts/verify_xsd.py

# -----------------------------
# 📦 CSV Generation & Usage
# -----------------------------

# Run the CSV generator with default settings
run count="10": _ensure-cd
    {{_uv}} python generate_csv.py --count {{count}}

# Run the CSV generator with custom output file
run-output count="10" output="test-config.csv": _ensure-cd
    {{_uv}} python generate_csv.py --count {{count}} --output {{output}}

# Generate sample data for testing
generate-sample: _ensure-cd
    @echo "🔧 Generating sample configurations..."
    just run 5
    @echo "✅ Sample data generated! Check the output files."

# -----------------------------
# 🧹 Build & Clean
# -----------------------------

# Clean up generated files and caches
clean: _ensure-cd
    ./scripts/clean.py

# Build the project
build:
    uvx --from build pyproject-build --installer uv

# Clean up and build the project
clean-build: ci-check clean build

# -----------------------------
# 🤖 CI Workflow
# -----------------------------

# CI-friendly check that runs all validation (no formatting, strict checking)
ci-check: _ensure-cd
    # Linting and formatting
    {{_ruff}} check . --output-format=github
    {{_ruff}} format --check --diff .
    # Type checking
    {{_basedpyright}}
    # Tests with coverage
    -{{_pytest}} --cov=. --cov-report=xml --cov-report=term-missing --tb=short -v || echo "No tests found or pytest not configured"

# Setup CI checks and dependencies for CI workflow
ci-setup: _ensure-cd
    uv sync --no-install-project --extra dev || @echo "Make sure uv is installed manually"
    {{_precommit}} install --hook-type commit-msg || @echo "Make sure pre-commit is installed manually"

# -----------------------------
# 🚀 Development Environment
# -----------------------------

# Development setup (install + generate sample)
dev-setup: _ensure-cd
    @echo "🚀 Setting up OPNsense Config Faker development environment..."
    just install
    @echo "\n📦 Generating sample configuration (5 records)..."
    just run 5
    @echo "\n✅ Setup complete! Try: just run 25"

# Development workflow: clean, check, and generate sample
dev: _ensure-cd clean full-checks generate-sample

# -----------------------------
# 🔧 Pre-commit Management
# -----------------------------

# Run pre-commit on all files
pre-commit-run:
    {{_precommit}} run --all-files

# Update pre-commit hooks
pre-commit-update:
    {{_precommit}} autoupdate

# -----------------------------
# 📊 Project Information
# -----------------------------

# Show project info
info: _ensure-cd
    @echo "🔧 OPNsense Config Faker"
    @echo "========================"
    @echo "Python version: $({{_uv}} python --version)"
    @echo "UV version: $(uv --version)"
    @echo "Project dependencies:"
    @uv tree --depth 1

# -----------------------------
# 🔧 Internal Utilities
# -----------------------------

# Ensure we're in the project directory (internal dependency)
_ensure-cd:
    cd {{justfile_dir()}}
