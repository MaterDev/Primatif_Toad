# Track-004: Context Integrity (Deep Fingerprinting)

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** Pending

## Objective

Ensure the "Context is stale" warning is 100% reliable by implementing
multi-level filesystem fingerprinting.

## Deliverables

- [ ] **Research:** Finalize the list of "High-Value" metadata files per tech
      stack.
- [ ] **Implementation:**
  - [ ] Update `Workspace::get_fingerprint` to perform a multi-level scan.
  - [ ] Implement a fast hashing algorithm for mtime aggregation.
- [ ] **Tests:**
  - [ ] Unit test: Verify fingerprint changes when `README.md` is edited.
  - [ ] Unit test: Verify fingerprint changes when a project is deleted.
  - [ ] Benchmark: Ensure 100-project scan is under 50ms.
- [ ] **Integration:** Confirm the CLI correctly prompts for `toad manifest`
      after external file changes.
