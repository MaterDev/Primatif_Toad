# Rust Coding Standards (Primatif Toad)

## 1. Modular Workspace Architecture

- **Crate-First:** Logic must reside in a dedicated crate within `crates/`.
- **Thin Binary:** `bin/toad` should only handle CLI parsing and orchestration.
- **Core Dependency:** All platform crates must depend on `toad-core` for shared
  types and workspace context.

## 2. Design Patterns

- **Strategy Pattern:** Use traits and strategies for features like tech-stack
  detection (discovery) and project scaffolding.
- **Stable Contracts:** Backend logic (Tauri commands or library functions) must
  remain view-agnostic and serve as a stable API.

## 3. Testing & Verification

- **Test Separation:** Move unit tests to a separate module or file (e.g.,
  `mod tests;` in `lib.rs` with logic in `src/tests.rs`) to keep implementation
  files lean.
- **Mandatory QA:** Every change must pass `just test` before being considered
  complete.

## 4. Documentation

- **Direct Imports:** Prefer direct relative imports (`../path/to/module`) over
  complex alias systems.
- **Self-Documenting Code:** Use descriptive naming and minimal, high-value
  comments that explain the "Why" rather than the "What."
