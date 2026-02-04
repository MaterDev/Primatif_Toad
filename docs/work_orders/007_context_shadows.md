# Work Order 007: Context Shadows (Project Manifest)

**Status: In-Progress (Phase 2: Technical & Contextual Refinement)**

## Motivation

To provide Gemini CLI with "Peripheral Vision" and "Semantic Intuition" over the
`projects/` directory. By creating a high-density manifest of all managed
projects, the AI can cross-reference patterns and understand the broader
workspace without expensive directory traversal.

## Implementation Plan

### Phase 1: Structural Discovery (Completed)

- [x] **Metadata Model:** Implement `ProjectDetail` with basic tech stack
      detection.
- [x] **Manifest Generation:** `toad manifest` command creates
      `shadows/MANIFEST.md`.

### Phase 2: Technical Refinement & "Trust" (Current)

- [ ] **Lazy Fingerprint (Context Health):**
  - [ ] Store the `projects/` root `mtime` in the manifest header.
  - [ ] Implement a lightweight check at CLI startup to warn the user if the
        manifest is "Stale."
  - [ ] Add an auto-sync hook to `toad create` to keep the fingerprint current.
- [ ] **Activity Tiers & VCS Awareness:**
  - [ ] **Activity:** Categorize projects as `[Active/Cold/Archive]` based on
        last modification time.
  - [ ] **VCS Status:** Add a column indicating if a project has uncommitted
        changes (`[Dirty]`/`[Clean]`).
- [ ] **Expanded Essence (v2):**
  - [ ] Increase extraction to 10 lines (max 600 characters).
  - [ ] **Semantic Filtering:** Exclude images (`![]`), badges, raw HTML, and
        link blocks.
  - [ ] Preserve headers for structural clarity in the summary.

### Phase 3: Semantic Hashtags (`toad harvest`)

- [ ] **Hashtag Logic:**
  - [ ] **Procedural Fallback:** Generate instant tags based on stack and file
        signatures (`#rust`, `#wails`, etc.).
  - [ ] **Agentic Harvest:** Use Gemini to analyze the "Expanded Essence" and
        generate deep concept tags.
- [ ] **Digest & Delta Cache:**
  - [ ] Implement `shadows/cache.json` using README hashes to skip redundant
        analysis.

## Success Criteria

- The manifest provides a "Living Dashboard" feel with Activity and VCS status.
- AI intuition is guarded by a Fingerprint that prevents operating on stale
  data.
- The "Essence" is a clean, narrative-focused paragraph (no noise).

## Progress Notes

- [2026-02-04] Completed Phase 1. Refined Phase 2 to focus on "Trust"
  (Fingerprinting) and "Dashboarding" (Activity/VCS).
