#!/bin/bash

# OPNsense Config Faker - Python Elimination Verification Script
# This script verifies that all Python code has been successfully removed from the project

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

echo "🔍 OPNsense Config Faker - Python Elimination Verification"
echo "=========================================================="

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    log_error "Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

log_info "Verifying Python elimination status..."

# Phase 1: Core Python files (should be removed)
log_info "Checking Phase 1 files (should be removed):"

PHASE1_FILES=(
    "main.py"
    "opnsense/factories/"
    "opnsense/generators/"
    "scripts/verify_xsd.py"
    "scripts/validate_xml.py"
    "scripts/generate_models.py"
    "scripts/python_compat.py"
    "tests/python_compat_test.py"
    "tests/python_reference.py"
    "tests/python_validation.py"
    "tests/python_benchmark.py"
    "tests/python_integration.py"
    "tests/python_unit.py"
    "tests/python_property.py"
    "tests/python_snapshot.py"
    "tests/python_scale.py"
    "tests/python_performance.py"
    "tests/python_compatibility.py"
    "tests/python_migration.py"
    "tests/python_validation_legacy.py"
    "tests/python_reference_legacy.py"
    "tests/python_compat_legacy.py"
    "tests/python_benchmark_legacy.py"
    "tests/python_integration_legacy.py"
    "tests/python_unit_legacy.py"
    "tests/python_property_legacy.py"
    "tests/python_snapshot_legacy.py"
    "tests/python_scale_legacy.py"
    "tests/python_performance_legacy.py"
    "tests/python_compatibility_legacy.py"
    "tests/python_migration_legacy.py"
    "tests/python_validation_legacy_legacy.py"
    "tests/python_reference_legacy_legacy.py"
    "tests/python_compat_legacy_legacy.py"
    "tests/python_benchmark_legacy_legacy.py"
    "tests/python_integration_legacy_legacy.py"
    "tests/python_unit_legacy_legacy.py"
    "tests/python_property_legacy_legacy.py"
    "tests/python_snapshot_legacy_legacy.py"
    "tests/python_scale_legacy_legacy.py"
    "tests/python_performance_legacy_legacy.py"
    "tests/python_compatibility_legacy_legacy.py"
    "tests/python_migration_legacy_legacy.py"
)

PHASE1_REMOVED=0
PHASE1_TOTAL=${#PHASE1_FILES[@]}

for file in "${PHASE1_FILES[@]}"; do
    if [[ -e "$file" ]]; then
        log_error "$file still exists (should be removed)"
    else
        log_success "$file removed"
        ((PHASE1_REMOVED++))
    fi
done

# Phase 2: Python tooling files (should be removed)
log_info "Checking Phase 2 files (Python tooling - should be removed):"

PHASE2_FILES=(
    "pyproject.toml"
    "requirements.txt"
    "requirements-dev.txt"
    "setup.py"
    "setup.cfg"
    "MANIFEST.in"
    "tox.ini"
    "pytest.ini"
    ".pytest_cache/"
    ".ruff_cache/"
    "uv.lock"
    ".venv/"
    "__pycache__/"
    "*.pyc"
    "*.pyo"
    "*.pyd"
    ".Python"
    "env/"
    "venv/"
    "ENV/"
    "env.bak/"
    "venv.bak/"
    "package.json"
    "pnpm-lock.yaml"
    "node_modules/"
)

PHASE2_REMOVED=0
PHASE2_TOTAL=${#PHASE2_FILES[@]}

for file in "${PHASE2_FILES[@]}"; do
    if [[ -e "$file" ]]; then
        log_error "$file still exists (should be removed)"
    else
        log_success "$file removed"
        ((PHASE2_REMOVED++))
    fi
done

# Phase 3: Legacy Python implementation (should be removed)
log_info "Checking Phase 3 files (Legacy Python implementation - should be removed):"

PHASE3_FILES=(
    "legacy/"
    "opnsense/"
)

PHASE3_REMOVED=0
PHASE3_TOTAL=${#PHASE3_FILES[@]}

for file in "${PHASE3_FILES[@]}"; do
    if [[ -e "$file" ]]; then
        log_error "$file still exists (should be removed)"
    else
        log_success "$file removed"
        ((PHASE3_REMOVED++))
    fi
done

# Check for any remaining Python files
log_info "Checking for any remaining Python files:"

REMAINING_PY_FILES=$(find . -name "*.py" -not -path "./.git/*" -not -path "./target/*" 2>/dev/null || true)

if [[ -n "$REMAINING_PY_FILES" ]]; then
    log_error "Found remaining Python files:"
    echo "$REMAINING_PY_FILES"
else
    log_success "No Python files found"
fi

# Check for Python-related configuration files
log_info "Checking for Python configuration files:"

PY_CONFIG_FILES=$(find . -name "*.toml" -o -name "*.cfg" -o -name "*.ini" -o -name "*.yml" -o -name "*.yaml" | grep -E "(pyproject|pytest|tox|ruff|black|flake8|mypy)" 2>/dev/null || true)

if [[ -n "$PY_CONFIG_FILES" ]]; then
    log_error "Found Python configuration files:"
    echo "$PY_CONFIG_FILES"
else
    log_success "No Python configuration files found"
fi

# Verify Rust tooling is in place
log_info "Verifying Rust tooling:"

RUST_FILES=(
    "Cargo.toml"
    "rust-toolchain.toml"
    "deny.toml"
    ".github/workflows/ci.yml"
)

for file in "${RUST_FILES[@]}"; do
    if [[ -f "$file" ]]; then
        log_success "$file exists"
    else
        log_error "$file missing"
    fi
done

# Check for cargo-deny configuration
if [[ -f "deny.toml" ]]; then
    log_success "cargo-deny configuration found"
else
    log_error "cargo-deny configuration missing"
fi

# Check for XSD schema file (should be preserved)
log_info "Checking for XSD schema file:"

if [[ -f "opnsense-config.xsd" ]]; then
    log_success "opnsense-config.xsd preserved (XML Schema Definition, not Python code)"
else
    log_error "opnsense-config.xsd missing (needed for OPNsense schema reference)"
fi

# Summary
echo ""
echo "📊 Python Elimination Summary"
echo "============================="
echo "Phase 1 (Core Python): $PHASE1_REMOVED/$PHASE1_TOTAL removed"
echo "Phase 2 (Python Tooling): $PHASE2_REMOVED/$PHASE2_TOTAL removed"
echo "Phase 3 (Legacy Implementation): $PHASE3_REMOVED/$PHASE3_TOTAL removed"

TOTAL_REMOVED=$((PHASE1_REMOVED + PHASE2_REMOVED + PHASE3_REMOVED))
TOTAL_FILES=$((PHASE1_TOTAL + PHASE2_TOTAL + PHASE3_TOTAL))

if [[ $TOTAL_REMOVED -eq $TOTAL_FILES ]] && [[ -z "$REMAINING_PY_FILES" ]] && [[ -z "$PY_CONFIG_FILES" ]]; then
    echo ""
    log_success "🎉 Python elimination complete! All Python code has been successfully removed."
    log_success "The project is now a pure Rust implementation."
    exit 0
else
    echo ""
    log_error "❌ Python elimination incomplete. Please address the remaining issues above."
    exit 1
fi
