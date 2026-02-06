# Track-014: Unified Taxonomy & Manifest Refactor

## Status
- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** In-Progress

## Objective
Remove metadata redundancy and implement an evidence-based "Ingredients" taxonomy across the platform.

## Deliverables
- [ ] **Phase 1: Core Model Refactor**
    - [ ] Update `ProjectDetail` in `toad-core` (Replace stack/hashtags with taxonomy).
    - [ ] Update `TagRegistry` to interact with the new model.
- [ ] **Phase 2: Exhaustive Discovery**
    - [ ] Refactor `discovery::strategies` to support multiple matches.
    - [ ] Update `scan_all_projects` to build the exhaustive taxonomy (Detected Stacks + File Markers + User Tags).
- [ ] **Phase 3: Manifest Refactor**
    - [ ] Update `toad-manifest` to report the unified taxonomy column.
    - [ ] Remove redundant hashtag detection logic from the manifest crate.
- [ ] **Phase 4: CLI Update**
    - [ ] Update `toad stats` and `toad reveal` to display the unified taxonomy.
- [ ] **Verification**
    - [ ] Run `toad manifest` on a hybrid (Rust+Node) project and verify both tags appear.
    - [ ] Perform SemVer bump.
- [ ] **Release v1.0.0 (RC)**
    - [ ] Final audit of all systems.
