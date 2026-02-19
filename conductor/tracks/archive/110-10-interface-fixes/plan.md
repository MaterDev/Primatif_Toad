# Implementation Plan: v1.1.0 Interface Fixes

## Tasks

### MCP Server (toad-mcp)

- [ ] **IF-1: Fix list_contexts Borrow**
  - Use `.clone()` when serializing `ctx.path` to avoid move errors.
- [ ] **IF-2: Reliable NoParams**
  - Ensure `NoParams` serializes to an empty object `{}`.

### CLI Binary (toad)

- [ ] **IF-3: Fix ggit Path Building**
  - Update `ggit` handlers to use `p.path.join(&sub.path)` instead of the
    project-relative path joined to `projects_dir`.
- [ ] **IF-4: Map Git 1 to OK**
  - Update `ggit commit` to treat exit code 1 (nothing to commit) as success.

### Hub & Governance (Primatif_Toad)

- [ ] **IF-5: Readable CHANGELOG**
  - Re-hydrate root `CHANGELOG.md` with high-level summaries from the v1.1.0
    cycle.
- [ ] **IF-6: Version Alignment**
  - Ensure all submodule pointers are aligned to their `dev` branches and
    versions match.

## Verification

- `cargo check` (to verify IF-1)
- `just test -p toad -p toad-mcp`
- Manual verification of `toad ggit status` from within a context.
