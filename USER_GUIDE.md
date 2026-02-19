# Toad User Guide

Welcome to **Toad**, the High-Performance Local-Ops Platform and AI-native
Context Oracle. This guide provides everything you need to know to master
ecosystem management and context engineering with Toad.

---

## 🚀 Quick Start (5-Minute Setup)

### 1. Install Toad

If you haven't already, install the Toad CLI to your system:

```bash
# In the Primatif_Toad directory
./scripts/install_toad.sh
```

### 2. Initialize your Workspace

Anchor Toad to your projects directory:

```bash
cd ~/path/to/your/projects
toad home .
```

### 3. Build your Context

Initialize the context engine to sync your projects and generate AI-ready
metadata:

```bash
toad init-context
```

---

## 🧠 Core Concepts

### 🔍 Workspace Discovery

Toad automatically scans your workspace starting from the `.toad-root` anchor.
It identifies projects based on their stack (Rust, Node.js, Python, etc.) and
analyzes their metadata.

### ⚓ Project Registry

The Project Registry is a high-performance cache (`registry.json`) that stores
metadata about every project in your ecosystem. Run `toad sync` whenever you add
or remove projects.

### 🧠 Context Engineering

Toad transforms raw repository data into high-fidelity context for AI agents. It
uses a tiered prompt architecture:

1. **`llms.txt`**: Entry point for LLM discovery.
2. **`SYSTEM_PROMPT.md`**: Bird's-eye view of the entire ecosystem.
3. **`MANIFEST.md`**: Detailed project table with extractive essence.
4. **`CONTEXT.md`**: Deep-dive operational briefing for a specific project.

### 🧬 DNA Patterns

Toad's intelligence engine automatically identifies architectural roles (e.g.,
Data Layer, API Surface) and capabilities (e.g., Dockerized, Async) by scanning
project structures.

---

## 🔧 Non-Interactive Execution

### Using `--yes` Flags

All destructive or interactive commands support the `--yes` (or `-y`) flag for
non-interactive execution, perfect for scripts and CI/CD workflows:

**Custom Workflows:**

```bash
toad cw delete my-workflow --yes
```

**Strategy Management:**

```bash
toad strategy remove custom-stack --yes
```

**Multi-Repo Git Operations:**

```bash
toad ggit commit -m "Update dependencies" --yes
toad ggit push --yes
toad ggit pull --yes
toad ggit checkout feature-branch --yes
toad ggit align --yes
```

Without `--yes`, these commands will prompt for confirmation. If you abort, the
message will remind you: `Aborted. (Use --yes to skip confirmation)`

### Command Aliases

Toad recognizes common command aliases to improve discoverability:

- `ls` → `list`
- `search`, `find` → `reveal`
- `check` → `status`
- `health`, `diagnose` → `doctor`
- `cleanup`, `purge` → `clean`
- `git` → `ggit`
- `workflow`, `workflows` → `cw`
- `unregister`, `remove`, `rm`, `del` → `delete`

### "Did You Mean?" Suggestions

If you mistype a command, Toad will suggest the correct one:

```bash
$ toad lst
error: unrecognized subcommand 'lst'
Hint: Did you mean 'list'?

$ toad search my-project
error: unrecognized subcommand 'search'
Hint: Did you mean 'reveal'?
```

---

## 🌊 Common Workflows

### Multi-Repo Git Operations

Orchestrate dozens of repositories simultaneously with `ggit`:

- `toad ggit status`: Check status across all repos.
- `toad ggit commit -m "msg" --cascade`: Commit submodules and update the hub
  atomically.
- `toad ggit sync`: Pull latest changes and align submodule pointers.

### Bulk Operations

Execute commands across projects filtered by name or tag:

- `toad do "cargo test" -q #backend`: Run tests on all backend projects.
- `toad tag <project> --tag core`: Organize your ecosystem with custom taxonomy.

### Context Management

Manage multiple workspace roots:

- `toad project register <name> <path>`: Add a new projects directory.
- `toad project switch <name>`: Change your active workspace.

### AI Integration

Distribute synchronized context to your favorite AI vendors:

```bash
toad skill sync
```

This distributes `.md` skill files to `.gemini/skills/`, `.windsurf/`, and other
IDE-specific directories.

---

## 📊 Analytics & Insights

### Key Analytics Commands

- `toad analyze deps`: Generates an ecosystem-wide dependency graph and
  identifies the **Critical Path**.
- `toad analyze health`: Composite 0-100 scoring based on VCS, docs, activity,
  and debt.
- `toad analyze debt`: Scans for `TODO`/`FIXME` comments and monolithic files
  (>700 lines).
- `toad analyze velocity`: Tracks development momentum and line churn over time.

---

## ⚙️ Advanced Topics

### Model Context Protocol (MCP) Server

Toad includes a full-featured MCP server that exposes its intelligence directly
to AI agents in real-time.

To use MCP in an editor/agent, you must:

1. Install `toad-mcp` locally.
2. Register it in your IDE's MCP configuration.

```bash
toad-mcp
```

Configure this server in your IDE (Cursor, Windsurf, etc.) to give agents
zero-latency vision across your ecosystem.

See the [MCP Guide](./docs/guides/MCP.md) for installation and configuration.

### Stack Strategies

Define custom logic for project detection and cleanup in `~/.toad/strategies/`.
Toad uses these to identify project types and know which files are safe to
`clean`.

### Custom Workflows

Register and orchestrate common development scripts:

- `toad cw register qa ./scripts/full-qa.sh`
- `toad cw run qa`

---

## 🏥 Troubleshooting (Toad Doctor)

If Toad is behaving unexpectedly, run the automated diagnostic suite:

```bash
toad doctor
```

This checks your environment variables, tool dependencies (git), registry
health, and connectivity.

---

## 📖 Reference Links

- [CLI Reference](./docs/guides/CLI.md)
- [MCP Guide](./docs/guides/MCP.md)
- [Architecture Guide](./docs/architecture/rust-architecture.md)

---

## 🐸 Using Conductor (Solo-Dev Workflow)

If you are using the **Conductor** extension for solo-dev orchestration:

1. **Start Session:** Ask "Read the conductor index and tell me what's next."
2. **New Feature:** Ask "Initialize a new Track for [Feature Name]."
3. **Quality Gate:** Always run `just qa` before finishing a track.
4. **Finish Track:** Commit with the structured Release format and move the
   track to `archive/`.
