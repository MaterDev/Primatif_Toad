# Specification: Dogfooding Improvements (111-dogfooding)

## Overview

Use Toad to improve Toad's own development workflow. Identify and implement features that would make developing Toad itself easier, faster, and more reliable. This is critical for long-term maintainability and developer experience.

## Sources

- **Current Workflow:** `Justfile`, `scripts/`, `conductor/`
- **Toad Features:** MCP, ggit, manifest, context, tags
- **Pre-release Review:** Final v1.1.0 check findings

---

## Problem Statement

**Opportunity:** Toad is not fully using its own features to improve its development workflow.

Current gaps:
- ❌ Toad doesn't use its own MCP server for development
- ❌ No tags on Toad's own submodules
- ❌ No custom workflows registered for common tasks
- ❌ Conductor tracks not integrated with Toad commands
- ❌ No automated context refresh in git hooks

**Opportunity:** By dogfooding Toad's features, we can identify UX issues and missing functionality that real users will encounter.

---

## Goals

1. **Use MCP for Development** — Configure Toad's MCP server for developing Toad
2. **Tag Submodules** — Apply taxonomy to Toad's own crates
3. **Register Workflows** — Convert common scripts to `toad cw` workflows
4. **Integrate Conductor** — Make conductor tracks discoverable via Toad
5. **Auto-Refresh Context** — Add git hooks for manifest updates

---

## Non-Goals

- Changing core Toad functionality (this is about using existing features)
- Adding new features (unless discovered through dogfooding)

---

## Architecture Decisions

### AD-1: MCP Configuration

Add Toad's own MCP server to Windsurf/Cursor config for developing Toad. This lets us:
- Query Toad's ecosystem while coding
- Test MCP tools in real development
- Identify UX issues with MCP

### AD-2: Taxonomy System

Apply tags to Toad's submodules:
- `#core` — toad-core (foundation)
- `#intelligence` — toad-discovery, toad-manifest
- `#orchestration` — toad-git, toad-ops
- `#interface` — bin/toad, bin/toad-mcp
- `#utility` — toad-scaffold

### AD-3: Custom Workflows

Register common development tasks as workflows:
- `toad cw run qa` → `just qa`
- `toad cw run release-check` → Pre-release verification
- `toad cw run sync-submodules` → `ggit sync` wrapper
- `toad cw run update-docs` → `just docs` + `toad skill sync`

### AD-4: Conductor Integration

Make conductor tracks discoverable:
- Add `toad track list` command (or use `toad reveal`)
- Add `toad track show <name>` command
- Generate track index in manifest

### AD-5: Git Hooks

Add post-commit hook to refresh context:
```bash
#!/bin/bash
# .git/hooks/post-commit
toad manifest --quiet
```

---

## Implementation Plan

### Phase 1: Apply Tags (5 min)

```bash
# Tag core infrastructure
toad tag toad-core --tag core --yes
toad tag toad-scaffold --tag utility --yes

# Tag intelligence layer
toad tag toad-discovery --tag intelligence --yes
toad tag toad-manifest --tag intelligence --yes

# Tag orchestration layer
toad tag toad-git --tag orchestration --yes
toad tag toad-ops --tag orchestration --yes

# Tag interface layer
toad tag Primatif_Toad --tag interface --yes
toad tag bin/toad-mcp --tag interface --yes

# Verify
toad status --tag core
toad status --tag intelligence
```

### Phase 2: Register Workflows (10 min)

**File:** `scripts/workflows/qa.sh`

```bash
#!/bin/bash
set -e
echo "🐸 Running full QA suite..."
just qa
echo "✅ QA Complete"
```

**File:** `scripts/workflows/release-check.sh`

```bash
#!/bin/bash
set -e
echo "🐸 Pre-release verification..."

# 1. Check all tests pass
echo "Running tests..."
cargo test --workspace --quiet

# 2. Check formatting
echo "Checking formatting..."
cargo fmt --all -- --check

# 3. Check clippy
echo "Running clippy..."
cargo clippy --workspace -- -D warnings

# 4. Check license boundaries
echo "Verifying license boundaries..."
./scripts/check_license_boundary.sh

# 5. Check submodule alignment
echo "Checking submodule alignment..."
toad ggit status | grep -q "Clean" || (echo "❌ Submodules not aligned" && exit 1)

# 6. Verify manifest is fresh
echo "Checking manifest freshness..."
toad manifest --check

echo "✅ Release check passed"
```

**File:** `scripts/workflows/update-docs.sh`

```bash
#!/bin/bash
set -e
echo "🐸 Updating documentation..."

# 1. Regenerate CLI docs
just docs

# 2. Sync AI skills
toad skill sync

# 3. Refresh manifest
toad manifest

echo "✅ Documentation updated"
```

Register workflows:

```bash
toad cw register qa ./scripts/workflows/qa.sh
toad cw register release-check ./scripts/workflows/release-check.sh
toad cw register update-docs ./scripts/workflows/update-docs.sh
```

### Phase 3: Add Git Hook (5 min)

**File:** `scripts/git-hooks/post-commit`

```bash
#!/bin/bash
# Auto-refresh Toad context after commits

# Only run if toad is installed
if command -v toad &> /dev/null; then
    # Run quietly to avoid noise
    toad manifest --quiet 2>/dev/null || true
fi
```

Update `Justfile` to install post-commit hook:

```makefile
setup-hooks:
    @mkdir -p .git/hooks
    @ln -sf ../../scripts/git-hooks/pre-commit .git/hooks/pre-commit
    @ln -sf ../../scripts/git-hooks/pre-push .git/hooks/pre-push
    @ln -sf ../../scripts/git-hooks/post-commit .git/hooks/post-commit
    @echo "✅ Git hooks installed."
```

### Phase 4: MCP Configuration (5 min)

**File:** `.windsurf/mcp_config.json` (or document in README)

```json
{
  "mcpServers": {
    "toad-dev": {
      "command": "toad-mcp",
      "env": {
        "TOAD_HOME": "/Users/jakehclark/Code/Primatif_Toad"
      }
    }
  }
}
```

### Phase 5: Conductor Integration (15 min)

Add track discovery to Toad:

**Option 1:** Use existing `toad reveal` command

```bash
# Search for tracks
toad reveal track

# This works because tracks are in conductor/tracks/
```

**Option 2:** Add dedicated command (future)

```rust
// bin/toad/src/cli.rs
Commands::Track {
    #[command(subcommand)]
    subcommand: TrackCommand,
}

enum TrackCommand {
    List,
    Show { name: String },
}
```

For now, document the pattern in README:

```markdown
## Development Workflow

### Finding Conductor Tracks

```bash
# List all tracks
ls conductor/tracks/

# Search for specific track
toad reveal 111-mcp

# View track spec
cat conductor/tracks/111-mcp-enhancements/spec.md
```
```

---

## Success Criteria

- [ ] All Toad submodules tagged appropriately
- [ ] Common workflows registered as `toad cw` commands
- [ ] Git hook auto-refreshes manifest
- [ ] MCP server configured for Toad development
- [ ] Conductor tracks discoverable via Toad commands
- [ ] Documentation updated with dogfooding examples

---

## Integration Points

- **Depends on:** Existing Toad features (tag, cw, manifest)
- **Consumed by:** Toad developers
- **Testing:** Use Toad to develop Toad for a week

---

## Benefits

1. **Identify UX Issues** — Using Toad daily reveals friction points
2. **Validate Features** — Proves features work in real workflows
3. **Improve Documentation** — Dogfooding exposes unclear docs
4. **Faster Development** — Automated workflows save time
5. **Better Context** — Auto-refresh keeps AI agents in sync

---

## Discovered Issues (Track as Findings)

During dogfooding, track any issues discovered:
- Missing features
- Confusing UX
- Performance problems
- Documentation gaps

These become future tracks or bug fixes.

---

## Future Enhancements

- `toad track` command for conductor integration
- Auto-tag on project creation
- Workflow templates
- Context-aware git commit messages
