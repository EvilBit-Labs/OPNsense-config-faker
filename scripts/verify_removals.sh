#!/bin/bash

# OPNsense Config Faker - Python Removal Verification
# This script verifies that Python files have been successfully removed

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

ERRORS=0

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    ((ERRORS++))
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_info() {
    echo -e "${YELLOW}[INFO]${NC} $1"
}

echo "Verifying Python elimination progress..."

# Phase 1 files (should be removed)
if [[ -f "main.py" ]]; then
    log_error "main.py still exists"
else
    log_success "main.py has been removed"
fi

if [[ -f "scripts/verify_xsd.py" ]]; then
    log_error "scripts/verify_xsd.py still exists"
else
    log_success "scripts/verify_xsd.py has been removed"
fi

if [[ -f "tests/test_generate_csv.py" ]]; then
    log_error "tests/test_generate_csv.py still exists"
else
    log_success "tests/test_generate_csv.py has been removed"
fi

if [[ -f "tests/test_model_generation.py" ]]; then
    log_error "tests/test_model_generation.py still exists"
else
    log_success "tests/test_model_generation.py has been removed"
fi

if [[ -d "opnsense/factories" ]]; then
    log_error "opnsense/factories directory still exists"
else
    log_success "opnsense/factories directory has been removed"
fi

if [[ -d "opnsense/generators" ]]; then
    log_error "opnsense/generators directory still exists"
else
    log_success "opnsense/generators directory has been removed"
fi

# Phase 2 files (should be removed)
if [[ -f "pyproject.toml" ]]; then
    log_error "pyproject.toml still exists"
else
    log_success "pyproject.toml has been removed"
fi

if [[ -f "setup.sh" ]]; then
    log_error "setup.sh still exists"
else
    log_success "setup.sh has been removed"
fi

if [[ -f "run_generator.sh" ]]; then
    log_error "run_generator.sh still exists"
else
    log_success "run_generator.sh has been removed"
fi

if [[ -f "generate_csv.py" ]]; then
    log_error "generate_csv.py still exists"
else
    log_success "generate_csv.py has been removed"
fi

if [[ -f "scripts/clean.py" ]]; then
    log_error "scripts/clean.py still exists"
else
    log_success "scripts/clean.py has been removed"
fi

# Files that should remain (for now)
if [[ -f "tests/python_reference.py" ]]; then
    log_info "tests/python_reference.py preserved (needed for migration validation)"
else
    log_error "tests/python_reference.py missing (needed for migration validation)"
fi

if [[ -d "opnsense/models" ]]; then
    log_info "opnsense/models/ preserved (XSD-generated files, remove after XSD migration)"
else
    log_error "opnsense/models/ missing (XSD-generated files needed until XSD migration)"
fi

echo
if [[ $ERRORS -gt 0 ]]; then
    echo "❌ Verification FAILED: $ERRORS issues found"
    exit 1
else
    echo "✅ Verification PASSED: Python elimination completed successfully"
    echo "📝 Remaining: opnsense/models/ (XSD files) and tests/python_reference.py (validation)"
    exit 0
fi
