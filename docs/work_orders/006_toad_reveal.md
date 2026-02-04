# Work Order 006: Toad Reveal (Project Discovery)

**Date:** 2026-02-03 **Status:** Complete **Goal:** Implement
`toad reveal <QUERY>`, a command to find projects within the `projects/`
directory using case-insensitive search.

## Execution Log

- [x] Created `crates/discovery` library crate.
- [x] Implemented `find_projects` logic with case-insensitive filtering and
      30-result limit.
- [x] Integrated `discovery` into `bin/toad`.
- [x] Added `Reveal { query }` subcommand to the CLI.
- [x] Added unit tests for discovery logic.
- [x] Verified manual execution.
