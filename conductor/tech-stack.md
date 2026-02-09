# Tech Stack & Architecture: Primatif Toad

## 🛠️ Core Technologies

- **Programming Language:** Rust (Stable).
- **Orchestration:** [Just](https://github.com/casey/just) (Command runner for
  quality gates and installation).
- **Environment:** macOS (Primary target, environment-agnostic via dynamic path
  resolution).
- **VCS:** Git (Required for project discovery and ecosystem health).
- **Tooling:** Node.js (Markdown linting), Dprint (Formatting).

## 🏗️ Architecture: The Platform Pattern

The project is structured as a modular Rust workspace to separate the
"Orchestrator" from specific "Capabilities."

### Component Hierarchy (The Toad Tiers)

The project follows a multi-repo "Open Core" architecture linked via Git
submodules in the `crates/` directory:

1. **`bin/toad` (The Interface) [MIT]:** A thin CLI wrapper that parses commands
   and delegates to library crates. Purely for formatting.
2. **`bin/toad-mcp` (The Oracle) [MIT]:** A Model Context Protocol server that
   exposes Toad's intelligence to AI agents.
3. **`crates/toad-git` (The Pulse) [BUSL-1.1]:** All Git operations and VCS
   intelligence.
4. **`crates/toad-ops` (The Hand) [BUSL-1.1]:** General operational logic and
   shell execution.
5. **`crates/toad-core` (The Source of Truth) [MIT]:** Shared data models,
   contracts, and workspace discovery.
6. **`crates/toad-discovery` (The Scanner) [BUSL-1.1]:** Scanning intelligence
   to detect tech stacks, submodules, and DNA patterns.
7. **`crates/toad-manifest` (The Chronicler) [BUSL-1.1]:** Context generation
   and tiered prompt mapping.
8. **`crates/toad-scaffold` (The Builder) [MIT]:** Project bootstrapping logic.

## 📐 Design Principles

1. **Modular by Default:** New capabilities must live in a dedicated crate.
2. **The 700-Line Limit:** No single file may exceed 700 lines. If a file
   approaches this limit, it MUST be refactored into smaller, logically distinct
   modules.
3. **View-Agnostic Backend:** Command logic MUST return structured data
   (`Result<T>`) and perform no direct I/O or printing.
4. **Licensing-Aware Architecture:** MIT for types/contracts; BUSL-1.1 for
   intelligence/logic.
5. **Dependency Direction:** MIT crates MUST NEVER depend on BUSL-1.1 crates
   (except binary crates).
6. **Git Intelligence Isolation:** All `git` execution MUST live in `toad-git`.
7. **The Context Window is Sacred:** Optimize all generated metadata for AI
   context windows (Progressive Disclosure).
8. **Strategy Pattern:** Use traits for extensible features.
9. **Separate Tests:** Unit tests live in companion `tests.rs` files.
10. **Clean Code:** Small functions, explicit `anyhow` error handling, zero
    global state.
11. **Whitelist-Only VCS:** Git ignores everything by default.

## 🎨 Aesthetic Standards

- **Theme:** Retro-Atari (Green-on-Black).
- **Assets:** Pixel-art ASCII banners and block-style progress indicators.
