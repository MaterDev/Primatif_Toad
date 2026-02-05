# Spec: Toad Clean (Disk Hygiene)

## Overview

Managed projects can accumulate gigabytes of build artifacts (Rust `target/`,
Node `node_modules/`). This command identifies and removes them from inactive or
specified projects.

## Requirements

- **Artifact Identification:** Detect common build artifact patterns (`target/`,
  `node_modules/`, `build/`, `dist/`).
- **Dry Run Support:** Must allow users to see how much space will be freed
  before deleting anything.
- **Filtering:** Use queries or activity tiers (e.g., "clean all cold projects")
  to target specific projects.
- **Safety:** Ensure core project files are never deleted.

## Design

1. Define a registry of artifact patterns per tech stack in `toad-ops`.
2. Iterate through projects matching the user query.
3. Calculate disk usage of target directories.
4. Remove directories and report total space reclaimed.
