# Spec: Toad Status

## Overview

The `status` command allows a developer to instantly see which of their managed
projects need attention (e.g., have pending commits).

## Requirements

- **Efficiency:** Must scan 70+ projects quickly (consider parallelism or
  caching).
- **Accuracy:** Must detect modified files, staged files, and untracked files.
- **Filtering:** Use the existing query logic to filter which projects to check.

## Design

1. Identify all projects via `toad-discovery`.
2. Iterate through project directories.
3. Execute `git status --porcelain`.
4. **UX Optimization:**
   - **Summary View:** If a project is clean, compress it into a summary count (e.g., "70/72 Clean").
   - **Dirty Promotion:** Display detailed status ONLY for projects with changes.
   - **Sorting:** Sort "Dirty" projects by modification time (most recent first).
5. Parse results and format into a "Toad Green" report.
