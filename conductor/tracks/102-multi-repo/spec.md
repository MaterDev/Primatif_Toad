# Specification: Multi-Repo Split (v1.0.2 Phase 1 & 2)

> **Source of Truth:** This track is derived from the release planning
> documents. All design decisions and task details live there.
>
> - Design: `docs/releases/v1.0.2/evolution.md` § Multi-Repo Architecture
> - Tasks: `docs/releases/v1.0.2/tasks.md` § Phase 1, Phase 2, Phase 3

## Overview
Extract each internal crate into its own independent Git repository and re-integrate them into the main workspace as Git submodules.

## Requirements
- Preserved commit history for each crate via `git-filter-repo`.
- Public GitHub repositories for all crates with descriptions and topic tags.
- Default branch set to `main` for all new repos.
- Functional Cargo workspace after submodule conversion.
- No source code in the main repo root `crates/` folder (only submodule pointers).
- First Hub commit must pin all submodules to their newly pushed `main` branch SHAs.
- Extracted repos verified: `git log`, `Cargo.toml`, `LICENSE`, `src/` all present.
- Final merge to `main`, tagged `v1.0.2`, pushed with `--recurse-submodules`.

## References
- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Multi-Repo Architecture, § History Preservation
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 1, Phase 2, Phase 3
