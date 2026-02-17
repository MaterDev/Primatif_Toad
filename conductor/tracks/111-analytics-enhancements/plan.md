# Plan: Analytics Enhancements (111-analytics-enhancements)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 3-4 hours
- **Target:** v1.1.1 (major feature addition)
- **Priority:** P1 (High value for developers)

---

## Tasks

### Phase 1: Data Structures (30 min)

- [x] Add analytics types to `crates/toad-core/src/models/analytics.rs`
  - `DependencyGraph`
  - `VelocityMetrics`
  - `DebtIndicators`
  - `HealthScore`
  - `TrendReport`
- [x] Add serialization derives
- [x] Export from `lib.rs`

### Phase 2: Analysis Functions (90 min)

- [x] Implement `analyze_dependencies` in `toad-ops`
  - Parse Cargo.toml files
  - Build dependency graph
  - Calculate critical path
- [x] Implement `analyze_velocity` in `toad-ops`
  - Query git log for commits
  - Calculate line changes
  - Identify contributors
- [x] Implement `analyze_debt` in `toad-ops`
  - Scan for TODO/FIXME/HACK comments
  - Check file sizes
  - Parse test coverage (if available)
- [x] Implement `calculate_health_score` in `toad-ops`
  - Score VCS cleanliness
  - Score test coverage
  - Score documentation
  - Score activity
  - Calculate composite

### Phase 3: CLI Commands (30 min)

- [x] Add `Analyze` command to `bin/toad/src/cli.rs`
- [x] Add subcommands: deps, velocity, debt, health, trends
- [x] Create `bin/toad/src/commands/analyze.rs`
- [x] Implement handlers for each subcommand
- [x] Add UI formatting functions

### Phase 4: MCP Integration (30 min)

- [x] Add analytics tools to `bin/toad-mcp/src/server.rs`
  - `analyze_dependencies`
  - `analyze_velocity`
  - `analyze_debt`
  - `analyze_health`
- [x] Add parameter schemas
- [x] Test via MCP client

### Phase 5: Testing (30 min)

- [x] Test on Toad itself
- [x] Verify dependency graph accuracy
- [x] Verify velocity calculations
- [x] Verify debt detection
- [x] Verify health scoring
- [x] Add unit tests for scoring logic

### Phase 6: Documentation (20 min)

- [x] Update CLI guide with analyze commands
- [x] Update MCP guide with analytics tools
- [x] Add examples to USER_GUIDE.md
- [x] Update skill with analytics commands

---

## Acceptance Criteria

- All 7 analytics types implemented
- CLI commands work and produce useful output
- MCP tools expose analytics to AI agents
- Health scoring is accurate and actionable
- Documentation complete with examples
