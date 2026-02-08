# Changelog

All notable changes to the Toad Control CLI are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [v1.0.2] — 2026-02-08 "Open Core"

### Architecture

- **Open-Core Licensing:** Split codebase into MIT (open) and BSL-1.1
  (source-available) components with enforced boundaries.
  - **MIT:** `bin/toad`, `toad-core`, `toad-scaffold`
  - **BSL-1.1:** `toad-discovery`, `toad-git`, `toad-manifest`, `toad-ops`
  - BSL-1.1 crates convert to MIT automatically on 2034-02-07.
- **Multi-Repo Architecture:** Each crate extracted into its own git repository
  under the `Primatif` GitHub organization, managed as git submodules within the
  main workspace. Cargo workspace builds identically.
- **License Boundary Enforcement:** `scripts/check_license_boundary.sh` prevents
  MIT crates from depending on BSL-1.1 crates. Enforced via pre-commit hook and
  the `just qa` pipeline.

### Added

- **`toad ggit` — Multi-Repo Git Orchestration:**
  - `toad ggit status` — Consolidated git status across all repositories.
  - `toad ggit commit -m "msg"` — Commit changes across repositories.
  - `toad ggit commit -m "msg" --cascade` — Commit submodules first, then
    cascade to the Hub root in a single atomic transaction.
  - `toad ggit push` — Push all repositories to their remotes.
  - `toad ggit pull` — Pull latest changes across all repositories.
  - `toad ggit sync` — Synchronize submodule refs with mandatory pre-flight
    safety checks (verifies HEAD is pushed, working tree is clean, not
    detached).
  - `toad ggit branches` — List all branches across repositories in a matrix.
  - `toad ggit align` — Force-align submodules to Hub root expectations.
  - All commands support `--query`, `--tag`, and `--fail-fast` filters.
- **`toad project` — Named Project Contexts:**
  - `toad project register <name> <path>` — Register a workspace root.
  - `toad project switch <name>` — Switch the active context.
  - `toad project current` — Show the active context.
  - `toad project list` — List all registered contexts with active indicator.
  - `toad project update` — Update an existing context's path or description.
  - `toad project delete` — Remove a context (with confirmation prompt).
  - `toad project info` — Show detailed info for a context.
  - Backward-compatible: existing `home_pointer` auto-migrates to a `default`
    context on first run.
- **`toad skill` — AI Skill Distribution:**
  - `toad skill sync` — Generate and distribute architectural blueprints, CLI
    references, and manifests to registered AI vendors. Replaces the old
    `toad manifest` command.
  - `toad skill list` — List distributed skills and registered vendors.
  - Supports: Windsurf, Gemini, Cursor, Claude, GitHub Copilot, Roo/Cline,
    Continue, Cody, PearAI, Supermaven, and custom vendor paths.
- **`toad cw` — Custom Workflows:**
  - `toad cw register <name> <script>` — Register a custom workflow script.
  - `toad cw run <name>` — Execute a registered workflow.
  - `toad cw list` — List all registered workflows.
  - `toad cw info` / `toad cw delete` — Manage registered workflows.
  - Reserved namespace check prevents collisions with built-in commands.
- **Submodule Awareness:**
  - `SubmoduleDetail` data model in `toad-core` with name, path, URL, expected
    vs. actual commit, initialization state, VCS status, stack, essence, and
    taxonomy.
  - Automatic `.gitmodules` parsing and submodule discovery in `toad-discovery`.
  - Submodule status displayed as indented children in `toad status` output.
  - Submodule rows included in manifest tables.
  - Orphan detection: child directories with `.git` that aren't tracked as
    submodules are flagged.
- **Hub Context Type:** `toad home` auto-detects Hub context when `.gitmodules`
  exists at the registered path.
- **Per-Context Storage:** Registry, shadows, and tags are stored per-context
  under `~/.toad/contexts/<name>/` instead of the workspace root.
- **Stale Context Warning:** Commands warn when the fingerprint has drifted and
  suggest running `toad skill sync`.
- **Architectural Blueprint Generator:** `toad-manifest` generates an agnostic
  dependency graph blueprint with internal dependency extraction from
  `Cargo.toml` and `package.json`.
- **CLI Reference Skill Generator:** Auto-generates a CLI reference skill from
  `toad --help` output for AI agent consumption.

### Changed

- **`toad manifest` renamed to `toad skill sync`** — The old command is removed.
  The new command generates and distributes multiple skills (blueprint, CLI
  reference, manifest) to AI vendors simultaneously.
- **`toad status` output** now includes submodule health as indented children
  with alignment indicators (aligned/drifted) and initialization state.
- **Fingerprinting** uses `unwrap_or(0)` instead of `?` propagation for mtime
  extraction, making it resilient in CI/CD environments where mtime may be
  unreliable.
- **Discovery** filters Hub submodule paths from `projects_dir` scan to prevent
  double-counting projects.

### Fixed

- Removed no-op `.chars().collect::<String>()` in manifest essence escaping.
- Collapsed nested `if` blocks in `toad-manifest` to satisfy clippy
  `collapsible_if` lint.
- Fixed `needless_borrow` clippy warnings in `toad ggit commit` handler.
- Migrated `project_tests.rs` from deprecated `Command::cargo_bin` to
  `cargo_bin_cmd!` macro.
- Updated `test_manifest` to use `toad skill sync` command surface.

---

## [v1.0.1] — 2026-02-05

### Added

- **`toad status`:** Git health and activity tier reporting across the
  ecosystem.
- **`toad do`:** Bulk command execution with safety guardrails, `--dry-run`, and
  `--fail-fast` modes.
- **`toad clean`:** Artifact removal with activity-tier filtering and disk usage
  reporting.
- **`toad stats`:** Visual disk usage heatmap (Atari Heatmap) and Bloat Index.
- **`toad tag` / `toad untag`:** Taxonomy tagging with harvest mode.
- **`toad sync`:** Registry cache synchronization.
- **Coverage uplift** to >80% across the platform.

---

## [v1.0.0] — 2026-02-04 "The Bloom"

### Added

- **Dynamic Strategy Engine:** Modular stack support plugins via TOML manifests
  in `~/.toad/strategies/`. Built-in support for Rust, Node.js, Python, Go,
  Java, Swift, Ruby, PHP, C/C++, and more.
- **`toad create`:** Project scaffolding with git initialization.
- **`toad reveal`:** Project discovery by name or `#tag`.
- **`toad home`:** Global workspace anchor for system-wide CLI access.
- **`toad strategy`:** Add, list, and manage stack support plugins.
- **`toad docs`:** Programmatic CLI documentation generation.
- **`toad manifest`:** AI context manifest generation ("Shadows").
- **Multi-Core Parallelism:** `rayon`-powered sub-second scanning across 100+
  projects.
- **Safety Guardrails:** Danger pattern detection with forced confirmations.
- **Visual Analytics:** Atari Heatmap for disk usage auditing.
- **Taxonomy System:** Evidence-based tagging pipeline for hybrid projects.
- **Crate-Driven Architecture:** `toad-core`, `toad-discovery`, `toad-ops`,
  `toad-scaffold`, `toad-manifest`, `toad-git` as workspace crates.

---

[v1.0.2]: https://github.com/Primatif/Primatif_Toad/compare/v1.0.2...dev
[v1.0.1]: https://github.com/Primatif/Primatif_Toad/compare/v1.0.0...v1.0.2
[v1.0.0]: https://github.com/Primatif/Primatif_Toad/releases/tag/v1.0.2
