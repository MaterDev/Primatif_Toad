# Track-011: Ecosystem Analytics (Insights)

## Status

- **Priority:** Medium
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** Pending

## Objective

Implement a high-performance analytics engine that provides visual and
data-driven insights into ecosystem disk usage and project health.

## Deliverables

- [ ] **Infrastructure & Setup**
  - [ ] Add `walkdir` to the workspace.
  - [ ] Initialize `toad-ops::stats` module.
- [ ] **Core Analytics Logic**
  - [ ] Implement recursive directory size summation.
  - [ ] Implement tech-stack aware artifact detection (to compute Bloat Index).
  - [ ] Integrate parallel iteration via `rayon`.
- [ ] **Visual UX (The Heatmap)**
  - [ ] Implement ASCII bar chart generator.
  - [ ] Add color-coding based on size thresholds.
  - [ ] Create `toad stats` summary and detailed view.
- [ ] **Insights Engine**
  - [ ] Cross-reference size with `ActivityTier` to flag "Dead Weight".
  - [ ] Implement smart caching using project fingerprints.
- [ ] **Verification**
  - [ ] Benchmark against 100+ mock projects.
  - [ ] Perform SemVer bump.
