# Specification: MCP Server Mode (110-5)

## Overview

Transform Toad from a passive file generator into a **live context oracle** that
AI agents can query directly via the Model Context Protocol (MCP). The server
exposes Toad's ecosystem knowledge through standardized tools and resources,
enabling any MCP-compatible client (Windsurf, Cursor, Claude Desktop, etc.) to
query project metadata, search across repos, and retrieve structured context
without touching the filesystem directly.

## Sources

- **Strategy:** `docs/releases/v1.1.0/evolution.md` (§ Phase 3.5)
- **Tasks:** `docs/releases/v1.1.0/tasks.md` (§ Phase 3.5)
- **MCP Spec:** <https://modelcontextprotocol.io/specification/2025-11-25>
- **Rust SDK:** <https://github.com/modelcontextprotocol/rust-sdk> (`rmcp`
  crate)
- **Best Practices:**
  <https://modelcontextprotocol.info/docs/best-practices/>

---

## Architecture Decisions

### AD-1: Separate Binary Crate (`bin/toad-mcp`)

The MCP server is a **standalone binary**, not a subcommand of `toad`. This
follows the MCP design principle that "servers should be extremely easy to
build" and keeps the CLI and server lifecycles independent.

- **Binary:** `bin/toad-mcp/` (already scaffolded in workspace)
- **License:** BSL-1.1 (same as toad-discovery, toad-manifest, toad-ops)
- **Depends on:** `toad-core` (MIT), `toad-discovery` (BSL), `toad-manifest`
  (BSL), `toad-ops` (BSL)
- **Does NOT depend on:** `bin/toad` (no CLI coupling)

### AD-2: stdio Transport (Primary)

The MCP spec defines two transports: **stdio** and **Streamable HTTP**. We
implement **stdio first** because:

- All major MCP clients (Windsurf, Cursor, Claude Desktop) support stdio
- stdio is simpler: no HTTP server, no port management, no CORS/Origin
  validation
- The client launches `toad-mcp` as a subprocess — zero configuration
- Streamable HTTP can be added later as a second transport without changing
  tool logic

### AD-3: Tools-First, Resources Later

MCP servers expose three primitive types: **Tools**, **Resources**, and
**Prompts**. We prioritize:

1. **Tools** (v1.1.0) — Model-controlled, schema-validated operations. This is
   where Toad's value lives: querying, filtering, searching.
2. **Resources** (future) — Application-controlled data access. Could expose
   `shadows/` files, `CONTEXT.md`, `MANIFEST.md` as browsable resources.
3. **Prompts** (future) — Reusable templates. Could expose "analyze ecosystem"
   or "plan migration" as structured prompt workflows.

### AD-4: Thin Server, Fat Library

The `toad-mcp` binary is a **thin adapter** between the MCP protocol and the
existing library crates. All business logic lives in `toad-core`,
`toad-discovery`, `toad-ops`, and `toad-manifest`. The server:

- Discovers the workspace via `Workspace::discover()`
- Delegates to library functions (same ones the CLI uses)
- Serializes results as JSON `Content::text()` responses
- Uses `NoOpReporter` for progress (headless execution)

This is the **Data-Service Architecture** mandate (M-4) in action.

### AD-5: Async Wrapper Over Sync Core

The `rmcp` SDK is async (tokio). Toad's library crates are synchronous. We
bridge this with `tokio::task::spawn_blocking()` to avoid blocking the tokio
runtime. This is the standard pattern for Rust MCP servers with sync backends.

### AD-6: Idempotent Read-Only Tools

All tools exposed in v1.1.0 are **read-only**. They query the registry, scan
projects, or generate reports — none mutate state. This satisfies mandate M-3
(Idempotent Discovery) and means:

- No confirmation prompts needed
- Safe to call repeatedly
- No side effects from MCP tool invocations
- Future write tools (e.g., `sync_context`) would require explicit opt-in

### AD-7: Structured Error Handling

Tool errors use the MCP `ErrorData` type with appropriate JSON-RPC error codes:

- `-32602` (Invalid params) — bad tool arguments
- `-32603` (Internal error) — `ToadError` variants mapped to structured
  messages
- Tool execution errors return `CallToolResult` with `is_error: true` and a
  human-readable message, per MCP spec recommendation (SEP-1303)

### AD-8: Server Instructions

The MCP `initialize` response includes an `instructions` field — a natural
language description of what the server does. This is critical for AI agents to
understand the server's purpose. Ours will describe Toad as an ecosystem
context oracle and list available tools with usage hints.

---

## Tool Definitions

### Tool 1: `list_projects`

**Purpose:** Return a filtered list of projects in the ecosystem.

| Parameter    | Type              | Required | Description                        |
| :----------- | :---------------- | :------- | :--------------------------------- |
| `query`      | `string`          | No       | Filter by project name (substring) |
| `tag`        | `string`          | No       | Filter by tag (e.g., `#backend`)   |
| `stack`      | `string`          | No       | Filter by stack name               |
| `activity`   | `string`          | No       | Filter by activity tier            |
| `vcs_status` | `string`          | No       | Filter by VCS status               |

**Returns:** JSON array of `ProjectDetail` objects (name, stack, activity, tags,
path, essence, vcs_status, taxonomy, submodules).

**Implementation:** Load `ProjectRegistry`, apply filters, serialize matching
projects.

### Tool 2: `get_project_detail`

**Purpose:** Return full context for a single project.

| Parameter | Type     | Required | Description          |
| :-------- | :------- | :------- | :------------------- |
| `name`    | `string` | Yes      | Exact project name   |

**Returns:** Single `ProjectDetail` JSON object with all fields, plus the
project's `CONTEXT.md` content if available.

**Implementation:** Load registry, find by name, optionally read
`shadows/{name}/CONTEXT.md`.

### Tool 3: `search_projects`

**Purpose:** Semantic search across project names, essence, tags, and taxonomy.

| Parameter | Type     | Required | Description              |
| :-------- | :------- | :------- | :----------------------- |
| `query`   | `string` | Yes      | Search term              |
| `tag`     | `string` | No       | Narrow search by tag     |

**Returns:** `SearchResult` JSON with matched projects, match reasons, and
relevance context.

**Implementation:** Delegates to `toad_discovery::search_projects()`.

### Tool 4: `get_ecosystem_summary`

**Purpose:** Return the system-prompt-tier overview of the entire ecosystem.

| Parameter     | Type      | Required | Description                    |
| :------------ | :-------- | :------- | :----------------------------- |
| `token_limit` | `integer` | No       | Max tokens (default from config) |

**Returns:** The `SYSTEM_PROMPT.md` content as text, truncated to token limit.

**Implementation:** Load config, generate or read cached `SYSTEM_PROMPT.md`.

### Tool 5: `get_ecosystem_status`

**Purpose:** Return ecosystem health — git status, staleness, activity tiers.

| Parameter | Type     | Required | Description                        |
| :-------- | :------- | :------- | :--------------------------------- |
| `query`   | `string` | No       | Filter by project name (substring) |
| `tag`     | `string` | No       | Filter by tag                      |

**Returns:** `StatusReport` JSON with per-project VCS status, activity, and
alignment issues.

**Implementation:** Delegates to `toad_discovery::generate_status_report()`.

### Tool 6: `get_project_stats`

**Purpose:** Return disk usage analytics for the ecosystem.

| Parameter | Type     | Required | Description                        |
| :-------- | :------- | :------- | :--------------------------------- |
| `query`   | `string` | No       | Filter by project name (substring) |
| `tag`     | `string` | No       | Filter by tag                      |

**Returns:** `AnalyticsReport` JSON with per-project sizes, bloat index, and
totals.

**Implementation:** Delegates to `toad_ops::stats::generate_analytics_report()`.

---

## Non-Goals (v1.1.0)

- **No Streamable HTTP transport** — stdio is sufficient for local MCP clients.
- **No MCP Resources** — tools cover the v1.1.0 use cases. Resources can be
  added when we want to expose `shadows/` files as browsable content.
- **No MCP Prompts** — prompts are a future enhancement for structured
  workflows.
- **No write/mutation tools** — all tools are read-only. `sync_context` or
  `run_command` would be Phase 5+.
- **No authentication** — stdio transport is local-only, launched by the client
  as a subprocess. Auth is not needed.
- **No caching layer** — the registry is already file-based and fast. In-memory
  caching can be added if profiling shows need.

---

## Client Configuration

### Windsurf / Cursor / Claude Desktop

The MCP server is configured in the client's MCP settings file:

```json
{
  "mcpServers": {
    "toad": {
      "command": "toad-mcp",
      "args": [],
      "env": {
        "TOAD_HOME": "~/.toad"
      }
    }
  }
}
```

If `toad-mcp` is installed globally (via `cargo install` or the install
script), no path is needed. Otherwise, use the full path to the binary.

---

## Testing Strategy

### Unit Tests

- Each tool handler function tested in isolation with mock workspace data
- Error paths tested: missing workspace, missing project, invalid params

### Integration Tests

- Full MCP lifecycle: initialize → list tools → call tool → verify response
- Use `rmcp` client SDK to drive the server as a subprocess
- Verify JSON-RPC compliance and schema correctness

### Manual Testing

- Use the **MCP Inspector** (`npx @modelcontextprotocol/inspector`) to
  interactively test tools during development
- Verify in Windsurf/Cursor with real ecosystem data

---

## Dependencies

```toml
[dependencies]
rmcp = { version = "0.8", features = ["server", "transport-io"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "1"
anyhow = "1"
toad-core = { path = "../../crates/toad-core" }
toad-discovery = { path = "../../crates/toad-discovery" }
toad-manifest = { path = "../../crates/toad-manifest" }
toad-ops = { path = "../../crates/toad-ops" }
```

---

## File Structure

```text
bin/toad-mcp/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point: stdio transport setup
│   ├── server.rs        # ToadService struct + ServerHandler impl
│   ├── tools.rs         # Tool definitions (list_projects, etc.)
│   └── errors.rs        # ToadError → MCP ErrorData mapping
```

Four files. Clean separation. Each under 200 lines.
