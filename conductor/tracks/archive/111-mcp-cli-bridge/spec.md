# Specification: MCP CLI Bridge (111-mcp-cli-bridge)

## Overview

Expose the full Toad CLI functionality through the MCP server, allowing AI
agents to execute any Toad command via MCP tools. Dangerous operations (those
requiring confirmation prompts) are excluded for safety.

## Sources

- **Current MCP Implementation:** `bin/toad-mcp/src/server.rs`
- **CLI Commands:** `bin/toad/src/cli.rs` and `bin/toad/src/commands/`
- **Pre-release Review:** Final v1.1.0 check findings

---

## Problem Statement

The MCP server currently exposes only **read-only query tools**:

- ✅ List/search projects
- ✅ Get ecosystem status
- ✅ Get project details
- ❌ No write operations (tag, clean, do, etc.)
- ❌ No git operations (ggit commands)
- ❌ No context management (project register/switch)

AI agents cannot perform actions, only query state.

---

## Goals

1. **Expose Safe CLI Commands** — All read-only and low-risk operations
2. **Exclude Dangerous Operations** — Commands that modify state without
   confirmation
3. **Maintain Safety** — AI agents cannot accidentally delete data or run
   destructive commands
4. **Preserve CLI Semantics** — MCP tools behave identically to CLI commands

---

## Non-Goals

- Exposing dangerous operations (clean, do, delete context)
- Interactive prompts (MCP is non-interactive)
- File system access beyond Toad's workspace

---

## Architecture Decisions

### AD-1: Safe vs. Dangerous Operations

**Safe Operations (Expose via MCP):**

- ✅ `toad reveal` — Search projects
- ✅ `toad status` — Git status across repos
- ✅ `toad stats` — Disk usage analytics
- ✅ `toad sync` — Rebuild registry cache
- ✅ `toad manifest` — Generate context files
- ✅ `toad ggit status` — Multi-repo git status
- ✅ `toad ggit branches` — List branches
- ✅ `toad project list` — List contexts
- ✅ `toad project switch` — Change active context
- ✅ `toad strategy list` — List stack strategies

**Dangerous Operations (Exclude from MCP):**

- ❌ `toad do` — Execute arbitrary shell commands
- ❌ `toad clean` — Delete build artifacts
- ❌ `toad tag --harvest` — Bulk tag assignment (safe but needs review)
- ❌ `toad untag` — Remove tags
- ❌ `toad project delete` — Delete context
- ❌ `toad ggit commit` — Commit changes
- ❌ `toad ggit push` — Push to remote
- ❌ `toad ggit pull` — Pull from remote
- ❌ `toad ggit sync` — Sync and align submodules
- ❌ `toad ggit checkout` — Change branches
- ❌ `toad cw run` — Execute custom workflows

**Rationale:**

- Commands that modify git state are excluded (AI shouldn't commit/push)
- Commands that delete data are excluded (AI shouldn't clean/delete)
- Commands that execute arbitrary code are excluded (AI shouldn't run shell
  commands)

### AD-2: Read-Only Git Operations

Git **read** operations are safe:

- `ggit status` — See what's changed
- `ggit branches` — List branches

Git **write** operations are dangerous:

- `ggit commit/push/pull/sync/checkout` — Modify state

### AD-3: Context Management

Context switching is **safe** because it only changes which registry is loaded:

- ✅ `project list` — View contexts
- ✅ `project switch` — Change active context
- ❌ `project delete` — Dangerous (deletes cached data)
- ✅ `project register` — Safe (creates new context)

### AD-4: Thin Wrapper Pattern

MCP tools delegate to existing CLI command handlers:

```rust
#[tool(description = "Rebuild the project registry cache")]
pub async fn sync_registry(&self, _params: Parameters<NoParams>) -> Result<CallToolResult, McpError> {
    let result = tokio::task::spawn_blocking(move || {
        let ws = Workspace::discover()?;
        // Call the same function the CLI uses
        toad_discovery::scan_all_projects(&ws)?;
        // Save registry
        // Return success message
        Ok::<_, toad_core::ToadError>("Registry synchronized".to_string())
    })
    .await
    .map_err(|e| crate::errors::toad_error_to_mcp(toad_core::ToadError::Other(e.to_string())))?
    .map_err(crate::errors::toad_error_to_mcp)?;
    
    Ok(CallToolResult::success(vec![Content::text(result)]))
}
```

### AD-5: No Confirmation Prompts

All MCP tools are **non-interactive**. Commands that normally require `--yes`
flag are either:

1. **Excluded** (if dangerous)
2. **Auto-confirmed** (if safe, like `tag --harvest`)

---

## Implementation Plan

### Phase 1: Add Safe Read Operations (20 min)

**File:** `bin/toad-mcp/src/server.rs`

Add tools for safe read-only operations:

```rust
#[tool(description = "Search for projects matching a query. Returns project names and basic info.")]
pub async fn reveal_projects(
    &self,
    params: Parameters<RevealParams>,
) -> Result<CallToolResult, McpError> {
    // Delegate to toad_discovery::find_projects
}

#[tool(description = "Get Git status across all projects. Shows uncommitted changes, unpushed commits, and branch info.")]
pub async fn get_git_status(
    &self,
    params: Parameters<StatusParams>,
) -> Result<CallToolResult, McpError> {
    // Delegate to toad_git::generate_multi_repo_status
}

#[tool(description = "Get disk usage analytics. Shows project sizes, build artifacts, and bloat analysis.")]
pub async fn get_disk_stats(
    &self,
    params: Parameters<StatsParams>,
) -> Result<CallToolResult, McpError> {
    // Delegate to toad_ops::generate_analytics_report
}

#[tool(description = "List all branches across projects. Shows current branch and available branches.")]
pub async fn list_branches(
    &self,
    params: Parameters<BranchesParams>,
) -> Result<CallToolResult, McpError> {
    // Delegate to toad_git::branches::list_local_branches
}
```

### Phase 2: Add Safe Write Operations (15 min)

```rust
#[tool(description = "Rebuild the project registry cache. Run this after adding/removing projects.")]
pub async fn sync_registry(
    &self,
    _params: Parameters<NoParams>,
) -> Result<CallToolResult, McpError> {
    // Delegate to toad_discovery::scan_all_projects
}

#[tool(description = "Generate AI context files (MANIFEST.md, ATLAS.json, CONTEXT.md). Run this after project changes.")]
pub async fn generate_manifest(
    &self,
    params: Parameters<ManifestParams>,
) -> Result<CallToolResult, McpError> {
    // Delegate to manifest generation logic
}

#[tool(description = "Switch to a different project context. Changes which workspace is active.")]
pub async fn switch_context(
    &self,
    params: Parameters<SwitchContextParams>,
) -> Result<CallToolResult, McpError> {
    // Already exists, verify it's working
}

#[tool(description = "Register a new project context. Creates a new workspace configuration.")]
pub async fn register_context(
    &self,
    params: Parameters<RegisterContextParams>,
) -> Result<CallToolResult, McpError> {
    // Delegate to project registration logic
}
```

### Phase 3: Add Conditional Safe Operations (10 min)

Some operations are safe with proper parameters:

```rust
#[tool(description = "Assign a tag to projects. Use query/tag filters to target specific projects. Auto-harvest mode detects stack tags automatically.")]
pub async fn tag_projects(
    &self,
    params: Parameters<TagParams>,
) -> Result<CallToolResult, McpError> {
    // Safe because it's additive (doesn't delete anything)
    // Auto-confirm (no prompt)
}
```

### Phase 4: Update Parameter Schemas (10 min)

Add parameter structs for new tools:

```rust
#[derive(Deserialize, JsonSchema)]
pub struct RevealParams {
    /// Search query (case-insensitive)
    pub query: String,
    /// Optional tag filter
    pub tag: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct StatusParams {
    /// Optional query to filter projects
    pub query: Option<String>,
    /// Optional tag filter
    pub tag: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct StatsParams {
    /// Optional query to filter projects
    pub query: Option<String>,
    /// Optional tag filter
    pub tag: Option<String>,
    /// Show details for all projects
    pub all: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct BranchesParams {
    /// Optional query to filter projects
    pub query: Option<String>,
    /// Optional tag filter
    pub tag: Option<String>,
    /// Show remote branches
    pub all: Option<bool>,
}

#[derive(Deserialize, JsonSchema)]
pub struct ManifestParams {
    /// Optional project name for project-specific context
    pub project: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct RegisterContextParams {
    /// Context name
    pub name: String,
    /// Absolute path to projects directory
    pub path: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct TagParams {
    /// Project name (optional if using filters)
    pub project: Option<String>,
    /// Tag name
    pub tag: Option<String>,
    /// Filter by name query
    pub query: Option<String>,
    /// Filter by existing tag
    pub filter_tag: Option<String>,
    /// Auto-harvest stack tags
    pub harvest: Option<bool>,
}
```

---

## Tool Descriptions (Enhanced)

All new tools get enhanced descriptions with usage hints:

```rust
#[tool(description = "Search for projects matching a query. Returns project names, paths, and basic metadata. Use this for quick discovery. For detailed info, follow up with get_project_detail.")]
pub async fn reveal_projects(...)

#[tool(description = "Get Git status across all projects. Shows uncommitted changes, unpushed commits, branch info, and submodule alignment. Use query/tag filters to narrow scope. For branch operations, use list_branches.")]
pub async fn get_git_status(...)

#[tool(description = "Get disk usage analytics for projects. Shows total size, build artifacts, and bloat analysis. Use this to identify cleanup opportunities. Note: This tool does NOT delete anything - it only reports.")]
pub async fn get_disk_stats(...)

#[tool(description = "Rebuild the project registry cache by scanning the workspace. Run this after adding/removing projects or when registry is stale. This is a safe operation that only updates cached metadata.")]
pub async fn sync_registry(...)

#[tool(description = "Generate AI context files (MANIFEST.md, ATLAS.json, SYSTEM_PROMPT.md, CONTEXT.md). Run this after project changes to refresh context. Optionally specify a project name for project-specific deep dive.")]
pub async fn generate_manifest(...)
```

---

## Safety Matrix

| Command                 | MCP Tool               | Safe? | Rationale                   |
| ----------------------- | ---------------------- | ----- | --------------------------- |
| `toad reveal`           | ✅ `reveal_projects`   | Yes   | Read-only                   |
| `toad status`           | ✅ `get_git_status`    | Yes   | Read-only                   |
| `toad stats`            | ✅ `get_disk_stats`    | Yes   | Read-only                   |
| `toad sync`             | ✅ `sync_registry`     | Yes   | Updates cache only          |
| `toad manifest`         | ✅ `generate_manifest` | Yes   | Generates files in shadows/ |
| `toad tag`              | ✅ `tag_projects`      | Yes   | Additive only               |
| `toad untag`            | ❌                     | No    | Removes data                |
| `toad do`               | ❌                     | No    | Arbitrary code execution    |
| `toad clean`            | ❌                     | No    | Deletes files               |
| `toad ggit status`      | ✅ `get_git_status`    | Yes   | Read-only                   |
| `toad ggit branches`    | ✅ `list_branches`     | Yes   | Read-only                   |
| `toad ggit commit`      | ❌                     | No    | Modifies git state          |
| `toad ggit push`        | ❌                     | No    | Modifies remote             |
| `toad ggit pull`        | ❌                     | No    | Modifies local              |
| `toad ggit sync`        | ❌                     | No    | Modifies submodules         |
| `toad ggit checkout`    | ❌                     | No    | Changes branches            |
| `toad project list`     | ✅ (exists)            | Yes   | Read-only                   |
| `toad project switch`   | ✅ (exists)            | Yes   | Changes config only         |
| `toad project register` | ✅ `register_context`  | Yes   | Creates config              |
| `toad project delete`   | ❌                     | No    | Deletes data                |
| `toad cw run`           | ❌                     | No    | Arbitrary code execution    |

---

## Success Criteria

- [ ] All safe read operations exposed via MCP
- [ ] Safe write operations (sync, manifest, tag) exposed
- [ ] Context management tools (list, switch, register) exposed
- [ ] No dangerous operations exposed
- [ ] All tools have enhanced descriptions with usage hints
- [ ] Parameter schemas are complete with doc comments
- [ ] Tools delegate to existing CLI logic (no duplication)
- [ ] MCP server builds and passes tests

---

## Integration Points

- **Depends on:** `toad-discovery`, `toad-git`, `toad-ops`, `toad-core`
- **Consumed by:** AI agents via MCP clients
- **Testing:** Manual testing with MCP client + unit tests

---

## Risks & Mitigations

| Risk                                   | Mitigation                                |
| -------------------------------------- | ----------------------------------------- |
| AI accidentally runs dangerous command | Exclude all dangerous operations from MCP |
| AI bypasses safety via `do` command    | `do` is explicitly excluded               |
| AI modifies git state                  | All git write operations excluded         |
| AI deletes data                        | `clean` and `delete` excluded             |
| Parameter validation                   | Use JsonSchema for strict validation      |

---

## Future Enhancements (v1.2.0+)

- **Approval workflow** — AI requests dangerous operation, user approves
- **Dry-run mode** — Preview what a command would do
- **Audit log** — Track all MCP operations
- **Rate limiting** — Prevent abuse
