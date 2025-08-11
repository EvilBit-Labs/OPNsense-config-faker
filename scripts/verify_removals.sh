#!/bin/bash

# OPNsense Config Faker - Python Removal Safety Check Script
# This script verifies that Python files can be safely removed without breaking the project

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
ERRORS=0
WARNINGS=0
CHECKS=0

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
    ((WARNINGS++))
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
    ((ERRORS++))
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

check_file_exists() {
    local file="$1"
    local description="$2"
    ((CHECKS++))

    if [[ -f "$file" ]]; then
        log_success "$description exists: $file"
    else
        log_error "$description missing: $file"
    fi
}

check_directory_exists() {
    local dir="$1"
    local description="$2"
    ((CHECKS++))

    if [[ -d "$dir" ]]; then
        log_success "$description exists: $dir"
    else
        log_error "$description missing: $dir"
    fi
}

check_no_python_references() {
    local file="$1"
    local description="$2"
    ((CHECKS++))

    if [[ -f "$file" ]]; then
        if grep -q -i "python" "$file" 2>/dev/null; then
            log_warning "$description contains Python references: $file"
        else
            log_success "$description has no Python references: $file"
        fi
    else
        log_warning "$description not found: $file"
    fi
}

check_no_python_workflows() {
    local workflow_file="$1"
    ((CHECKS++))

    if [[ -f "$workflow_file" ]]; then
        # Look for actual Python tooling, excluding job names that contain "python"
        if grep -v "python-safety-check" "$workflow_file" | grep -q -E "(python|pip|uv|pytest|pyproject\.toml)" 2>/dev/null; then
            log_error "CI workflow contains Python references: $workflow_file"
        else
            log_success "CI workflow has no Python references: $workflow_file"
        fi
    else
        log_error "CI workflow file missing: $workflow_file"
    fi
}

check_no_python_compat_feature() {
    local cargo_file="$1"
    ((CHECKS++))

    if [[ -f "$cargo_file" ]]; then
        if grep -q "python-compat" "$cargo_file" 2>/dev/null; then
            log_error "Cargo.toml still contains python-compat feature: $cargo_file"
        else
            log_success "Cargo.toml has no python-compat feature: $cargo_file"
        fi
    else
        log_error "Cargo.toml missing: $cargo_file"
    fi
}

check_no_python_tests() {
    local test_file="$1"
    ((CHECKS++))

    if [[ -f "$test_file" ]]; then
        log_error "Python compatibility test still exists: $test_file"
    else
        log_success "Python compatibility test removed: $test_file"
    fi
}

# Main verification logic
main() {
    log_info "Starting Python removal safety checks..."
    echo

    # Check for target files that should be removed
    log_info "Checking target files for removal..."

    # Files that should be removed
    check_file_exists "main.py" "Main Python CLI"
    check_file_exists "scripts/verify_xsd.py" "Python XSD verification script"
    check_file_exists "tests/test_generate_csv.py" "Python CSV test"
    check_file_exists "tests/test_model_generation.py" "Python model generation test"
    check_file_exists "tests/__init__.py" "Python test package init"
    check_file_exists "opnsense/__init__.py" "Python package init"

    # Directories that should be removed
    check_directory_exists "opnsense/factories" "Python factories directory"
    check_directory_exists "opnsense/generators" "Python generators directory"

    echo

    # Check CI workflows for Python references
    log_info "Checking CI workflows for Python references..."
    check_no_python_workflows ".github/workflows/ci.yml"
    echo

    # Check documentation for Python references
    log_info "Checking documentation for Python references..."
    check_no_python_references "README.md" "README.md"
    check_no_python_references "TESTING.md" "TESTING.md"
    check_no_python_references "ELIMINATION_PLAN.md" "ELIMINATION_PLAN.md"
    echo

    # Check Cargo.toml for python-compat feature
    log_info "Checking Cargo.toml for python-compat feature..."
    check_no_python_compat_feature "Cargo.toml"
    echo

    # Check for Python compatibility tests
    log_info "Checking for Python compatibility tests..."
    check_no_python_tests "tests/python_compat.rs"
    echo

    # Check justfile for Python references
    log_info "Checking justfile for Python references..."
    check_no_python_references "justfile" "justfile"
    echo

    # Summary
    log_info "Safety check summary:"
    log_info "  Total checks performed: $CHECKS"
    log_info "  Warnings: $WARNINGS"
    log_info "  Errors: $ERRORS"
    echo

    if [[ $ERRORS -gt 0 ]]; then
        log_error "Safety check FAILED with $ERRORS error(s)"
        log_error "Do not proceed with Python removal until all errors are resolved"
        exit 1
    elif [[ $WARNINGS -gt 0 ]]; then
        log_warning "Safety check completed with $WARNINGS warning(s)"
        log_warning "Review warnings before proceeding with Python removal"
        exit 0
    else
        log_success "Safety check PASSED - Python removal is safe to proceed"
        exit 0
    fi
}

# Run main function
main "$@"
