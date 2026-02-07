# 🌿 Stack Support Plugin System

Toad uses a modular, evidence-based "Strategy Engine" to identify projects,
assign tags, and manage disk hygiene. This system allows users to add support
for any programming language, framework, or tool without modifying the Toad
source code.

## 🪷 How it Works

A **Stack Strategy** is a TOML manifest that defines:

1. **Evidence:** Files that indicate a folder is a specific type of project.
2. **Taxonomy:** Tags that should be automatically assigned to the project.
3. **Hygiene:** Directories or files that are considered "build artifacts" (safe
   to delete).
4. **Priority:** The order in which strategies are checked.

## 🌾 Strategy Locations

- **Built-in:** Managed by Toad (located in `~/.toad/strategies/builtin/`).
  These are your defaults (Rust, Node, Go, etc.).
- **Custom:** User-defined (located in `~/.toad/strategies/custom/`). These take
  precedence and are perfect for personal tools or obscure languages.

## 🛠️ Managing Strategies via CLI

### List Active Strategies

See what Toad is currently looking for:

```bash
toad strategy list
```

### Add a New Strategy

Create a new support plugin interactively or via flags:

```bash
toad strategy add Elixir --match "mix.exs" --clean "deps,_build" --tag "#elixir"
```

### Inspect a Strategy

View the "DNA" of a specific stack:

```bash
toad strategy info Rust
```

## 📝 Manifest Format (TOML)

Custom strategies are stored in `~/.toad/strategies/custom/<name>.toml`.

```toml
name = "Elixir"
match_files = ["mix.exs"]
artifacts = ["deps", "_build"]
tags = ["#elixir", "#functional"]
priority = 10
```

- `name`: Display name for the stack.
- `match_files`: List of files that, if present in a project root, trigger this
  strategy.
- `artifacts`: List of directories/files that can be safely deleted to reclaim
  space.
- `tags`: List of hashtags auto-assigned to projects matching this strategy.
- `priority`: Higher numbers are checked first. Monorepos usually have higher
  priority (20) than individual languages (10).

## 🐸 Advanced Use Cases

- **Hybrid Projects:** If a project matches multiple strategies (e.g., a Rust
  project with a Dockerfile), it will receive tags and cleanup rules from
  **all** matching strategies.
- **Project Overrides:** You can define a strategy that matches a very specific
  file (e.g., `.my-tool-config`) to apply custom tags to specific projects.
