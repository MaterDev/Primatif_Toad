# Work Order 008: Architectural Refinement (The Great Decoupling)

**Status: Completed**

## Motivation

To transform Primatif_Toad from a monolithic CLI into a modular Meta-Engineering
Platform. Current logic for path resolution, data modeling, and manifest
formatting is scattered or tightly coupled, making it difficult to add new
"Shadow" features or language support without side effects.

## Implementation Plan

### Phase 1: The Core Foundation (`crates/toad-core`)

- [x] Create `toad-core` crate.
- [x] Move shared enums (`ProjectStack`, `ActivityTier`, `VcsStatus`) and
      `ProjectDetail` struct.
- [x] Centralize `Workspace` context: path constants and `Fingerprint` logic.
- [x] **Verification:** All existing crates compile using `toad-core` types.

### Phase 2: Manifest Logic Extraction (`crates/toad-manifest`)

- [x] Create `toad-manifest` crate.
- [x] Move Markdown table/list formatting logic from `bin/toad`.
- [x] Implement `ManifestHeader` logic (Fingerprint string parsing/generation).
- [x] **Verification:** New unit tests for Markdown generation parity.

### Phase 3: Modular Discovery (`crates/toad-discovery`)

- [x] Refactor `detect_stack` into a Strategy Pattern (e.g., `DiscoveryStrategy`
      trait).
- [x] Clean up `extract_essence` to be a standalone utility.
- [x] **Verification:** `just test` passes with 100% parity on project
      detection.

### Phase 4: Documentation Update

- [x] Update `README.md` with the new Crate-based architecture diagram.
- [x] Update `.gemini/GEMINI.md` to reflect the new "Modular Crate" mandate.
- [x] Update `docs/CONVENTIONS.md` to include rules for adding new Crate
      capabilities.

## Success Criteria

- [x] `bin/toad/src/main.rs` is < 150 lines of code.
- [x] Manifest generation is testable without running the CLI.
- [x] Adding a new language (e.g., "Zig") requires modifying exactly one file in
      `discovery`.

## Progress Notes

- [2026-02-04] Refactor complete. The system is now modular, decoupled, and
  highly extensible.
