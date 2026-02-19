#!/usr/bin/env bash
# MCP Validation Test Suite
# Tests toad-mcp server tool accessibility and JSON response validation

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
SANDBOX_DIR=""
TOAD_MCP_BIN="$PROJECT_ROOT/target/debug/toad-mcp"

# Colors (disabled in CI)
if [ -n "${CI:-}" ]; then
    RED=""
    GREEN=""
    YELLOW=""
    BLUE=""
    RESET=""
else
    RED='\033[0;31m'
    GREEN='\033[0;32m'
    YELLOW='\033[1;33m'
    BLUE='\033[0;34m'
    RESET='\033[0m'
fi

# Test counters
TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${RESET} $*"
}

log_success() {
    echo -e "${GREEN}[PASS]${RESET} $*"
}

log_error() {
    echo -e "${RED}[FAIL]${RESET} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARN]${RESET} $*"
}

# Test assertion helpers
assert_json_valid() {
    local json="$1"
    local test_name="$2"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if echo "$json" | jq empty 2>/dev/null; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        log_success "$test_name: Valid JSON"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "$test_name: Invalid JSON"
        echo "$json" | head -20
        return 1
    fi
}

assert_has_field() {
    local json="$1"
    local field="$2"
    local test_name="$3"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    if echo "$json" | jq -e "$field" >/dev/null 2>&1; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        log_success "$test_name: Has field '$field'"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "$test_name: Missing field '$field'"
        return 1
    fi
}

assert_field_type() {
    local json="$1"
    local field="$2"
    local expected_type="$3"
    local test_name="$4"
    
    TESTS_RUN=$((TESTS_RUN + 1))
    
    local actual_type
    actual_type=$(echo "$json" | jq -r "$field | type" 2>/dev/null || echo "null")
    
    if [ "$actual_type" = "$expected_type" ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        log_success "$test_name: Field '$field' is $expected_type"
        return 0
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "$test_name: Field '$field' is $actual_type, expected $expected_type"
        return 1
    fi
}

# MCP protocol helpers
send_mcp_request() {
    local method="$1"
    local params="${2:-{}}"
    local id="${3:-1}"
    
    local request
    request=$(jq -n \
        --arg method "$method" \
        --argjson params "$params" \
        --argjson id "$id" \
        '{jsonrpc: "2.0", method: $method, params: $params, id: $id}')
    
    echo "$request"
}

call_mcp_tool() {
    local tool_name="$1"
    local arguments="${2:-{}}"
    
    local params
    params=$(jq -n \
        --arg name "$tool_name" \
        --argjson arguments "$arguments" \
        '{name: $name, arguments: $arguments}')
    
    send_mcp_request "tools/call" "$params"
}

# Setup and teardown
setup_sandbox() {
    log_info "Creating test sandbox..."
    SANDBOX_DIR=$(mktemp -d -t toad-mcp-test.XXXXXX)
    
    # Create minimal toad workspace
    mkdir -p "$SANDBOX_DIR/projects"
    mkdir -p "$SANDBOX_DIR/.toad"
    
    # Create minimal config
    cat > "$SANDBOX_DIR/.toad/config.toml" <<EOF
[workspace]
name = "test-workspace"
root = "$SANDBOX_DIR/projects"

[ai]
token_limit = 1000
EOF
    
    # Create a test project
    mkdir -p "$SANDBOX_DIR/projects/test-project"
    cat > "$SANDBOX_DIR/projects/test-project/README.md" <<EOF
# Test Project
A test project for MCP validation.
EOF
    
    export TOAD_HOME="$SANDBOX_DIR/.toad"
    log_success "Sandbox created at $SANDBOX_DIR"
}

cleanup_sandbox() {
    if [ -n "$SANDBOX_DIR" ] && [ -d "$SANDBOX_DIR" ]; then
        log_info "Cleaning up sandbox..."
        rm -rf "$SANDBOX_DIR"
        log_success "Sandbox cleaned up"
    fi
}

# Build toad-mcp if needed
ensure_mcp_binary() {
    if [ ! -f "$TOAD_MCP_BIN" ]; then
        log_info "Building toad-mcp..."
        cd "$PROJECT_ROOT"
        cargo build -p toad-mcp
        log_success "toad-mcp built"
    fi
}

# Test MCP server initialization
test_mcp_initialize() {
    log_info "Testing MCP initialization..."
    
    local init_request
    init_request=$(send_mcp_request "initialize" '{"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}')
    
    local response
    response=$(echo "$init_request" | "$TOAD_MCP_BIN" 2>/dev/null | head -1)
    
    assert_json_valid "$response" "Initialize"
    assert_has_field "$response" ".result.protocolVersion" "Initialize"
    assert_has_field "$response" ".result.capabilities" "Initialize"
    assert_has_field "$response" ".result.serverInfo" "Initialize"
    assert_field_type "$response" ".result.serverInfo.name" "string" "Initialize"
    assert_field_type "$response" ".result.serverInfo.version" "string" "Initialize"
}

# Test tools/list
test_tools_list() {
    log_info "Testing tools/list..."
    
    local list_request
    list_request=$(send_mcp_request "tools/list" '{}')
    
    local response
    response=$(echo -e "$list_request" | "$TOAD_MCP_BIN" 2>/dev/null | tail -1)
    
    assert_json_valid "$response" "Tools List"
    assert_has_field "$response" ".result.tools" "Tools List"
    assert_field_type "$response" ".result.tools" "array" "Tools List"
    
    # Check that we have tools
    local tool_count
    tool_count=$(echo "$response" | jq '.result.tools | length')
    
    TESTS_RUN=$((TESTS_RUN + 1))
    if [ "$tool_count" -gt 0 ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        log_success "Tools List: Found $tool_count tools"
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        log_error "Tools List: No tools found"
    fi
    
    # Verify tool schema structure
    local first_tool
    first_tool=$(echo "$response" | jq '.result.tools[0]')
    
    assert_has_field "$first_tool" ".name" "Tool Schema"
    assert_has_field "$first_tool" ".description" "Tool Schema"
    assert_has_field "$first_tool" ".inputSchema" "Tool Schema"
}

# Test individual MCP tools
test_list_projects_tool() {
    log_info "Testing list_projects tool..."
    
    local request
    request=$(call_mcp_tool "list_projects" '{}')
    
    local response
    response=$(echo "$request" | "$TOAD_MCP_BIN" 2>/dev/null | tail -1)
    
    assert_json_valid "$response" "list_projects"
    assert_has_field "$response" ".result" "list_projects"
    
    # The result should contain content array
    assert_has_field "$response" ".result.content" "list_projects"
    assert_field_type "$response" ".result.content" "array" "list_projects"
}

test_get_manifest_tool() {
    log_info "Testing get_manifest tool..."
    
    local request
    request=$(call_mcp_tool "get_manifest" '{}')
    
    local response
    response=$(echo "$request" | "$TOAD_MCP_BIN" 2>/dev/null | tail -1)
    
    assert_json_valid "$response" "get_manifest"
    assert_has_field "$response" ".result" "get_manifest"
    assert_has_field "$response" ".result.content" "get_manifest"
}

test_get_ecosystem_summary_tool() {
    log_info "Testing get_ecosystem_summary tool..."
    
    local request
    request=$(call_mcp_tool "get_ecosystem_summary" '{"token_limit": 500}')
    
    local response
    response=$(echo "$request" | "$TOAD_MCP_BIN" 2>/dev/null | tail -1)
    
    assert_json_valid "$response" "get_ecosystem_summary"
    assert_has_field "$response" ".result" "get_ecosystem_summary"
}

test_search_projects_tool() {
    log_info "Testing search_projects tool..."
    
    local request
    request=$(call_mcp_tool "search_projects" '{"query": "test"}')
    
    local response
    response=$(echo "$request" | "$TOAD_MCP_BIN" 2>/dev/null | tail -1)
    
    assert_json_valid "$response" "search_projects"
    assert_has_field "$response" ".result" "search_projects"
}

test_get_active_context_tool() {
    log_info "Testing get_active_context tool..."
    
    local request
    request=$(call_mcp_tool "get_active_context" '{}')
    
    local response
    response=$(echo "$request" | "$TOAD_MCP_BIN" 2>/dev/null | tail -1)
    
    assert_json_valid "$response" "get_active_context"
    assert_has_field "$response" ".result" "get_active_context"
}

# Test error handling
test_invalid_tool() {
    log_info "Testing invalid tool error handling..."
    
    local request
    request=$(call_mcp_tool "nonexistent_tool" '{}')
    
    local response
    response=$(echo "$request" | "$TOAD_MCP_BIN" 2>/dev/null | tail -1)
    
    assert_json_valid "$response" "Invalid Tool"
    assert_has_field "$response" ".error" "Invalid Tool"
    assert_has_field "$response" ".error.code" "Invalid Tool"
}

test_invalid_params() {
    log_info "Testing invalid parameters error handling..."
    
    local request
    request=$(call_mcp_tool "search_projects" '{}')
    
    local response
    response=$(echo "$request" | "$TOAD_MCP_BIN" 2>/dev/null | tail -1)
    
    assert_json_valid "$response" "Invalid Params"
    # Should either succeed with empty results or return an error
    if echo "$response" | jq -e '.error' >/dev/null 2>&1; then
        assert_has_field "$response" ".error.code" "Invalid Params"
    else
        assert_has_field "$response" ".result" "Invalid Params"
    fi
}

# Main test runner
main() {
    log_info "Starting MCP Validation Test Suite"
    echo
    
    # Ensure binary exists
    ensure_mcp_binary
    
    # Setup test environment
    setup_sandbox
    
    # Trap cleanup
    trap cleanup_sandbox EXIT
    
    # Run tests
    test_mcp_initialize
    echo
    test_tools_list
    echo
    test_list_projects_tool
    echo
    test_get_manifest_tool
    echo
    test_get_ecosystem_summary_tool
    echo
    test_search_projects_tool
    echo
    test_get_active_context_tool
    echo
    test_invalid_tool
    echo
    test_invalid_params
    echo
    
    # Print summary
    echo "========================================"
    log_info "Test Summary"
    echo "  Total:  $TESTS_RUN"
    echo -e "  ${GREEN}Passed: $TESTS_PASSED${RESET}"
    echo -e "  ${RED}Failed: $TESTS_FAILED${RESET}"
    echo "========================================"
    
    if [ $TESTS_FAILED -eq 0 ]; then
        log_success "All tests passed!"
        exit 0
    else
        log_error "$TESTS_FAILED test(s) failed"
        exit 1
    fi
}

# Check dependencies
if ! command -v jq &> /dev/null; then
    log_error "jq is required but not installed. Please install jq."
    exit 1
fi

main "$@"
