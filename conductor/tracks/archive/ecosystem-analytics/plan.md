# Track-011: Ecosystem Analytics (Insights)

## Status

- **Priority:** Medium
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** Pending

## Objective

Implement a high-performance analytics engine that provides visual and
data-driven insights into ecosystem disk usage and project health.

## Deliverables

- [x] **Infrastructure & Setup** [commit: bfcb9ac]
  - [x] Add `walkdir` to the workspace. [commit: bfcb9ac]
  - [x] Initialize `toad-ops::stats` module. [commit: bfcb9ac]
- [x] **Core Analytics Logic** [commit: bfcb9ac]
  - [x] Implement recursive directory size summation. [commit: bfcb9ac]
  - [x] Implement tech-stack aware artifact detection (to compute Bloat Index).
        [commit: bfcb9ac]
  - [x] Integrate parallel iteration via `rayon`. [commit: bfcb9ac]
- [x] **Visual UX (The Heatmap)** [commit: bfcb9ac]
  - [x] Implement ASCII bar chart generator. [commit: bfcb9ac]
  - [x] Add color-coding based on size thresholds. [commit: bfcb9ac]
  - [x] Create `toad stats` summary and detailed view. [commit: bfcb9ac]
- [ ] **Insights Engine** (Deferred to ROI Track)
  - [ ] Cross-reference size with `ActivityTier` to flag "Dead Weight".
  - [ ] Implement smart caching using project fingerprints.
- [x] **Verification** [commit: bfcb9ac]
  - [x] Benchmark against 100+ mock projects. [commit: bfcb9ac]
  - [x] Perform SemVer bump. [commit: bfcb9ac]
