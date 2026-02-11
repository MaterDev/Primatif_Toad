# Implementation Plan: Global Install Architecture

## Tasks (from docs/releases/v1.1.0/tasks.md)

- [x] **P0-1: Workspace Struct Refactor**
- [x] **P0-2: Relocate shadows_dir**
- [x] **P0-3: Simplify Workspace::discover()**
- [x] **P0-4: Extend GlobalConfig (auto_sync, context_budget)**
- [x] **P0-5: Update Tests (Path separation & discovery tiers)**
- [x] **P0-6: Backward Compatibility & Migration**
- [x] **P0-7: Schema-First Contract (Mandate: Audit core structs)**
- [x] **P0-8: Embed Default Strategies**

## Cross-Cutting Mandates

- [x] **M-1: Schema-First Contract** (Derive Serialize + Deserialize)
- [x] **M-2: Layered Output Strategy** (Support --json)
- [x] **M-3: Idempotent Discovery** (Read-only ops are side-effect free)
- [x] **M-4: Data-Service Architecture** (Prep for toad-api)
