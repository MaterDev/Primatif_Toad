# Specification: Interface Standardization & Command Discovery (112-interface-standardization)

## Overview

Standardize the CLI interface to improve predictability and discoverability.
This includes unified non-interactive flags and intelligent command suggestions.

## Goals

1. **Unified Flags** - Ensure all destructive or interactive commands support a
   standardized `--yes` or `--force` flag.
2. **Standardized Error Messages** - Update error messages to suggest the use of
   non-interactive flags when an operation fails due to missing input.
3. **Command Aliases & Suggestions** - Add aliases for common guesses (e.g.,
   `unregister` for `delete`) and implement "did you mean" logic for typos.

## Implementation Details
- **Zero-Intervention Mandate**: All destructive or state-changing operations MUST provide a non-interactive path via flags (e.g., `--yes`, `--force`).
- Update `clap` definitions across all command modules.
- Centralize flag handling where possible.
- Implement a suggestion engine or alias map in `bin/toad/src/cli.rs`.

