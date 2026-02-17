# Specification: Analytics Refinements (111-analytics-refinements)

## Overview

Address technical debt and performance bottlenecks identified during the code review of the initial analytics implementation. This track focuses on moving from naive heuristics to robust, standard-based analysis.

## Objectives

1.  **Robust Dependency Analysis** — Replace string matching in `Cargo.toml` with `cargo_metadata` for accurate relationship mapping.
2.  **Cycle Detection** — Implement circular dependency detection in the project graph.
3.  **Accuracy Improvements** — Refine technical debt scanning to avoid false positives.
4.  **Performance Optimization** — Streamline data collection where possible.

---

## Technical Details

### 1. Robust Dependencies
Currently, `analyze_dependencies` looks for `project_name =` strings. This fails for:
- Workspace-level dependencies
- Version overrides
- Different casing or spacing
- Non-Rust projects (partially handled, but needs clarity)

**Fix:** Use the `cargo_metadata` crate to extract the actual dependency tree for Rust projects.

### 2. Circular Dependency Detection
Implement a standard DFS-based cycle detection algorithm to populate the `circular_dependencies` field in `DependencyGraph`.

### 3. Technical Debt Scanning
Ensure the `ignore` walker is correctly configured to skip non-text files and binary blobs to prevent "TODO" false positives in compiled artifacts or images.

---

## Success Criteria

- [ ] `toad analyze deps` accurately shows internal dependencies in complex workspaces.
- [ ] Circular dependencies are detected and reported if they exist.
- [ ] No regressions in build time or CLI responsiveness.
- [ ] Code review "Opportunities for Improvement" are addressed.
