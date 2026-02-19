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

## Implementation Details

- Update `toad-discovery` to return `Result` or `Diagnostic` objects during
  stack detection.
- Modify `toad doctor` to aggregate these diagnostics.
