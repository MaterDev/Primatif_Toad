# Plan: Toad Sync (Registry Cache)

## Phase 1: Registry Foundation [checkpoint: d6293bd]
- [x] Task: Implement registry caching (JSON storage) de06ab6
    - [x] Write Tests: Define `Registry` struct and verify serialization/deserialization to `~/.toad/registry.json`.
    - [x] Implement: Create the storage logic in `crates/toad-core` or a new module.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Registry Foundation' (Protocol in workflow.md)

## Phase 2: Sync Command [checkpoint: f3b0f73]
- [x] Task: Add `toad sync` command for manual refreshes ebf7704
    - [x] Write Tests: Verify `toad sync` triggers a full scan and updates the cache.
    - [x] Implement: Add the `sync` command to `bin/toad` and delegate to `toad-ops`.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Sync Command' (Protocol in workflow.md)

## Phase 3: Integration and Optimization
- [x] Task: Refactor `reveal` to prefer cache over disk scan 19d71ac
    - [x] Write Tests: Verify `toad reveal` uses cached data when available.
    - [x] Implement: Update `reveal` logic to check the registry before scanning.
- [x] Task: Implement staleness detection between cache and filesystem 52dd194
    - [x] Write Tests: Verify cache is invalidated when a "Ghost" project is detected.
    - [x] Implement: Add fingerprint/existence checks during cache lookup.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Integration and Optimization' (Protocol in workflow.md)
