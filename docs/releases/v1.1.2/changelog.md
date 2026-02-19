# v1.1.2 Changelog

**Release Date:** 2026-02-19  
**Codename:** —

## Overview

Minor release focused on CLI usability improvements and metadata health monitoring.

---

## Added

### Interface Standardization

#### Non-Interactive Execution Support

- Added `--yes` / `-y` flags to all destructive commands for CI/CD workflows
  - `toad cw delete <name> --yes` - Remove workflow without confirmation
  - `toad strategy remove <name> --yes` - Remove strategy without confirmation
  - `toad ggit commit -m "msg" --yes` - Commit across repos without confirmation
  - `toad ggit push --yes` - Push across repos without confirmation
  - `toad ggit pull --yes` - Pull across repos without confirmation
  - `toad ggit checkout <branch> --yes` - Checkout branch without confirmation
  - `toad ggit align --yes` - Force-align submodules without confirmation

#### Command Discovery & Suggestions

- Command alias system with 15+ common aliases
  - `ls` → `list`
  - `search`, `find` → `reveal`
  - `check` → `status`
  - `health`, `diagnose` → `doctor`
  - `cleanup`, `purge` → `clean`
  - `git` → `ggit`
  - `workflow`, `workflows` → `cw`
  - `unregister`, `remove`, `rm`, `del` → `delete`
- "Did you mean?" suggestions using Levenshtein distance algorithm
  - Automatically suggests correct commands for typos
  - Maximum edit distance of 2 for suggestions
  - Integrated into main CLI error handling

### Diagnostic Resilience

#### Metadata Health Monitoring

- Automatic detection of malformed project metadata files
  - Validates `Cargo.toml` syntax for Rust projects
  - Validates `package.json` syntax for Node projects
  - Captures detailed parse error messages
- Diagnostic reporting in `toad doctor`
  - New "Metadata Issues" section with detailed errors
  - Shows file name, error type, and specific error details
  - Severity indicators: ❌ Error, ⚠️ Warning, ℹ️ Info
- Warning indicators in `toad status`
  - Projects with metadata issues show ⚠️ symbol
  - Inline diagnostic details displayed under each project
  - Summary section shows total diagnostic count
  - Directs users to `toad doctor` for full details

---

## Changed

- Enhanced `toad doctor` output with metadata diagnostics section
- Updated `toad status` to display inline diagnostic warnings
- Improved error messages to mention `--yes` flag when operations are aborted
  - All confirmation prompts now display: "Aborted. (Use --yes to skip confirmation)"

---

## Documentation

- Added "Non-Interactive Execution" section to USER_GUIDE.md
  - Documented all `--yes` flags with examples
  - Listed all command aliases
  - Included "Did You Mean?" examples
- Added "Metadata Health Monitoring" section to USER_GUIDE.md
  - Documented diagnostic detection for Rust and Node projects
  - Included examples of diagnostic output in `toad doctor` and `toad status`
  - Documented severity levels and interpretation guide

---

## Technical Details

### New Types (toad-core)

```rust
pub struct ParseDiagnostic {
    pub project_name: String,
    pub project_path: PathBuf,
    pub file_name: String,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub details: Option<String>,
}

pub enum DiagnosticSeverity {
    Error,   // Critical issues
    Warning, // Should be addressed
    Info,    // Informational
}

pub struct DiagnosticReport {
    pub diagnostics: Vec<ParseDiagnostic>,
}
```

### New Functions (toad-discovery)

- `detect_metadata_issues(path: &Path, project_name: &str) -> DiagnosticReport`
  - Validates Cargo.toml and package.json
  - Returns structured diagnostic report

### Modified Commands

- `toad status` - Now collects and displays diagnostics
- `toad doctor` - Enhanced with diagnostic reporting section

---

## Files Modified

### Created

- `bin/toad/src/suggestions.rs` - Command suggestion engine
- `crates/toad-core/src/models/diagnostics.rs` - Diagnostic types

### Modified

- `bin/toad/src/cli.rs` - Added `--yes` flags
- `bin/toad/src/main.rs` - Integrated suggestion engine
- `bin/toad/src/commands/cw.rs` - Confirmation prompts
- `bin/toad/src/commands/strategy.rs` - Confirmation prompts
- `bin/toad/src/commands/ggit.rs` - 5 confirmation prompts
- `bin/toad/src/commands/status.rs` - Diagnostic collection
- `bin/toad/src/commands/doctor.rs` - Diagnostic display
- `bin/toad/src/ui.rs` - Status formatting with diagnostics
- `crates/toad-core/src/models/mod.rs` - Export diagnostics
- `crates/toad-core/src/lib.rs` - Re-export diagnostics
- `crates/toad-discovery/src/scanner.rs` - Detection logic
- `crates/toad-discovery/src/lib.rs` - Export detection function
- `crates/toad-discovery/Cargo.toml` - Added `toml = "0.8"` dependency
- `crates/toad-ops/src/doctor.rs` - Added diagnostics field
- `USER_GUIDE.md` - New sections for v1.1.2 features

---

## Conductor Tracks

This release completes two conductor tracks:

- **v1.1.2-interface-standardization** - CLI UX improvements
- **v1.1.2-diagnostic-resilience** - Enhanced error detection

Both tracks followed the conductor system workflow with proper planning, implementation, and documentation phases.
