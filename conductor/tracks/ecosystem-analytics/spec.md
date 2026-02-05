# Spec: Ecosystem Analytics (Insights)

## Overview

As the project count grows, it becomes difficult to track which projects are
consuming the most resources. This module provides data-driven insights and
visual health indicators for the managed ecosystem, helping solo developers
identify bloat and prioritize maintenance.

## Requirements

1. **High-Speed Disk Audit:** Calculate total disk size of every project in
   `projects/` using multi-threaded recursive summation.
2. **The "Atari" Heatmap:** Display a visual horizontal bar chart using block
   characters (`■`), color-coded by size thresholds (Green < 200MB, Yellow <
   1GB, Red > 1GB).
3. **The "Bloat Index":** Distinguish between source code and build artifacts
   (e.g., `target/`, `node_modules/`). Calculate the percentage of "bloat" for
   each project.
4. **"Dead Weight" Detection:** Cross-reference project size with **Activity
   Tiers** to identify large projects that haven't been touched in 30+ days.
5. **Smart Caching:** Reuse the deep fingerprinting logic to skip re-scanning
   projects that haven't changed since the last analytics run.
6. **Human-Readable Output:** Automatic unit scaling (B, KB, MB, GB, TB).

## Design

- **Crate:** Enhance `toad-ops` with a `stats` module or create
  `crates/toad-stats`.
- **Parallelism:** Use `rayon` to scale the audit across all cores.
- **File Traversal:** Use `walkdir` for efficient recursive size calculation.
- **UI:** Utilize `indicatif` for progress and `colored` for the heatmap.
- **Persistence:** Store results in `shadows/stats.json` to allow for "Growth
  Over Time" analytics in the future.
