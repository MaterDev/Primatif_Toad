# Plan: Toad Sync (Registry Cache)

## Phase 1: Registry Foundation
- [x] Task: Implement registry caching (JSON storage) de06ab6
    - [x] Write Tests: Define `Registry` struct and verify serialization/deserialization to `~/.toad/registry.json`.
    - [x] Implement: Create the storage logic in `crates/toad-core` or a new module.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Registry Foundation' (Protocol in workflow.md)

## Phase 2: Sync Command
- [ ] Task: Add `toad sync` command for manual refreshes
    - [ ] Write Tests: Verify `toad sync` triggers a full scan and updates the cache.
    - [ ] Implement: Add the `sync` command to `bin/toad` and delegate to `toad-ops`.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Sync Command' (Protocol in workflow.md)

## Phase 3: Integration and Optimization
- [ ] Task: Refactor `reveal` to prefer cache over disk scan
    - [ ] Write Tests: Verify `toad reveal` uses cached data when available.
    - [ ] Implement: Update `reveal` logic to check the registry before scanning.
- [ ] Task: Implement staleness detection between cache and filesystem
    - [ ] Write Tests: Verify cache is invalidated when a "Ghost" project is detected.
    - [ ] Implement: Add fingerprint/existence checks during cache lookup.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Integration and Optimization' (Protocol in workflow.md)
