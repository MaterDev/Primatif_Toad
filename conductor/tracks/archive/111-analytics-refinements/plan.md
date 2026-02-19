# Plan: Analytics Refinements (111-analytics-refinements)

> **Spec:** [./spec.md](./spec.md)

---

## Tasks

### Phase 1: Dependency Engine (45 min)

- [x] Add `cargo_metadata` dependency to `crates/toad-ops/Cargo.toml`
- [x] Refactor `analyze_dependencies` to use `cargo_metadata::MetadataCommand`
- [x] Implement robust matching for local paths vs registry crates
- [x] Verify accuracy against Toad's own multi-crate structure

### Phase 2: Graph Theory (30 min)

- [x] Implement cycle detection algorithm (DFS) in `toad-ops/src/analytics.rs`
- [x] Populate `circular_dependencies` in `DependencyGraph`
- [x] Add unit test with a mocked cyclic graph

### Phase 3: Hardening (20 min)

- [x] Refine `analyze_debt` to strictly filter by text-based extensions if
      possible
- [x] Ensure `ignore` walker handles hidden files consistently
- [x] Optimize Git velocity calls (reduce process spawning if feasible)

### Phase 4: Verification (15 min)

- [x] `just build`
- [x] `cargo run --bin toad -- analyze deps`
- [x] `cargo run --bin toad -- analyze health`

---

## Acceptance Criteria

- Dependency graph is 100% accurate for Rust projects.
- Circular dependencies are identified.
- No false positives in TODO/FIXME scanning for binary files.
