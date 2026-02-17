# Plan: Integration Checklist (111-integration-checklist)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** Ongoing (Quality Gate)
- **Target:** v1.1.1 (and beyond)
- **Priority:** P1 (Consistency & Discoverability)

---

## Tasks

### Phase 1: MCP Tool Completion (30 min)

- [x] Audit `bin/toad-mcp/src/server.rs` against the tool inventory in
      `spec.md`.
- [x] Implement missing tools:
  - [x] `analyze_trends`
  - [x] `analyze_patterns`
- [x] Verify `analyze_submodules` returns structured data consistent with other
      tools.
- [x] Verify `run_health_check` is properly exposed.

### Phase 2: Documentation Sync (45 min)

- [x] Run `cargo run --bin toad -- docs` to regenerate `docs/guides/CLI.md`.
- [x] Update `docs/guides/MCP.md` with all new tools and descriptions.
- [x] Update `README.md`:
  - [x] Version badge to `v1.1.1`.
  - [x] "What's New" section for `v1.1.1`.
  - [x] "Core Commands" with Analytics & Doctor.
- [x] Update `conductor/USER_GUIDE.md` (create if missing) with v1.1.1 features.

### Phase 3: Pattern Verification (30 min)

- [x] Audit `bin/toad/src/main.rs` to ensure all new commands respect `--json`
      and `--no-sync`.
- [x] Audit `bin/toad/src/commands/analyze.rs` for consistent emoji usage and
      colored output.
- [x] Audit `bin/toad-mcp/src/tools/analysis.rs` for consistent error handling
      and `spawn_blocking` usage.

### Phase 4: Skill Synchronization (15 min)

- [x] Update AI skills (`.gemini/skills/`) with new command context.
- [x] Run `cargo run --bin toad -- skill sync` to update vendors.

---

## Acceptance Criteria

- MCP server exposes 32+ tools with clear descriptions.
- `README.md` and all documentation reflect v1.1.1 capabilities.
- All CLI commands produce valid JSON when `--json` is passed.
- Skills are synchronized and up-to-date.
