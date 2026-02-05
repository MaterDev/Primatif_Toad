# Track-002: Toad Do (Bulk Operations)

## Status

- **Priority:** High

- **Owner:** Gemini (Solo-Dev Support)

- **Status:** Completed

## Objective

Implement a `do` command for the Toad CLI that allows executing a shell command
across multiple projects matching a query.

## Deliverables

- [x] Logic to execute shell commands in project directories. [commit: 0178bd5]

- [x] Integration into `bin/toad` CLI (`toad do <command> --query <str>`).
      [commit: 48b347c]

- [ ] Parallel execution support (optional/future).

- [x] Visual feedback for batch operation results. [commit: 48b347c]

- [x] Unit tests for command execution logic. [commit: 0178bd5]

- [x] Integration tests for CLI command. [commit: 1c34bb1]
