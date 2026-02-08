# Specification: MCP Server Mode (110-5)

## Overview

Transform Toad into a live context oracle that AI agents can query directly via
the Model Context Protocol.

## Sources

- **Strategy:** `docs/releases/v1.1.0/evolution.md` (§ Phase 3.5)
- **Tasks:** `docs/releases/v1.1.0/tasks.md` (§ Phase 3.5)

## Requirements

1. Separate `toad-mcp` binary crate.
2. Expose core tools: `list_projects`, `get_project_detail`, `search_projects`,
   `get_ecosystem_summary`.
3. Idempotent discovery (read-only ops are side-effect free).
