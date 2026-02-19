# Specification: v1.1.1 QA Testing (111-v1.1.1-qa)

## Overview

Perform a comprehensive QA audit of the Primatif_Toad ecosystem (CLI + MCP) to
ensure stability, usability, and architectural integrity before the next major
release. This track focuses on empirical validation, documentation of issues,
and establishing a baseline for automated regression testing.

## Goals

1. **Validate Core CLI** - Comprehensive testing of all subcommands.
2. **Verify MCP Integration** - Ensure all MCP tools return high-fidelity,
   accurate data.
3. **Audit Ecosystem Health** - Use Toad's own analysis tools to find structural
   weaknesses.
4. **Safety Verification** - Test guardrails and destructive operations in a
   controlled sandbox.
5. **Document Baseline** - Create a detailed QA report and regression test plan.

## Test Suites

### TS-1: Discovery & Inspection

- `toad list`, `project list`, `status`, `reveal`, `stats`
- Cross-crate metadata accuracy.

### TS-2: Deep Analysis

- `analyze health`, `debt`, `deps`, `velocity`, `patterns`
- Insight quality and actionability.

### TS-3: State Management (Sandboxed)

- `tag`, `untag`, `sync`, `manifest`, `init-context`, `create`
- Workflow registration and execution (`cw`).

### TS-4: MCP Bridge

- Tool parity with CLI.
- Output formatting and token efficiency.

### TS-5: Diagnostics

- `toad doctor` accuracy.
- License boundary enforcement.

## Success Criteria

- [x] QA Test Plan documented in track.
- [x] All test suites executed and documented.
- [x] Sandboxed operations verified without data loss in main hub.
- [x] Final QA report generated and archived in `docs/releases/v1.1.1/`.
- [x] Future regression test plan established.
