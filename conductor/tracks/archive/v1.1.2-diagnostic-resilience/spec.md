# Specification: Diagnostic Resilience (v1.1.2-diagnostic-resilience)

## Overview

Improve Toad's ability to detect and report "silent failures" caused by
malformed or unparsable project metadata (e.g., `Cargo.toml`, `package.json`).

## Goals

1. **Detect Malformed Files** - Identify when a project has a metadata file that
   matches a strategy but fails to parse.
2. **Enhanced Diagnostics** - Update `toad doctor` to report these parsing
   errors as warnings or critical issues.
3. **Status Integration** - Show "Malformed Metadata" warnings in `toad status`.

## Implementation Status

### ✅ Completed

#### Diagnostic Type System

Created comprehensive diagnostic types in `toad-core`:

- `ParseDiagnostic` - Tracks metadata parsing errors with severity levels
- `DiagnosticSeverity` - Enum: Error, Warning, Info
- `DiagnosticReport` - Collection of diagnostics with helper methods

**Files Created:**

- `crates/toad-core/src/models/diagnostics.rs` - New diagnostic types

**Files Modified:**

- `crates/toad-core/src/models/mod.rs` - Export diagnostic types
- `crates/toad-core/src/lib.rs` - Re-export from crate root

### 🚧 Remaining Work

1. **Update toad-discovery scanner** - Modify `get_project_metadata()` to
   attempt parsing `Cargo.toml`, `package.json`, etc. and capture parse errors
   as diagnostics
2. **Enhance toad doctor** - Add diagnostic reporting section showing malformed
   metadata files
3. **Update toad status** - Display warning indicators for projects with parse
   errors
4. **Add to health check** - Include diagnostic count in `toad doctor` health
   score
5. **Testing** - Create test projects with malformed metadata to verify
   detection

## Implementation Details

- Update `toad-discovery` to return `Result` or `Diagnostic` objects during
  stack detection. ✅ (types created)
- Modify `toad doctor` to aggregate these diagnostics. 🚧
- Add visual indicators in `toad status` for projects with metadata issues. 🚧
