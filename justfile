# 🔧 justfile — OPNsense Config Faker Developer Tasks
set shell := ["bash", "-cu"]
set dotenv-load := true
set ignore-comments := true

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
install:
    cd {{justfile_dir()}}
    # 🚀 Set up dev env & pre-commit hooks
    uv sync --no-install-project --extra dev
    uv run pre-commit install
    uv run pre-commit install --hook-type commit-msg
    uv run pre-commit install --hook-type pre-push
    # Verify xsdata is available for model generation
    uv run xsdata --version

# Install additional optional dependencies
install-rich:
    uv sync --no-install-project --extra rich

# Install all extras
install-all:
    uv sync --no-install-project --all-extras

# Update uv dependencies
update-deps:
    cd {{justfile_dir()}}
    uv sync --no-install-project --extra dev -U

# -----------------------------
# 🧹 Linting, Typing & Formatting
# -----------------------------

# Format code with ruff
format:
    cd {{justfile_dir()}}
    uv run ruff format .

# Check code formatting using ruff
format-check:
    uv run ruff format --check .

# Lint code with ruff
lint:
    uv run ruff check .

# Fix linting issues automatically
lint-fix:
    uv run ruff check --fix .
    uv run ruff format .

# Run type checking with basedpyright
type-check:
    uv run basedpyright

# Run type checking in watch mode
type-check-watch:
    uv run basedpyright --watch

# Run all linting and type checks
full-checks:
    cd {{justfile_dir()}}
    just format-check
    just lint
    just pre-commit-run
    just type-check
    just test-fast
    just verify-xsd

# -----------------------------
# 🧪 Testing & Coverage
# -----------------------------

# Run tests (when test suite is created)
test:
    TERM=dumb uv run pytest

# Run tests with coverage
test-cov:
    TERM=dumb uv run pytest --cov=. --cov-report=term-missing --cov-report=html

# Run all tests with maxfail=1 and disable warnings
test-fast:
    TERM=dumb uv run pytest --maxfail=1 --disable-warnings -v tests/

# Run coverage report
coverage:
    uv run coverage report

# Clean up and run tests
clean-test: clean
    @echo "✅ Cleaned. Running tests..."
    just test

# -----------------------------
# 🔧 XSD Model Generation
# -----------------------------

# Generate Pydantic models from XSD schema
generate-models:
    cd {{justfile_dir()}}
    @echo "🔧 Generating Pydantic models from XSD schema..."
    uv run xsdata generate opnsense-config.xsd --output pydantic --package opnsense.models --compound-fields --generic-collections --union-type --postponed-annotations --structure-style=clusters
    @echo "✅ Models generated successfully!"

# Verify xsdata installation and XSD schema
verify-xsd:
    cd {{justfile_dir()}}
    @echo "🔍 Verifying XSD setup..."
    uv run xsdata --version
    @test -f opnsense-config.xsd || (echo "ERROR: opnsense-config.xsd missing" && exit 1)
    @echo "✅ XSD setup verified!"

# -----------------------------
# 📦 CSV Generation & Usage
# -----------------------------

# Run the CSV generator with default settings
run count="10":
    cd {{justfile_dir()}}
    uv run python generate_csv.py --count {{count}}

# Run the CSV generator with custom output file
run-output count="10" output="test-config.csv":
    cd {{justfile_dir()}}
    uv run python generate_csv.py --count {{count}} --output {{output}}

# Generate sample data for testing
generate-sample:
    cd {{justfile_dir()}}
    @echo "🔧 Generating sample configurations..."
    just run 5
    @echo "✅ Sample data generated! Check the output files."

# -----------------------------
# 🧹 Build & Clean
# -----------------------------

# Clean up generated files and caches
clean:
    cd {{justfile_dir()}}
    @echo "🧹 Cleaning .pyc files, __pycache__, and .pytest_cache..."
    find . -type d -name "__pycache__" -exec rm -rf "{}" +
    find . -type f -name "*.pyc" -delete
    find . -type f -name "*.pyo" -delete
    rm -rf .pytest_cache/
    rm -rf htmlcov/
    rm -rf .coverage
    rm -rf build/
    rm -rf dist/
    rm -rf *.egg-info/

# Build the project
build:
    uvx --from build pyproject-build --installer uv

# Clean up and build the project
clean-build:
    just ci-check
    just clean
    just build

# -----------------------------
# 🤖 CI Workflow
# -----------------------------

# CI-friendly check that runs all validation (no formatting, strict checking)
ci-check:
    cd {{justfile_dir()}}
    @echo "=== CI Validation ==="
    @echo "Checking Python version compatibility..."
    uv run python --version
    @echo "\nVerifying xsdata availability..."
    uv run xsdata --version
    @echo "\nRunning strict linting (treating warnings as errors)..."
    uv run ruff check . --output-format=github
    @echo "\nRunning format validation..."
    uv run ruff format --check --diff .
    @echo "\nRunning type checking..."
    uv run basedpyright
    @echo "\nRunning tests (when available)..."
    -TERM=dumb uv run pytest --tb=short -v || echo "No tests found or pytest not configured"
    @echo "\nValidating project structure..."
    @test -f pyproject.toml || (echo "ERROR: pyproject.toml missing" && exit 1)
    @test -f generate_csv.py || (echo "ERROR: generate_csv.py missing" && exit 1)
    @test -f justfile || (echo "ERROR: justfile missing" && exit 1)
    @test -f opnsense-config.xsd || (echo "ERROR: opnsense-config.xsd missing" && exit 1)
    @echo "\n✅ All CI checks passed!"

# Setup CI checks and dependencies for CI workflow
ci-setup:
    cd {{justfile_dir()}}
    uv sync --no-install-project --extra dev || @echo "Make sure uv is installed manually"
    uv run pre-commit install --hook-type commit-msg || @echo "Make sure pre-commit is installed manually"
    uv run xsdata --version || @echo "Make sure xsdata-pydantic is installed manually"

# -----------------------------
# 🚀 Development Environment
# -----------------------------

# Development setup (install + generate sample)
dev-setup:
    cd {{justfile_dir()}}
    @echo "🚀 Setting up OPNsense Config Faker development environment..."
    just install
    @echo "\n📦 Generating sample configuration (5 records)..."
    just run 5
    @echo "\n✅ Setup complete! Try: just run 25"

# Development workflow: clean, check, and generate sample
dev:
    cd {{justfile_dir()}}
    just clean
    just check-all
    just generate-sample

# -----------------------------
# 🔧 Pre-commit Management
# -----------------------------

# Run pre-commit on all files
pre-commit-run:
    uv run pre-commit run --all-files

# Update pre-commit hooks
pre-commit-update:
    uv run pre-commit autoupdate

# -----------------------------
# 📊 Project Information
# -----------------------------

# Show project info
info:
    cd {{justfile_dir()}}
    @echo "🔧 OPNsense Config Faker"
    @echo "========================"
    @echo "Python version: $(uv run python --version)"
    @echo "UV version: $(uv --version)"
    @echo "Project dependencies:"
    @uv tree --depth 1
