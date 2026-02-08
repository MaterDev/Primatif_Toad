# Specification: Custom Workflows (v1.0.2 Phase 7)

> **Source of Truth:** This track is derived from the release planning
> documents. All design decisions and task details live there.
>
> - Design: `docs/releases/v1.0.2/evolution.md` § Custom Workflows
> - Tasks: `docs/releases/v1.0.2/tasks.md` § Phase 7

## Overview

Enable users to extend Toad with project-specific shell scripts registered as
first-class subcommands (`toad cw`).

## Requirements

- Register, list, run, update, and delete custom shell scripts.
- Prevent namespace collisions with built-in Toad commands.
- Reserved namespace list maintained **centrally in `toad-ops`** via
  `reserved_command_names()` — derived from `bin/toad`'s command definitions,
  not a static list.
- Unit test enforces sync between `reserved_command_names()` and the actual
  `Commands` enum.
- Track execution history (last run, exit code).
- Global scope: workflows registered per-user in
  `~/.toad/custom_workflows.json`, not per-context.
- Shell-only (`.sh`), must be executable.

## References

- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Custom
  Workflows
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 7
