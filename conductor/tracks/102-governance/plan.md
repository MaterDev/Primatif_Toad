# Implementation Plan: Governance & AI Navigation (v1.0.2 Phase 6)

> **Source of Truth:** `docs/releases/v1.0.2/tasks.md` § Phase 6.
> This plan is an execution guide. The release docs are authoritative.

## Phase 1: Documentation Updates
- [ ] **P6-1: Update `.gemini/GEMINI.md`**
  - Add Licensing Architecture section with MIT/BUSL-1.1 split.
  - Add dependency direction rule.
- [ ] **P6-2: Update `conductor/tech-stack.md`**
  - Add license annotations on all crates.
- [ ] **P6-3: Update `conductor/product-guidelines.md`**
  - Add Licensing & Architecture Boundaries section.
- [ ] **P6-4: Document Crate Decision Framework**
  - New crate creation must include LICENSE file and `license` field before any code.
- [ ] **P6-5: Update `.gemini/settings.json`** — **Critical**
  - Add `conductor/` to context folders.
  - Register `CROSS_REPO_MAP.md` as a context file so every new AI chat
    session starts with the inter-repo dependency graph. Without this,
    agents are blind to the multi-repo architecture.

## Phase 2: Context Map Generation
- [ ] **P6-6: `toad manifest` Update** (`toad-manifest`, BUSL-1.1)
  - Implement dependency graph extraction (parse `Cargo.toml` across crates).
  - Implement type flow scanning (`pub struct`/`pub enum` → `use` tracing).
  - Implement call chain documentation for major commands.
  - Generate `CROSS_REPO_MAP.md` at workspace root.
  - Regenerate on every `toad manifest` run.

## Phase 3: Verification
- [ ] **P6-7: AI Navigation Test**
  - Verify an agent can trace a cross-repo bug using only the map.
  - Verify the map stays accurate after adding a new type and re-running manifest.
  - Verify the map includes all v1.0.2 types (`RepoStatus`, `BranchGroup`,
    `GitOpResult`, `PreflightResult`, `CustomWorkflow`, `WorkflowRegistry`,
    `ProjectContext`).

## References
- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Governance, § Cross-Repo Context Map
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 6
