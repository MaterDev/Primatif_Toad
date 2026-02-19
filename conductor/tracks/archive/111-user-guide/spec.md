# Specification: User Guide Updates (111-user-guide)

## Overview

Update the user guide documentation to reflect v1.1.0 features, improve clarity,
and ensure new users can get started quickly with Toad's expanded capabilities.

## Sources

- **Existing Guide:** `conductor/USER_GUIDE.md` (if exists) or create new
- **CLI Guide:** `docs/guides/CLI.md`
- **MCP Guide:** `docs/guides/MCP.md`
- **Plugin Guide:** `docs/guides/PLUGINS.md`

---

## Problem Statement

User-facing documentation may be:

1. **Outdated** — Missing v1.1.0 features (MCP, DNA, context engineering)
2. **Incomplete** — No comprehensive getting-started guide
3. **Scattered** — Information spread across multiple docs

---

## Goals

1. **Create/Update USER_GUIDE.md** — Comprehensive guide for new users
2. **Update Existing Guides** — Ensure accuracy for v1.1.0
3. **Add Quick Start** — Get users productive in 5 minutes
4. **Document Workflows** — Common use cases and patterns

---

## Non-Goals

- API documentation (that's for developers)
- Deep architectural details (that's for conductor/)
- Tutorial videos (text-first approach)

---

## Architecture Decisions

### AD-1: Single Entry Point

Create `USER_GUIDE.md` at repo root as the primary entry point for users. It
links to specialized guides in `docs/guides/`.

### AD-2: Progressive Disclosure

Structure the guide in layers:

1. **Quick Start** — 5-minute setup
2. **Core Concepts** — Understanding Toad's model
3. **Common Workflows** — Practical examples
4. **Advanced Topics** — Power user features
5. **Reference** — Links to detailed guides

### AD-3: Keep Guides Focused

Each guide in `docs/guides/` covers one topic:

- `CLI.md` — Command reference
- `MCP.md` — MCP server setup and tools
- `PLUGINS.md` — Stack strategy system

---

## Implementation Plan

### Phase 1: Create USER_GUIDE.md (20 min)

**File:** `USER_GUIDE.md` (repo root)

`````markdown
# Toad User Guide

> **Version:** v1.1.0 "Deep Croak"

Toad is an AI-native ecosystem context oracle that provides zero-latency vision
across multi-repo codebases.

## Quick Start (5 minutes)

### 1. Install Toad

````bash
git clone https://github.com/Primatif/Primatif_Toad
cd Primatif_Toad
just setup
just install
```json
````
`````

### 2. Initialize Your Workspace

````bash
cd /path/to/your/projects
toad home .
```json

### 3. Generate Context

```bash
toad init-context
```json

### 4. Explore Your Ecosystem

```bash
toad status              # Health check
toad reveal <query>      # Find projects
toad stats               # Disk usage analytics
```json

## Core Concepts

### Workspace Discovery

Toad discovers your workspace through three tiers:

1. `$TOAD_HOME` environment variable
2. `~/.toad/config.json` global configuration
3. Error with setup instructions

### Project Registry

Toad maintains a cached registry of all projects in
`~/.toad/shadows/registry.json`. This enables fast queries without scanning the
filesystem.

### Context Engineering

Toad generates tiered metadata for AI agents:

- `llms.txt` — Entry point (100 tokens)
- `SYSTEM_PROMPT.md` — Overview (2k tokens)
- `MANIFEST.md` — Full table (10k tokens)
- `ATLAS.json` — DNA map (5k tokens)
- `CONTEXT.md` — Per-project deep dive (4k tokens)

### DNA Patterns

Toad analyzes project structure to identify:

- **Roles** — Data Layer, API Surface, CLI, Tests
- **Capabilities** — Dockerized, Async, Serialization
- **Patterns** — Framework usage, architectural style

## Common Workflows

### Multi-Repo Git Operations

```bash
# Check status across all repos
toad ggit status

# Commit changes with cascade to hub
toad ggit commit -m "feat: add feature" --cascade

# Sync and align submodules
toad ggit sync

# Checkout branch across repos
toad ggit checkout dev --query backend
```json

### Bulk Operations

```bash
# Run command across filtered projects
toad do "npm install" --query frontend --yes

# Clean build artifacts
toad clean --tier cold --dry-run
toad clean --tier cold --yes

# Auto-assign taxonomy tags
toad tag --harvest
```json

### Context Management

```bash
# Register a new workspace
toad project register work ~/Code/work-projects

# Switch contexts
toad project switch work

# List all contexts
toad project list
```json

### AI Integration

```bash
# Generate and distribute skills
toad skill sync

# Get task-specific context
toad context --task "add authentication" --project api-server

# Compare projects for migration
toad context --compare old-api --project new-api
```json

## Advanced Topics

### MCP Server

Toad can run as a Model Context Protocol server, exposing its intelligence to AI
agents in Windsurf, Cursor, and Claude Desktop.

See [MCP Guide](docs/guides/MCP.md) for setup and available tools.

### Stack Strategies

Toad uses a plugin system to identify projects and assign taxonomy. You can add
support for any language or framework.

See [Plugin Guide](docs/guides/PLUGINS.md) for creating custom strategies.

### Custom Workflows

Register shell scripts as first-class Toad commands:

```bash
toad cw register deploy ./scripts/deploy.sh
toad cw run deploy
```json

## Troubleshooting

### Context is Stale

If you see "Context is stale" warnings:

```bash
toad manifest  # Refresh context
```json

### Submodules Not Initialized

```bash
just init  # Initialize all submodules
```json

### MCP Server Not Found

```bash
cargo install --path bin/toad-mcp
```json

### Registry is Empty

```bash
toad sync  # Rebuild registry
```json

## Reference

- [CLI Reference](docs/guides/CLI.md) — All commands and options
- [MCP Guide](docs/guides/MCP.md) — MCP server setup and tools
- [Plugin Guide](docs/guides/PLUGINS.md) — Stack strategy system
- [Architecture](docs/architecture/) — Technical deep dives
- [Changelog](CHANGELOG.md) — Release history

## Getting Help

- **Issues:** <https://github.com/Primatif/Primatif_Toad/issues>
- **Discussions:** <https://github.com/Primatif/Primatif_Toad/discussions>

```json
### Phase 2: Update MCP Guide (10 min)

**File:** `docs/guides/MCP.md`

- Add all 16 tools to "Available Tools" section
- Add "Common Workflows" section (already planned in MCP enhancements track)
- Update troubleshooting section

### Phase 3: Update CLI Guide (5 min)

**File:** `docs/guides/CLI.md`

- Verify it's up to date (regenerated by `toad docs`)
- Add link to USER_GUIDE.md at top

### Phase 4: Update README (10 min)

**File:** `README.md`

- Update "What's New" section for v1.1.0
- Add link to USER_GUIDE.md
- Ensure quick start is accurate

---

## Success Criteria

- [ ] `USER_GUIDE.md` exists at repo root
- [ ] Quick start gets users productive in 5 minutes
- [ ] All v1.1.0 features are documented
- [ ] Common workflows have clear examples
- [ ] Links between guides work correctly
- [ ] README points to USER_GUIDE.md

---

## Integration Points

- **Depends on:** Existing guides in `docs/guides/`
- **Consumed by:** New users and AI agents
- **Testing:** Follow quick start guide manually

---

## Future Enhancements

- Video tutorials
- Interactive examples
- Community cookbook
```json
````
