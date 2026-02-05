# Track-005: High-Performance Orchestration

## Status

- **Priority:** Medium
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** Pending

## Objective

Scale Toad to handle massive project lists by introducing multi-core parallelism
and thread-safe UI feedback.

## Deliverables

- [x] **Infrastructure:** [commit: 91cad97]
  - [x] Add `rayon` and `indicatif` to `Cargo.toml`. [commit: 91cad97]
- [ ] **Parallel Scanning:**
  - [ ] Refactor `discovery::scan_all_projects` to use parallel iterators.
- [ ] **Parallel Execution:**
  - [ ] Refactor `Commands::Do` to use parallel execution.
  - [ ] Implement result buffering to prevent console log interleaving.
- [ ] **UI:**
  - [ ] Implement a multi-project progress bar for batch operations.
- [ ] **Verification:**
  - [ ] Stress test with 100+ dummy projects.
  - [ ] Verify `Ctrl-C` behavior during parallel runs.
