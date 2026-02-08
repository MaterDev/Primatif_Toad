# Product Definition: Primatif Toad

## Vision

**Primatif Toad** is a DevOps Overlay and Developer CLI designed to manage a
modular ecosystem of independent projects. It follows an **Open Core** model,
maintaining a legitimate open-source foundation (MIT CLI and core types) while
protecting its advanced intelligence layer under a source-available license
(BUSL-1.1).

## Core Goals

1. **Named Project Contexts:** Manage multiple independent workspace roots
   (e.g., development workspaces, client projects) with explicit context
   switching and isolated scan caches.
2. **Workspace Orchestration:** Manage the `projects/` directory as an external
   collection of independent repositories using a "whitelist-only" strategy.
3. **Open Core Licensing:** Maintain a clear boundary between open-source
   contracts (MIT) and protected intelligence (BUSL-1.1).
4. **Multi-Repo Git Orchestration:** Provide first-class group-git operations
   (`toad ggit`) to manage distributed submodule ecosystems.
5. **Discovery & Scaffolding:** Automate project pattern detection and component
   generation via specialized Rust crates.
6. **Unified Tooling:** Centralize quality gates and management scripts for a
   diverse set of local codebases.
7. **AI-First Design:** Maintain high-quality context metadata ("Shadows") and
   architectural maps to enable AI agents to assist across the ecosystem.

## Core Roles

- **Toad status:** Health checks across the ecosystem.
- **Toad do:** Bulk shell operations across multiple projects.
- **Toad reveal:** Deep discovery of project capabilities.
- **Toad manifest:** Context generation for AI orchestration.
- **Scaffold:** Standardized project bootstrapping.
