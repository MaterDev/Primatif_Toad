# Plan: MCP CLI Bridge (111-mcp-cli-bridge)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 55 minutes
- **Target:** v1.1.0 (functional completeness)
- **Priority:** P1 (High value for AI agents)

---

## Tasks

### Phase 1: Add Safe Read Operations

- [x] Add `reveal_projects` tool to `bin/toad-mcp/src/server.rs` (d095921)
  - Delegates to `toad_discovery::find_projects`
  - Parameters: query, tag
- [x] Add `get_git_status` tool (if not already covered by existing tools) (d095921)
  - Delegates to `toad_git::generate_multi_repo_status`
  - Parameters: query, tag
- [x] Add `get_disk_stats` tool (d095921)
  - Delegates to `toad_ops::generate_analytics_report`
  - Parameters: query, tag, all
- [x] Add `list_branches` tool (d095921)
  - Delegates to `toad_git::branches::list_local_branches`
  - Parameters: query, tag, all

### Phase 2: Add Safe Write Operations

- [x] Add `sync_registry` tool (d095921)
  - Delegates to `toad_discovery::scan_all_projects`
  - No parameters
- [x] Add `generate_manifest` tool (d095921)
  - Delegates to manifest generation logic
  - Parameters: project (optional)
- [x] Verify `switch_context` tool exists and works (d095921)
- [x] Add `register_context` tool (d095921)
  - Delegates to project registration logic
  - Parameters: name, path

### Phase 3: Add Conditional Safe Operations

- [x] Add `tag_projects` tool (d095921)
  - Delegates to tag assignment logic
  - Parameters: project, tag, query, filter_tag, harvest
  - Auto-confirms (no prompt)

### Phase 4: Add Parameter Schemas

- [x] Add `RevealParams` struct (d095921)
- [x] Add `StatusParams` struct (d095921)
- [x] Add `StatsParams` struct (d095921)
- [x] Add `BranchesParams` struct (d095921)
- [x] Add `ManifestParams` struct (d095921)
- [x] Add `RegisterContextParams` struct (d095921)
- [x] Add `TagParams` struct (d095921)

### Phase 5: Enhance Tool Descriptions

- [x] Update all existing tool descriptions with usage hints (d095921)
- [x] Add "when to use" guidance (d095921)
- [x] Add "what comes next" suggestions (d095921)
- [x] Add "alternatives" references (d095921)

### Phase 6: Testing

- [x] Build: `cargo build -p toad-mcp` (d095921)
- [x] Test each new tool via MCP client (d095921)
- [x] Verify parameter validation works (d095921)
- [x] Test error handling for missing projects (d095921)
- [x] Verify dangerous operations are NOT exposed (d095921)

---

## Acceptance Criteria

- All safe read operations exposed via MCP
- Safe write operations (sync, manifest, tag) exposed
- Context management tools complete
- No dangerous operations exposed
- All tools have enhanced descriptions
- Parameter schemas complete with doc comments
- MCP server builds without errors
