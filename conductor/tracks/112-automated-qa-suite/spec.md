# Specification: Automated QA Suite (112-automated-qa-suite)

## Overview

Develop a robust, automated regression testing suite for the CLI and MCP server
to ensure long-term integrity and prevent regressions.

## Goals

1. **CLI Integration Tests** - Automate the "Sandbox" testing workflow.
2. **MCP E2E Tests** - Validate MCP tool outputs against expected JSON schemas.
3. **Quality Gates** - Integrate these tests into the `just qa` workflow.

## Implementation Details

- Use `scripts/sandbox/mksandbox.sh` as the foundation for integration tests.
- Implement a test runner (possibly in Rust or Bash) that verifies CLI outputs.
- Create JSON schema validators for MCP responses.
