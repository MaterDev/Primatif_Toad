# Specification: Group-Git Orchestration (v1.0.2 Phase 5)

> **Source of Truth:** This track is derived from the release planning
> documents. All design decisions and task details live there.
>
> - Design: `docs/releases/v1.0.2/evolution.md` § Multi-Repo Git Orchestration
> - Tasks: `docs/releases/v1.0.2/tasks.md` § Phase 5

## Overview

Implement first-class, structured git operations (`toad ggit`) for managing
multi-repo ecosystems. This includes branch group tracking and ghost commit
prevention.

## Requirements

- `toad ggit` namespace for branch, status, commit, push, pull, sync, log, and
  diff operations.
- All commands accept `--project <name>` for single-repo focus mode.
- Central `run_git()` helper in `toad-git` — all git operations go through it.
- Consolidated output: default one-line-per-repo summaries, `--verbose` for full
  raw output.
- Pre-flight checks to prevent pushing parent pointers to un-pushed submodule
  commits (ghost commit prevention).
- Branch group matrix view with alignment and merge-status tracking.
- Branch lifecycle: create, delete, merge-status across all targeted repos.
- `toad ggit sync` updates parent submodule refs after verifying all submodules
  are clean and pushed.

## References

- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Multi-Repo Git
  Orchestration
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 5
