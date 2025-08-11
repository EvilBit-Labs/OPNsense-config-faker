# 🔧 justfile — OPNsense Config Faker Developer Tasks
set dotenv-load := true
set ignore-comments := true

# Default recipe - shows available commands
default:
    just --list

# Show help
help:
    just --list

# -----------------------------
# 🔧 Setup & Installation
# -----------------------------

# Install cargo-llvm-cov for coverage
install-cov:
    cargo install cargo-llvm-cov

# Setup development environment
setup:
    @echo "🚀 Setting up Rust development environment..."
    rustup component add clippy rustfmt
    just install-cov
    @echo "✅ Setup complete!"

# -----------------------------
# 🧹 Linting, Formatting & Checking
# -----------------------------

# Format code with rustfmt
format:
    cargo fmt

# Check code formatting
format-check:
    cargo fmt --check

# Lint code with clippy (strict warnings as errors)
lint:
    cargo clippy -- -D warnings

# Run all linting and formatting checks
check: format-check lint
    @echo "✅ All checks passed!"

# Fix linting and formatting issues
fix: format
    cargo clippy --fix --allow-dirty

# -----------------------------
# 🦀 Standardized Rust Tasks
# -----------------------------

# Format all Rust code
rust-fmt:
    cargo fmt --all

# Check Rust code formatting
rust-fmt-check:
    cargo fmt --all -- --check

# Lint Rust code with clippy (strict mode)
rust-clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Run all Rust tests
rust-test:
    cargo test --all-features --workspace

# Run Rust test coverage with HTML report
rust-cov:
    cargo llvm-cov --all-features --workspace --open

# Run Rust benchmarks
rust-bench:
    cargo bench

# Quality assurance: format check, clippy, and tests
qa: rust-fmt-check rust-clippy rust-test
    @echo "✅ All QA checks passed!"

# Quality assurance with coverage
qa-cov: rust-fmt-check rust-clippy rust-test rust-cov
    @echo "✅ All QA checks with coverage completed!"

# -----------------------------
# 🧪 Testing & Coverage
# -----------------------------

# Run all tests
test:
    cargo test --all-features

# Run tests excluding benchmarks
test-no-bench:
    cargo test --all-features --lib --bins --tests

# Run integration tests only
test-integration:
    cargo test --test '*' --all-features

# Run unit tests only
test-unit:
    cargo test --lib --all-features

# Run doctests only
test-doc:
    cargo test --doc --all-features

# Run coverage with cargo-llvm-cov and enforce 90% threshold
coverage:
    @echo "🔍 Running coverage with >90% threshold..."
    cargo llvm-cov --all-features --workspace --lcov --fail-under-lines 90 --output-path lcov.info
    @echo "✅ Coverage passed 90% threshold!"

# Run coverage for CI - generates report even if some tests fail
coverage-ci:
    @echo "🔍 Running coverage for CI with >90% threshold..."
    cargo llvm-cov --all-features --workspace --lcov --fail-under-lines 90 --output-path lcov.info --ignore-run-fail
    @echo "✅ Coverage report generated!"

# Run coverage report in HTML format for local viewing
coverage-html:
    @echo "🔍 Generating HTML coverage report..."
    cargo llvm-cov --all-features --workspace --html --output-dir target/llvm-cov/html
    @echo "📊 HTML report available at target/llvm-cov/html/index.html"

# Run coverage report in HTML format ignoring test failures
coverage-html-ci:
    @echo "🔍 Generating HTML coverage report (ignoring test failures)..."
    cargo llvm-cov --all-features --workspace --html --output-dir target/llvm-cov/html --ignore-run-fail
    @echo "📊 HTML report available at target/llvm-cov/html/index.html"

# Run coverage report to terminal
coverage-report:
    cargo llvm-cov --all-features --workspace

# Clean coverage artifacts
coverage-clean:
    cargo llvm-cov clean --workspace

# -----------------------------
# 🔧 Building & Running
# -----------------------------

# Build the project in debug mode
build:
    cargo build --all-features

# Build the project in release mode
build-release:
    cargo build --release --all-features

# Build documentation
doc:
    cargo doc --all-features --no-deps

# Build and open documentation
doc-open:
    cargo doc --all-features --no-deps --open

# Run the CLI tool with sample arguments
run *args:
    cargo run --all-features -- {{args}}

# Run benchmarks (exclude from coverage)
bench:
    cargo bench --all-features

# -----------------------------
# 🧹 Clean & Maintenance
# -----------------------------

# Clean build artifacts
clean:
    cargo clean
    rm -f lcov.info

# Update dependencies
update:
    cargo update

# Check for security advisories
audit:
    cargo audit

# -----------------------------
# 🤖 CI Workflow
# -----------------------------

# CI-friendly check that runs all validation
ci-check: format-check lint test coverage-ci
    @echo "✅ All CI checks passed!"

# Fast CI check without coverage (for quick feedback)
ci-check-fast: format-check lint test-no-bench
    @echo "✅ Fast CI checks passed!"

# CI-friendly QA check (respects TERM=dumb, see TESTING.md)
ci-qa: rust-fmt-check rust-clippy rust-test
    @echo "✅ CI QA checks passed!"

# -----------------------------
# 🚀 Development Workflow
# -----------------------------

# Development workflow: format, lint, test, coverage
dev: format lint test coverage
    @echo "✅ Development checks complete!"

# Watch for changes and run tests
watch:
    cargo watch -x "test --all-features"

# Watch for changes and run checks
watch-check:
    cargo watch -x "check --all-features" -x "clippy -- -D warnings"

# -----------------------------
# 📊 Project Information
# -----------------------------

# Show project information
info:
    @echo "🔧 OPNsense Config Faker"
    @echo "======================="
    @echo "Rust version: $(rustc --version)"
    @echo "Cargo version: $(cargo --version)"
    @echo "Project features:"
    @cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].features | keys[]' 2>/dev/null || echo "  - python-compat"
    @echo "  - slow-tests"
