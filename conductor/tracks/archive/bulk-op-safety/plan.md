# Track-006: Bulk Operation Safety (Guardrails)

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** In-Progress

## Objective

Implement a multi-layered safety architecture for the `toad do` command and
provide an enhanced sandbox environment for safe verification of bulk
operations.

## Deliverables

- [x] **Phase 0: Sandbox Refactor - [ ] **Phase 0: Sandbox Refactor &
      Enhancement** Enhancement** [commit: 60a2a3f]
  - [x] Move sandbox script to `scripts/sandbox/`. [commit: 60a2a3f]
  - [x] Create `scripts/sandbox/README.md`. [commit: 60a2a3f]
  - [x] Add configurable args (tech stack mix, nesting depth). [commit: 60a2a3f]
  - [x] Modularize script into sub-files if necessary. [commit: 60a2a3f]
- [x] **Phase 1: Ghost Mode** [commit: 60a2a3f]
  - [x] Implement `--dry-run` flag in the `Do` subcommand. [commit: 60a2a3f]
  - [x] Display full absolute paths and exact commands in dry-run mode. [commit:
        60a2a3f]
- [x] **Phase 2: Destructive Guardrails** [commit: 60a2a3f]
  - [x] Create a "Danger Pattern" registry in `toad-ops`. [commit: 60a2a3f]
  - [x] Implement command string scanning. [commit: 60a2a3f]
  - [x] Implement forced "Type PROCEED" prompt for destructive commands.
        [commit: 60a2a3f]
- [x] **Phase 3: Resilience** [commit: 60a2a3f]
  - [x] Implement 30s timeout per project operation. [commit: 60a2a3f]
  - [x] Implement `--fail-fast` halt logic. [commit: 60a2a3f]
- [ ] **Phase 4: Auditing**
  - [ ] Implement simple local audit logger (`~/.toad/ops.log`).
- [ ] **Verification**
  - [ ] Test destructive pattern detection using the enhanced sandbox.
  - [ ] Verify dry-run output accuracy.
  - [ ] Perform SemVer bump.
