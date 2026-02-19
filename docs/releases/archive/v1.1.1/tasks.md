# v1.1.1 "Polish & Insights" — Task Breakdown

> Spec-driven task list derived from
> [`docs/releases/v1.1.1/evolution.md`](./evolution.md).

---

## Legend

- **Status:** `[ ]` Not Started · `[~]` In Progress · `[x]` Done
- **Ref:** `§` = section header in evolution.md

---

## Phase 0: Critical Fixes (P0)

> Ref: `§ Core Objectives > 1`

### P0-1: ggit Submodule Path Fixes

- Ref: Track `111-ggit-fixes`
- [ ] Fix `ggit sync` preflight submodule path bug (line 179)
- [ ] Verify `ggit checkout` uses correct paths
- [ ] Verify `ggit branches` uses correct paths
- [ ] Test on hub project with submodules
- [ ] Test on external registered project

---

## Phase 1: Production Infrastructure (P0)

> Ref: `§ Core Objectives > 1, § Theme 1`

### P1-1: CI/CD Pipeline

- Ref: Track `111-ci-cd-release`
- [ ] Create `.github/workflows/ci.yml`
  - [ ] Test job with matrix (Ubuntu, macOS)
  - [ ] Lint job (clippy, rustfmt, markdownlint, dprint)
  - [ ] License boundary check job
  - [ ] Build verification job
  - [ ] Add cargo caching
- [ ] Create `.github/workflows/release.yml`
  - [ ] Release creation on version tags
  - [ ] Multi-platform binary builds (Linux, macOS)
  - [ ] Asset uploads to GitHub releases
- [ ] Add CI status badge to README.md
- [ ] Create CONTRIBUTING.md
- [ ] Test CI pipeline on dev branch
- [ ] Verify all jobs pass

---

## Phase 2: Analytics System (P1)

> Ref: `§ Core Objectives > 2, § Theme 2`

### P2-1: Data Structures

- Ref: Track `111-analytics-enhancements`
- [ ] Add `DependencyGraph` type to `toad-core`
- [ ] Add `VelocityMetrics` type
- [ ] Add `DebtIndicators` type
- [ ] Add `HealthScore` type
- [ ] Add `TrendReport` type
- [ ] Export all types from `lib.rs`

### P2-2: Analysis Functions

- Ref: Track `111-analytics-enhancements`
- [ ] Implement `analyze_dependencies` in `toad-ops`
- [ ] Implement `analyze_velocity` in `toad-ops`
- [ ] Implement `analyze_debt` in `toad-ops`
- [ ] Implement `calculate_health_score` in `toad-ops`
- [ ] Implement `analyze_trends` in `toad-ops`
- [ ] Add unit tests for scoring logic

### P2-3: CLI Commands

- Ref: Track `111-analytics-enhancements`
- [ ] Add `Analyze` command to CLI
- [ ] Add subcommands: deps, velocity, debt, health, trends
- [ ] Create `commands/analyze.rs`
- [ ] Implement handlers for each subcommand
- [ ] Add UI formatting functions
- [ ] Test on Toad itself

### P2-4: MCP Integration

- Ref: Track `111-analytics-enhancements`
- [ ] Add `analyze_dependencies` tool to MCP server
- [ ] Add `analyze_velocity` tool
- [ ] Add `analyze_debt` tool
- [ ] Add `analyze_health` tool
- [ ] Add parameter schemas
- [ ] Test via MCP client

---

## Phase 3: MCP Enhancements (P1)

> Ref: `§ Core Objectives > 3, § Theme 3`

### P3-1: Enhanced Tool Descriptions

- Ref: Track `111-mcp-enhancements`
- [x] Update all existing tool descriptions with usage hints
- [ ] Add "when to use" guidance
- [ ] Add "what comes next" suggestions
- [ ] Add "alternatives" references

### P3-2: New MCP Tools

- Ref: Track `111-mcp-enhancements`
- [ ] Add `get_atlas` tool
- [ ] Add `get_manifest` tool
- [ ] Add `get_project_context` tool
- [ ] Test all new tools via MCP client
- [ ] Verify error handling for missing files

### P3-3: CLI Bridge

- Ref: Track `111-mcp-cli-bridge`
- [ ] Add `reveal_projects` tool
- [ ] Add `get_git_status` tool
- [ ] Add `get_disk_stats` tool
- [ ] Add `list_branches` tool
- [ ] Add `sync_registry` tool
- [ ] Add `generate_manifest` tool
- [ ] Add `tag_projects` tool
- [ ] Add `register_context` tool
- [ ] Verify dangerous operations excluded
- [ ] Test all tools via MCP client

### P3-4: Documentation

- Ref: Track `111-mcp-enhancements`
- [ ] Update MCP guide with all 20+ tools
- [ ] Add "Common Workflows" section
- [ ] Add usage examples
- [ ] Update troubleshooting section

---

## Phase 4: Developer Experience (P1)

> Ref: `§ Core Objectives > 4, § Theme 4`

### P4-1: Doctor Command

- Ref: Track `111-doctor-command`
- [ ] Add `Doctor` command to CLI
- [ ] Implement installation checks
- [ ] Implement workspace checks
- [ ] Implement git checks
- [ ] Implement MCP checks
- [ ] Implement artifact checks
- [ ] Add traffic light output (✅/⚠️/❌)
- [ ] Add actionable recommendations
- [ ] Test on healthy and broken workspaces

### P4-2: User Guide

- Ref: Track `111-user-guide`
- [ ] Create `USER_GUIDE.md` at repo root
- [ ] Write Quick Start section (5-minute setup)
- [ ] Write Core Concepts section
- [ ] Write Common Workflows section
- [ ] Write Advanced Topics section
- [ ] Write Troubleshooting section
- [ ] Add Reference links
- [ ] Update README with link to guide

### P4-3: Skill Updates

- Ref: Track `111-skill-updates`
- [ ] Enhance blueprint skill with DNA patterns
- [ ] Add stack distribution to blueprint
- [ ] Add common patterns to CLI skill
- [ ] Create new MCP skill
- [ ] Update `skill sync` to generate all 3 skills
- [ ] Test skill distribution to AI vendors

---

## Phase 5: Dogfooding (P2)

> Ref: `§ Core Objectives > 5, § Theme 5`

### P5-1: Apply Taxonomy

- Ref: Track `111-dogfooding`
- [ ] Tag core crates: `#core`
- [ ] Tag intelligence crates: `#intelligence`
- [ ] Tag orchestration crates: `#orchestration`
- [ ] Tag interface crates: `#interface`
- [ ] Tag utility crates: `#utility`
- [ ] Verify tags: `toad status --tag core`

### P5-2: Register Workflows

- Ref: Track `111-dogfooding`
- [ ] Create `scripts/workflows/qa.sh`
- [ ] Create `scripts/workflows/release-check.sh`
- [ ] Create `scripts/workflows/update-docs.sh`
- [ ] Make scripts executable
- [ ] Register workflows: `toad cw register`
- [ ] Test workflows: `toad cw run qa`

### P5-3: Git Hooks

- Ref: Track `111-dogfooding`
- [ ] Create `scripts/git-hooks/post-commit`
- [ ] Add auto-refresh logic
- [ ] Update Justfile to install post-commit hook
- [ ] Run `just setup-hooks`
- [ ] Test: commit and verify manifest refreshes

### P5-4: MCP Configuration

- Ref: Track `111-dogfooding`
- [ ] Document MCP setup for Toad development
- [ ] Test MCP server with Toad workspace
- [ ] Verify tools work for querying Toad

---

## Cross-Cutting Concerns

### MCP Integration (Critical)

- [ ] Verify all new analytics exposed via MCP tools
- [ ] Verify `toad doctor` results accessible via MCP
- [ ] Verify all CLI bridge tools working
- [ ] Test MCP tool descriptions are clear
- [ ] Ensure parameter schemas are complete

### Documentation

- [ ] Update CHANGELOG.md for v1.1.1
- [ ] Update README.md with v1.1.1 features
  - [ ] Update "What's New" section
  - [ ] Update Core Commands section
  - [ ] Add analytics commands
  - [ ] Add doctor command
  - [ ] Update version badge
- [ ] Regenerate CLI docs: `toad docs`
- [ ] Update all guides for new features
  - [ ] CLI.md (analytics, doctor)
  - [ ] MCP.md (all new tools)
  - [ ] USER_GUIDE.md (complete)
- [ ] Run `toad skill sync` to update AI skills

### Feature Alignment

- [ ] Verify analytics follow existing patterns
- [ ] Verify doctor output matches status/stats style
- [ ] Verify MCP tools follow naming conventions
- [ ] Verify all commands support --json flag
- [ ] Verify all commands respect --no-sync flag

### Testing

- [ ] Run `just qa` and verify all checks pass
- [ ] Test all new commands on Toad itself
- [ ] Verify CI pipeline passes
- [ ] Manual testing on macOS and Linux

### Release Preparation

- [ ] Update version in Cargo.toml files
- [ ] Update README badges
- [ ] Create release notes
- [ ] Tag release: `git tag v1.1.1`
- [ ] Push tag to trigger release workflow

---

## Future (Post v1.1.1)

Items enabled by v1.1.1 but deferred to v1.2.0:

- GitHub API integration for richer analytics
- Performance profiling and optimization
- Predictive analytics
- Custom dashboard (web UI)
- Plugin system for custom analytics
- Multi-workspace support
- Cloud sync for context
