# Specification: Licensing Migration (v1.0.2 Phase 0)

> **Source of Truth:** This track is derived from the release planning
> documents. All design decisions and task details live there.
>
> - Design: `docs/releases/v1.0.2/evolution.md` § The Licensing Strategy
> - Tasks: `docs/releases/v1.0.2/tasks.md` § Phase 0

## Overview
Transition the project to an "Open Core" licensing model by splitting the monolithic MIT license into MIT (Open contracts) and BUSL-1.1 (Source-available intelligence).

## Requirements
- Each crate must have its own `LICENSE` file.
- `Cargo.toml` license fields must match the designated tier.
- Existing source files must have license notice headers added.
- Hardcoded Git operations outside of `toad-git` must be migrated.
- Hard gates for license boundary enforcement must be implemented.

## Boundary Rules
- **MIT:** `toad-core`, `scaffold`, `bin/toad`.
- **BUSL-1.1:** `discovery`, `toad-git`, `toad-manifest`, `toad-ops`.

## References
- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § The Licensing Strategy, § Governance
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 0
