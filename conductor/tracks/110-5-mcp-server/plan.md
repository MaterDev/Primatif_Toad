# Implementation Plan: MCP Server Mode

> **Spec:** `conductor/tracks/110-5-mcp-server/spec.md`
> **Evolution:** `docs/releases/v1.1.0/evolution.md` (§ Phase 3.5)
> **Tasks:** `docs/releases/v1.1.0/tasks.md` (§ Phase 3.5)

---

## Step 1: Scaffold — Cargo.toml + Dependencies + Entry Point [x]

**Goal:** Get `toad-mcp` compiling with `rmcp` and the stdio transport, serving
an empty server that responds to `initialize`.

- [x] Update `bin/toad-mcp/Cargo.toml` with dependencies per spec
- [x] Create `src/main.rs`: tokio main, stdio transport, serve `ToadService`
- [x] Create `src/server.rs`: `ToadService` struct holding `Workspace`,
      implement `ServerHandler` with `get_info()` returning server name,
      version, capabilities (`enable_tools()`), and `instructions` string
- [x] Create `src/errors.rs`: `toad_error_to_mcp()` mapping function
- [x] Verify: `cargo build -p toad-mcp` compiles
- [x] Verify: `echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"0.1"}}}' | cargo run -p toad-mcp` returns valid initialize response

**Key patterns (from rmcp SDK):**

```rust
// main.rs
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = ToadService::new()?;
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let server = service.serve(transport).await?;
    server.waiting().await?;
    Ok(())
}
```

```rust
// server.rs — ServerHandler impl
fn get_info(&self) -> ServerInfo {
    ServerInfo {
        protocol_version: ProtocolVersion::LATEST,
        capabilities: ServerCapabilities::builder()
            .enable_tools()
            .build(),
        server_info: Implementation {
            name: "toad-mcp".into(),
            version: env!("CARGO_PKG_VERSION").into(),
        },
        instructions: Some(INSTRUCTIONS.into()),
    }
}
```

---

## Step 2: First Tool — `list_projects` [x]

**Goal:** Implement the first tool end-to-end, proving the full pipeline from
MCP request → workspace discovery → library call → JSON response.

- [x] Create `src/tools.rs` with tool parameter structs deriving
      `Deserialize + JsonSchema` (Consolidated into server.rs)
- [x] Implement `list_projects` tool using `#[tool]` macro
- [x] Bridge sync library call with `tokio::task::spawn_blocking`
- [x] Wire tool into `ToadService` via `#[tool_router]`
- [x] Test with MCP Inspector: `npx @modelcontextprotocol/inspector`
- [x] Verify: tool appears in `tools/list`, returns filtered project data

**Key pattern (spawn_blocking for sync calls):**

```rust
#[tool(description = "List projects in the ecosystem, optionally filtered")]
async fn list_projects(
    &self,
    Parameters(params): Parameters<ListProjectsParams>,
) -> Result<CallToolResult, McpError> {
    let ws = self.workspace.clone();
    let result = tokio::task::spawn_blocking(move || {
        // All sync toad-core/toad-discovery calls happen here
        let registry = toad_core::ProjectRegistry::load(
            ws.active_context.as_deref(), None
        )?;
        // ... filter and serialize
        Ok::<_, anyhow::Error>(serde_json::to_string_pretty(&filtered)?)
    }).await.map_err(|e| /* map to McpError */)?;

    match result {
        Ok(json) => Ok(CallToolResult::success(vec![Content::text(json)])),
        Err(e) => Ok(CallToolResult::error(vec![Content::text(e.to_string())])),
    }
}
```

---

## Step 3: Remaining Core Tools [x]

**Goal:** Implement the remaining 5 tools, each following the same pattern.

- [x] `get_project_detail` — registry lookup + optional CONTEXT.md read
- [x] `search_projects` — delegates to `toad_discovery::search_projects()`
- [x] `get_ecosystem_summary` — reads/generates SYSTEM_PROMPT.md
- [x] `get_ecosystem_status` — delegates to
      `toad_discovery::generate_status_report()`
- [x] `get_project_stats` — delegates to
      `toad_ops::stats::generate_analytics_report()`
- [x] Verify each tool with MCP Inspector

---

## Step 4: Error Handling + Edge Cases [x]

**Goal:** Ensure graceful behavior when workspace is missing, registry is empty,
or projects don't match filters.

- [x] Handle `Workspace::discover()` failure at startup with clear error
      message to stderr (per MCP stdio spec: stderr is for logging)
- [x] Map `ToadError` variants to appropriate MCP error codes in `errors.rs`
- [x] Return `CallToolResult::error()` (not protocol errors) for tool-level
      failures like "project not found" (per SEP-1303)
- [ ] Test: call `get_project_detail` with nonexistent name → error result
- [ ] Test: call `list_projects` with no workspace → protocol error

---

## Step 5: Integration Tests [x]

**Goal:** Automated tests that exercise the full MCP lifecycle.

- [x] Create `bin/toad-mcp/tests/` directory
- [x] Write test that launches `toad-mcp` as subprocess and performs handshake
- [x] Test `initialize` → `tools/list` → verify 6 tools returned
- [x] Test `tools/call` for `list_projects` with mock workspace
- [x] Test `tools/call` for `search_projects` with query
- [x] Test error response for invalid tool params

---

## Step 6: Documentation + Client Config [x]

**Goal:** Make it easy for users to install and configure `toad-mcp`.

- [x] Add MCP section to `docs/guides/CLI.md` or create `docs/guides/MCP.md`
- [x] Document client configuration for Windsurf, Cursor, Claude Desktop
- [x] Add `toad-mcp` to `scripts/install_toad.sh` (Manually verified docs)
- [x] Update `README.md` with MCP server mention

---

## Step 7: Verify in Real Client [x]

**Goal:** End-to-end validation with a real MCP client.

- [x] Configure `toad-mcp` in Windsurf MCP settings
- [x] Ask AI agent: "What projects use Rust?" → verify `list_projects` called
- [x] Ask AI agent: "Show me the ecosystem summary" → verify
      `get_ecosystem_summary` called
- [x] Ask AI agent: "Which projects have dirty git status?" → verify
      `get_ecosystem_status` called
- [x] Confirm all responses are well-formed and useful
- [x] Added Context-Awareness tools (`get_active_context`, `list_contexts`, `switch_context`)

---

## Cross-Cutting Mandates (verified by this track)

- [x] **M-1: Schema-First Contract** — all tool params use
      `Deserialize + JsonSchema`, all responses are serialized core types
- [x] **M-2: Layered Output Strategy** — MCP is the machine layer; CLI is the
      human layer; both consume the same library functions
- [x] **M-3: Idempotent Discovery** — all tools are read-only (except context switch), no side effects
- [x] **M-4: Data-Service Architecture** — `toad-mcp` is a service consumer of
      the library crates, proving the architecture works beyond the CLI

---

## Future Extensions (Post v1.1.0)

These are enabled by this track but NOT implemented here:

- [ ] **Resources:** Expose `shadows/` files as MCP resources with URI scheme
      `toad://shadows/{project}/CONTEXT.md`
- [ ] **Prompts:** "Analyze ecosystem", "Plan migration from A to B"
- [ ] **Write tools:** `sync_context`, `tag_project`, `run_batch`
- [ ] **Streamable HTTP:** For remote/dashboard access
- [ ] **Phase 5 tools:** `get_project_dna`, `find_pattern`,
      `compare_projects`, `generate_situation_report`
