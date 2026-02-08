# Implementation Plan: Global Install Architecture

## Tasks (from docs/releases/v1.1.0/tasks.md)

- [ ] **P0-1: Workspace Struct Refactor**
- [ ] **P0-2: Relocate shadows_dir**
- [ ] **P0-3: Simplify Workspace::discover()**
- [ ] **P0-4: Extend GlobalConfig (auto_sync, context_budget)**
- [ ] **P0-5: Update Tests (Path separation & discovery tiers)**
- [ ] **P0-6: Backward Compatibility & Migration**
- [ ] **P0-7: Schema-First Contract (Mandate: Audit core structs)**
- [ ] **P0-8: Embed Default Strategies**

## Cross-Cutting Mandates
- [ ] **M-1: Schema-First Contract** (Derive Serialize + Deserialize)
- [ ] **M-2: Layered Output Strategy** (Support --json)
- [ ] **M-3: Idempotent Discovery** (Read-only ops are side-effect free)
- [ ] **M-4: Data-Service Architecture** (Prep for toad-api)
