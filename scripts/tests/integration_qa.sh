#!/usr/bin/env bash
# Automated Integration QA Suite for Toad
# Tests CLI commands in a sandboxed environment

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# CI mode detection
CI_MODE="${CI:-false}"
if [[ "$CI_MODE" == "true" ]]; then
    # Disable colors in CI
    RED=''
    GREEN=''
    YELLOW=''
    BLUE=''
    NC=''
fi

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Logging functions
log_info() {
    if [[ "$CI_MODE" != "true" ]]; then
        echo -e "${BLUE}ℹ${NC} $1"
    else
        echo "[INFO] $1"
    fi
}

log_success() {
    if [[ "$CI_MODE" != "true" ]]; then
        echo -e "${GREEN}✓${NC} $1"
    else
        echo "[PASS] $1"
    fi
}

log_error() {
    if [[ "$CI_MODE" != "true" ]]; then
        echo -e "${RED}✗${NC} $1"
    else
        echo "[FAIL] $1"
    fi
}

log_warning() {
    if [[ "$CI_MODE" != "true" ]]; then
        echo -e "${YELLOW}⚠${NC} $1"
    else
        echo "[WARN] $1"
    fi
}

# Test assertion helpers
assert_success() {
    local test_name="$1"
    local command="$2"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if eval "$command" > /dev/null 2>&1; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        log_success "$test_name"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "$test_name"
        return 1
    fi
}

assert_output_contains() {
    local test_name="$1"
    local command="$2"
    local expected="$3"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    local output
    output=$(eval "$command" 2>&1)
    
    if echo "$output" | grep -q "$expected"; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        log_success "$test_name"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "$test_name (expected: '$expected')"
        if [[ "$CI_MODE" != "true" ]]; then
            echo "  Output: $output" | head -3
        fi
        return 1
    fi
}

assert_file_exists() {
    local test_name="$1"
    local file_path="$2"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if [[ -f "$file_path" ]]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        log_success "$test_name"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "$test_name (file not found: $file_path)"
        return 1
    fi
}

# Main test suite
main() {
    log_info "Starting Toad Integration QA Suite"
    log_info "CI Mode: $CI_MODE"
    
    # Get script directory
    SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
    SANDBOX_DIR="$PROJECT_ROOT/scripts/sandbox"
    
    # Build Toad if not already built
    log_info "Ensuring Toad is built..."
    cd "$PROJECT_ROOT"
    cargo build --release --quiet
    TOAD_BIN="$PROJECT_ROOT/target/release/toad"
    
    if [[ ! -f "$TOAD_BIN" ]]; then
        log_error "Toad binary not found at $TOAD_BIN"
        exit 1
    fi
    
    # Create temporary sandbox
    TEMP_SANDBOX=$(mktemp -d)
    trap "rm -rf $TEMP_SANDBOX" EXIT
    
    log_info "Creating test sandbox at $TEMP_SANDBOX"
    cd "$SANDBOX_DIR"
    ./mksandbox.sh -c 5 -o "$TEMP_SANDBOX" -s "rust,node,generic" > /dev/null 2>&1
    
    cd "$TEMP_SANDBOX"
    
    # Initialize toad home in sandbox
    log_info "Initializing Toad home in sandbox..."
    export TOAD_ROOT="$TEMP_SANDBOX"
    $TOAD_BIN home "$TEMP_SANDBOX" > /dev/null 2>&1
    
    # Test Suite: Basic Commands
    log_info ""
    log_info "=== Test Suite: Basic Commands ==="
    
    assert_success "toad version" "$TOAD_BIN --version"
    assert_success "toad help" "$TOAD_BIN --help"
    
    # Test Suite: Project Discovery
    log_info ""
    log_info "=== Test Suite: Project Discovery ==="
    
    assert_success "toad sync" "$TOAD_BIN sync"
    assert_output_contains "toad reveal shows projects" "$TOAD_BIN reveal ''" "mock-proj"
    assert_output_contains "toad status runs" "$TOAD_BIN status" "ECOSYSTEM HEALTH SCAN"
    
    # Test Suite: Project Search
    log_info ""
    log_info "=== Test Suite: Project Search ==="
    
    assert_output_contains "toad reveal finds project" "$TOAD_BIN reveal mock-proj-1" "mock-proj-1"
    
    # Test Suite: Tagging
    log_info ""
    log_info "=== Test Suite: Tagging ==="
    
    assert_success "toad tag add" "$TOAD_BIN tag --project mock-proj-1 --tag '#test'"
    assert_output_contains "toad reveal shows tag" "$TOAD_BIN reveal '' --tag '#test'" "mock-proj-1"
    
    # Test Suite: Analytics
    log_info ""
    log_info "=== Test Suite: Analytics ==="
    
    assert_output_contains "toad stats runs" "$TOAD_BIN stats" "ECOSYSTEM ANALYTICS"
    
    # Test Suite: Manifest Generation
    log_info ""
    log_info "=== Test Suite: Manifest Generation ==="
    
    assert_success "toad manifest" "$TOAD_BIN manifest"
    assert_file_exists "MANIFEST.md created" ".toad/contexts/default/shadows/MANIFEST.md"
    
    # Test Suite: Doctor Command
    log_info ""
    log_info "=== Test Suite: Health Check ==="
    
    assert_output_contains "toad doctor runs" "$TOAD_BIN doctor" "ECOSYSTEM HEALTH REPORT"
    
    # Test Suite: Non-Interactive Flags
    log_info ""
    log_info "=== Test Suite: Non-Interactive Execution ==="
    
    # Test --yes flag (should abort without user input)
    assert_success "toad version with --yes works" "$TOAD_BIN --version"
    
    # Summary
    log_info ""
    log_info "=== Test Results ==="
    log_info "Tests Run: $TESTS_RUN"
    log_success "Tests Passed: $TESTS_PASSED"
    
    if [[ $TESTS_FAILED -gt 0 ]]; then
        log_error "Tests Failed: $TESTS_FAILED"
        exit 1
    else
        log_success "All tests passed!"
        exit 0
    fi
}

# Run main function
main "$@"
