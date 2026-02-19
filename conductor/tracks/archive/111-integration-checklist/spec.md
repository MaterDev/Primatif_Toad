# Specification: Integration Checklist (111-integration-checklist)

## Overview

Ensure all new v1.1.1 features are properly integrated across MCP,
documentation, and existing feature patterns. This is a quality gate to maintain
consistency and discoverability.

## Sources

- **User Request:** "make sure anything new that we are adding are also going to
  be discoverable within the mcp server, included in docs, and also aligned with
  our other features"
- **All v1.1.1 Tracks**

---

## Problem Statement

New features can be added in isolation without proper integration:

- ❌ MCP tools not exposing new functionality
- ❌ Documentation not updated
- ❌ Inconsistent patterns across features
- ❌ Missing --json support
- ❌ README not reflecting new capabilities

**Risk:** Users and AI agents won't discover new features, reducing their value.

---

## Integration Requirements

### 1. **MCP Discoverability**

Every new CLI command MUST have corresponding MCP tools:

| CLI Command               | MCP Tool               | Status     |
| ------------------------- | ---------------------- | ---------- |
| `toad analyze deps`       | `analyze_dependencies` | ✅ Planned |
| `toad analyze velocity`   | `analyze_velocity`     | ✅ Planned |
| `toad analyze debt`       | `analyze_debt`         | ✅ Planned |
| `toad analyze health`     | `analyze_health`       | ✅ Planned |
| `toad analyze trends`     | `analyze_trends`       | ⚠️ Add      |
| `toad analyze patterns`   | `analyze_patterns`     | ⚠️ Add      |
| `toad analyze submodules` | `analyze_submodules`   | ⚠️ Add      |
| `toad doctor`             | `run_health_check`     | ⚠️ Add      |
| `toad reveal`             | `reveal_projects`      | ✅ Planned |
| `toad ggit status`        | `get_git_status`       | ✅ Planned |

**Action Items:**

- Add missing MCP tools for trends, patterns, submodules
- Add `run_health_check` tool for doctor command
- Verify all tools have enhanced descriptions
- Test all tools via MCP client

### 2. **Documentation Coverage**

Every new feature MUST be documented in:

| Feature   | CLI.md | MCP.md | USER_GUIDE.md | README.md | Skills |
| --------- | ------ | ------ | ------------- | --------- | ------ |
| Analytics | ⚠️      | ⚠️      | ⚠️             | ⚠️         | ⚠️      |
| Doctor    | ⚠️      | ⚠️      | ⚠️             | ⚠️         | ⚠️      |
| MCP Tools | ✅     | ⚠️      | ⚠️             | ⚠️         | ⚠️      |
| Workflows | N/A    | N/A    | ⚠️             | ⚠️         | ⚠️      |

**Action Items:**

- Update CLI.md with `analyze` and `doctor` commands
- Update MCP.md with all 25+ tools
- Add analytics section to USER_GUIDE.md
- Update README.md "What's New" and "Core Commands"
- Regenerate skills with new commands

### 3. **Feature Alignment**

All new features MUST follow existing patterns:

#### **CLI Patterns**

- ✅ Use `--json` flag for structured output
- ✅ Use `--no-sync` flag to skip auto-refresh
- ✅ Use `-q/--query` for filtering
- ✅ Use `-t/--tag` for tag filtering
- ✅ Use colored output (colored crate)
- ✅ Use consistent status indicators (✅/⚠️/❌)

#### **MCP Patterns**

- ✅ Use `Parameters<T>` wrapper
- ✅ Use `tokio::task::spawn_blocking` for sync work
- ✅ Return `CallToolResult::success(vec![Content::text(...)])`
- ✅ Convert errors via `toad_error_to_mcp`
- ✅ Include usage hints in descriptions

#### **Error Handling**

- ✅ Use `ToadResult<T>` in library crates
- ✅ Use `anyhow::Result` in binary crates
- ✅ Provide helpful error messages
- ✅ Suggest corrective actions

#### **Output Formatting**

- ✅ Use `ui::format_*` functions for CLI output
- ✅ Support JSON serialization for all reports
- ✅ Use consistent table formatting
- ✅ Use consistent color scheme

### 4. **README Update Requirements**

The README.md MUST be updated with:

#### **Version Badge**

```markdown
[![Version: v1.1.1](https://img.shields.io/badge/version-v1.1.1-green.svg)](Cargo.toml)
```

#### **What's New Section**

```markdown
## What's New in v1.1.1 "Polish & Insights"

- **Rich Analytics:** Dependency graphs, velocity metrics, technical debt
  indicators, and health scoring
- **Health Checks:** `toad doctor` command for troubleshooting and validation
- **Complete MCP:** 25+ tools exposing full Toad functionality to AI agents
- **CI/CD Pipeline:** Automated testing and release workflows
- **Enhanced Documentation:** Comprehensive user guide and updated skills
- **Dogfooding:** Toad uses its own features for development
```

#### **Core Commands Section**

Add new commands:

```markdown
### Analytics & Insights

- **`toad analyze deps`** — Dependency graph and critical path analysis
- **`toad analyze velocity`** — Development velocity and activity trends
- **`toad analyze debt`** — Technical debt indicators and recommendations
- **`toad analyze health`** — Project health scoring with actionable insights
- **`toad doctor`** — Health check and troubleshooting diagnostics
```

---

## Implementation Checklist

### Phase 1: MCP Integration Audit

- [ ] List all new CLI commands
- [ ] Verify each has corresponding MCP tool
- [ ] Add missing MCP tools:
  - [ ] `analyze_trends`
  - [ ] `analyze_patterns`
  - [ ] `analyze_submodules`
  - [ ] `run_health_check`
- [ ] Test all MCP tools via client
- [ ] Verify parameter schemas complete
- [ ] Verify error handling consistent

### Phase 2: Documentation Audit

- [ ] Update CLI.md
  - [ ] Add `analyze` command section
  - [ ] Add `doctor` command section
  - [ ] Regenerate via `toad docs`
- [ ] Update MCP.md
  - [ ] List all 25+ tools
  - [ ] Add tool descriptions
  - [ ] Add common workflows
  - [ ] Add examples
- [ ] Update USER_GUIDE.md
  - [ ] Add analytics section
  - [ ] Add troubleshooting with doctor
  - [ ] Add workflow examples
- [ ] Update README.md
  - [ ] Update version badge
  - [ ] Update "What's New"
  - [ ] Update "Core Commands"
  - [ ] Add analytics section
- [ ] Update skills
  - [ ] Regenerate blueprint
  - [ ] Regenerate CLI skill
  - [ ] Regenerate MCP skill
  - [ ] Run `toad skill sync`

### Phase 3: Feature Alignment Audit

- [ ] Verify all commands support `--json`
- [ ] Verify all commands support `--no-sync`
- [ ] Verify all commands use consistent filtering
- [ ] Verify all commands use colored output
- [ ] Verify all commands use consistent status indicators
- [ ] Verify all MCP tools follow patterns
- [ ] Verify all error messages are helpful

### Phase 4: Integration Testing

- [ ] Test analytics via CLI
- [ ] Test analytics via MCP
- [ ] Test doctor via CLI
- [ ] Test doctor via MCP (if exposed)
- [ ] Test all new MCP tools
- [ ] Verify JSON output works
- [ ] Verify --no-sync works
- [ ] Test on Toad itself

---

## Success Criteria

- [ ] All new CLI commands have MCP tools
- [ ] All features documented in relevant guides
- [ ] README.md updated with v1.1.1 features
- [ ] All features follow existing patterns
- [ ] All features support --json flag
- [ ] All MCP tools tested and working
- [ ] Skills updated with new commands
- [ ] Integration testing complete

---

## MCP Tool Inventory (Target: 25+ tools)

### Existing (13 tools)

1. `list_projects`
2. `get_project_detail`
3. `get_project_dna`
4. `compare_projects`
5. `search_projects_by_dna`
6. `search_projects`
7. `get_ecosystem_summary`
8. `get_ecosystem_status`
9. `get_project_stats`
10. `get_active_context`
11. `list_contexts`
12. `switch_context`
13. `get_current_context`

### Planned - MCP Enhancements (3 tools)

1. `get_atlas`
2. `get_manifest`
3. `get_project_context`

### Planned - CLI Bridge (8 tools)

1. `reveal_projects`
2. `get_git_status`
3. `get_disk_stats`
4. `list_branches`
5. `sync_registry`
6. `generate_manifest`
7. `tag_projects`
8. `register_context`

### Planned - Analytics (7 tools)

1. `analyze_dependencies`
2. `analyze_velocity`
3. `analyze_debt`
4. `analyze_health`
5. `analyze_trends`
6. `analyze_patterns`
7. `analyze_submodules`

### Missing - Doctor (1 tool)

1. `run_health_check`

**Total: 32 tools** (exceeds 25+ target ✅)

---

## Documentation Inventory

### CLI.md

- Current: Basic command list
- Needed: Detailed sections for analyze, doctor
- Action: Regenerate via `toad docs` after implementation

### MCP.md

- Current: 13 tools documented
- Needed: All 32 tools with descriptions and workflows
- Action: Manual update with examples

### USER_GUIDE.md

- Current: Doesn't exist yet
- Needed: Complete guide with all features
- Action: Create per 111-user-guide track

### README.md

- Current: v1.1.0 features
- Needed: v1.1.1 features, updated commands
- Action: Update "What's New" and "Core Commands"

### Skills

- Current: Blueprint, CLI, MCP (if exists)
- Needed: All three updated with v1.1.1 features
- Action: Run `toad skill sync` after updates

---

## Alignment Patterns Reference

### CLI Output Pattern

```rust
// Colored status indicators
println!("{} {}", "SUCCESS:".green().bold(), message);
println!("{} {}", "WARNING:".yellow().bold(), message);
println!("{} {}", "ERROR:".red().bold(), message);

// JSON support
if json {
    println!("{}", serde_json::to_string_pretty(&result)?);
} else {
    ui::format_report(&result);
}
```

### MCP Tool Pattern

```rust
#[tool(description = "Clear description with usage hints. Mention related tools.")]
pub async fn tool_name(
    &self,
    params: Parameters<ParamsType>,
) -> Result<CallToolResult, McpError> {
    let result = tokio::task::spawn_blocking(move || {
        let ws = Workspace::discover()?;
        // Do work
        Ok::<_, ToadError>(output)
    })
    .await
    .map_err(|e| toad_error_to_mcp(ToadError::Other(e.to_string())))?
    .map_err(toad_error_to_mcp)?;
    
    Ok(CallToolResult::success(vec![Content::text(result)]))
}
```

### Error Message Pattern

```rust
// Helpful error with corrective action
return Err(ToadError::Other(format!(
    "Registry not found. Run 'toad sync' to build registry."
)));
```

---

## Future Enhancements

- Automated integration testing
- Documentation linting
- MCP tool discovery automation
- Pattern enforcement via lints
