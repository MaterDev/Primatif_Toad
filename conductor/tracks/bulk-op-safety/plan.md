# Track-006: Bulk Operation Safety (Guardrails)

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** Pending

## Objective

Implement a multi-layered safety architecture for the `toad do` command to
prevent accidental data loss.

## Deliverables

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
- [ ] **Verification:**
  - [ ] Test `rm -rf` detection.
  - [ ] Test timeout behavior with `sleep 100`.
