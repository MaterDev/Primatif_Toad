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

### Component Hierarchy

1. **`bin/toad` (The Orchestrator):** A thin CLI wrapper that parses commands
   and delegates to the internal crates.
2. **`crates/toad-git` (The Pulse):** Modular Git status and health logic.
3. **`crates/toad-core` (The Source of Truth):** Shared data models, workspace
   configuration, and global state management.
4. **`crates/toad-discovery` (The Scanner):** Implements the Strategy Pattern to
   detect tech stacks and project structures in the `projects/` directory.
5. **`crates/toad-manifest` (The Chronicler):** Handles report generation and AI
   context maps ("Shadows").
6. **`crates/scaffold` (The Builder):** Logic for bootstrapping new project
   templates.

## 📐 Design Principles

1. **Modular by Default:** New capabilities must live in a dedicated crate.
2. **View-Agnostic Backend:** The logic layer must remain decoupled from the CLI
   interface (stable API contracts).
3. **Strategy Pattern:** Use traits for extensible features like project
   discovery to allow adding new tech-stack detection without refactoring the
   core.
4. **Whitelist-Only VCS:** Git ignores everything by default (`/*`) to prevent
   accidental commits of managed projects or sensitive metadata.

## 🎨 Aesthetic Standards

- **Theme:** Retro-Atari (Green-on-Black).
- **Assets:** Pixel-art ASCII banners and block-style progress indicators.
