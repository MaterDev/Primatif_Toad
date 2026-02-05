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

- [ ] **Phase 0: Sandbox Refactor & Enhancement**
  - [ ] Move sandbox script to `scripts/sandbox/`.
  - [ ] Create `scripts/sandbox/README.md`.
  - [ ] Add configurable args (tech stack mix, nesting depth).
  - [ ] Modularize script into sub-files if necessary.
- [ ] **Phase 1: Ghost Mode**
  - [ ] Implement `--dry-run` flag in the `Do` subcommand.
  - [ ] Display full absolute paths and exact commands in dry-run mode.
- [ ] **Phase 2: Destructive Guardrails**
  - [ ] Create a "Danger Pattern" registry in `toad-ops`.
  - [ ] Implement command string scanning.
  - [ ] Implement forced "Type PROCEED" prompt for destructive commands.
- [ ] **Phase 3: Resilience**
  - [ ] Implement 30s timeout per project operation.
  - [ ] Implement `--fail-fast` halt logic.
- [ ] **Phase 4: Auditing**
  - [ ] Implement simple local audit logger (`~/.toad/ops.log`).
- [ ] **Verification**
  - [ ] Test destructive pattern detection using the enhanced sandbox.
  - [ ] Verify dry-run output accuracy.
  - [ ] Perform SemVer bump.
