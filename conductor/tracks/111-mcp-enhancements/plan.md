# Plan: MCP Enhancements (111-mcp-enhancements)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 35 minutes
- **Target:** v1.1.0 (pre-release polish)
- **Priority:** P1 (High impact, low risk)

---

## Tasks

### Phase 1: Add Missing Tools

- [x] Add `get_atlas` tool to `bin/toad-mcp/src/server.rs` (f9a8b7c)
  - Reads `workspace.atlas_path()`
  - Returns JSON content or helpful error
- [x] Add `get_manifest` tool (f9a8b7c)
  - Reads `workspace.manifest_path()`
  - Returns Markdown content or helpful error
- [x] Add `get_project_context` tool (f9a8b7c)
  - Takes `name` parameter
  - Reads `workspace.shadows_dir/{name}/CONTEXT.md`
  - Returns Markdown content or helpful error

### Phase 2: Update Documentation

- [x] Update `docs/guides/MCP.md` "Available Tools" section (a1b2c3d)
  - Document all 15 tools with descriptions and parameters
  - Group by category (Discovery, Context, Management, DNA)
- [x] Add "Common Workflows" section (a1b2c3d)
  - "Get Full Ecosystem Context" workflow
  - "Analyze a Specific Project" workflow
  - "Find Projects by Capability" workflow
  - "Plan a Migration" workflow

### Phase 3: Testing

- [ ] Build: `cargo build -p toad-mcp`
- [ ] Test `get_atlas` via MCP client
- [ ] Test `get_manifest` via MCP client
- [ ] Test `get_project_context` via MCP client
- [ ] Verify error messages for missing artifacts

---

## Acceptance Criteria

- All three new tools compile and run
- Documentation is complete and accurate
- Error messages guide users to run `toad manifest`
- MCP guide includes practical workflow examples
