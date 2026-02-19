# Specification: Interface Standardization & Command Discovery (v1.1.2-interface-standardization)

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

## Implementation Status

### ✅ Completed

#### Unified `--yes` Flags

All destructive commands now support `--yes` / `-y` for non-interactive
execution:

- `toad cw delete --yes` - Remove workflow without confirmation
- `toad strategy remove --yes` - Remove strategy without confirmation
- `toad ggit commit --yes` - Commit across repos without confirmation
- `toad ggit push --yes` - Push across repos without confirmation
- `toad ggit pull --yes` - Pull across repos without confirmation
- `toad ggit checkout --yes` - Checkout branch without confirmation
- `toad ggit align --yes` - Force-align submodules without confirmation

**Files Modified:**

- `bin/toad/src/cli.rs` - Added `--yes` flags to CLI definitions
- `bin/toad/src/commands/cw.rs` - Implemented confirmation prompt
- `bin/toad/src/commands/strategy.rs` - Implemented confirmation prompt
- `bin/toad/src/commands/ggit.rs` - Implemented confirmation prompts for all git
  operations

#### Command Suggestion Infrastructure

Created suggestion engine with Levenshtein distance algorithm:

- `bin/toad/src/suggestions.rs` - New module with alias mapping and typo
  detection
- Supports common aliases: `unregister`→`delete`, `ls`→`list`,
  `search`→`reveal`, etc.
- Levenshtein distance-based "did you mean" suggestions (max distance: 2)

**Files Created:**

- `bin/toad/src/suggestions.rs` - Command suggestion engine with tests

### 🚧 Remaining Work

1. **Integrate suggestion engine into main.rs** - Hook up the suggestion system
   to handle unknown commands
2. **Error message standardization** - Update error messages to suggest `--yes`
   flag when operations fail due to missing confirmation
3. **Testing** - Verify all `--yes` flags work correctly in CI/CD workflows

## Implementation Details

- **Zero-Intervention Mandate**: All destructive or state-changing operations
  MUST provide a non-interactive path via flags (e.g., `--yes`, `--force`).
- Update `clap` definitions across all command modules. ✅
- Centralize flag handling where possible. ✅
- Implement a suggestion engine or alias map in `bin/toad/src/cli.rs`. ✅ (in
  suggestions.rs)
