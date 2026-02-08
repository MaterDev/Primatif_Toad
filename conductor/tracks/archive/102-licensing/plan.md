# Implementation Plan: Licensing Migration (v1.0.2 Phase 0)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 0. This plan is
> an execution guide. The release docs are authoritative.

## Phase 0: License Files & Cargo.toml Updates

- [x] **P0-1: Write BUSL-1.1 License Text**
- [x] **P0-2: Write MIT License for Open Crates**
- [x] **P0-3: Update Cargo.toml License Fields**

## Phase 1: Git Operations Boundary Migration

- [x] **P0-4: Move `git init` logic**
- [x] **P0-4b: Migrate Safety Patterns**
- [x] **P0-4c: Verify Boundary**

## Phase 2: Boundary Enforcement (Hard Gates)

- [x] **P0-5: Enforcement Scripts**
- [x] **Justfile Integration:** Added `check-licenses` and updated `qa` suite.
- [x] **Sync Version:** Updated `sync_version.sh` for multi-repo ecosystem.

## Phase 3: License Notices

- [x] **P0-6: Add License Notices to Source Files**

## Phase 4: Verification

- [x] **QA Pass:** `just qa` passes with all tests and boundary checks.

## References

- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § The Licensing
  Strategy, § Governance
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 0
