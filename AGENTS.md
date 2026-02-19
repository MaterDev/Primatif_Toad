# Toad Development Agent Instructions

**Always reference these essential context files before working on any task:**

- @.gemini/GEMINI.md - Core AI agent instructions and workflow rules
- @conductor/index.md - Project architecture and component overview
- @.gemini/toad-blueprint.md - Toad system architecture and DNA patterns
- @.gemini/toad-cli.md - CLI command reference and usage

## Track Naming Convention

All conductor tracks use **version-based naming** to signal release targets:

**Format:** `conductor/tracks/v{MAJOR}.{MINOR}.{PATCH}-{slug}/`

- Version indicates the target release
- Completing all tracks for a version signals readiness for that version bump
- Post-release improvements use current version (no bump)

**Examples:**

- `v1.1.2-interface-standardization/` → New features for v1.1.2
- `v1.1.1-dogfooding/` → Post-release workflow (no version bump)
