# Specification: Rust Context & Architecture Guide

## Overview
This track involves creating a comprehensive "Rust in Primatif Toad" guide. The goal is to onboard new contributors (experienced developers) by explaining how Rust is leveraged specifically within this project's modular architecture. It serves as a bridge between general Rust knowledge and the project's specific implementation patterns.

## Target Audience
- **New Contributors:** Experienced developers who need to understand the "Primatif Toad way" of writing Rust.

## Functional Requirements
- **Introduction to Workspace Orchestration:** Explain the multi-crate workspace structure and how dependencies are managed.
- **CLI Architecture Deep-Dive:** Detail the delegation pattern between `bin/toad` and the capability crates.
- **Trait-Based Strategy Pattern:** Provide a clear explanation of how traits are used for extensibility, specifically in `toad-discovery`.
- **Core Data Models:** Explain the role of `toad-core` as the central source of truth.
- **Code Excerpts & Explanations:** Include real-world code snippets from the following crates to illustrate concepts:
    - `bin/toad` (Entry point and command parsing)
    - `crates/toad-core` (Data models and shared logic)
    - `crates/toad-discovery` (Strategy pattern and traits)
    - `crates/toad-ops` (Operational logic and execution)

## Non-Functional Requirements
- **Consistency:** Must align with the principles defined in `tech-stack.md` and `workflow.md`.
- **Clarity:** Use professional, technical language suitable for experienced developers.
- **Maintainability:** The guide should be structured so that it can be updated easily as the architecture evolves.

## Acceptance Criteria
- [ ] A new guide file (e.g., `docs/rust-architecture.md`) is created.
- [ ] The guide covers Workspace Orchestration, CLI Architecture, and the Strategy Pattern.
- [ ] Code examples from all requested crates are included and explained.
- [ ] The guide is reviewed and approved as a clear onboarding resource.

## Out of Scope
- A general "How to install Rust" guide (covered by official Rust docs).
- Detailed documentation of every single function in the codebase.
