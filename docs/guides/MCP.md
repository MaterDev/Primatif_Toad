# Toad MCP Server

Toad can be used as a **live context oracle** via the Model Context Protocol
(MCP). This allows AI agents in compatible editors (Windsurf, Cursor, Claude
Desktop) to query your ecosystem directly.

## Installation

The MCP server is part of the `toad` workspace. You can install it using:

```bash
cargo install --path bin/toad-mcp
```

## Configuration

Add the following to your editor's MCP settings:

### Windsurf

Open `~/.codeium/windsurf/mcp_config.json` and add:

```json
{
  "mcpServers": {
    "toad": {
      "command": "toad-mcp",
      "env": {
        "TOAD_HOME": "/Users/your-user/.toad"
      }
    }
  }
}
```

### Cursor

Open Cursor Settings -> MCP and add a new server:

- **Name:** `toad`
- **Type:** `command`
- **Command:** `toad-mcp`

### Claude Desktop

Open `~/Library/Application Support/Claude/claude_desktop_config.json` and add:

```json
{
  "mcpServers": {
    "toad": {
      "command": "toad-mcp"
    }
  }
}
```

## Available Tools

The following tools are exposed by the server, grouped by functional category.

### 🔍 Discovery Tools

Tools for exploring the ecosystem and finding projects.

1. **`list_projects`**: Returns basic metadata for all projects.
   - **Params:** `query`, `tag`, `stack`, `activity`, `vcs_status`.
2. **`search_projects`**: Ranked semantic search across names, essence
   (READMEs), and tags.
   - **Params:** `query` (term), `tag` (filter).
3. **`search_projects_by_dna`**: Find projects by architectural patterns (roles,
   capabilities).
   - **Params:** `query` (pattern), `tag`.
4. **`get_ecosystem_summary`**: High-level token-budgeted overview of all
   projects.
   - **Params:** `token_limit`.
5. **`get_ecosystem_status`**: Global health report showing VCS state and
   activity tiers.
   - **Params:** `query`, `tag`.
6. **`reveal_projects`**: Search for projects matching a query. Returns names,
   paths, and basic metadata.
   - **Params:** `query`, `tag`.
7. **`get_git_status`**: Consolidated Git status across all repositories. Shows
   uncommitted changes and branch info.
   - **Params:** `query`, `tag`.
8. **`list_branches`**: List all branches across projects. Shows current branch
   and available local/remote branches.
   - **Params:** `query`, `tag`, `all` (show remote).

### 🧠 Context Tools

Tools for deep-diving into project architecture and implementation.

1. **`get_project_detail`**: Full metadata for a specific project.
   - **Params:** `name` (exact).
2. **`get_project_dna`**: Structural patterns only (roles, capabilities,
   patterns).
   - **Params:** `name`.
3. **`get_project_context`**: Direct access to the project's `CONTEXT.md`.
   - **Params:** `name`.
4. **`get_atlas`**: Direct access to `ATLAS.json` (DNA map for all projects).
5. **`get_manifest`**: Direct access to `MANIFEST.md` (detailed project table).
6. **`generate_manifest`**: Re-generate AI context files (refreshes intuition).
   - **Params:** `project` (optional focus).

### ⚙️ Management Tools

Tools for managing Toad's runtime environment.

1. **`get_active_context`**: Identify the current workspace root (Hub or Pond).
2. **`list_contexts`**: List all registered workspace contexts.
3. **`switch_context`**: Change the active workspace root.
   - **Params:** `name`.
4. **`sync_registry`**: Rebuild the project registry cache by scanning the
   workspace.
5. **`tag_projects`**: Assign or harvest tags for projects.
   - **Params:** `project`, `tag`, `query`, `filter_tag`, `harvest`.
6. **`register_context`**: Register a new projects directory as a context.
   - **Params:** `name`, `path`.

### 📊 Analysis Tools

Advanced ecosystem insights and resource auditing.

1. **`analyze_dependencies`**: Analyze project relationships and identify the
   critical path.
   - **Params:** `query`.
2. **`analyze_velocity`**: Track development momentum and activity metrics.
   - **Params:** `days`, `query`.
3. **`analyze_debt`**: Identify technical debt, monolithic files, and code
   smells.
   - **Params:** `query`.
4. **`analyze_health`**: Calculate composite 0-100 health scores with proactive
   advice.
   - **Params:** `query`.
5. **`analyze_trends`**: Analyze historical trends (health, storage, activity).
   - **Params:** `days`.
6. **`analyze_patterns`**: Detect cross-project architectural consistency and
   patterns.
7. **`analyze_submodules`**: Audit submodule alignment and branch consistency.
8. **`get_project_stats`**: Detailed disk usage and bloat analytics.
   - **Params:** `query`, `tag`.
9. **`get_disk_stats`**: Ecosystem-wide storage audit (build artifacts vs
   source).
   - **Params:** `query`, `tag`, `all` (show details).
10. **`compare_projects`**: Architectural compatibility and migration
    pre-flight.
    - **Params:** `source`, `target`.
11. **`run_health_check`**: Run comprehensive Toad diagnostics (Doctor command).

## Common Workflows

### Analyze a Specific Project

1. `get_project_detail` — Get basic metadata and stack.
2. `get_project_context` — Read entry points and core logic.
3. `get_project_dna` — Understand architectural patterns.

### Find Projects by Capability

1. `search_projects_by_dna` with query: "REST API".
2. Filter results by stack or activity.
3. `get_project_context` for the most relevant matches.

### Plan a Migration

1. `compare_projects` with source and target names.
2. `get_project_context` for both projects to see implementation details.
3. Review compatibility score and mismatched capabilities.

## Troubleshooting

- **Logs:** MCP server logs to `stderr`. Most clients capture these in their
  output panels.
- **Initialization:** Ensure you have run `toad home <path>` at least once to
  initialize the global configuration.
- **Staleness:** AI agents will see a "Context is stale" warning if the physical
  files have changed since the last `toad manifest` run.
