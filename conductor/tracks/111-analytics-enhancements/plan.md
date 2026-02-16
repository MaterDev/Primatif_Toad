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

- [ ] Add analytics types to `crates/toad-core/src/models.rs`
  - `DependencyGraph`
  - `VelocityMetrics`
  - `DebtIndicators`
  - `HealthScore`
  - `TrendReport`
- [ ] Add serialization derives
- [ ] Export from `lib.rs`

### Phase 2: Analysis Functions (90 min)

- [ ] Implement `analyze_dependencies` in `toad-ops`
  - Parse Cargo.toml files
  - Build dependency graph
  - Calculate critical path
- [ ] Implement `analyze_velocity` in `toad-ops`
  - Query git log for commits
  - Calculate line changes
  - Identify contributors
- [ ] Implement `analyze_debt` in `toad-ops`
  - Scan for TODO/FIXME/HACK comments
  - Check file sizes
  - Parse test coverage (if available)
- [ ] Implement `calculate_health_score` in `toad-ops`
  - Score VCS cleanliness
  - Score test coverage
  - Score documentation
  - Score activity
  - Calculate composite

### Phase 3: CLI Commands (30 min)

- [ ] Add `Analyze` command to `bin/toad/src/cli.rs`
- [ ] Add subcommands: deps, velocity, debt, health, trends
- [ ] Create `bin/toad/src/commands/analyze.rs`
- [ ] Implement handlers for each subcommand
- [ ] Add UI formatting functions

### Phase 4: MCP Integration (30 min)

- [ ] Add analytics tools to `bin/toad-mcp/src/server.rs`
  - `analyze_dependencies`
  - `analyze_velocity`
  - `analyze_debt`
  - `analyze_health`
- [ ] Add parameter schemas
- [ ] Test via MCP client

### Phase 5: Testing (30 min)

- [ ] Test on Toad itself
- [ ] Verify dependency graph accuracy
- [ ] Verify velocity calculations
- [ ] Verify debt detection
- [ ] Verify health scoring
- [ ] Add unit tests for scoring logic

### Phase 6: Documentation (20 min)

- [ ] Update CLI guide with analyze commands
- [ ] Update MCP guide with analytics tools
- [ ] Add examples to USER_GUIDE.md
- [ ] Update skill with analytics commands

---

## Acceptance Criteria

- All 7 analytics types implemented
- CLI commands work and produce useful output
- MCP tools expose analytics to AI agents
- Health scoring is accurate and actionable
- Documentation complete with examples
