# Spec: Macro Tagging & Global Context (Anchor System)

## Overview

Toad is a meta-engineering tool that must be accessible from any directory. This
track implements a "Global Anchor" system using a home configuration file and a
root marker, allowing the CLI to find its managed projects and metadata
regardless of the user's current location. It also adds macro-level tagging for
bulk ecosystem management.

## Requirements

1. **Workspace Marker (`.toad-root`):**
   - A file used to identify the root of a Toad Control Plane workspace.
2. **Global Anchor (`~/.toad/config.json`):**
   - Stores the absolute (canonicalized) path to the active Toad workspace.
   - Automatically initializes to the current directory (if a `.toad-root` is
     found) upon first run.
3. **Command: `toad home [path]`:**
   - **View:** Run without args to show current home pointer and status.
   - **Set:** Run with a path (relative or absolute) to update the global
     pointer.
   - **Validation:** Verify the target directory contains a `.toad-root` marker
     before setting.

4. **Three-Tier Discovery Priority:**
   1. **Environment:** `TOAD_ROOT` variable (Manual override).
   2. **Local:** Upward search for `.toad-root` (Context-aware).
   3. **Global:** `~/.toad/config.json` (System-wide default).

5. **Macro Tagging:**
   - Bulk assign/remove tags using query and stack filters.
   - Safety preflight visualization for all bulk metadata changes.

## Design

- **Core:** Add `GlobalConfig` struct to `toad-core` using the `dirs` crate for
  cross-platform home directory access.
- **Core:** Refactor `Workspace::new()` to `Workspace::discover()` implementing
  the 3-tier logic with `fs::canonicalize`.
- **CLI:** Add `toad home` command for pointer management and validation.
- **CLI:** Expand `toad tag` with macro flags (`--query`, `--stack`, `--all`).
