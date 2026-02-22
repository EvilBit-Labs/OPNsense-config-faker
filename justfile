# Cross-platform justfile for opnsense-config-faker (Go)

set shell := ["bash", "-cu"]
set windows-shell := ["powershell", "-NoProfile", "-Command"]
set dotenv-load := true
set ignore-comments := true

mise_exec := "mise exec --"
root := justfile_dir()

# =============================================================================
# GENERAL COMMANDS
# =============================================================================

default:
    @just --choose

help:
    @just --list

# =============================================================================
# SETUP
# =============================================================================

# Install all tools and download Go modules
install:
    mise trust
    mise install
    @{{ mise_exec }} go mod download

# Full development setup
setup: install
    @{{ mise_exec }} pre-commit install

# Update all Go dependencies
update-deps:
    @{{ mise_exec }} go get -u ./...
    @{{ mise_exec }} go mod tidy

# =============================================================================
# DEVELOPMENT
# =============================================================================

# Run the CLI with optional arguments
run *args:
    @{{ mise_exec }} go run . {{ args }}

# Run the CLI in development mode
dev *args:
    @{{ mise_exec }} go run . {{ args }}

# =============================================================================
# QUALITY
# =============================================================================

# Format all Go source files
format:
    @{{ mise_exec }} gofmt -w .

# Check formatting (fails if any files need formatting)
format-check:
    @test -z "$({{ mise_exec }} gofmt -l .)" || (echo "Files need formatting:" && {{ mise_exec }} gofmt -l . && exit 1)

# Lint with golangci-lint
lint:
    @{{ mise_exec }} golangci-lint run ./...

# Run go vet
check:
    @{{ mise_exec }} go vet ./...

# Run go mod tidy and verify
modernize:
    @{{ mise_exec }} go mod tidy
    @{{ mise_exec }} go mod verify

# =============================================================================
# TESTING
# =============================================================================

# Run all tests
test:
    @{{ mise_exec }} go test ./...

# Run all tests with verbose output
test-v:
    @{{ mise_exec }} go test -v ./...

# Run tests with coverage
test-coverage:
    @{{ mise_exec }} go test -coverprofile=coverage.out ./...
    @{{ mise_exec }} go tool cover -func=coverage.out

# Run integration tests
test-integration:
    @{{ mise_exec }} go test -tags=integration ./...

# Run tests with race detector
test-race:
    @{{ mise_exec }} go test -race ./...

# Run benchmarks
bench:
    @{{ mise_exec }} go test -bench=. -benchmem ./...

# =============================================================================
# BUILD
# =============================================================================

# Build all packages
build:
    @{{ mise_exec }} go build ./...

# Build release binary with stripped symbols
build-release:
    @{{ mise_exec }} go build -ldflags="-s -w" -o dist/ ./...

# Clean build artifacts
clean:
    @just rmrf dist
    @just rmrf coverage.out

# Clean and rebuild
rebuild: clean build

# =============================================================================
# RELEASE
# =============================================================================

# Check GoReleaser configuration
release-check:
    @{{ mise_exec }} goreleaser check

# Build snapshot release (no publishing)
release-snapshot:
    @{{ mise_exec }} goreleaser release --snapshot --clean

# Build locally with GoReleaser
release-local:
    @{{ mise_exec }} goreleaser build --clean

# Full release
release:
    @{{ mise_exec }} goreleaser release --clean

# =============================================================================
# DOCUMENTATION
# =============================================================================

# Serve docs locally
docs:
    @{{ mise_exec }} mkdocs serve

# Build documentation
docs-build:
    @{{ mise_exec }} mkdocs build

# Test documentation links
docs-test:
    @{{ mise_exec }} markdownlint-cli2 docs/**/*.md README.md

# Generate Go documentation
generate-docs:
    @{{ mise_exec }} go doc -all ./...

# =============================================================================
# SECURITY
# =============================================================================

# Run security scanner
scan:
    @{{ mise_exec }} gosec ./...

# Generate SBOM
sbom:
    @echo "SBOM generation requires cyclonedx-gomod (install via: go install github.com/CycloneDX/cyclonedx-gomod/cmd/cyclonedx-gomod@latest)"

# Run all security checks
security-all: scan

# =============================================================================
# CI
# =============================================================================

# Mandatory pre-commit quality gate
ci-check: check format-check lint test test-integration

# Quick smoke test
ci-smoke: build test

# Full CI pipeline
ci-full: ci-check test-race bench

# =============================================================================
# CROSS-PLATFORM HELPERS
# =============================================================================

[windows]
rmrf path:
    if (Test-Path "{{ path }}") { Remove-Item "{{ path }}" -Recurse -Force }

[unix]
rmrf path:
    /bin/rm -rf "{{ path }}"

[windows]
ensure-dir dir:
    New-Item -ItemType Directory -Force -Path "{{ dir }}" | Out-Null

[unix]
ensure-dir dir:
    /bin/mkdir -p "{{ dir }}"
