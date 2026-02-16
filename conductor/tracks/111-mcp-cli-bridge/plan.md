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

- [ ] Add `reveal_projects` tool to `bin/toad-mcp/src/server.rs`
  - Delegates to `toad_discovery::find_projects`
  - Parameters: query, tag
- [ ] Add `get_git_status` tool (if not already covered by existing tools)
  - Delegates to `toad_git::generate_multi_repo_status`
  - Parameters: query, tag
- [ ] Add `get_disk_stats` tool
  - Delegates to `toad_ops::generate_analytics_report`
  - Parameters: query, tag, all
- [ ] Add `list_branches` tool
  - Delegates to `toad_git::branches::list_local_branches`
  - Parameters: query, tag, all

### Phase 2: Add Safe Write Operations

- [ ] Add `sync_registry` tool
  - Delegates to `toad_discovery::scan_all_projects`
  - No parameters
- [ ] Add `generate_manifest` tool
  - Delegates to manifest generation logic
  - Parameters: project (optional)
- [ ] Verify `switch_context` tool exists and works
- [ ] Add `register_context` tool
  - Delegates to project registration logic
  - Parameters: name, path

### Phase 3: Add Conditional Safe Operations

- [ ] Add `tag_projects` tool
  - Delegates to tag assignment logic
  - Parameters: project, tag, query, filter_tag, harvest
  - Auto-confirms (no prompt)

### Phase 4: Add Parameter Schemas

- [ ] Add `RevealParams` struct
- [ ] Add `StatusParams` struct
- [ ] Add `StatsParams` struct
- [ ] Add `BranchesParams` struct
- [ ] Add `ManifestParams` struct
- [ ] Add `RegisterContextParams` struct
- [ ] Add `TagParams` struct

### Phase 5: Enhance Tool Descriptions

- [ ] Update all existing tool descriptions with usage hints
- [ ] Add "when to use" guidance
- [ ] Add "what comes next" suggestions
- [ ] Add "alternatives" references

### Phase 6: Testing

- [ ] Build: `cargo build -p toad-mcp`
- [ ] Test each new tool via MCP client
- [ ] Verify parameter validation works
- [ ] Test error handling for missing projects
- [ ] Verify dangerous operations are NOT exposed

---

## Acceptance Criteria

- All safe read operations exposed via MCP
- Safe write operations (sync, manifest, tag) exposed
- Context management tools complete
- No dangerous operations exposed
- All tools have enhanced descriptions
- Parameter schemas complete with doc comments
- MCP server builds without errors
