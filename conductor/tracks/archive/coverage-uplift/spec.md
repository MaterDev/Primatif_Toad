# Spec: Coverage Uplift

## Overview

Primatif Toad mandates >80% code coverage. Current baseline (v0.3.0) is below
this for core orchestration and data models.

## Requirements

- Target: >80% total coverage across all platform crates.
- Priority areas: bin/toad (orchestration), toad-core (workspace logic).
- Exclusions: Externally managed projects in the `projects/` directory.

## Design

1. Identify uncovered branches in `bin/toad`.
2. Add integration tests for the CLI subcommands.
3. Add unit tests for `Workspace` methods in `toad-core`.
4. Verify results with `just coverage`.
