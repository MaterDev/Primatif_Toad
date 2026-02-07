# v1.1.0 "Deep Croak" — Task Breakdown

> Spec-driven task list derived from
> [`docs/releases/v1.1.0/evolution.md`](./evolution.md). Each task references
> its source section for traceability.

---

## Legend

- **Status:** `[ ]` Not Started · `[~]` In Progress · `[x]` Done
- **Ref:** `§` = section header in the source document

---

## Pre-Flight: Blockers (Must resolve before or during Phase 0)

> Ref: `§ Blockers & Risks > 🔴 Critical Blockers`

- [ ] **BLK-1: Resolve "No-Print" Violation**
  - Ref: `§ Blockers & Risks > 🔴 Critical Blockers > 1`
  - [ ] Audit `main.rs` — catalog every command that prints directly
  - [ ] Define structured result types for each command (e.g., `StatusReport`,
        `StatsReport`, `RevealResult`)
  - [ ] Extract command logic into library functions in the appropriate crates
  - [ ] Reduce `main.rs` to a thin formatter that consumes result types

- [ ] **BLK-2: Decouple Terminal Progress**
  - Ref: `§ Blockers & Risks > 🔴 Critical Blockers > 2`
  - [ ] Define a `ProgressReporter` trait in `toad-core`
  - [ ] Implement `IndicatifReporter` for the CLI
  - [ ] Implement `NoOpReporter` for non-terminal consumers
  - [ ] Replace all direct `indicatif` calls in library crates with the trait

- [ ] **BLK-3: Typed Error Surface**
  - Ref: `§ Blockers & Risks > 🔴 Critical Blockers > 3`
  - [ ] Design a `ToadError` enum covering core failure states (e.g.,
        `ProjectNotFound`, `PermissionDenied`, `StaleContext`, `ConfigMissing`)
  - [ ] Replace `anyhow!` string errors in core logic with `ToadError` variants
  - [ ] Keep `anyhow` at the binary boundary for convenience; library crates
        return `ToadError`

- [ ] **BLK-4: Decouple Interactive Prompts**
  - Ref: `§ Blockers & Risks` (implied by No-Print Violation + MCP/Dashboard
    goals)
  - [ ] Audit all `stdin` prompts in `main.rs` (e.g., `toad do` confirmation)
  - [ ] Ensure every interactive command accepts `--yes` / `-y` to skip prompts
  - [ ] Library functions must never read stdin — confirmation is a parameter
        (`confirmed: bool`)
  - [ ] MCP: agent passes confirmation as a tool parameter
  - [ ] Dashboard: confirmation becomes an HTTP request/response pair

---

## Phase 0: Global Install Architecture

> Ref: `§ Roadmap to v1.1.0 > Phase 0: Global Install Architecture`

### P0-1: Workspace Struct Refactor

- Ref: `§ Phase 0 > Refactoring Required > 1`
- [ ] Split `Workspace` into two path concepts: `toad_home` and `projects_dir`
- [ ] `toad_home` resolves to `~/.toad/` (or `$TOAD_HOME`)
- [ ] `projects_dir` resolves from `config.json` `home_pointer`
- [ ] Update all downstream consumers of `Workspace`

### P0-2: Relocate shadows_dir

- Ref: `§ Phase 0 > Refactoring Required > 2, 3, 4`
- [ ] Change `shadows_dir` to resolve to `~/.toad/shadows/`
- [ ] Change `tags_path` to resolve to `~/.toad/shadows/tags.json`
- [ ] Change `manifest_path` to resolve to `~/.toad/shadows/MANIFEST.md`
- [ ] Ensure `ensure_shadows()` creates dirs under `~/.toad/`

### P0-3: Simplify Workspace::discover()

- Ref: `§ Phase 0 > Refactoring Required > 5`
- [ ] Tier 1: `$TOAD_HOME` env var
- [ ] Tier 2: `~/.toad/config.json` → `home_pointer`
- [ ] Tier 3: Error with setup instructions
- [ ] Remove upward `.toad-root` file search

### P0-4: Extend GlobalConfig

- Ref: `§ Phase 0 > Refactoring Required > 6`
- [ ] Add `auto_sync: bool` field
- [ ] Add `context_budget` object (`ecosystem_tokens`, `project_tokens`)
- [ ] Ensure backward-compatible deserialization (missing fields get defaults)

### P0-5: Update Tests

- Ref: `§ Phase 0 > Refactoring Required > 7`
- [ ] Update `test_workspace_paths` for new path separation
- [ ] Update `test_workspace_discovery_tiers` for simplified discovery
- [ ] Use independent `tempdir` for `toad_home` and `projects_dir`

### P0-6: Backward Compatibility & Migration

- Ref: `§ Phase 0 > Backward Compatibility`
- [ ] Detect old-style `.toad-root` workspace on startup
- [ ] Auto-migrate: copy `shadows/` to `~/.toad/shadows/`
- [ ] Write new `config.json` with `home_pointer` set to old workspace root
- [ ] Print one-time migration notice
- [ ] `TOAD_ROOT` env var now sets `projects_dir` only

### P0-7: Schema-First Contract (Mandate)

- Ref: `§ Strategic Positioning > Modern Engineering Mandates > 1`
- [ ] Audit all core structs — ensure `Serialize + Deserialize` on every public
      type
- [ ] Add derives to any new result types created in BLK-1

### P0-8: Embed Default Strategies

- Ref: `§ Blockers & Risks > 🟡 Moderate Risks > (implied by global install)`
- [ ] Embed default strategy TOML files via `include_str!` or build script
- [ ] User-defined strategies in `~/.toad/strategies/` override defaults

---

## Phase 1: Deep Extraction & Structured Data

> Ref: `§ Roadmap to v1.1.0 > Phase 1: Deep Extraction & Structured Data`

### P1-1: Semantic Essence Extraction

- Ref: `§ Phase 1 > 1. Semantic Essence Extraction`
- [ ] Modify `extract_essence` to include headers as context markers
- [ ] Add detection for "Capability" indicators ("Provides", "Exposes", "Main
      Entry")
- [ ] Limit extraction by semantic value, not just line count

### P1-2: Structured JSON Context

- Ref: `§ Phase 1 > 2. Structured JSON Context`
- [ ] Implement `toad manifest --json` flag
- [ ] Make JSON generation the default side-effect of `toad manifest`
- [ ] Generate `~/.toad/shadows/context.json` with full `ProjectDetail` list +
      stats

### P1-3: Stats Integration

- Ref: `§ Phase 1 > 3. Stats Integration`
- [ ] Add `bloat_index` field to `ProjectDetail`
- [ ] Add `total_size` field to `ProjectDetail`
- [ ] Include both in manifest and JSON output

### P1-4: Global --json Flag

- Ref: `§ Strategic Positioning > Modern Engineering Mandates > 2`
- [ ] Add `--json` global flag to `Cli` struct
- [ ] Wire every command to output `serde_json::to_string_pretty` when flag is
      set
- [ ] Ensure all result types from BLK-1 are serializable

---

## Phase 2: Context Integrity & Maintenance

> Ref: `§ Roadmap to v1.1.0 > Phase 2: Context Integrity & Maintenance`

### P2-1: Reliable Staleness Detection

- Ref: `§ Phase 2 > 1. Reliable Staleness Detection`
- [ ] Fix fingerprint comparison logic (change `>` to `!=` or hash check)
- [ ] Add `--check` flag to `toad manifest` (report staleness, exit non-zero if
      stale)

### P2-2: Auto-Sync — Opportunistic Refresh

- Ref: `§ Phase 2 > 2. Auto-Sync Triggers > Opportunistic Refresh`
- [ ] Add fingerprint check to startup of every Toad command
- [ ] If stale, silently regenerate `shadows/` before executing
- [ ] Respect `--no-sync` flag to skip auto-refresh
- [ ] Print staleness warning when `--no-sync` is active and context is stale

### P2-3: Auto-Sync — Post-Mutation Hook

- Ref: `§ Phase 2 > 2. Auto-Sync Triggers > Post-Mutation Hook`
- [ ] After `toad do` operations, trigger context refresh
- [ ] Print one-line summary: `Context updated (N projects changed).`

### P2-4: Auto-Sync — Watch Mode (Optional)

- Ref: `§ Phase 2 > 2. Auto-Sync Triggers > Watch Mode`
- [ ] Implement `toad sync --watch` as lightweight daemon
- [ ] (Nice-to-have, not primary mechanism)

### P2-5: Diff-Aware Context (Changelog)

- Ref: `§ Phase 2 > 3. Diff-Aware Context (Changelog)`
- [ ] Generate `~/.toad/shadows/CHANGELOG.json`
- [ ] Track changes to: `vcs_status`, `activity` tier, new/removed projects, new
      high-value files
- [ ] Store diffs between syncs, not just current state

### P2-6: Token Toxicity Mitigation

- Ref: `§ Blockers & Risks > 🟡 Moderate Risks > 1`
- [ ] Use fingerprinting to only regenerate context for changed projects
- [ ] Implement token estimator (`chars / 4` approximation)
- [ ] Truncate generated context that exceeds budget with link to full version

---

## Phase 3: Agent Interface & Meta-Ops

> Ref: `§ Roadmap to v1.1.0 > Phase 3: Agent Interface & Meta-Ops`

### P3-1: AGENTS.md Generation

- Ref: `§ Phase 3 > 1. AGENTS.md Generation`
- [ ] Implement `toad agents` command (generate for all projects)
- [ ] Implement `toad agents <name>` (generate for single project)
- [ ] Template: include stack, build commands, test patterns, taxonomy
- [ ] Write to `~/.toad/shadows/{project}/AGENTS.md`

### P3-2: Tiered Prompt Generation — llms.txt

- Ref: `§ Phase 3 > 2. Tiered Prompt Generation`
- [ ] Generate `~/.toad/shadows/llms.txt` — ecosystem table of contents (~500
      tokens)
- [ ] Include links to all project `CONTEXT.md` and `AGENTS.md` files

### P3-3: Tiered Prompt Generation — SYSTEM_PROMPT.md

- Ref: `§ Phase 3 > 2. Tiered Prompt Generation`
- [ ] Generate `~/.toad/shadows/SYSTEM_PROMPT.md` — bird's-eye ecosystem view
      (~2k tokens)
- [ ] Summarize all projects, stacks, activity tiers, and tags

### P3-4: Tiered Prompt Generation — Per-Project CONTEXT.md

- Ref: `§ Phase 3 > 2. Tiered Prompt Generation`
- [ ] Generate `~/.toad/shadows/{project}/CONTEXT.md` — deep project dive (~4k
      tokens)
- [ ] Include essence, stack details, key files, taxonomy, DNA (when available)

### P3-5: Context-Aware Pre-flights

- Ref: `§ Phase 3 > 3. Context-Aware Pre-flights`
- [ ] Teach `toad do` to read `context.json` before execution
- [ ] Warn on stack mismatches (e.g., `cargo` on a non-Rust project)

### P3-6: Semantic Reveal Enhancement

- Ref: `§ Phase 3 > 4. Semantic Reveal`
- [ ] Enhance `toad reveal` to search within `essence` and tags using cached
      registry
- [ ] Support richer query matching against `context.json`

### P3-7: Context Initialization Command

- Ref: `§ Phase 3 > 5. Context Initialization`
- [ ] Implement `toad init-context` command
- [ ] Chain: `manifest` → `agents` → tiered prompts → `llms.txt`
- [ ] Support `--dry-run` flag
- [ ] Support `--project <name>` flag
- [ ] Support `--no-sync` flag
- [ ] After initial run, opportunistic refresh (P2-2) keeps it current

---

## Phase 3.5: MCP Server Mode

> Ref: `§ Roadmap to v1.1.0 > Phase 3.5: MCP Server Mode (The Big Play)`

### P3.5-1: MCP Server Binary

- Ref: `§ Phase 3.5 > 1. MCP Server Implementation`
- [ ] Create `bin/toad-mcp/` as a separate binary crate
- [ ] Add `rmcp` and `tokio` dependencies
- [ ] Import `toad-api` (or equivalent library crate) for all data access
- [ ] Use `tokio::task::spawn_blocking` to wrap sync library calls

### P3.5-2: Expose Core Tools

- Ref: `§ Phase 3.5 > 2. Exposed Tools`
- [ ] Implement `list_projects` tool — filtered by stack, activity, vcs_status,
      tag
- [ ] Implement `get_project_detail` tool — full context for one project
- [ ] Implement `search_projects` tool — semantic search across essence, tags,
      taxonomy
- [ ] Implement `get_ecosystem_summary` tool — system prompt tier on demand

### P3.5-3: Idempotent Discovery (Mandate)

- Ref: `§ Strategic Positioning > Modern Engineering Mandates > 3`
- [ ] Ensure all read-only MCP tools are side-effect free
- [ ] Calling `list_projects` or `get_project_detail` N times must not mutate
      state

---

## Phase 4: Pattern Intelligence & Portability

> Ref: `§ Roadmap to v1.1.0 > Phase 4: Pattern Intelligence & Portability`

### P4-1: Structural DNA Mapping

- Ref: `§ Phase 4 > 1. Structural DNA Mapping`
- [ ] Extend `StackStrategy` to detect structural roles (data layer, business
      logic, API surface, tests, deployment)
- [ ] Implement directory name matching for known patterns (`src/models/`,
      `src/services/`, etc.)
- [ ] Implement file existence checks (`Dockerfile`, `docker-compose.yml`)
- [ ] Implement grep-level text search for entry point declarations (`pub fn`,
      `export default`, `func`)
- [ ] Generate `~/.toad/shadows/ATLAS.json` with DNA maps for all projects

### P4-2: Capability & Solution Indexing

- Ref: `§ Phase 4 > 2. Capability & Solution Indexing`
- [ ] Define "Recipe" / "Solvability Pattern" schema
- [ ] Index patterns per project (e.g., "Uses JWT Auth", "Actor Pattern",
      "Event-Driven")
- [ ] Store in `ATLAS.json` alongside DNA data

### P4-3: Architectural Clustering

- Ref: `§ Phase 4 > 3. Architectural Clustering`
- [ ] Group projects by pattern/vibe ("High-Performance CLI", "CRUD Service",
      etc.)
- [ ] Enable "Inspiration" search: query by pattern across all projects

### P4-4: Context Injection — toad context Command

- Ref: `§ Phase 4 > 4. Context Injection & Synthesis`
- [ ] Implement `toad context --task "..."` command
- [ ] Implement `toad context --inspire "..."` command
- [ ] Generate Situation Report markdown output (source DNA, target DNA,
      compatibility notes, snippets)

### P4-5: Migration Pre-flights

- Ref: `§ Phase 4 > 4. Context Injection & Synthesis > Migration Pre-flights`
- [ ] Implement repo comparison utility
- [ ] Detect dependency mismatches (e.g., `anyhow` vs `thiserror`)
- [ ] Output compatibility advisory for AI consumption

---

## Phase 5: Synthesis & Industry Standards

> Ref: `§ Roadmap to v1.1.0 > Phase 5: Synthesis & Industry Standards`

### P5-1: MCP Server Extensions (Pattern-Aware Tools)

- Ref: `§ Phase 5 > 1. MCP Server Extensions`
- [ ] Add `get_project_dna` tool to MCP server
- [ ] Add `find_pattern(name)` tool to MCP server
- [ ] Add `compare_projects(a, b)` tool to MCP server
- [ ] Add `generate_situation_report(task)` tool to MCP server

### P5-2: Standardized AI Entry Points — AGENTS.md Sync

- Ref: `§ Phase 5 > 2. Standardized AI Entry Points`
- [ ] Ensure `AGENTS.md` files stay in sync with DNA and pattern data
- [ ] Regenerate on context refresh when DNA changes

### P5-3: Standardized AI Entry Points — llms.txt

- Ref: `§ Phase 5 > 2. Standardized AI Entry Points`
- [ ] Extend `llms.txt` to include links to DNA summaries (not just CONTEXT.md
      and AGENTS.md)
- [ ] Document the adaptation from the web standard (llmstxt.org)

### P5-4: Pattern Synthesis Logic

- Ref: `§ Phase 5 > 3. Pattern Synthesis Logic`
- [ ] Implement `shadows/SYNTHESIS.md` generation
- [ ] Aggregate "Inspiration" snippets from multiple source projects
- [ ] Format for deep analysis by AI assistants

---

## Cross-Cutting: Engineering Mandates

> Ref: `§ Strategic Positioning > Modern Engineering Mandates`

These mandates apply to **all** tasks above:

- [ ] **M-1: Schema-First Contract** — Every core struct derives
      `Serialize + Deserialize`
  - Ref: `§ Modern Engineering Mandates > 1`
- [ ] **M-2: Layered Output Strategy** — Every command supports `--json`
  - Ref: `§ Modern Engineering Mandates > 2`
- [ ] **M-3: Idempotent Discovery** — Read-only ops are side-effect free
  - Ref: `§ Modern Engineering Mandates > 3`
- [ ] **M-4: Data-Service Architecture** — `toad-api` is a local service, not a
      CLI backend
  - Ref: `§ Modern Engineering Mandates > 4`

---

## Future (Post v1.1.0): Dashboard

> Ref: `§ Strategic Positioning > Future Goal: The Toad Dashboard` Ref:
> `§ Strategic Positioning > Dashboard Visualization Ideas`

These are **not** v1.1.0 tasks. They are listed here to show what the v1.1.0
architecture enables:

- [ ] `toad dashboard` command — starts axum HTTP server on localhost
- [ ] Ecosystem Heatmap (visual `toad stats`)
- [ ] Health Dashboard (git status, activity tiers, staleness)
- [ ] Tag Cloud (interactive taxonomy)
- [ ] Structural DNA Viewer (side-by-side project comparison)
- [ ] Context Inspector (browse `shadows/` tree, token counts)
- [ ] Drift Timeline (`CHANGELOG.json` visualization)
- [ ] Dependency Graph (cross-project relationships from Atlas)
