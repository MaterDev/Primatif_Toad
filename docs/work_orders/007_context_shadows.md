# Work Order 007: Context Shadows (Project Manifest)

**Status: Completed**

## Motivation

To provide Gemini CLI with "Peripheral Vision" and "Semantic Intuition" over the
`projects/` directory. By creating a high-density manifest of all managed
projects, the AI can cross-reference patterns and understand the broader
workspace without expensive directory traversal.

## Implementation Plan

### Phase 1: Structural Discovery (`crates/discovery`)

- [x] **Metadata Model:** Implement `ProjectDetail` to track: name, path, stack,
      essence (procedural), and tokens (agentic).
- [x] **Hierarchical Detection:** Implement a "Shallow Dive" (depth 2)
      algorithm:
  - [x] Root Anchor detection (`nx.json`, `go.work`, `Cargo.toml` workspace).
  - [x] Standard Signature detection (`package.json`, `go.mod`, etc.).
  - [x] Sub-project sampling for detected monorepos.
- [x] **Procedural Essence:** Implement a stable-compatible essence extractor to
      pull key sentences from READMEs.

### Phase 2: The Intuition Interface (`bin/toad`)

- [x] **Token Budgeting:** Formatted output as a Markdown table optimized for AI
      skimming.
- [x] **Manifest Generation:** `toad manifest` creates `shadows/MANIFEST.md`.
- [x] **Whitelist:** `shadows/` directory is created at the root and ignored by
      Git per user preference.

### Phase 3: Agentic & Local Harvesting (Future Expansion)

- [ ] **Digest & Delta Caching:** Store README hashes to skip redundant
      analysis.
- [ ] **Agentic Refinement:** Use Gemini for high-level semantic token
      generation.

## Success Criteria

- [x] Gemini can answer "Which projects in my workspace use [X] technology?"
      using ONLY the manifest.
- [x] The manifest stays updated with a single command.
- [x] Zero-redundancy: Command is fast and stable-compatible.

## Progress Notes

- [2026-02-04] Work Order Completed. Manifest moved to root `shadows/` and set
  to ignored. System successfully installed.
