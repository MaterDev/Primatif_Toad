# Implementation Plan: Custom Workflows (v1.0.2 Phase 7)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 7.
> This plan is an execution guide. The release docs are authoritative.

## Phase 1: Data Models
- [ ] **P7-1: Workflow Registry** (`toad-core`, MIT)
  - Add `CustomWorkflow` and `WorkflowRegistry` structs.
  - `reserved_namespaces` field is a cache — authoritative list is in `toad-ops`.

## Phase 2: Namespace Authority & Execution
- [ ] **P7-2: Reserved Namespace Authority** (`toad-ops`, BUSL-1.1)
  - Add `reserved_command_names() -> Vec<&'static str>` — single source of truth.
  - Sync JSON cache on startup.
  - Unit test: compare function output against `Commands` enum in `bin/toad`.
- [ ] **P7-3: Execution Logic** (`toad-ops`, BUSL-1.1)
  - Implement register, update, delete, run, list, info.
  - Validate script path (absolute, exists, `.sh`, executable).

## Phase 3: CLI Surface
- [ ] **P7-4: `toad cw` Command** (`bin/toad`, MIT)
  - Implement full CLI surface: run, register, update, delete, list, info.
  - Exit with script's exit code.

## Phase 4: Integration & Dogfooding
- [ ] **P7-5: Integration Tests**
  - Full lifecycle: register → list → run → info → delete.
  - Namespace collision rejection.
  - Missing script detection.
  - Argument passthrough.
- [ ] **P5b-2: Release Waterfall Script** _(project-specific)_
  - Create `scripts/publish_waterfall.sh`.
  - Register as `toad cw release` for dogfooding.

## References
- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Custom Workflows
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 7, Phase 5b
