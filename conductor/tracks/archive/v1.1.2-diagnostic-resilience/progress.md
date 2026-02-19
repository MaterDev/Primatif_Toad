# v1.1.2 Diagnostic Resilience - Progress Report

**Track:** `v1.1.2-diagnostic-resilience`\
**Status:** 🟡 In Progress (40% Complete)\
**Started:** 2026-02-19

## Summary

Building diagnostic infrastructure to detect and report malformed project
metadata files (Cargo.toml, package.json, etc.) that cause silent failures
during project scanning.

## Completed Work

### ✅ Phase 1: Diagnostic Type System

Created comprehensive diagnostic types in `toad-core`:

#### New Types

**`ParseDiagnostic`** - Tracks individual parsing errors:

```rust
pub struct ParseDiagnostic {
    pub project_name: String,
    pub project_path: PathBuf,
    pub file_name: String,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub details: Option<String>,
}
```

**`DiagnosticSeverity`** - Error classification:

```rust
pub enum DiagnosticSeverity {
    Error,   // Critical issue preventing functionality
    Warning, // Should be addressed but not breaking
    Info,    // Informational message
}
```

**`DiagnosticReport`** - Collection with helper methods:

```rust
pub struct DiagnosticReport {
    pub diagnostics: Vec<ParseDiagnostic>,
}

impl DiagnosticReport {
    pub fn has_errors(&self) -> bool
    pub fn has_warnings(&self) -> bool
    pub fn error_count(&self) -> usize
    pub fn warning_count(&self) -> usize
    pub fn merge(&mut self, other: DiagnosticReport)
}
```

#### Builder Pattern Support

```rust
ParseDiagnostic::error(name, path, file, message)
    .with_details(error_string)
```

## Remaining Work

### 🚧 Phase 2: Detection Logic (60%)

1. **Update `toad-discovery/src/scanner.rs`**
   - Modify `get_project_metadata()` to attempt parsing metadata files
   - For Rust projects: Try parsing `Cargo.toml` with `toml` crate
   - For Node projects: Try parsing `package.json` with `serde_json`
   - Capture parse errors as `ParseDiagnostic` objects
   - Return `DiagnosticReport` alongside metadata

2. **Update `toad-discovery/src/detection.rs`**
   - Add `detect_metadata_issues()` function
   - Check for common malformations:
     - Invalid TOML syntax
     - Invalid JSON syntax
     - Missing required fields
     - Type mismatches

3. **Enhance `bin/toad/src/commands/doctor.rs`**
   - Add new diagnostic section to health check
   - Display projects with malformed metadata
   - Show severity, file name, and error message
   - Include diagnostic count in summary

4. **Update `bin/toad/src/commands/status.rs`**
   - Add warning indicator (⚠️) for projects with parse errors
   - Show diagnostic count in project listing
   - Add `--show-diagnostics` flag for detailed view

5. **Update `toad-ops/src/doctor.rs`**
   - Include diagnostic report in `HealthCheckReport`
   - Add `malformed_metadata_count` field
   - Mark as warning if diagnostics exist

## Files Modified

- `crates/toad-core/src/models/mod.rs` - Added diagnostics module
- `crates/toad-core/src/lib.rs` - Exported diagnostic types

## Files Created

- `crates/toad-core/src/models/diagnostics.rs` - Diagnostic type system (130
  lines)

## Dogfooding Notes

### ✅ What Worked Well

- Type system design is clean and extensible
- Builder pattern makes diagnostic creation ergonomic
- Severity levels provide clear classification

### 🐛 Issues Found

None yet - types compile cleanly

### 💡 Improvement Opportunities

1. **Diagnostic Codes** - Consider adding error codes (e.g., `TOML001`,
   `JSON002`) for documentation
2. **Fix Suggestions** - Could include suggested fixes in diagnostics
3. **Batch Validation** - Add `toad validate` command to check all projects
4. **CI Integration** - Diagnostics could fail CI if errors exist

## Implementation Strategy

### Detection Approach

```rust
// In get_project_metadata()
let mut diagnostics = DiagnosticReport::new();

if path.join("Cargo.toml").exists() {
    match fs::read_to_string(path.join("Cargo.toml")) {
        Ok(content) => {
            if let Err(e) = toml::from_str::<toml::Value>(&content) {
                diagnostics.add(ParseDiagnostic::error(
                    name.clone(),
                    path.clone(),
                    "Cargo.toml".to_string(),
                    "Failed to parse Cargo.toml".to_string(),
                ).with_details(e.to_string()));
            }
        }
        Err(e) => { /* file read error */ }
    }
}
```

### Doctor Integration

```rust
// In doctor.rs
if !report.diagnostics.is_empty() {
    println!("\n{} Metadata Issues", "»".blue());
    for diag in &report.diagnostics {
        let icon = match diag.severity {
            DiagnosticSeverity::Error => "❌",
            DiagnosticSeverity::Warning => "⚠️",
            DiagnosticSeverity::Info => "ℹ️",
        };
        println!("  {} {} - {}: {}", 
            icon, diag.project_name, diag.file_name, diag.message);
    }
}
```

## Next Steps

1. Implement detection logic in toad-discovery
2. Add diagnostic reporting to toad doctor
3. Update toad status with warning indicators
4. Create test projects with malformed metadata
5. Verify detection works correctly
6. Update documentation
