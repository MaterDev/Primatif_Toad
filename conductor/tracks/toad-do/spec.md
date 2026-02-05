# Spec: Toad Do

## Overview

The `do` command enables bulk maintenance and operations across a subset of
projects defined by a search query.

## Requirements

- **Query Filter:** Must use the same query logic as `reveal` and `status`.
- **Safety:** Should clearly announce which projects will be affected before
  execution (or provide a dry-run).
- **Isolation:** Each command must run with the project directory as its current
  working directory.
- **Reporting:** Must summarize successes and failures at the end of the batch.

## Design

1. Identify target projects via `toad-discovery` and query.
2. Iterate through project paths.
3. For each project:
   - Set CWD to project path.
   - Execute the user-provided shell command.
   - Capture and store stdout/stderr/exit code.
4. Display a tabular or grouped report of the outcomes.
