# v1.1.2 Interface Standardization - Progress Report

**Track:** `v1.1.2-interface-standardization`\
**Status:** 🟡 In Progress (70% Complete)\
**Started:** 2026-02-19

## Summary

Implementing unified `--yes` flags across all destructive CLI commands and
building a command suggestion engine to improve CLI discoverability.

## Completed Work

### ✅ Phase 1: Unified Non-Interactive Flags

All destructive/interactive commands now support `--yes` / `-y` flag:

#### Custom Workflows

- `toad cw delete <name> --yes` - Remove workflow without confirmation

#### Strategy Management

- `toad strategy remove <name> --yes` - Remove custom strategy without
  confirmation

#### Multi-Repo Git Operations

- `toad ggit commit -m "msg" --yes` - Commit across repos without confirmation
- `toad ggit push --yes` - Push changes without confirmation
- `toad ggit pull --yes` - Pull changes without confirmation
- `toad ggit checkout <branch> --yes` - Switch branches without confirmation
- `toad ggit align --yes` - Force-align submodules without confirmation

**Implementation Pattern:**

```rust
if !yes {
    use std::io::{self, Write};
    print!("Confirm action? [y/N]: ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if !input.trim().to_lowercase().starts_with('y') {
        println!("Aborted.");
        return Ok(());
    }
}
```

### ✅ Phase 2: Command Suggestion Infrastructure

Created `bin/toad/src/suggestions.rs` with:

1. **Alias System** - Common command aliases:
   - `unregister`, `remove`, `rm`, `del` → `delete`
   - `ls` → `list`
   - `search`, `find` → `reveal`
   - `check` → `status`
   - `health`, `diagnose` → `doctor`
   - `cleanup`, `purge` → `clean`
   - `git` → `ggit`
   - `workflow`, `workflows` → `cw`

2. **Levenshtein Distance Algorithm** - Typo detection with max distance of 2
3. **Suggestion Function** - Returns "Did you mean 'X'?" for close matches
4. **Test Coverage** - Unit tests for distance calculation and suggestions

## Remaining Work

### 🚧 Phase 3: Integration (30%)

1. **Hook suggestion engine into main.rs**
   - Catch unknown command errors from clap
   - Call `suggest_command()` with valid command list
   - Display suggestion before exiting

2. **Error Message Standardization**
   - Update error messages to mention `--yes` flag when confirmation fails
   - Add helpful hints: "Use --yes to skip confirmation prompts"

3. **Testing & Validation**
   - Test all `--yes` flags in CI/CD workflows
   - Verify suggestion engine with common typos
   - Update documentation with new flags

## Files Modified

- `bin/toad/src/cli.rs` - Added `--yes` flags to 7 commands
- `bin/toad/src/commands/cw.rs` - Confirmation prompt for delete
- `bin/toad/src/commands/strategy.rs` - Confirmation prompt for remove
- `bin/toad/src/commands/ggit.rs` - Confirmation prompts for 5 git operations
- `bin/toad/src/main.rs` - Added suggestions module import

## Files Created

- `bin/toad/src/suggestions.rs` - Command suggestion engine (130 lines)

## Dogfooding Notes

### ✅ What Worked Well

- Toad's own MCP server (`mcp3_get_project_detail`) provided instant context
- Build completed successfully with only minor warnings (unused functions)
- Pattern consistency across all confirmation prompts

### 🐛 Issues Found

1. **Dead code warnings** - Suggestion functions not yet integrated (expected)
2. **Markdown lint warnings** - Missing blank lines around headings in specs

### 💡 Improvement Opportunities

1. **Suggestion integration** - Should be completed to eliminate dead code
   warnings
2. **Global `--yes` flag** - Consider adding global flag that applies to all
   subcommands
3. **Dry-run mode** - Some commands have `--dry-run`, others don't -
   standardize?

## Next Steps

1. Complete suggestion engine integration
2. Test all `--yes` flags with actual workflows
3. Update USER_GUIDE.md with new flags
4. Mark track as complete when all tests pass
