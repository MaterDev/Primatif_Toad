# Specification: Submodule Awareness (v1.0.2 Phase 4)

> **Source of Truth:** This track is derived from the release planning
> documents. All design decisions and task details live there.
>
> - Design: `docs/releases/v1.0.2/evolution.md` § Submodule-Aware Ecosystem Management
> - Tasks: `docs/releases/v1.0.2/tasks.md` § Phase 4

## Overview
Enable Toad to automatically discover and report the status of Git submodules within managed projects, treating them as first-class entities.

## Requirements
- Parse `.gitmodules` files during project scanning.
- Report submodule initialization state and VCS health.
- Display submodules as child entities in `toad status`.

## References
- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Submodule-Aware Ecosystem Management
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 4
