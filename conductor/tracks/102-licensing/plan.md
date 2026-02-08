# Implementation Plan: Licensing Migration (v1.0.2 Phase 0)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 0.
> This plan is an execution guide. The release docs are authoritative.

## Phase 0: License Files & Cargo.toml Updates
- [ ] **P0-1: Write BUSL-1.1 License Text**
  - Draft `LICENSE` with Primatif-specific terms (2034-02-07 change date).
  - Place in `discovery`, `toad-git`, `toad-manifest`, `toad-ops`.
- [ ] **P0-2: Write MIT License for Open Crates**
  - Place MIT `LICENSE` in `toad-core`, `scaffold`.
- [ ] **P0-3: Update Cargo.toml License Fields**
  - Set `license = "MIT"` for open crates.
  - Set `license = "BUSL-1.1"` for intelligence crates.
  - Update workspace `Cargo.toml`.

## Phase 1: Git Operations Boundary Migration
- [ ] **P0-4: Move `git init` logic**
  - Remove `init_git()` from `scaffold`.
  - Add `init_repo()` to `toad-git`.
  - Orchestrate in `bin/toad`.
- [ ] **P0-4b: Migrate Safety Patterns**
  - Move destructive git patterns from `toad-ops` to `toad-git`.
- [ ] **P0-4c: Verify Boundary**
  - Ensure no `Command::new("git")` exists outside `toad-git`.

## Phase 2: Boundary Enforcement (Hard Gates)
- [ ] **P0-5: Enforcement Scripts**
  - Create `scripts/check_license_boundary.sh`.
  - Implement Git pre-commit hook.
  - Configure `deny.toml` for `cargo-deny`.

## Phase 3: License Notices
- [ ] **P0-6: Add License Notices to Source Files**
  - Add SPDX header comments to all `.rs` files in each crate.
  - MIT crates: `// SPDX-License-Identifier: MIT`
  - BUSL-1.1 crates: `// SPDX-License-Identifier: BUSL-1.1`

## References
- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § The Licensing Strategy, § Governance
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 0
