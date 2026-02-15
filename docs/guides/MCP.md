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

The following tools are exposed by the server:

1. **`list_projects`**: Returns a filtered list of projects in the ecosystem.
   - Params: `query`, `tag`, `stack`, `activity`, `vcs_status`.
2. **`get_project_detail`**: Returns full context for a single project by name.
   - Params: `name` (exact).
3. **`search_projects`**: Semantic search across project names, essence, and
   tags.
   - Params: `query`, `tag`.
4. **`get_ecosystem_summary`**: Returns the `SYSTEM_PROMPT.md` overview.
   - Params: `token_limit`.
5. **`get_ecosystem_status`**: Returns health report (VCS, activity).
   - Params: `query`, `tag`.
6. **`get_project_stats`**: Returns disk usage and bloat analytics.
   - Params: `query`, `tag`.

## Troubleshooting

- **Logs:** MCP server logs to `stderr`. Most clients capture these in their
  output panels.
- **Initialization:** Ensure you have run `toad home <path>` at least once to
  initialize the global configuration.
- **Staleness:** AI agents will see a "Context is stale" warning if the physical
  files have changed since the last `toad manifest` run.
