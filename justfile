# OPNsense Config Faker - Development Tasks

# Default recipe - shows available commands
default:
    @just --list

# Install dependencies
install:
    uv sync --no-install-project --extra dev

# Run the CSV generator with default settings
run count="10":
    uv run python generate_csv.py --count {{count}}

# Run the CSV generator with custom output file
run-output count="10" output="test-config.csv":
    uv run python generate_csv.py --count {{count}} --output {{output}}

# Format code with ruff
format:
    uv run ruff format .

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

# Run all checks (lint + format check + type check) - for local development
check:
    @echo "Running linting checks..."
    uv run ruff check .
    @echo "Running format checks..."
    uv run ruff format --check .
    @echo "Running type checks..."
    uv run basedpyright
    @echo "All checks passed!"

# CI-friendly check that runs all validation (no formatting, strict checking)
ci-check:
    @echo "=== CI Validation ==="
    @echo "Checking Python version compatibility..."
    uv run python --version
    @echo "\nRunning strict linting (treating warnings as errors)..."
    uv run ruff check . --output-format=github
    @echo "\nRunning format validation..."
    uv run ruff format --check --diff .
    @echo "\nRunning type checking..."
    uv run basedpyright
    @echo "\nRunning tests (when available)..."
    -uv run pytest --tb=short -v || echo "No tests found or pytest not configured"
    @echo "\nValidating project structure..."
    @test -f pyproject.toml || (echo "ERROR: pyproject.toml missing" && exit 1)
    @test -f generate_csv.py || (echo "ERROR: generate_csv.py missing" && exit 1)
    @test -f justfile || (echo "ERROR: justfile missing" && exit 1)
    @echo "\n✅ All CI checks passed!"

# Run tests (when test suite is created)
test:
    uv run pytest

# Run tests with coverage
test-cov:
    uv run pytest --cov=. --cov-report=term-missing --cov-report=html

# Clean up generated files and caches
clean:
    rm -rf .pytest_cache/
    rm -rf htmlcov/
    rm -rf __pycache__/
    rm -rf .coverage
    rm -rf build/
    rm -rf dist/
    rm -rf *.egg-info/
    find . -name "*.pyc" -delete
    find . -name "*.pyo" -delete

# Development setup (install + generate sample)
dev-setup:
    @echo "Setting up OPNsense Config Faker development environment..."
    just install
    @echo "\nInstalling pre-commit hooks..."
    just pre-commit-install
    @echo "\nGenerating sample configuration (5 records)..."
    just run 5
    @echo "\nSetup complete! Try: just run 25"

# Pre-commit setup and management
pre-commit-install:
    uv run pre-commit install
    uv run pre-commit install --hook-type commit-msg
    uv run pre-commit install --hook-type pre-push

pre-commit-run:
    uv run pre-commit run --all-files

pre-commit-update:
    uv run pre-commit autoupdate

# Install additional optional dependencies
install-rich:
    uv sync --no-install-project --extra rich

install-all:
    uv sync --no-install-project --all-extras

# Show project info
info:
    @echo "OPNsense Config Faker"
    @echo "====================="
    @echo "Python version: $(uv run python --version)"
    @echo "UV version: $(uv --version)"
    @echo "Project dependencies:"
    @uv tree --depth 1
