# Implementation Plan: v1.1.0 Blockers

## Tasks (from docs/releases/v1.1.0/tasks.md)

- [x] **BLK-1: Resolve "No-Print" Violation** [9a3f3d7]
  - [x] Audit `main.rs` for direct printing.
  - [x] Extract logic into library functions returning result types.
- [x] **BLK-2: Decouple Terminal Progress** [c749070]
  - [x] Define `ProgressReporter` trait in `toad-core`.
  - [x] Replace `indicatif` calls with trait calls.
- [x] **BLK-3: Typed Error Surface**
  - [x] Implement `ToadError` enum in `toad-core`.
  - [x] Replace `anyhow!` in library crates with `ToadError`.
- [x] **BLK-4: Decouple Interactive Prompts** [4dc2536]
  - [x] Audit stdin prompts in library crates.
  - [x] Move prompt logic to binary boundary.

## Cross-Cutting Mandates

- [x] **M-1: Schema-First Contract**
- [x] **M-2: Layered Output Strategy**
- [x] **M-3: Idempotent Discovery**
- [x] **M-4: Data-Service Architecture**
