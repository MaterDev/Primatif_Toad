# v1.0.2 "Open Core": Licensing Architecture & Multi-Repo Split

## Vision

The v1.0.2 release restructures Toad from a single monolithic repository into
an **open-core multi-repo architecture**. The goal is to maintain Toad as a
legitimate open-source project (MIT-licensed CLI and core types) while
protecting the intelligence layer under a source-available license (BSL-1.1).

This is a **structural and legal change only** — zero Rust code changes are
required. The Cargo workspace continues to function identically. The user
experience is unchanged. What changes is the ownership and licensing boundary
around each component.

## Core Objectives

1. **Open-Core Licensing:** Split the codebase into MIT (open) and BSL-1.1
   (source-available) components with clear boundaries.
2. **Multi-Repo Architecture:** Each crate becomes its own git repository,
   managed as a git submodule within the main workspace.
3. **Unified Development:** Despite separate repos, the developer experience
   remains a single `cargo build` from the workspace root.
4. **Commercial Protection:** The scanning, context generation, and operational
   intelligence layers are protected under BSL-1.1 with an 8-year conversion
   to MIT.
5. **Source Availability:** All code remains publicly visible, cloneable, and
   explorable. The BSL-1.1 license only restricts commercial use without
   permission.

## Non-Goals

- This is **not** a code refactor. No Rust source code changes.
- This is **not** making any code private. All repos are public.
- This is **not** changing the user-facing CLI or any command behavior.

---

## The Licensing Strategy

### The Legal Pattern: Open Core

The "open core" model is well-established in the software industry. The
principle:

- The **binary and glue code** (CLI, argument parsing, output formatting) is
  MIT-licensed. This is the "open source" face of the project.
- The **core data types and contracts** (structs, traits, enums) are
  MIT-licensed. This lets the community build on the data model.
- The **intelligence layer** (scanning, context generation, operations,
  analysis) is BSL-1.1 licensed. This protects the commercial value.

Because Primatif owns all the code, the MIT binary can legally depend on
BSL-1.1 crates. The license only restricts what **others** can do with each
piece independently.

### BSL-1.1 (Business Source License) Terms

The BSL-1.1 is a **source-available** license, not a proprietary one:

- **Visibility:** Code is fully public and readable on GitHub
- **Cloning:** Anyone can clone, fork, and study the code
- **Attribution:** Required — copyright notice must be preserved
- **Non-Commercial Use:** Permitted (personal projects, education, research)
- **Commercial Use:** Requires a separate license from Primatif
- **Change Date:** 8 years from release date (**2034-02-07**)
- **Change License:** MIT — after the change date, the code automatically
  becomes MIT-licensed

**Used by:** MariaDB, CockroachDB, Sentry, HashiCorp (Terraform, Vault).

### License Header Template (BSL-1.1 Crates)

```text
Business Source License 1.1

Licensor:           Primatif
Licensed Work:      [crate-name] [version]
                    The Licensed Work is (c) 2026 Primatif

Additional Use Grant: You may use the Licensed Work for non-commercial
                      and personal purposes, provided that you include
                      attribution to Primatif in any derivative works.

Change Date:        2034-02-07
Change License:     MIT

For information about alternative licensing arrangements for the Licensed
Work, please contact: [contact info]
```

---

## The License Split

### MIT (Open Source)

| Crate       | Rationale |
| :---------- | :-------- |
| `bin/toad`  | CLI glue — argument parsing, output formatting, user interaction. This is the open-source face of the project. |
| `toad-core` | Data models, `Workspace`, `GlobalConfig`, `StackStrategy`, `ProjectDetail`. These are the **contracts** that define the ecosystem. Keeping this MIT lets the community build tooling on the same data model. |
| `scaffold`  | Basic project creation. Low commercial value, high community value. |

### BSL-1.1 (Source-Available, 8-Year Conversion to MIT)

| Crate                | Rationale |
| :------------------- | :-------- |
| `discovery`          | The scanning and intelligence engine. This is the core value — how Toad understands an ecosystem's structure, stacks, and relationships. |
| `toad-git`           | Git status analysis and VCS intelligence. Feeds into discovery and operational awareness. |
| `toad-manifest`      | Context generation logic — the AI-context brain. Becomes even more valuable with v1.1.0's deep extraction and structured data. |
| `toad-ops`           | Batch operations, safety guardrails, audit trail. Operational intelligence that powers `toad do`, `toad clean`, and future automation. |
| Future: `toad-mcp`   | The MCP server from v1.1.0. Direct commercial value as the agent query interface. |

### Dependency Graph & License Boundaries

```text
bin/toad (MIT)
├── toad-core       (MIT)        ← leaf, no internal deps
├── scaffold        (MIT)        ← leaf, no internal deps
├── discovery       (BSL-1.1)    ← depends on: toad-core (MIT), toad-git (BSL-1.1)
├── toad-git        (BSL-1.1)    ← depends on: toad-core (MIT)
├── toad-manifest   (BSL-1.1)    ← depends on: toad-core (MIT)
└── toad-ops        (BSL-1.1)    ← depends on: toad-core (MIT)
```

**Key observation:** All BSL-1.1 crates depend on `toad-core` (MIT), but no MIT
crate depends on a BSL-1.1 crate except the binary itself. This is a clean
boundary — the MIT types flow downward, the BSL-1.1 intelligence flows upward
into the MIT binary.

---

## Multi-Repo Architecture

### The Problem

Currently, all crates live in a single git repository. This means:

- A single LICENSE file covers everything
- There's no way to assign different licenses to different components
- The entire codebase shares one git history
- Publishing individual crates to crates.io requires a single license

### The New Architecture: Git Submodules

Each crate becomes its own git repository, referenced as a submodule in the
main workspace:

```text
Primatif_Toad/                          ← Main repo (MIT)
├── LICENSE (MIT)
├── Cargo.toml                          ← Workspace manifest (unchanged)
├── bin/toad/                           ← MIT CLI binary (stays in main repo)
├── crates/
│   ├── toad-core/                      ← submodule → Primatif/toad-core (MIT)
│   │   └── LICENSE (MIT)
│   ├── scaffold/                       ← submodule → Primatif/toad-scaffold (MIT)
│   │   └── LICENSE (MIT)
│   ├── discovery/                      ← submodule → Primatif/toad-discovery (BSL-1.1)
│   │   └── LICENSE (BSL-1.1)
│   ├── toad-git/                       ← submodule → Primatif/toad-git (BSL-1.1)
│   │   └── LICENSE (BSL-1.1)
│   ├── toad-manifest/                  ← submodule → Primatif/toad-manifest (BSL-1.1)
│   │   └── LICENSE (BSL-1.1)
│   └── toad-ops/                       ← submodule → Primatif/toad-ops (BSL-1.1)
│       └── LICENSE (BSL-1.1)
├── conductor/                          ← Stays in main repo
├── docs/                               ← Stays in main repo
└── scripts/                            ← Stays in main repo
```

### Why Git Submodules

- **`Cargo.toml` stays unchanged** — `path = "../toad-core"` works identically
  whether the directory is a submodule or a regular folder
- **`cargo build` works normally** — the workspace doesn't know or care about
  git boundaries
- **Each submodule has its own git history** — clean separation for licensing
- **Each submodule has its own LICENSE** — legally distinct components
- **All repos are public** — anyone can see the code, but the BSL-1.1 repos
  restrict commercial use
- **Development workflow is unchanged** — you edit files in the same workspace,
  just commit to different repos

### GitHub Repository Map

| GitHub Repo                | Visibility | License | Source Directory                 |
| :------------------------- | :--------- | :------ | :------------------------------- |
| `Primatif/Primatif_Toad`   | Public     | MIT     | Root (bin/toad, conductor, docs) |
| `Primatif/toad-core`       | Public     | MIT     | `crates/toad-core/`              |
| `Primatif/toad-scaffold`   | Public     | MIT     | `crates/scaffold/`               |
| `Primatif/toad-discovery`  | Public     | BSL-1.1 | `crates/discovery/`              |
| `Primatif/toad-git`        | Public     | BSL-1.1 | `crates/toad-git/`               |
| `Primatif/toad-manifest`   | Public     | BSL-1.1 | `crates/toad-manifest/`          |
| `Primatif/toad-ops`        | Public     | BSL-1.1 | `crates/toad-ops/`               |

---

## History Preservation

Git history for each crate is preserved using `git filter-repo`:

1. Make a fresh clone of `Primatif_Toad` for each crate
2. Run `git filter-repo --subdirectory-filter crates/<name>` on each clone
3. Push each filtered clone to its new GitHub repo
4. In the real `Primatif_Toad` repo, remove the directories and add submodules

This preserves all commit history per-crate while keeping the main repo clean.

---

## Backward Compatibility

- **Users:** `cargo install toad` continues to work. The binary is MIT. Users
  don't interact with individual crates.
- **Contributors:** Clone the main repo with `--recurse-submodules`. The
  workspace builds identically.
- **CI/CD:** Add `git submodule update --init --recursive` to CI pipelines.
- **Toad itself:** Each crate repo can be added to Toad's `projects/` directory
  for self-management. `toad status` will show git health across all crate
  repos. `toad tag` can apply license taxonomy (`#mit`, `#bsl`).

---

## Submodule-Aware Ecosystem Management

### The Problem

Many real-world projects use git submodules to compose multi-repo architectures
— monorepos with vendored dependencies, open-core projects with separately
licensed components, embedded firmware with shared libraries, etc. Toad
currently treats each entry in `projects/` as a flat, independent repo. It has
no awareness of submodules nested inside those projects.

After v1.0.2, Toad's own codebase will use submodules. But this feature must
not be built as a special case for self-management — it must be a **generic,
first-class capability** that any Toad user can leverage for their own
multi-repo ecosystems.

### The Feature: Submodule Discovery & Status

Toad should automatically detect and surface git submodules within any managed
project. This means:

1. **Discovery:** When scanning a project, detect `.gitmodules` and enumerate
   all registered submodules. Each submodule becomes a child entity of the
   parent project.
2. **Status:** For each submodule, report:
   - **Initialization state** — is it initialized or just a pointer?
   - **VCS status** — clean, dirty, or detached HEAD?
   - **Commit alignment** — is the submodule on the commit the parent expects,
     or has it drifted?
   - **Remote status** — is the submodule's remote reachable?
3. **Operations:** `toad do` should be able to target submodules within a
   project, not just top-level projects. `toad status` should show submodule
   health as part of the parent project's status.
4. **Relationship Mapping:** Surface the parent → submodule relationship in
   Toad's data model so that AI agents and the manifest can reason about
   multi-repo dependency graphs.

### Data Model Changes (`toad-core`, MIT)

The existing `ProjectDetail` struct has a `sub_projects: Vec<String>` field
that currently stores names only. This needs to evolve into a richer model:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmoduleDetail {
    pub name: String,
    pub path: PathBuf,
    pub url: String,
    pub expected_commit: Option<String>,
    pub actual_commit: Option<String>,
    pub initialized: bool,
    pub vcs_status: VcsStatus,
}
```

`ProjectDetail.sub_projects` should be replaced (or augmented) with:

```rust
pub submodules: Vec<SubmoduleDetail>,
```

These are **data models** (types and contracts), so they belong in `toad-core`
(MIT) — consistent with the licensing architecture.

### Discovery Logic (`discovery`, BSL-1.1)

The scanning engine needs to:

- Parse `.gitmodules` to enumerate submodules
- Check each submodule's initialization state
- Populate `SubmoduleDetail` for each discovered submodule
- Include submodule data in the project scan results

This is **scanning intelligence**, so it belongs in `discovery` (BSL-1.1).

### Git Analysis (`toad-git`, BSL-1.1)

The git analysis layer needs to:

- Determine the expected vs. actual commit for each submodule
- Detect detached HEAD states in submodules
- Report whether a submodule has local changes (dirty)
- Check remote reachability for submodule URLs

This is **VCS intelligence**, so it belongs in `toad-git` (BSL-1.1).

### CLI Surface (`bin/toad`, MIT)

- `toad status` should display submodule status as indented children of their
  parent project
- `toad do` should support targeting submodules (e.g., `toad do -p myproject`
  runs in all submodules of `myproject`)
- `toad reveal` should include submodule details in its output

### Dogfooding: Toad Manages Itself

After the submodule feature is built, Toad's own crate repos (now submodules
of the main repo) become the validation case:

- `toad status` shows the health of all crate submodules
- `toad do -q toad "cargo test"` runs tests across all crate submodules
- `toad tag toad-core mit` / `toad tag discovery bsl` applies license taxonomy
- v1.1.0's `context.json` will map the submodule dependency graph for AI agents

This is a natural demonstration of Toad's value proposition: managing a
multi-repo ecosystem from a single control plane. But the feature itself is
generic — any user with submodules in their projects benefits equally.

### Scope Boundary: v1.0.2 vs. v1.1.0

**v1.0.2 delivers:**

- `SubmoduleDetail` data model in `toad-core`
- Basic submodule discovery (parse `.gitmodules`, check init state)
- Basic submodule status in `toad status` output
- First-class `toad repo` git orchestration commands (see below)
- Dogfooding with Toad's own crate submodules

**v1.1.0 extends with:**

- Deep submodule context in `context.json` (commit history, dependency graphs)
- MCP queries about submodule relationships
- Submodule-aware manifest generation
- Cross-submodule dependency analysis for AI agents

---

## Project Contexts (`toad project`)

### The Problem: Which Workspace Am I In?

Toad currently has a single workspace concept: the `home_pointer` in
`~/.toad/config.json` points to one directory, and all commands operate
against that directory's projects. This works for the simple case — one user,
one workspace.

But in practice, a developer often works across multiple distinct contexts:

- **Their code workspace** — `~/Code/` where personal and work projects live
- **Toad development** — `~/Primatif_Toad/` where Toad's own crate submodules
  are the "projects"
- **A client workspace** — `~/ClientWork/` with a different set of repos
- **A monorepo** — `~/BigCorp/platform/` with submodules for microservices

Without explicit context switching, commands like `toad status`, `toad ggit`,
and `toad do` don't know which set of projects they should target. Worse,
when Toad is used to develop *itself*, its own crate submodules need to be
treated as projects — but the current `home_pointer` might be pointing at the
user's code workspace.

**We've been conflating these contexts throughout the design.** Some examples
in this document assume `toad ggit` operates on Toad's own crates, others
assume it operates on user projects. The `toad project` feature makes this
explicit.

### The Solution: Named Project Contexts

Users register any number of **named project contexts** — each is a name
paired with an absolute path to a directory that Toad should treat as a
workspace root. Switching between contexts is a single command.

There are no hardcoded context names. "toad-dev", "my-code", "client-work"
are all user-defined. Toad's own development workspace is just another
registered context — no special-casing.

### Command Surface

```text
toad project register <name> <path> [--description "<text>"]
    Register a new project context. The path must be an absolute path to
    an existing directory. The name must not collide with an existing
    registered context.

    Example:
      toad project register my-code ~/Code --description "Personal projects"
      toad project register toad-dev ~/Primatif_Toad --description "Toad development"
      toad project register client ~/ClientWork --description "Client projects"

toad project switch <name>
    Switch the active project context. All subsequent toad commands will
    operate against this context's path until switched again.

    Example:
      toad project switch toad-dev
      → All toad commands now target ~/Primatif_Toad

toad project current
    Show the currently active project context (name, path, description).

    Example output:
      Active context: toad-dev
      Path:           /Users/jake/Primatif_Toad
      Description:    Toad development

toad project list
    List all registered project contexts with their names, paths, and
    which one is currently active.

    Example output:
      Name        Path                        Description           Active
      my-code     /Users/jake/Code            Personal projects     
      toad-dev    /Users/jake/Primatif_Toad   Toad development      ✅
      client      /Users/jake/ClientWork      Client projects       

toad project update <name> [--path <path>] [--description "<text>"]
    Update an existing context's path and/or description.

toad project delete <name>
    Remove a registered context. Prompts for confirmation. Cannot delete
    the currently active context (must switch first).

toad project info <name>
    Show detailed information about a specific context.
```

### Storage: `~/.toad/config.json` (Extended)

The existing `GlobalConfig` currently has only `home_pointer`. This extends
it with a `project_contexts` map and an `active_context` field:

```json
{
  "home_pointer": "/Users/jake/Code",
  "active_context": "my-code",
  "project_contexts": {
    "my-code": {
      "path": "/Users/jake/Code",
      "description": "Personal projects",
      "registered_at": "2026-02-08T00:20:00Z"
    },
    "toad-dev": {
      "path": "/Users/jake/Primatif_Toad",
      "description": "Toad development",
      "registered_at": "2026-02-08T00:20:00Z"
    },
    "client": {
      "path": "/Users/jake/ClientWork",
      "description": "Client projects",
      "registered_at": "2026-02-08T00:20:00Z"
    }
  }
}
```

**Backward compatibility:** The `home_pointer` field is retained for backward
compatibility. On first run after upgrade, if `project_contexts` is empty,
Toad auto-registers the existing `home_pointer` as a context named `default`
and sets it as active. Existing users experience zero disruption.

### Data Models (`toad-core`, MIT)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectContext {
    pub path: PathBuf,
    pub description: Option<String>,
    pub registered_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    /// Legacy field — retained for backward compatibility
    pub home_pointer: PathBuf,
    /// The name of the currently active project context
    pub active_context: Option<String>,
    /// All registered project contexts
    pub project_contexts: HashMap<String, ProjectContext>,
}
```

### How It Affects Other Commands

**Every command that resolves a workspace root now reads `active_context`
instead of `home_pointer` directly.**

The `Workspace::discover()` method changes:

1. Load `GlobalConfig`
2. If `active_context` is set, resolve the path from `project_contexts`
3. Fall back to `home_pointer` if no context is set (backward compat)
4. `TOAD_ROOT` env var still overrides everything (unchanged)

This means:

- `toad status` shows projects in the active context's directory
- `toad ggit status` shows submodule status in the active context's directory
- `toad do` runs commands against projects in the active context's directory
- `toad cw` runs custom workflows (these are global, not context-specific)
- `toad create` creates projects in the active context's directory

**Example workflow:**

```bash
# Register both contexts
toad project register my-code ~/Code --description "Personal projects"
toad project register toad-dev ~/Primatif_Toad --description "Toad development"

# Work on personal projects
toad project switch my-code
toad status                    # Shows projects in ~/Code
toad ggit status               # Shows submodule status in ~/Code projects

# Switch to Toad development
toad project switch toad-dev
toad status                    # Shows Toad's crate submodules
toad ggit status               # Shows git status of toad-core, toad-git, etc.
toad ggit branch feat/new-feature -p toad-core -p discovery

# Check which context you're in
toad project current           # → toad-dev
```

### Why Not Hardcoded Modes?

An earlier design considered `toad mode toad` / `toad mode code` — two
predefined modes. Named project contexts are strictly better:

- **Unlimited contexts** — not limited to two
- **User-defined names** — meaningful to the user, not to Toad
- **No special-casing** — Toad's own development workspace is just another
  context, not a privileged mode
- **Extensible** — a user can add contexts for different clients, different
  machines, different workflows
- **Discoverable** — `toad project list` shows all contexts at a glance

### Storage Reorganization: `~/.toad/` Directory Layout

The current `~/.toad/` directory assumes a single workspace:

```text
~/.toad/                          # CURRENT (single workspace)
├── config.json                   # { home_pointer: "/Users/jake/Code" }
├── registry.json                 # ProjectRegistry — fingerprint, projects, last_sync
└── strategies/
    ├── builtin/                  # Stack detection strategies (TOML)
    └── custom/                   # User-defined strategy overrides

<workspace_root>/                 # CURRENT (shadows at workspace root)
├── projects/                     # Actual project directories
└── shadows/
    ├── MANIFEST.md               # Generated manifest
    └── tags.json                 # Tag registry
```

**The problem:** Both `registry.json` and `shadows/` are workspace-specific
but stored in ways that break with multiple contexts:

- `registry.json` sits at `~/.toad/` — a single global file for one
  workspace's scan cache. Switching contexts makes it stale.
- `shadows/` sits at the workspace root — this pollutes the user's project
  directory with Toad-internal files (`MANIFEST.md`, `tags.json`). Users
  shouldn't have to `.gitignore` Toad artifacts in their own repos. More
  importantly, each registered context needs its own manifest, tags, and
  scan cache since they describe completely different sets of projects.

**The solution:** All per-context artifacts live under
`~/.toad/contexts/<name>/`. This includes the registry, shadows, and any
future per-context state. Global artifacts (config, strategies, custom
workflows) stay at the `~/.toad/` root. Nothing is written to the user's
workspace directory.

```text
~/.toad/                          # NEW (multi-context)
├── config.json                   # GlobalConfig with project_contexts + active_context
├── custom_workflows.json         # toad cw registry (global, not context-specific)
├── strategies/
│   ├── builtin/                  # Stack detection strategies (global)
│   └── custom/                   # User strategy overrides (global)
└── contexts/
    ├── my-code/
    │   ├── registry.json         # ProjectRegistry for ~/Code
    │   └── shadows/
    │       ├── MANIFEST.md       # Generated manifest for ~/Code projects
    │       └── tags.json         # Tags for ~/Code projects
    ├── toad-dev/
    │   ├── registry.json         # ProjectRegistry for ~/Primatif_Toad
    │   └── shadows/
    │       ├── MANIFEST.md       # Generated manifest for Toad crate submodules
    │       └── tags.json         # Tags for Toad crate submodules
    └── client/
        ├── registry.json         # ProjectRegistry for ~/ClientWork
        └── shadows/
            ├── MANIFEST.md
            └── tags.json
```

**What goes where:**

| Artifact | Scope | Location |
|---|---|---|
| `config.json` | Global | `~/.toad/config.json` |
| `custom_workflows.json` | Global | `~/.toad/custom_workflows.json` |
| `strategies/` | Global | `~/.toad/strategies/` |
| `registry.json` | Per-context | `~/.toad/contexts/<name>/registry.json` |
| `shadows/MANIFEST.md` | Per-context | `~/.toad/contexts/<name>/shadows/MANIFEST.md` |
| `shadows/tags.json` | Per-context | `~/.toad/contexts/<name>/shadows/tags.json` |

**Why everything per-context lives under `~/.toad/contexts/`:**

- Each context has different projects → different manifests, tags, and scan
  caches. Mixing them would produce incorrect results.
- Keeping shadows out of the workspace root means Toad doesn't pollute the
  user's project directories. No `.gitignore` entries needed for Toad files.
- All Toad state is centralized in `~/.toad/` — easy to back up, easy to
  inspect, easy to clean up.
- When a context is deleted, removing `~/.toad/contexts/<name>/` cleanly
  removes all associated state in one operation.

**Code changes required:**

1. `ProjectRegistry::registry_path()` currently returns
   `~/.toad/registry.json`. It must change to accept a context name and return
   `~/.toad/contexts/<name>/registry.json`.

2. `ProjectRegistry::load()` and `save()` must accept the active context name
   (or the `GlobalConfig` itself) to resolve the correct path.

3. `Workspace::with_root()` currently sets `shadows_dir` to
   `<root>/shadows/`. This must change: `shadows_dir` should resolve to
   `~/.toad/contexts/<name>/shadows/` based on the active context. The
   `Workspace` struct needs access to the context name (or the resolved
   context directory) during construction.

4. `Workspace::manifest_path()` and `Workspace::tags_path()` derive from
   `shadows_dir` — no changes needed once `shadows_dir` points to the
   correct location.

5. `Workspace::ensure_shadows()` creates the shadows directory — still works,
   just creates it under `~/.toad/contexts/<name>/shadows/` instead.

6. `toad project register` must create the `~/.toad/contexts/<name>/`
   directory (including `shadows/` subdirectory) when registering a new
   context.

7. `toad project delete` must remove the `~/.toad/contexts/<name>/` directory
   (after confirmation) when deleting a context. This cleanly removes the
   registry, shadows, and any future per-context state.

**Backward compatibility migration:**

On first run after upgrade, if `~/.toad/registry.json` exists at the old
location and `~/.toad/contexts/` does not exist:

1. Create `~/.toad/contexts/default/shadows/`
2. Move `~/.toad/registry.json` → `~/.toad/contexts/default/registry.json`
3. If `<home_pointer>/shadows/` exists, move its contents
   (`MANIFEST.md`, `tags.json`) →
   `~/.toad/contexts/default/shadows/`
4. Remove the now-empty `<home_pointer>/shadows/` directory
5. This happens alongside the `home_pointer` → `default` context migration

After migration, the old `~/.toad/registry.json` no longer exists and the
old `<workspace_root>/shadows/` directory is gone. All per-context
reads/writes go through `~/.toad/contexts/<name>/`.

**Installation flow update:**

The `toad home <path>` command currently:

1. Validates the path exists
2. Creates `.toad-root` marker if missing (with confirmation)
3. Saves `GlobalConfig { home_pointer: path }`

With project contexts, `toad home <path>` becomes a convenience alias for:

1. `toad project register default <path>` (if no contexts exist yet)
2. `toad project switch default`

Or, if contexts already exist, `toad home <path>` registers a new context
with an auto-generated name (e.g., the directory's basename) and switches to
it. The `toad home` command (no args) continues to show the current workspace
root — now resolved from the active context.

Long-term, `toad home` may be deprecated in favor of `toad project` commands,
but it remains as a quick-start shortcut for new users who don't need multiple
contexts yet.

### Developer Setup: Toad Working on Itself

When a new contributor clones Primatif_Toad for development, they need to:

1. Clone the main repo
2. Initialize and clone all submodules (the crate repos)
3. Register the Toad development context

This is automated by `scripts/dev_setup.sh`:

```bash
#!/bin/bash
# Fresh clone setup for Toad development

# 1. Clone with submodules
git clone --recurse-submodules https://github.com/Primatif/Primatif_Toad.git
cd Primatif_Toad

# 2. Build to verify everything works
cargo build
cargo test

# 3. Register as a project context (if toad is installed)
if command -v toad &> /dev/null; then
    toad project register toad-dev "$(pwd)" \
        --description "Toad development workspace"
    toad project switch toad-dev
    echo "✅ Registered and switched to 'toad-dev' context"
fi
```

### History Cleanup: Post-Split

After the crate code is extracted into submodule repos (Phase 2), the main
repo's git history still contains the full crate source code. This bloats
the repo unnecessarily since the code now lives in the submodule repos.

**Solution:** Use `git filter-repo` to remove the crate directories from the
main repo's history after the submodule conversion is complete. This is a
one-time, irreversible operation that should be done carefully:

```bash
# Remove crate source from history (keep submodule references)
git filter-repo --path crates/toad-core/src --invert-paths
git filter-repo --path crates/toad-git/src --invert-paths
# ... etc for each crate
```

**This is a project-specific script** (`scripts/history_cleanup.sh`), not a
`toad` CLI feature. It runs once during the v1.0.2 migration and never again.

---

## Multi-Repo Git Orchestration (`toad ggit`)

### The Problem

Working across multiple submodule repos inside a parent workspace creates
friction. The developer must constantly `cd` into submodule directories to run
git commands, mentally track which repos have uncommitted changes, coordinate
branch names across repos, and manage PRs for each repo independently.

Using `toad do` to batch raw shell commands is a workaround, not a solution.
It provides no structure, no error handling specific to git operations, no
awareness of branch state, and no ability for AI agents to reason about the
multi-repo git workflow.

### The Solution: `toad ggit` — First-Class Group-Git Operations

`toad ggit` ("group-git") is a dedicated subcommand namespace for **structured, multi-repo
git operations**. These are not shell passthrough — they are purpose-built
commands that understand the submodule topology and provide structured output
that AI agents can consume.

**Key design principles:**

1. **Project-scoped by default.** Every command accepts `--project <name>` (or
   `-p <name>`) to target a specific submodule. Without it, the command
   operates on all submodules (or the parent, depending on context).
2. **Focus mode.** When targeting a single project, the command behaves as if
   you're inside that repo — no need to `cd` into the directory.
3. **Structured output.** Commands return structured results (not raw git
   output) that AI agents can parse and act on.
4. **BSL-1.1 protected.** The orchestration logic lives in `toad-git` (BSL-1.1).
   The CLI surface is in `bin/toad` (MIT). Data models are in `toad-core` (MIT).
5. **Naming:** `ggit` stands for "group-git" — short, memorable, and
   immediately communicates the multi-repo intent.

### Command Surface

```text
toad ggit status [--project <name>]
    Show branch, dirty state, ahead/behind for one or all repos.

toad ggit branch <name> [--project <name>]
    Create a branch in one or all repos.

toad ggit checkout <branch> [--project <name>]
    Switch branch in one or all repos.

toad ggit add [--all] [--project <name>]
    Stage changes in one or all repos.

toad ggit commit -m "<message>" [--project <name>]
    Commit staged changes in one or all repos.

toad ggit push [--project <name>]
    Push current branch in one or all repos.

toad ggit pull [--project <name>]
    Pull latest from remote in one or all repos.

toad ggit sync
    Update all submodule references in the parent repo to match
    the current HEAD of each submodule. This is the "publish the
    submodule state to the parent" operation.

toad ggit log [--project <name>] [-n <count>]
    Show recent commits for one or all repos.

toad ggit diff [--project <name>]
    Show uncommitted changes for one or all repos.
```

### Workflow Examples

**Cross-repo feature branch:**

```bash
# Create a feature branch across all crate repos
toad ggit branch feat/new-scanner

# Work on discovery crate specifically
toad ggit checkout feat/new-scanner -p discovery
# ... edit files in crates/discovery/ ...
toad ggit add --all -p discovery
toad ggit commit -m "feat(discovery): add new scanner" -p discovery
toad ggit push -p discovery

# Also update toad-core with new types
toad ggit add --all -p toad-core
toad ggit commit -m "feat(core): add scanner types" -p toad-core
toad ggit push -p toad-core

# Update parent repo's submodule references
toad ggit sync
```

**Hyperfocus on a single repo:**

```bash
# See full status of just the discovery repo
toad ggit status -p discovery

# View recent commits
toad ggit log -p discovery -n 10

# Check diff before committing
toad ggit diff -p discovery

# Commit and push
toad ggit commit -m "fix: edge case in scanner" -p discovery
toad ggit push -p discovery
```

**AI agent workflow:**

```bash
# AI agent checks which repos have uncommitted changes
toad ggit status
# Structured output tells the agent exactly which repos are dirty,
# which branches they're on, and whether they're ahead/behind remote.

# AI agent commits changes it made to a specific crate
toad ggit add --all -p toad-manifest
toad ggit commit -m "feat(manifest): add submodule context" -p toad-manifest
toad ggit push -p toad-manifest
```

### Architecture & License Placement

| Layer | Crate | License | Responsibility |
| :---- | :---- | :------ | :------------- |
| Data Models | `toad-core` | MIT | `RepoStatus`, `BranchInfo`, `CommitInfo` structs |
| Orchestration | `toad-git` | BSL-1.1 | All git operation logic, submodule targeting, branch coordination |
| CLI Surface | `bin/toad` | MIT | `toad ggit` subcommand parsing and output formatting |

### Data Models (`toad-core`, MIT)

New types needed to support structured git operations:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoStatus {
    pub name: String,
    pub path: PathBuf,
    pub branch: Option<String>,
    pub is_detached: bool,
    pub dirty_files: u32,
    pub staged_files: u32,
    pub untracked_files: u32,
    pub ahead: u32,
    pub behind: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchInfo {
    pub name: String,
    pub is_current: bool,
    pub upstream: Option<String>,
    pub last_commit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub short_hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: String,
}
```

### Orchestration Logic (`toad-git`, BSL-1.1)

The `toad-git` crate currently only has `status.rs` with a single
`check_status` function. It needs to expand into a full git operations module:

```text
crates/toad-git/src/
├── lib.rs
├── status.rs          ← existing (expand with RepoStatus)
├── branch.rs          ← new: create, checkout, list, delete
├── commit.rs          ← new: stage, commit, diff
├── remote.rs          ← new: push, pull, fetch
├── sync.rs            ← new: submodule ref update in parent
├── log.rs             ← new: commit log with structured output
└── tests.rs           ← existing (expand)
```

Each module exposes functions that:

1. Accept a `&Path` to the target repo (resolved from `--project` name)
2. Execute the appropriate git commands
3. Return structured results (the `toad-core` types above)
4. Handle errors with context (which repo, what operation, what failed)

### Why Not `toad do`?

`toad do` is a generic batch shell executor. It's the right tool for running
arbitrary commands across projects. But git operations need more:

- **Structured output:** `toad ggit status` returns `RepoStatus` structs, not
  raw text. AI agents can parse and reason about this.
- **Error context:** When a push fails, `toad ggit push` tells you which repo,
  which branch, and why — not just a shell exit code.
- **Topology awareness:** `toad ggit sync` knows about the parent-submodule
  relationship and updates references accordingly. `toad do` has no concept
  of this.
- **License boundary:** As a first-class feature in `toad-git` (BSL-1.1), this
  is protected commercial value. `toad do` is generic infrastructure.
- **AI ergonomics:** Structured commands with predictable flags and output are
  far more reliable for AI agents than shell command strings.

### Branch Orchestration & Lifecycle Tracking

In a multi-repo workspace, each submodule has its own full branch lifecycle:
feature branches → development → main, with PRs at each merge point. Without
coordination, this quickly becomes unmanageable — you lose track of which repos
have a given feature branch, whether it's been merged, and how commits relate
across repos.

`toad ggit` solves this with **branch groups** — a logical grouping of branches
across repos that represent the same unit of work.

#### The Problem

When working on `feat/new-scanner` across `toad-core`, `discovery`, and
`toad-git`:

- You create the branch in 3 repos but forget `toad-manifest`
- You push changes to `discovery` but forget to push `toad-core`
- You merge the PR in `discovery` but the PR in `toad-core` is still open
- Your AI agent doesn't know which repos still need work on this feature
- Commit messages across repos don't reference each other
- After merging to development, you can't easily verify all repos are aligned

#### Branch Naming Convention

`toad ggit` enforces (or strongly encourages) a consistent naming convention
across repos:

```text
{type}/{scope}

Examples:
  feat/new-scanner
  fix/status-display
  chore/update-deps
  release/v1.0.2
```

When you run `toad ggit branch feat/new-scanner`, the same branch name is
created in all targeted repos. This shared name is the **branch group
identifier** — it's how Toad correlates work across repos.

#### Additional Commands

```text
toad ggit branches [--group <name>]
    List all branches across all repos. Without --group, shows a matrix
    of which branches exist in which repos. With --group, shows detailed
    status for that branch group.

    Example output (no filter):
    Branch               toad-core  discovery  toad-git  scaffold
    main                 ✅          ✅          ✅         ✅
    development          ✅          ✅          ✅         ✅
    feat/new-scanner     ✅          ✅          ✅         —
    fix/status-display   —           ✅          —          —

    Example output (--group feat/new-scanner):
    feat/new-scanner
      toad-core    ✅ 3 ahead, 0 behind development  (PR #12 open)
      discovery    ✅ 1 ahead, 0 behind development  (PR #8 merged)
      toad-git     ✅ 2 ahead, 1 behind development  (no PR)
      scaffold     —  branch does not exist

toad ggit merge-status <branch>
    Show which repos have merged the given branch into their development
    (or main) branch. Useful for verifying a feature is fully landed.

    Example output:
    feat/new-scanner → development
      toad-core    ❌ not merged  (PR #12 open, 3 commits)
      discovery    ✅ merged      (PR #8, merged 2h ago)
      toad-git     ❌ not merged  (no PR created)

toad ggit pr [--project <name>]
    Show PR status for the current branch across all repos. If no PR
    exists, indicate that. Requires GitHub API access (token).

toad ggit align <branch>
    Verify all repos on the given branch are pushed, up-to-date with
    their remote, and have no uncommitted changes. Reports any repo
    that is out of alignment.
```

#### Commit Message Convention

To connect commits across repos, `toad ggit commit` encourages (and can
enforce) a convention that includes the branch group context:

```text
{type}({scope}): {description}

Examples:
  feat(discovery): add .gitmodules parser
  feat(core): add SubmoduleDetail type
  fix(toad-git): handle detached HEAD in status
```

When committing across multiple repos in the same session, `toad ggit` can
optionally append a **group reference** to the commit body:

```text
feat(discovery): add .gitmodules parser

Part of: feat/new-scanner
Related: toad-core@abc1234, toad-git@def5678
```

This is opt-in — `toad ggit commit -m "msg" --link` adds the group reference
automatically. Without `--link`, it's a plain commit.

#### Data Models for Branch Tracking (`toad-core`, MIT)

```rust
/// Tracks the state of a branch across multiple repos.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchGroup {
    /// The shared branch name (e.g., "feat/new-scanner")
    pub name: String,
    /// Per-repo status for this branch
    pub repos: Vec<BranchGroupEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchGroupEntry {
    /// Repo/project name
    pub repo: String,
    /// Whether the branch exists in this repo
    pub exists: bool,
    /// Ahead/behind relative to the target branch (e.g., development)
    pub ahead: u32,
    pub behind: u32,
    /// Whether the branch has been merged into the target
    pub merged: bool,
    /// PR status if known
    pub pr_status: Option<PrStatus>,
    /// Latest commit on this branch in this repo
    pub head_commit: Option<CommitInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrStatus {
    /// No PR exists for this branch in this repo
    None,
    /// PR is open
    Open { number: u32, url: String },
    /// PR has been merged
    Merged { number: u32, url: String },
    /// PR was closed without merging
    Closed { number: u32, url: String },
}
```

#### Orchestration Logic (`toad-git`, BSL-1.1)

New modules needed for branch orchestration:

```text
crates/toad-git/src/
├── ...existing modules...
├── branches.rs        ← new: cross-repo branch listing and grouping
├── merge_status.rs    ← new: merge detection across repos
├── align.rs           ← new: alignment verification
└── pr.rs              ← new: PR status via GitHub API (optional)
```

The `pr.rs` module is optional for v1.0.2 — it requires GitHub API
integration (personal access token). The core branch tracking
(`branches.rs`, `merge_status.rs`, `align.rs`) works purely with local
git data and is the priority.

#### Pre-flight Check: Preventing Ghost Commits

Submodules are stateful pointers. When `toad ggit sync` updates the parent
repo's index to reference a submodule's HEAD commit, that commit must actually
exist on the submodule's remote — otherwise the parent repo becomes "broken"
for everyone else who clones or pulls it.

**The Ghost Commit problem:**

1. You work in `toad-core`, commit locally but forget to push
2. `toad ggit sync` updates the parent's submodule pointer to your local SHA
3. You push the parent repo
4. Anyone who clones the parent gets a pointer to a commit that doesn't exist
   on the `toad-core` remote — the repo is broken

**The fix: `toad ggit sync` includes a mandatory pre-flight check.**

Before updating any submodule SHA in the parent, `sync` verifies:

1. The submodule's local HEAD commit exists on its remote
   (`git branch -r --contains <sha>`)
2. The submodule has no uncommitted changes (dirty state)
3. The submodule is not in a detached HEAD state (unless intentional)

If any check fails, `sync` **blocks** and reports exactly which repos need
attention:

```text
$ toad ggit sync

  Pre-flight check failed:

  toad-core     ❌ HEAD (abc1234) not pushed to remote
                   Run: toad ggit push -p toad-core
  discovery     ❌ Dirty — 2 uncommitted files
                   Run: toad ggit commit -p discovery
  toad-git      ✅ Ready
  scaffold      ✅ Ready

  Sync aborted. Fix the issues above and retry.
```

**`--force` with confirmation:** For local-only workflows where you
intentionally don't need the remote check, `toad ggit sync --force` bypasses
the pre-flight — but because this is a dangerous operation that can break the
repo for others, it requires an interactive confirmation:

```text
$ toad ggit sync --force

  ⚠️  WARNING: Forcing sync bypasses the pre-flight check.
  The parent repo may reference commits that don't exist on remotes.
  This can break the repo for other contributors.

  Are you sure? [y/N]: _
```

AI agents should never use `--force` without explicit human approval.

**Standalone pre-flight:** `toad ggit preflight` runs the same checks without
performing the sync. Useful for CI pipelines, pre-commit hooks, or just
checking readiness before syncing:

```text
toad ggit preflight
    Run the pre-flight check for all submodules without syncing.
    Returns structured pass/fail per repo.
```

#### Output Consolidation

When `toad ggit` executes git operations across multiple repos, each git
command produces its own stdout/stderr output. Dumping raw git output from 6+
repos into the terminal is noisy and unusable — especially for AI agents.

**Design: Toad captures and consolidates all git output.**

1. **Capture:** All git command stdout/stderr is captured (not streamed
   directly to the terminal).
2. **Summarize:** Toad produces a structured summary per repo:
   - Success: repo name + one-line result (e.g., "pushed 3 commits to origin/main")
   - Failure: repo name + error message + the raw git stderr for debugging
3. **Detail on demand:** `--verbose` (or `-v`) streams the full raw git output
   per repo as it executes, for debugging or when you want to see exactly
   what git is doing.
4. **Structured for AI:** The default (non-verbose) output is designed to be
   parseable by AI agents — structured, predictable, and actionable.

**Example: `toad ggit push`**

Default output (consolidated):

```text
$ toad ggit push

  toad-core     ✅ Pushed 2 commits to origin/feat/new-scanner
  discovery     ✅ Pushed 1 commit to origin/feat/new-scanner
  toad-git      ✅ Already up-to-date
  scaffold      ⚠️  No remote configured
  toad-ops      ❌ Push rejected — remote has diverged
                   Hint: run toad ggit pull -p toad-ops first
```

Verbose output (`-v`):

```text
$ toad ggit push -v

  ── toad-core ──
  Enumerating objects: 5, done.
  Counting objects: 100% (5/5), done.
  ...
  To github.com:Primatif/toad-core.git
     abc1234..def5678  feat/new-scanner -> feat/new-scanner

  ── discovery ──
  ...
```

**Architecture:**

| Layer | Responsibility |
| :---- | :------------- |
| `toad-git` (BSL-1.1) | Captures git stdout/stderr, returns structured `GitOpResult` |
| `bin/toad` (MIT) | Formats `GitOpResult` into consolidated or verbose terminal output |

**Data model (`toad-core`, MIT):**

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitOpResult {
    pub repo: String,
    pub operation: String,
    pub success: bool,
    pub summary: String,
    pub raw_stdout: String,
    pub raw_stderr: String,
}
```

#### Workflow: Full Feature Lifecycle

```bash
# 1. Start a feature across relevant repos
toad ggit branch feat/new-scanner -p toad-core -p discovery -p toad-git

# 2. Work on individual repos (hyperfocus)
toad ggit checkout feat/new-scanner -p discovery
# ... edit code ...
toad ggit commit -m "feat(discovery): add scanner" -p discovery --link
toad ggit push -p discovery

# 3. Check cross-repo status at any time
toad ggit branches --group feat/new-scanner

# 4. Verify alignment before creating PRs
toad ggit align feat/new-scanner

# 5. After PRs are merged, verify the feature is fully landed
toad ggit merge-status feat/new-scanner

# 6. Clean up: delete the feature branch from all repos
toad ggit branch -d feat/new-scanner

# 7. Sync parent repo to point to latest development commits
toad ggit sync
```

---

## Publishing Waterfall: Crates.io Release Automation (Project-Specific)

> **This is NOT a `toad ggit` feature.** `toad ggit` is a generic multi-repo
> git orchestration tool for any project. Crates.io publishing is specific to
> Rust workspaces and to the Toad project itself. The solution lives in
> `scripts/`, not in the CLI.

### The Problem

Crates.io does not allow path dependencies. To `cargo publish` the `bin/toad`
binary, every internal crate it depends on must already be published. Because
the crates have inter-dependencies, publishing must follow the dependency
graph in strict topological order:

```text
Layer 0:  toad-core          (no internal deps — publish first)
             ↓
Layer 1:  toad-git            (depends on toad-core)
          toad-manifest       (depends on toad-core)
          scaffold            (no internal deps, but publish with the batch)
             ↓
Layer 2:  discovery           (depends on toad-core + toad-git)
          toad-ops            (depends on toad-core)
             ↓
Layer 3:  bin/toad            (depends on everything — publish last)
```

A single struct change in `toad-core` triggers a **waterfall**: publish
`toad-core`, wait for the crates.io index to update, bump the version in
every downstream `Cargo.toml`, publish those crates, wait again, and finally
publish `bin/toad`. This is an order of magnitude more work than a monorepo
`cargo publish`.

### Dual Dependency Strategy

Each crate's `Cargo.toml` must use the dual `version` + `path` pattern so
that local development uses path resolution while crates.io uses the
published version:

```toml
[dependencies]
toad-core = { version = "1.0.2", path = "../toad-core" }
```

This must be set up correctly from the start when splitting into submodules.
The existing `sync_version.sh` script handles README badge syncing; the
publish script extends this pattern to `Cargo.toml` version fields.

### Solution: `scripts/publish_waterfall.sh`

A shell script in `scripts/` that automates the full publish cascade:

1. **Resolve the dependency graph** — hardcoded or parsed from `Cargo.toml`
   files, ordered topologically
2. **Bump versions** — update `version = "x.y.z"` in each crate's
   `Cargo.toml` and all downstream dependency references
3. **Dry-run check** — `cargo publish --dry-run` for each crate in order
4. **Publish in order** — `cargo publish` for each crate, waiting for
   crates.io index propagation between layers (poll with
   `cargo search <crate> | grep <version>`)
5. **Commit version bumps** — after all publishes succeed, commit the
   `Cargo.toml` changes
6. **Tag the release** — `git tag v<version>` in each submodule and the
   parent repo

This script is invoked manually as part of the release workflow — it is not
a `toad` CLI command. It lives alongside `sync_version.sh` and `install_toad.sh`
in the `scripts/` directory.

### Why Not a `toad ggit` Command?

- `toad ggit` is **language-agnostic** — it works with any multi-repo setup
  (Rust, TypeScript, Python, mixed). Publishing to crates.io is Rust-specific.
- `toad ggit` is **generic** — it doesn't know about `Cargo.toml`, `package.json`,
  or any package manager. Adding language-specific publishing would violate
  the tool's design boundary.
- The publish waterfall is a **project workflow**, not a git operation. It
  belongs in `scripts/` where project-specific automation lives.
- With the Custom Workflows feature (below), this script can be registered
  as `toad cw release` — giving it a first-class CLI entry point without
  polluting the generic `toad` command namespace.

---

## Custom Workflows (`toad cw`)

Users inevitably need project-specific automation that doesn't belong in the
generic `toad` CLI — language-specific publish scripts, deploy pipelines,
database migrations, environment setup, etc. Rather than forcing users to
remember script paths or maintain aliases, Toad provides a **custom workflow
registry** that lets users register, manage, and invoke their own shell
scripts as first-class `toad` subcommands.

### Design Principles

- **Shell only.** Custom workflows must be shell scripts (`.sh`). This keeps
  the execution model simple, portable, and auditable.
- **Namespace protection.** Users cannot register a workflow name that
  collides with a built-in `toad` command (`status`, `do`, `ggit`, `create`,
  `home`, `stats`, `clean`, `tag`, `untag`, `cw`, `project`, etc.). The
  reserved namespace list is maintained **centrally in `toad-ops`** and
  derived programmatically from `bin/toad`'s command definitions — so any
  future command added to the binary automatically blocks its name in the
  workflow registry. No manual list maintenance required.
- **Updatable.** Registered workflows can be updated (new script path, new
  description) without re-registering.
- **Global scope.** Workflows are registered per-user in `~/.toad/`, not
  per-workspace. A registered workflow is available from any Toad workspace.
- **Discoverable.** Users and AI agents can list, inspect, and reason about
  registered workflows through structured commands.

### Command Surface

```text
toad cw <name> [args...]
    Run a registered custom workflow. Any additional arguments are passed
    through to the script as positional args.

    Example:
      toad cw release 1.0.2
      → Executes the registered "release" script with "1.0.2" as $1

toad cw register <name> <script_path> --description "<text>"
    Register a new custom workflow. The script_path must be an absolute
    path to an existing .sh file. The script is NOT copied — Toad stores
    a reference to the path. The description is required.

    Validation:
      - Name must not collide with reserved namespaces
      - Script must exist and be a .sh file
      - Script must be executable (or Toad warns and offers to chmod +x)

    Example:
      toad cw register release /Users/jake/Primatif_Toad/scripts/publish_waterfall.sh \
        --description "Publish all crates to crates.io in dependency order"

toad cw update <name> [--script <path>] [--description "<text>"]
    Update an existing workflow's script path and/or description.

toad cw delete <name>
    Remove a registered workflow. Prompts for confirmation.

toad cw list
    List all registered workflows with their names, descriptions, and
    script paths.

    Example output:
    Name       Description                                          Script
    release    Publish all crates to crates.io in dependency order   /Users/jake/.../publish_waterfall.sh
    deploy     Deploy staging environment                            /Users/jake/.../deploy_staging.sh
    seed-db    Seed development database                             /Users/jake/scripts/seed.sh

toad cw info <name>
    Show detailed information about a specific workflow.

    Example output:
    Name:          release
    Description:   Publish all crates to crates.io in dependency order
    Script:        /Users/jake/Primatif_Toad/scripts/publish_waterfall.sh
    Registered:    2026-02-08 00:04:00
    Last Updated:  2026-02-08 00:04:00
    Last Run:      2026-02-08 12:30:00 (exit code 0)
```

### Storage: `~/.toad/custom_workflows.json`

The workflow registry is a JSON file in the user's global Toad config
directory (`~/.toad/`), alongside the existing `GlobalConfig`:

```json
{
  "workflows": {
    "release": {
      "description": "Publish all crates to crates.io in dependency order",
      "script_path": "/Users/jake/Primatif_Toad/scripts/publish_waterfall.sh",
      "registered_at": "2026-02-08T00:04:00Z",
      "updated_at": "2026-02-08T00:04:00Z",
      "last_run_at": "2026-02-08T12:30:00Z",
      "last_exit_code": 0
    },
    "deploy": {
      "description": "Deploy staging environment",
      "script_path": "/Users/jake/projects/infra/scripts/deploy_staging.sh",
      "registered_at": "2026-02-07T10:00:00Z",
      "updated_at": "2026-02-07T10:00:00Z",
      "last_run_at": null,
      "last_exit_code": null
    }
  },
  "reserved_namespaces": [
    "status", "do", "ggit", "create", "home", "stats",
    "clean", "tag", "untag", "cw", "project", "help", "version"
  ]
}
```

The `reserved_namespaces` list in the JSON is a **cache**. The authoritative
list is maintained centrally in `toad-ops` via a
`reserved_command_names() -> Vec<&str>` function that returns all built-in
command names. This function is the single source of truth:

- When `toad cw register` runs, it calls `reserved_command_names()` to
  validate — not the cached JSON list.
- When `toad` starts, it syncs the JSON cache from the function so that
  external tools reading the JSON see the current list.
- When a new command is added to `bin/toad`, the developer adds its name to
  the `reserved_command_names()` function in `toad-ops`. This is enforced
  by a unit test that compares the function's output against the actual
  `Commands` enum variants in `bin/toad`.

### Data Models (`toad-core`, MIT)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomWorkflow {
    pub description: String,
    pub script_path: PathBuf,
    pub registered_at: String,
    pub updated_at: String,
    pub last_run_at: Option<String>,
    pub last_exit_code: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRegistry {
    pub workflows: HashMap<String, CustomWorkflow>,
    /// Cache of reserved names — authoritative list is in toad-ops
    pub reserved_namespaces: Vec<String>,
}
```

### Execution Model

When `toad cw <name> [args...]` is invoked:

1. Load `~/.toad/custom_workflows.json`
2. Look up `name` in the registry — error if not found
3. Verify the script file still exists at the registered path — error with
   a clear message if it's been moved or deleted
4. Execute the script via `sh <script_path> [args...]`
5. Stream stdout/stderr directly to the terminal (no capture — the user
   wants to see their script's output in real time)
6. After completion, update `last_run_at` and `last_exit_code` in the
   registry
7. Exit with the script's exit code

### Architecture

| Layer | Responsibility |
| :---- | :------------- |
| `toad-core` (MIT) | `CustomWorkflow`, `WorkflowRegistry` structs, JSON load/save |
| `toad-ops` (BSL-1.1) | Workflow execution logic (validate, run, update metadata) |
| `bin/toad` (MIT) | CLI surface (`toad cw` subcommand), output formatting |

### Why `toad-ops` and Not `bin/toad`?

The execution logic (script validation, path checking, exit code tracking)
is reusable intelligence that could be consumed by other interfaces (e.g.,
a future `toad-mcp` server for AI agents). Keeping it in `toad-ops` follows
the same pattern as `toad do` — the binary handles CLI concerns, the crate
handles operational logic.

---

## Git Operations Boundary: Everything Git → `toad-git` (BSL-1.1)

All git-related logic in Toad — existing and future — must live within the
`toad-git` crate (BSL-1.1). This is not just about the new `toad ggit`
commands. **Any code that executes git commands, parses git output, or reasons
about git state is git intelligence and belongs in the protected layer.**

### The Rule

No crate outside of `toad-git` may:

- Execute `git` commands (via `Command::new("git")` or any git library)
- Parse `.git/` directory contents
- Interpret git status, branch, or commit data
- Detect or reason about git-specific patterns (e.g., destructive git commands)

Other crates may **consume** the structured types that `toad-git` returns
(e.g., `RepoStatus`, `BranchInfo`, `VcsStatus`), but the logic that produces
those types must live in `toad-git`.

### Current Violations (Pre-v1.0.2)

An audit of the current codebase reveals the following violations:

1. **`scaffold/src/scaffold.rs` — `init_git()`** (MIT crate)
   - Runs `Command::new("git").arg("init")` directly
   - **Fix:** Move `init_git()` to `toad-git` as `toad_git::init::init_repo()`.
     The `scaffold` crate calls `toad_git::init::init_repo(&path)` instead.
   - **Impact:** `scaffold` gains a dependency on `toad-git`. Since `scaffold`
     is MIT and `toad-git` is BSL-1.1, this means `scaffold` can no longer be
     used independently without `toad-git`. **Decision needed:** either accept
     this dependency (scaffold becomes effectively BSL-1.1 in practice), or
     make `git init` optional in scaffold and let the caller handle it.

2. **`toad-ops/src/safety.rs` — destructive command detection** (BSL-1.1 crate)
   - Pattern-matches git command strings (`git reset --hard`, `git push -f`)
   - **Status:** Already in a BSL-1.1 crate, so no license violation. However,
     this is git-aware intelligence that arguably belongs in `toad-git` for
     architectural consistency. Consider moving the git-specific patterns to
     `toad-git` and having `toad-ops` call a `toad_git::safety::is_destructive_git_command()`
     function.

3. **`discovery/src/lib.rs` — `detect_vcs_status()`** (BSL-1.1 crate)
   - Calls `toad_git::status::check_status()` and maps the result
   - **Status:** No violation — correctly delegates to `toad-git`. The mapping
     from `GitStatus` to `VcsStatus` is just type conversion, not git logic.

4. **`toad-core/src/lib.rs` — `.git/index` in fingerprint paths** (MIT crate)
   - Uses `.git/index` as a path constant for mtime fingerprinting
   - **Status:** Borderline — it's a string constant, not git logic. But it
     encodes knowledge about git internals. Consider moving the git-specific
     fingerprint paths to `toad-git` and having `toad-core` accept them as
     configuration.

### Architectural Consequence for `scaffold`

The `scaffold` crate is currently MIT and has no dependencies on BSL-1.1
crates. Moving `git init` to `toad-git` creates a dependency chain:

```text
scaffold (MIT) → toad-git (BSL-1.1)
```

**Options:**

- **Option A: Accept the dependency.** `scaffold` becomes MIT in license but
  BSL-1.1 in practice (can't function without `toad-git`). This is fine if
  scaffold is always distributed as part of Toad.
- **Option B: Make git init optional.** `scaffold` creates the project
  structure but doesn't init git. The caller (`bin/toad`) calls
  `toad_git::init::init_repo()` after scaffolding. This keeps `scaffold`
  truly independent.
- **Option C: Move scaffold to BSL-1.1.** If scaffold's value is tightly
  coupled to git operations, just relicense it.

**Recommended: Option B.** Keep `scaffold` as a pure filesystem operation
(MIT). Let `bin/toad` orchestrate the git init step after scaffolding. This
maintains the clean license boundary and keeps `scaffold` usable as a
standalone library.

---

## Governance: Licensing-Aware Development

The multi-license architecture introduces a new constraint that all AI agents
and contributors must respect: **every code change must land in the correct
license boundary.**

### The Rule

When extending Toad with new functionality, the developer (human or AI) must
ask:

1. **Is this a data model, trait, or contract?** → It belongs in `toad-core`
   (MIT). Types that define the shape of data flow openly.
2. **Is this intelligence, analysis, or operational logic?** → It belongs in a
   BSL-1.1 crate (`discovery`, `toad-ops`, `toad-manifest`, `toad-git`, or a
   new BSL-1.1 crate). This is the protected value.
3. **Is this CLI glue, formatting, or user interaction?** → It belongs in
   `bin/toad` (MIT). The thin orchestrator layer.

### New Crate Decision Framework

When creating a new crate (e.g., `toad-mcp` for v1.1.0):

- **Default to BSL-1.1** for any crate that contains intelligence, analysis,
  or operational logic.
- **Use MIT only** for crates that define shared contracts (types, traits) or
  provide basic utility with no commercial differentiator.
- **Document the license choice** in the crate's `Cargo.toml` and `LICENSE`
  file before writing any code.

### Dependency Direction Rule

The license boundary enforces a clean dependency direction:

- **MIT crates must never depend on BSL-1.1 crates** (except the binary
  itself). This keeps the open-source types independent.
- **BSL-1.1 crates may depend on MIT crates** (they consume the open
  contracts).
- **BSL-1.1 crates may depend on other BSL-1.1 crates** (e.g., `discovery`
  depends on `toad-git`).

```text
MIT Layer (open contracts)          BSL-1.1 Layer (protected intelligence)
┌─────────────┐                     ┌──────────────┐
│  toad-core   │◄────────────────────│  discovery    │
│  (types)     │◄──────────┐        │  (scanning)   │
└─────────────┘            │        └──────┬───────┘
┌─────────────┐            │               │
│  scaffold    │            │        ┌──────▼───────┐
│  (creation)  │            ├────────│  toad-git     │
└─────────────┘            │        │  (vcs)        │
                           │        └──────────────┘
                           │        ┌──────────────┐
                           ├────────│  toad-manifest│
                           │        │  (context)    │
                           │        └──────────────┘
                           │        ┌──────────────┐
                           └────────│  toad-ops     │
                                    │  (operations) │
                                    └──────────────┘
                    ▲
                    │ bin/toad (MIT) consumes everything
```

### License Boundary Enforcement: Hard Gates

The Dependency Direction Rule above is a human rule. Human rules get broken —
by tired devs, by AI agents that hallucinate library availability, by
well-intentioned refactors that "just need this one import." Relying on
`.gemini/GEMINI.md` or code review to catch violations is insufficient.
**Rules without enforcement are just suggestions.**

Toad needs a **hard gate** that fails the build immediately if an MIT crate
ever gains a dependency on a BSL-1.1 crate.

#### Two Layers of Enforcement

**Layer 1: `scripts/check_license_boundary.sh` (simple, fast, always runs)**

A lightweight shell script that parses `Cargo.toml` files and checks the
internal dependency graph against the license map. No external tools needed.

```bash
# The license map (source of truth)
MIT_CRATES="toad-core scaffold"
BSL_CRATES="discovery toad-git toad-manifest toad-ops"

# For each MIT crate, check if it depends on any BSL crate
# If so, exit 1 with a clear error message
```

This script:

- Runs in **< 1 second** (just `grep` on `Cargo.toml` files)
- Requires **no external dependencies** (no `cargo-deny`, no Rust toolchain)
- Can run as a **git pre-commit hook** (via `scripts/git-hooks/`)
- Can run in **CI** as the first step of any pipeline
- Produces a **clear, actionable error**:

```text
❌ LICENSE BOUNDARY VIOLATION

  scaffold (MIT) depends on toad-git (BSL-1.1)

  MIT crates must never depend on BSL-1.1 crates.
  See: docs/releases/v1.0.2/evolution.md § Dependency Direction Rule

  Fix: Remove the dependency, or move the crate to BSL-1.1.
```

**Layer 2: `cargo-deny` (comprehensive, catches transitive deps)**

[`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) is a Rust
ecosystem tool that enforces license policies on the full dependency graph,
including transitive dependencies. It catches cases the simple script can't:

- A BSL-1.1 crate accidentally pulling in a GPL transitive dependency
- An MIT crate depending on a third-party crate that re-exports BSL code
- License field mismatches between `Cargo.toml` and actual `LICENSE` files

Configuration lives in `deny.toml` at the workspace root:

```toml
[licenses]
unlicensed = "deny"
allow = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "ISC"]

# BSL-1.1 is allowed only for our own crates
[[licenses.exceptions]]
allow = ["BUSL-1.1"]
name = "discovery"
[[licenses.exceptions]]
allow = ["BUSL-1.1"]
name = "toad-git"
[[licenses.exceptions]]
allow = ["BUSL-1.1"]
name = "toad-manifest"
[[licenses.exceptions]]
allow = ["BUSL-1.1"]
name = "toad-ops"

[bans]
# Prevent MIT crates from depending on BSL-1.1 crates
# cargo-deny doesn't have a direct "license boundary" check,
# so we use the [bans] section to deny specific crate combinations
```

**Note:** `cargo-deny` doesn't natively support "crate A with license X must
not depend on crate B with license Y" as a first-class rule. The simple
script (Layer 1) is the primary enforcement mechanism for the internal
boundary. `cargo-deny` is the secondary layer for third-party license
compliance.

#### Where Enforcement Runs

| Gate | When | What it catches |
| :--- | :--- | :-------------- |
| `scripts/check_license_boundary.sh` | Pre-commit hook, CI | MIT crate depends on BSL-1.1 crate |
| `cargo deny check licenses` | CI | Third-party license violations |
| `cargo deny check bans` | CI | Banned crate usage |

#### Why Both?

- The **simple script** is the fast, zero-dependency gate that catches the
  most likely violation (internal dependency direction). It runs everywhere,
  including in git hooks where you don't want to invoke `cargo`.
- **`cargo-deny`** is the comprehensive gate that catches everything else
  (transitive deps, third-party licenses). It requires the Rust toolchain
  and is slower, so it runs in CI only.

### AI Agent Context Updates

The following context files must be updated to encode these rules so that
Gemini, Windsurf, and any other AI agent respects the licensing boundaries
during development:

- **`.gemini/GEMINI.md`** — Add a Licensing Architecture section with the
  boundary rules and the decision framework for new code placement.
- **`conductor/tech-stack.md`** — Update the Component Hierarchy to include
  license annotations per crate, and add a "Licensing Architecture" section
  to the Design Principles.
- **`conductor/product.md`** — Add "Open Core Licensing" as a core goal.
- **`conductor/product-guidelines.md`** — Add a "Licensing & Architecture
  Boundaries" section with the dependency direction rule.

### Cross-Repo Context Map: Solving AI Agent Navigation

Splitting into submodules increases the conceptual boundaries an AI agent
must navigate. A bug in `toad status` requires reading `bin/toad` (CLI call),
`discovery` (scanning logic), `toad-git` (git status), and `toad-core`
(the `RepoStatus` type). Previously all in one tree, now the agent needs to
understand which crate owns which responsibility.

**The physical reality is less scary than it sounds.** With git submodules,
the code is still in the same directory tree — `crates/toad-git/src/status.rs`
is at the exact same path before and after the split. For tools like
Windsurf/Cascade that operate on the workspace directory, `grep`, `find`, and
import-following all work identically. The submodule boundary is invisible to
file search.

**But the conceptual map matters.** An AI agent needs to know:

- Where types are defined vs consumed
- The dependency direction and license boundaries
- The call chain for common operations
- Which crate to edit for a given change

**The solution: `toad manifest` generates a Cross-Repo Context Map.**

The existing `toad manifest` command generates project context. It must be
extended to include a structured map of cross-repo relationships that AI
agents can consume as their first stop for understanding the architecture.

#### What the Cross-Repo Map Contains

```text
# Cross-Repo Context Map (auto-generated by toad manifest)

## Dependency Graph

  toad-core (MIT) ← toad-git (BSL-1.1)
  toad-core (MIT) ← toad-manifest (BSL-1.1)
  toad-core (MIT) ← discovery (BSL-1.1)
  toad-core (MIT) ← toad-ops (BSL-1.1)
  toad-git (BSL-1.1) ← discovery (BSL-1.1)
  scaffold (MIT) ← (no internal deps)
  bin/toad (MIT) ← ALL

## Type Flow

  RepoStatus    defined: toad-core    populated: toad-git    displayed: bin/toad
  BranchInfo    defined: toad-core    populated: toad-git    displayed: bin/toad
  CommitInfo    defined: toad-core    populated: toad-git    displayed: bin/toad
  BranchGroup   defined: toad-core    populated: toad-git    displayed: bin/toad
  GitOpResult   defined: toad-core    populated: toad-git    displayed: bin/toad
  ProjectDetail defined: toad-core    populated: discovery   displayed: bin/toad
  VcsStatus     defined: toad-core    populated: discovery   displayed: bin/toad
  CustomWorkflow defined: toad-core   managed: toad-ops      displayed: bin/toad

## Call Chains (common operations)

  toad status:
    bin/toad::main → discovery::scan_projects → toad_git::status::check_status
    Types: ProjectDetail, VcsStatus, GitStatus

  toad ggit status:
    bin/toad::main → toad_git::status::repo_status (for each submodule)
    Types: RepoStatus

  toad ggit sync:
    bin/toad::main → toad_git::sync::preflight_check → toad_git::sync::sync_submodule_refs
    Types: PreflightResult, GitOpResult

  toad ggit branches:
    bin/toad::main → toad_git::branches::list_branches_across
    Types: BranchGroup, BranchGroupEntry

  toad cw <name>:
    bin/toad::main → toad_ops::custom_workflow::run
    Types: WorkflowRegistry, CustomWorkflow

  toad create:
    bin/toad::main → scaffold::create_project → toad_git::init::init_repo
    Types: (none — filesystem + git side effects)

## Crate Responsibilities

  toad-core     Data models, types, workspace discovery. The "contract" layer.
  toad-git      All git operations. Status, branch, commit, remote, sync.
  discovery     Project scanning, file analysis, stack detection.
  toad-ops      Shell execution, safety checks, cleaning, custom workflows.
  toad-manifest Context generation, manifest output.
  scaffold      Project creation (filesystem only, no git).
  bin/toad      CLI surface, user interaction, output formatting.

## License Boundary

  MIT:      toad-core, scaffold, bin/toad
  BSL-1.1:  discovery, toad-git, toad-manifest, toad-ops
  Rule:     MIT crates must never depend on BSL-1.1 crates (except bin/toad).
```

#### How It's Generated

`toad manifest` already walks the workspace and generates context. The
cross-repo map is an additional output section that:

1. **Parses `Cargo.toml` files** to extract the dependency graph
2. **Reads the license map** (from `deny.toml` or a hardcoded list) to
   annotate each crate
3. **Scans `pub struct` and `pub enum` declarations** in `toad-core` to
   build the Type Flow table
4. **Reads `use` statements** across crates to trace where types are
   consumed
5. **Outputs the map** as a structured text section in the manifest

The map is regenerated every time `toad manifest` runs. It should also be
committed as a static file (e.g., `CROSS_REPO_MAP.md` at the workspace root)
so that AI agents can read it without running `toad manifest`.

#### Why This Is Better Than a Monorepo

A well-generated cross-repo map is actually *more* navigable for AI agents
than a monorepo with no documentation. In a monorepo, the agent has to
*discover* the architecture by reading code. With the cross-repo map, the
architecture is *declared* — the agent knows exactly where to look before
reading a single line of code.

This turns the architect's concern on its head: the split *forces* us to
document the architecture explicitly, which makes the codebase more
navigable, not less.
