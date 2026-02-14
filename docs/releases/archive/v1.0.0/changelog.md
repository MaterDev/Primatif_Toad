# v1.0.0 — 2026-02-04 "The Bloom"

## Added

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
