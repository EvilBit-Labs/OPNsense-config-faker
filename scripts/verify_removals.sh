#!/bin/bash

# OPNsense Config Faker - Python Removal Verification
# This script verifies that Python files have been successfully removed

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

ERRORS=0

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    ((ERRORS++))
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

echo "Verifying Python removal..."

# Check that key Python files have been removed
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

echo
if [[ $ERRORS -gt 0 ]]; then
    echo "❌ Verification FAILED: $ERRORS Python files/directories still exist"
    exit 1
else
    echo "✅ Verification PASSED: All Python files have been removed"
    exit 0
fi
