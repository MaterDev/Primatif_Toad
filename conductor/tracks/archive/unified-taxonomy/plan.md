# Track-014: Unified Taxonomy & Manifest Refactor

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** Completed

## Objective

Remove metadata redundancy and implement an evidence-based "Ingredients"
taxonomy across the platform.

## Deliverables

- [x] **Phase 1: Core Model Refactor**
  - [x] Update `ProjectDetail` in `toad-core` (Replace stack/hashtags with
        taxonomy).
  - [x] Update `TagRegistry` to interact with the new model.
- [x] **Phase 2: Exhaustive Discovery**
  - [x] Refactor `discovery::strategies` to support multiple matches.
  - [x] Update `scan_all_projects` to build the exhaustive taxonomy (Detected
        Stacks + File Markers + User Tags).
- [x] **Phase 3: Manifest Refactor**
  - [x] Update `toad-manifest` to report the unified taxonomy column.
  - [x] Remove redundant hashtag detection logic from the manifest crate.
- [x] **Phase 4: CLI Update**
  - [x] Update `toad stats` and `toad reveal` to display the unified taxonomy.
- [x] **Verification**
  - [x] Run `toad manifest` on a hybrid (Rust+Node) project and verify both tags
        appear.
  - [x] Perform SemVer bump.
- [x] **Release v1.0.0 (RC)**
  - [x] Final audit of all systems.

