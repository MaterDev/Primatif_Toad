# Implementation Plan: v1.1.0 Blockers

## Tasks (from docs/releases/v1.1.0/tasks.md)

- [~] **BLK-1: Resolve "No-Print" Violation**
  - [ ] Audit `main.rs` for direct printing.
  - [ ] Extract logic into library functions returning result types.
- [ ] **BLK-2: Decouple Terminal Progress**
  - [ ] Define `ProgressReporter` trait in `toad-core`.
  - [ ] Replace `indicatif` calls with trait calls.
- [ ] **BLK-3: Typed Error Surface**
  - [ ] Implement `ToadError` enum.
  - [ ] Replace `anyhow!` in library crates with `ToadError`.
- [ ] **BLK-4: Decouple Interactive Prompts**
  - [ ] Audit stdin prompts.
  - [ ] Move prompt logic to binary boundary.

## Cross-Cutting Mandates

- [ ] **M-1: Schema-First Contract**
- [ ] **M-2: Layered Output Strategy**
- [ ] **M-3: Idempotent Discovery**
- [ ] **M-4: Data-Service Architecture**
