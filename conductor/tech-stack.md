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

1. **`bin/toad` (The Orchestrator) [MIT]:** A thin CLI wrapper that parses
   commands and delegates to the internal crates.
2. **`crates/toad-git` (The Pulse) [BUSL-1.1]:** All Git operations, status,
   branch orchestration, and VCS intelligence.
3. **`crates/toad-ops` (The Hand) [BUSL-1.1]:** General operational logic, shell
   execution, and custom workflow management.
4. **`crates/toad-core` (The Source of Truth) [MIT]:** Shared data models,
   contracts, workspace discovery, and Named Project Contexts.
5. **`crates/toad-discovery` (The Scanner) [BUSL-1.1]:** Scanning intelligence
   to detect tech stacks and submodule structures.
6. **`crates/toad-manifest` (The Chronicler) [BUSL-1.1]:** Handles report
   generation, AI context maps, and cross-repo dependency maps.
7. **`crates/toad-scaffold` (The Builder) [MIT]:** Logic for bootstrapping new
   project templates (filesystem only).

## 📐 Design Principles

1. **Modular by Default:** New capabilities must live in a dedicated crate.
2. **Licensing-Aware Architecture:** Every code change must land in the correct
   boundary. MIT for types/contracts; BUSL-1.1 for intelligence/logic.
3. **Dependency Direction:** MIT crates MUST NEVER depend on BUSL-1.1 crates
   (except `bin/toad`).
4. **Git Intelligence Isolation:** All logic that executes `git` commands,
   parses `.git` internals, or reasons about VCS state MUST live in `toad-git`.
5. **The SDK Contract:** `toad-core` is a stable SDK for external plugins.
   Prioritize stability and zero-dependency growth for Core.
6. **View-Agnostic Backend:** The logic layer must remain decoupled from the CLI
   interface (stable API contracts).
7. **Strategy Pattern:** Use traits for extensible features.
8. **Separate Tests:** Implementation files must stay lean. Move unit tests to a
   companion `tests.rs` file.
9. **Clean Code (Platform MVP):** Favor small, single-responsibility functions,
   explicit error handling (`anyhow`), and avoid global state.
10. **Whitelist-Only VCS:** Git ignores everything by default.

## 🎨 Aesthetic Standards

- **Theme:** Retro-Atari (Green-on-Black).
- **Assets:** Pixel-art ASCII banners and block-style progress indicators.
