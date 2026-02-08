# v1.0.2 "Open Core" — Task Breakdown

> Spec-driven task list derived from
> [`docs/releases/v1.0.2/evolution.md`](./evolution.md). Each task references
> its source section for traceability.

---

## Legend

- **Status:** `[ ]` Not Started · `[~]` In Progress · `[x]` Done
- **Ref:** `§` = section header in the source document

---

## Phase 0: License Files & Cargo.toml Updates

> Ref: `§ The Licensing Strategy`, `§ The License Split`
>
> _No structural changes. Can be done entirely within the current single-repo
> layout. This phase is safe to commit and ship before the submodule
> conversion._

### P0-1: Write BSL-1.1 License Text

- Ref: `§ BSL-1.1 Terms`, `§ License Header Template`
- [ ] Draft the BSL-1.1 LICENSE file with Primatif-specific terms:
  - Licensor: Primatif
  - Change Date: 2034-02-07
  - Change License: MIT
  - Additional Use Grant: non-commercial, personal, educational with attribution
- [ ] Place LICENSE file in each BSL-1.1 crate directory:
  - `crates/discovery/LICENSE`
  - `crates/toad-git/LICENSE`
  - `crates/toad-manifest/LICENSE`
  - `crates/toad-ops/LICENSE`

### P0-2: Write MIT License for Open Crates

- Ref: `§ The License Split > MIT (Open Source)`
- [ ] Copy the existing root MIT LICENSE into each MIT crate directory:
  - `crates/toad-core/LICENSE`
  - `crates/scaffold/LICENSE`
- [ ] Verify the root `LICENSE` file covers `bin/toad/` (it does — the root
      license applies to everything not overridden)

### P0-3: Update Cargo.toml License Fields

- Ref: `§ The License Split`
- [ ] Set `license = "MIT"` in:
  - `crates/toad-core/Cargo.toml`
  - `crates/scaffold/Cargo.toml`
  - `bin/toad/Cargo.toml`
- [ ] Set `license = "LicenseRef-BSL-1.1"` in:
  - `crates/discovery/Cargo.toml`
  - `crates/toad-git/Cargo.toml`
  - `crates/toad-manifest/Cargo.toml`
  - `crates/toad-ops/Cargo.toml`
- [ ] Update the workspace `Cargo.toml` — remove `license = "MIT"` from
      `[workspace.package]` since crates now have individual licenses
- [ ] Run `cargo build` to verify no breakage

### P0-4: Git Operations Boundary — Migration

- Ref: `§ Git Operations Boundary: Everything Git → toad-git (BSL-1.1)`
- [ ] **scaffold: Remove `init_git()`** — delete the `init_git()` function from
      `scaffold/src/scaffold.rs` and remove the `Command::new("git")` call
- [ ] **toad-git: Add `init` module** — create `toad-git/src/init.rs` with:
  - `init_repo(path: &Path) -> Result<()>` — runs `git init` in the given dir
- [ ] **bin/toad: Orchestrate git init after scaffold** — in the `Create`
      command handler, call `toad_git::init::init_repo()` after
      `scaffold::create_project()` returns. This keeps scaffold as a pure
      filesystem operation (Option B from evolution.md).
- [ ] **toad-git: Add git-aware safety functions** — consider moving the
      git-specific destructive command patterns from `toad-ops/src/safety.rs` to
      `toad-git/src/safety.rs` as
      `is_destructive_git_command(cmd: &str) -> bool`. Have `toad-ops` call this
      function instead of inlining the patterns.
- [ ] **toad-core: Evaluate `.git/index` fingerprint path** — decide whether the
      `.git/index` path constant in `toad-core/src/lib.rs` should be provided by
      `toad-git` as configuration or remain as a simple string constant (low
      priority, borderline case)
- [ ] `cargo build` and `cargo test` pass after all migrations
- [ ] Verify no crate outside `toad-git` executes `git` commands:
      `grep -r 'Command::new("git")' crates/ --include="*.rs"` returns only hits
      in `crates/toad-git/`

### P0-5: License Boundary Enforcement — Hard Gates

- Ref: `§ Governance > License Boundary Enforcement: Hard Gates`
- **This must be in place before the repo split.** Without enforcement, the
  boundary is a suggestion that will be broken by the first AI agent or tired
  dev who adds a convenient import.

**Layer 1: `scripts/check_license_boundary.sh`**

- [ ] Create `scripts/check_license_boundary.sh`:
  - Hardcode the license map (MIT crates, BSL-1.1 crates) at the top
  - For each MIT crate, parse its `Cargo.toml` `[dependencies]` section
  - Check if any dependency name matches a BSL-1.1 crate
  - If violation found: print a clear error with the offending crate pair, a
    reference to the evolution.md rule, and a fix suggestion
  - Exit 0 on pass, exit 1 on violation
- [ ] Make the script executable (`chmod +x`)
- [ ] Verify the script catches a simulated violation (temporarily add
      `toad-git` as a dep of `scaffold`, run the script, confirm it fails, then
      revert)

**Layer 2: Git pre-commit hook** <!-- markdownlint-disable-line MD036 -->

- [ ] Add `scripts/git-hooks/pre-commit` that runs
      `scripts/check_license_boundary.sh`
- [ ] Update `scripts/install_toad.sh` or add a setup note to configure
      `git config core.hooksPath scripts/git-hooks` so the hook is active for
      all contributors

**Layer 3: `deny.toml` + `cargo-deny` (CI only)**

- [ ] Create `deny.toml` at the workspace root with:
  - `[licenses]` section: allow MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause,
    ISC; deny unlicensed; add `[[licenses.exceptions]]` for each BSL-1.1 crate
  - `[bans]` section: document that internal boundary enforcement is handled by
    the shell script, not `cargo-deny`
- [ ] Verify `cargo deny check licenses` passes on the current workspace
- [ ] Document in CI config (or a CI setup note) that the pipeline must run:
  1. `scripts/check_license_boundary.sh` (fast, first)
  2. `cargo deny check licenses` (comprehensive, second)

### P0-6: Add License Notices to Source Files (Optional)

- Ref: `§ License Header Template`
- [ ] Decide whether to add SPDX headers to each `.rs` file in BSL-1.1 crates
      (e.g., `// SPDX-License-Identifier: LicenseRef-BSL-1.1`)
- [ ] If yes, add headers to all `.rs` files in BSL-1.1 crates
- [ ] Add `// SPDX-License-Identifier: MIT` to MIT crate source files

---

## Phase 1: GitHub Repository Creation

> Ref: `§ GitHub Repository Map`
>
> _Manual step — create the repos on GitHub. These start empty; history is
> pushed in Phase 2._

### P1-1: Create MIT Repos (Public)

- [ ] Create `Primatif/toad-core` — Public, no README (will be pushed)
- [ ] Create `Primatif/toad-scaffold` — Public, no README

### P1-2: Create BSL-1.1 Repos (Public)

- [ ] Create `Primatif/toad-discovery` — Public, no README
- [ ] Create `Primatif/toad-git` — Public, no README
- [ ] Create `Primatif/toad-manifest` — Public, no README
- [ ] Create `Primatif/toad-ops` — Public, no README

### P1-3: Configure Repo Settings

- [ ] Set default branch to `main` for all new repos
- [ ] Add repo descriptions:
  - `toad-core`: "Core data models and workspace management for Toad"
  - `toad-scaffold`: "Project scaffolding for Toad"
  - `toad-discovery`: "Ecosystem scanning and intelligence engine for Toad"
  - `toad-git`: "Git status analysis for Toad"
  - `toad-manifest`: "Context generation and manifest engine for Toad"
  - `toad-ops`: "Batch operations and safety engine for Toad"
- [ ] Add topic tags: `toad`, `primatif`, `rust`, and license-specific (`mit` or
      `bsl-1.1`)

---

## Phase 2: History Extraction & Submodule Conversion

> Ref: `§ History Preservation`, `§ Multi-Repo Architecture`
>
> _This is the critical phase. Work on a branch. The main repo's `crates/`
> directories are replaced with submodule references._

### P2-1: Install Prerequisites

- [ ] Install `git-filter-repo` (`brew install git-filter-repo`)
- [ ] Verify: `git filter-repo --version`

### P2-2: Extract Crate Histories

For each crate, repeat this process:

- [ ] **toad-core:**
  - `git clone Primatif_Toad toad-core-extract`
  - `cd toad-core-extract`
  - `git filter-repo --subdirectory-filter crates/toad-core`
  - `git remote add origin git@github.com:Primatif/toad-core.git`
  - `git push -u origin main`
- [ ] **scaffold:**
  - Same process with `--subdirectory-filter crates/scaffold`
  - Push to `Primatif/toad-scaffold`
- [ ] **discovery:**
  - Same process with `--subdirectory-filter crates/discovery`
  - Push to `Primatif/toad-discovery`
- [ ] **toad-git:**
  - Same process with `--subdirectory-filter crates/toad-git`
  - Push to `Primatif/toad-git`
- [ ] **toad-manifest:**
  - Same process with `--subdirectory-filter crates/toad-manifest`
  - Push to `Primatif/toad-manifest`
- [ ] **toad-ops:**
  - Same process with `--subdirectory-filter crates/toad-ops`
  - Push to `Primatif/toad-ops`

### P2-3: Verify Extracted Repos

- [ ] For each extracted repo, verify:
  - `git log` shows the correct filtered history
  - `Cargo.toml` is present at the root
  - `LICENSE` file is present (from Phase 0)
  - `src/` directory is present

### P2-4: Convert Main Repo to Submodules

- [ ] Create a branch: `git checkout -b feat/multi-repo-split`
- [ ] For each crate, remove the directory and add as submodule:

  ```bash
  git rm -rf crates/toad-core
  git submodule add git@github.com:Primatif/toad-core.git crates/toad-core

  git rm -rf crates/scaffold
  git submodule add git@github.com:Primatif/toad-scaffold.git crates/scaffold

  git rm -rf crates/discovery
  git submodule add git@github.com:Primatif/toad-discovery.git crates/discovery

  git rm -rf crates/toad-git
  git submodule add git@github.com:Primatif/toad-git.git crates/toad-git

  git rm -rf crates/toad-manifest
  git submodule add git@github.com:Primatif/toad-manifest.git crates/toad-manifest

  git rm -rf crates/toad-ops
  git submodule add git@github.com:Primatif/toad-ops.git crates/toad-ops
  ```

- [ ] Verify `.gitmodules` file is generated correctly
- [ ] **Version pinning:** Verify that each submodule entry in the commit
      explicitly references the SHA from the newly pushed `main` branch of each
      crate repo. Run `git submodule status` and confirm every submodule points
      to a valid, reachable commit on its remote `main`. This ensures
      `git clone --recurse-submodules` gets a known-good state.
- [ ] Commit: `feat(licensing): convert crates to git submodules`

### P2-5: Verify Workspace Integrity

- [ ] `cargo build` succeeds from workspace root
- [ ] `cargo test` passes across all crates
- [ ] `just` quality gates pass (if applicable)
- [ ] All `path = "../toad-core"` references still resolve correctly

### P2-6: Merge & Tag

- [ ] Merge `feat/multi-repo-split` into `main`
- [ ] Tag: `git tag v1.0.2`
- [ ] Push with submodules: `git push --recurse-submodules=on-demand`

---

## Phase 3: Documentation & CI Updates

> Ref: `§ Backward Compatibility`

### P3-1: Update README

- [ ] Add "Architecture" section explaining the multi-repo structure
- [ ] Add clone instructions: `git clone --recurse-submodules`
- [ ] Add license badges for both MIT and BSL-1.1
- [ ] Document the license split (which crates are MIT vs BSL-1.1)

### P3-2: Update Contributing Guide

- [ ] Document the submodule workflow for contributors:
  - How to clone with submodules
  - How to commit changes that span multiple crates
  - How to update submodule references in the main repo
- [ ] Note that PRs to BSL-1.1 crates require a CLA or similar agreement

### P3-3: Update CI/CD

- [ ] Add `git submodule update --init --recursive` to CI pipeline
- [ ] Ensure CI can access all public submodule repos
- [ ] Add per-crate CI workflows to each extracted repo (cargo test, clippy)

### P3-4: Update Justfile

- [ ] Add `just submodule-update` recipe for pulling latest submodule refs
- [ ] Add `just submodule-status` recipe for checking submodule state
- [ ] Verify existing recipes still work with submodule layout

---

## Phase 4: Submodule-Aware Ecosystem Management

> Ref: `§ Submodule-Aware Ecosystem Management`
>
> _Build generic submodule discovery and status as a first-class Toad feature.
> This is not a self-management special case — any Toad user with submodules in
> their projects benefits. Toad's own crate submodules serve as the dogfooding
> validation._

### P4-1: Data Model — `SubmoduleDetail` (`toad-core`, MIT)

- Ref: `§ Data Model Changes`
- [ ] Add `SubmoduleDetail` struct to `toad-core/src/lib.rs`:
  - `name: String`
  - `path: PathBuf`
  - `url: String`
  - `expected_commit: Option<String>`
  - `actual_commit: Option<String>`
  - `initialized: bool`
  - `vcs_status: VcsStatus`
- [ ] Add `submodules: Vec<SubmoduleDetail>` field to `ProjectDetail`
- [ ] Decide migration path for existing `sub_projects: Vec<String>` — deprecate
      or keep alongside `submodules` for backward compatibility
- [ ] Add `Display` impl for `SubmoduleDetail` (consistent with existing
      `VcsStatus` and `ActivityTier` display patterns)
- [ ] Add unit tests for `SubmoduleDetail` serialization/deserialization

### P4-2: Submodule Discovery (`discovery`, BSL-1.1)

- Ref: `§ Discovery Logic`
- [ ] Add `.gitmodules` parsing logic to the discovery crate:
  - Parse the INI-style `.gitmodules` file to extract submodule name, path, and
    URL
  - Handle edge cases: missing `.gitmodules`, empty file, malformed entries
- [ ] For each parsed submodule, check initialization state:
  - Initialized: the submodule directory exists and contains a `.git` file or
    directory
  - Uninitialized: the directory is empty or doesn't exist
- [ ] Populate `SubmoduleDetail` for each discovered submodule
- [ ] Integrate submodule discovery into the existing project scan pipeline so
      that `submodules` is populated on every `ProjectDetail`
- [ ] Add unit tests with fixture `.gitmodules` files (0 submodules, 1
      submodule, multiple submodules, malformed)

### P4-3: Submodule Git Analysis (`toad-git`, BSL-1.1)

- Ref: `§ Git Analysis`
- [ ] For initialized submodules, determine:
  - Expected commit (what the parent repo's index says)
  - Actual commit (what HEAD points to inside the submodule)
  - Whether the submodule has local changes (dirty working tree)
- [ ] Detect detached HEAD states in submodules
- [ ] Populate `expected_commit`, `actual_commit`, and `vcs_status` on
      `SubmoduleDetail`
- [ ] Add unit tests for commit alignment detection and dirty state

### P4-4: CLI Integration (`bin/toad`, MIT)

- Ref: `§ CLI Surface`
- [ ] Update `toad status` to display submodule status as indented children of
      their parent project:

  ```text
  myproject          ✅ Clean    🔥 Active    rust
    ├─ lib-core      ✅ Clean    (submodule)
    ├─ lib-utils     ⚠️ Dirty    (submodule, +2 commits ahead)
    └─ vendor/dep    ❓ Uninit   (submodule, not initialized)
  ```

- [ ] Update `toad reveal` to include submodule details in output
- [ ] Consider: should `toad do` recurse into submodules by default, or require
      an explicit flag (e.g., `--recurse-submodules`)?

### P4-5: Dogfooding — Toad Manages Its Own Submodules

- Ref: `§ Dogfooding: Toad Manages Itself`
- [ ] After Phase 2 (submodule conversion), verify Toad discovers its own crate
      submodules automatically
- [ ] `toad status` shows all crate submodules with correct init/VCS state
- [ ] Apply license taxonomy tags:
  - `toad tag toad-core mit`
  - `toad tag scaffold mit`
  - `toad tag discovery bsl`
  - `toad tag toad-git bsl`
  - `toad tag toad-manifest bsl`
  - `toad tag toad-ops bsl`
- [ ] `toad do -q toad "cargo test"` runs tests across all crate submodules
- [ ] `toad stats` shows disk usage across all crate submodules

---

## Phase 4b: Project Contexts (`toad project`)

> Ref: `§ Project Contexts (toad project)`
>
> _Named project contexts let users register multiple workspace roots and switch
> between them. This is foundational — all commands resolve their targets based
> on the active context. Must be in place before Phase 5 (`toad ggit`) since git
> orchestration needs to know which workspace it's operating in._

### P4b-1: Data Models — Project Context (`toad-core`, MIT)

- Ref: `§ Project Contexts > Data Models`
- [ ] Add `ProjectContext` struct to `toad-core`:
  - `path: PathBuf`
  - `description: Option<String>`
  - `registered_at: String`
- [ ] Extend `GlobalConfig` with new fields:
  - `active_context: Option<String>`
  - `project_contexts: HashMap<String, ProjectContext>`
- [ ] Retain `home_pointer` for backward compatibility
- [ ] Update `GlobalConfig::load()` to handle both old format (just
      `home_pointer`) and new format (with `project_contexts`)
- [ ] On first load of old-format config, auto-migrate: register `home_pointer`
      as a context named `default`, set as active
- [ ] Add unit tests for migration from old to new config format
- [ ] Add unit tests for serialization/deserialization of extended config

### P4b-2: Storage Reorganization — Per-Context Artifacts (`toad-core`, MIT)

- Ref: `§ Project Contexts > Storage Reorganization`
- [ ] Add `context_dir(name: &str) -> Result<PathBuf>` method to `GlobalConfig`
      that returns `~/.toad/contexts/<name>/`
- [ ] Add `context_shadows_dir(name: &str) -> Result<PathBuf>` method that
      returns `~/.toad/contexts/<name>/shadows/`
- [ ] Update `ProjectRegistry::registry_path()` to accept a context name
      parameter and return `~/.toad/contexts/<name>/registry.json` instead of
      `~/.toad/registry.json`
- [ ] Update `ProjectRegistry::load()` and `save()` to accept the active context
      name (resolved from `GlobalConfig`) to determine the correct registry path
- [ ] Update all call sites of `ProjectRegistry::load()` and `save()` in
      `bin/toad` and other crates to pass the active context name
- [ ] Update `Workspace::with_root()` — `shadows_dir` must resolve to
      `~/.toad/contexts/<name>/shadows/` instead of `<root>/shadows/`. The
      `Workspace` struct needs the active context name (or the resolved context
      directory) during construction.
- [ ] `Workspace::manifest_path()` and `Workspace::tags_path()` derive from
      `shadows_dir` — no changes needed once `shadows_dir` points to the correct
      per-context location
- [ ] `Workspace::ensure_shadows()` still works — just creates the directory
      under `~/.toad/contexts/<name>/shadows/` instead of `<root>/shadows/`
- [ ] On `toad project register`, create `~/.toad/contexts/<name>/shadows/`
      directory
- [ ] On `toad project delete`, remove `~/.toad/contexts/<name>/` directory
      (after the existing confirmation prompt) — this cleanly removes registry,
      shadows, and any future per-context state
- [ ] **Backward compatibility migration:** On first run, if
      `~/.toad/registry.json` exists at the old location and `~/.toad/contexts/`
      does not exist:
  1. Create `~/.toad/contexts/default/shadows/`
  2. Move `~/.toad/registry.json` → `~/.toad/contexts/default/registry.json`
  3. If `<home_pointer>/shadows/` exists, move its contents (`MANIFEST.md`,
     `tags.json`) → `~/.toad/contexts/default/shadows/`
  4. Remove the now-empty `<home_pointer>/shadows/` directory
  5. This runs alongside the `home_pointer` → `default` context migration in
     P4b-1
- [ ] After migration, the old `~/.toad/registry.json` and
      `<workspace_root>/shadows/` must no longer exist
- [ ] Add unit tests for context directory creation (including shadows subdir)
- [ ] Add unit tests for context directory cleanup on delete
- [ ] Add unit tests for registry.json migration from old to new location
- [ ] Add unit tests for shadows migration from workspace root to context dir
- [ ] Add unit tests verifying `ProjectRegistry::load()` reads from the correct
      context-scoped path
- [ ] Add unit tests verifying `Workspace::manifest_path()` and
      `Workspace::tags_path()` resolve under `~/.toad/contexts/<name>/shadows/`

### P4b-3: Workspace Resolution Update (`toad-core`, MIT)

- Ref: `§ Project Contexts > How It Affects Other Commands`
- [ ] Update `Workspace::discover()` to resolve via active context:
  1. `TOAD_ROOT` env var (unchanged, highest priority)
  2. Upward search for `.toad-root` (unchanged)
  3. `active_context` → look up path in `project_contexts`
  4. Fall back to `home_pointer` if no context set
- [ ] All existing commands that call `Workspace::discover()` automatically get
      context-aware resolution — no changes needed in command handlers
- [ ] Add unit tests verifying context-based resolution, fallback to
      `home_pointer`, and env var override

### P4b-4: Installation Flow Update — `toad home` (`bin/toad`, MIT)

- Ref: `§ Project Contexts > Storage Reorganization > Installation flow update`
- [ ] Update `toad home <path>` to work with project contexts:
  - If no contexts exist yet: register path as `default` context, switch to it
  - If contexts already exist: register a new context using the directory's
    basename as the name, switch to it
  - Still creates `.toad-root` marker if missing (with confirmation)
- [ ] Update `toad home` (no args) to show the current workspace root resolved
      from the active context (not raw `home_pointer`)
- [ ] Keep `home_pointer` in sync with the active context's path for backward
      compatibility with any external tools reading `config.json`
- [ ] Add unit tests for `toad home` with and without existing contexts

### P4b-5: CLI — `toad project` Subcommand (`bin/toad`, MIT)

- Ref: `§ Project Contexts > Command Surface`
- [ ] Add `Project` variant to the CLI `Commands` enum in `main.rs`
- [ ] Add `ProjectCommand` sub-enum with variants:
  - `Register { name: String, path: String, description: Option<String> }`
  - `Switch { name: String }`
  - `Current`
  - `List`
  - `Update { name: String, path: Option<String>, description: Option<String> }`
  - `Delete { name: String }`
  - `Info { name: String }`
- [ ] `toad project register` validates:
  - Path is absolute and exists
  - Name doesn't collide with an existing context
- [ ] `toad project switch` updates `active_context` in config and saves
- [ ] `toad project current` displays active context name, path, description
- [ ] `toad project list` renders a formatted table with active indicator
- [ ] `toad project delete` prompts for `[y/N]` confirmation; rejects if the
      context is currently active (must switch first)
- [ ] Format output with existing Toad visual style (colored, structured)
- [ ] Add `--help` documentation for all `toad project` subcommands

### P4b-6: Developer Setup Script (Project-Specific)

- Ref: `§ Project Contexts > Developer Setup: Toad Working on Itself`
- [ ] Create `scripts/dev_setup.sh`:
  1. Clone main repo with `--recurse-submodules`
  2. Run `cargo build` and `cargo test` to verify
  3. If `toad` is installed, register `toad-dev` context and switch to it
- [ ] Make the script executable
- [ ] Add usage instructions as a comment header in the script
- [ ] Document in `README.md` or `CONTRIBUTING.md` that new contributors should
      run `scripts/dev_setup.sh`

### P4b-7: History Cleanup Script (Project-Specific, One-Time)

- Ref: `§ Project Contexts > History Cleanup: Post-Split`
- [ ] Create `scripts/history_cleanup.sh`:
  - Use `git filter-repo` to remove crate `src/` directories from the main
    repo's history after submodule conversion
  - Preserve submodule reference entries (`.gitmodules`, submodule pointers)
  - Include a dry-run mode that shows what would be removed
  - Include a confirmation prompt before executing (irreversible operation)
- [ ] Document that this runs once during v1.0.2 migration and never again
- [ ] Test on a throwaway clone before running on the real repo

### P4b-8: Integration Testing

- [ ] Test register → switch → current → list → delete lifecycle
- [ ] Test backward compatibility: old config with only `home_pointer`
      auto-migrates to `default` context on first load
- [ ] Test backward compatibility: old `~/.toad/registry.json` migrates to
      `~/.toad/contexts/default/registry.json` on first load
- [ ] Test backward compatibility: old `<workspace_root>/shadows/` contents
      migrate to `~/.toad/contexts/default/shadows/` on first load
- [ ] Test `toad status` resolves against the active context's path
- [ ] Test `toad project switch` changes which projects `toad status` shows
- [ ] Test `toad project switch` causes `ProjectRegistry` to load from the new
      context's `~/.toad/contexts/<name>/registry.json`
- [ ] Test `TOAD_ROOT` env var still overrides the active context
- [ ] Test `toad project delete` rejects deleting the active context
- [ ] Test `toad project delete` removes `~/.toad/contexts/<name>/` directory
      (including shadows subdirectory)
- [ ] Test `toad home <path>` registers and switches context correctly
- [ ] Test `Workspace::manifest_path()` resolves to
      `~/.toad/contexts/<name>/shadows/MANIFEST.md` (not workspace root)
- [ ] Test `Workspace::tags_path()` resolves to
      `~/.toad/contexts/<name>/shadows/tags.json` (not workspace root)
- [ ] Dogfood: register `toad-dev` and `my-code` contexts, switch between them,
      verify all commands target the correct workspace and load the correct
      per-context registry

---

## Phase 5: Multi-Repo Git Orchestration (`toad ggit`)

> Ref: `§ Multi-Repo Git Orchestration`
>
> _Build first-class git operations for multi-repo workflows. These are
> structured, project-scoped commands — not `toad do` batch jobs. The
> orchestration logic lives in `toad-git` (BSL-1.1) as protected commercial
> value. Any Toad user with submodules or multi-repo setups benefits._

### P5-1: Data Models — Git Operation Types (`toad-core`, MIT)

- Ref: `§ Data Models (toad-core, MIT)`
- [ ] Add `RepoStatus` struct to `toad-core`:
  - `name: String`
  - `path: PathBuf`
  - `branch: Option<String>`
  - `is_detached: bool`
  - `dirty_files: u32`
  - `staged_files: u32`
  - `untracked_files: u32`
  - `ahead: u32`
  - `behind: u32`
- [ ] Add `BranchInfo` struct:
  - `name: String`
  - `is_current: bool`
  - `upstream: Option<String>`
  - `last_commit: Option<String>`
- [ ] Add `CommitInfo` struct:
  - `hash: String`
  - `short_hash: String`
  - `message: String`
  - `author: String`
  - `timestamp: String`
- [ ] Add `GitOpResult` struct to `toad-core`:
  - `repo: String`
  - `operation: String`
  - `success: bool`
  - `summary: String` — one-line human-readable result
  - `raw_stdout: String` — captured git stdout
  - `raw_stderr: String` — captured git stderr
- [ ] Add unit tests for serialization/deserialization of all new types

### P5-2: Expand `toad-git` — Status & Branch (`toad-git`, BSL-1.1)

- Ref: `§ Orchestration Logic (toad-git, BSL-1.1)`
- [ ] Refactor existing `status.rs` to return `RepoStatus` instead of the simple
      `GitStatus` enum (backward-compatible: keep enum, add richer fn)
- [ ] Add `branch.rs` module:
  - `create_branch(path: &Path, name: &str) -> Result<()>`
  - `checkout_branch(path: &Path, name: &str) -> Result<()>`
  - `list_branches(path: &Path) -> Result<Vec<BranchInfo>>`
  - `delete_branch(path: &Path, name: &str) -> Result<()>`
  - `current_branch(path: &Path) -> Result<Option<String>>`
- [ ] Add unit tests for branch operations (create, checkout, list, delete)

### P5-3: Expand `toad-git` — Commit & Diff (`toad-git`, BSL-1.1)

- Ref: `§ Orchestration Logic (toad-git, BSL-1.1)`
- [ ] Add `commit.rs` module:
  - `stage_all(path: &Path) -> Result<()>`
  - `stage_files(path: &Path, files: &[&str]) -> Result<()>`
  - `commit(path: &Path, message: &str) -> Result<CommitInfo>`
  - `diff_summary(path: &Path) -> Result<String>`
- [ ] Add `log.rs` module:
  - `recent_commits(path: &Path, count: usize) -> Result<Vec<CommitInfo>>`
- [ ] Add unit tests for commit and log operations

### P5-4: Expand `toad-git` — Remote, Sync & Pre-flight (`toad-git`, BSL-1.1)

- Ref: `§ Orchestration Logic (toad-git, BSL-1.1)` and `§ Pre-flight Check`
- [ ] Add `remote.rs` module — all functions capture git output and return
      `GitOpResult` instead of raw `()`:
  - `push(path: &Path) -> Result<GitOpResult>`
  - `pull(path: &Path) -> Result<GitOpResult>`
  - `fetch(path: &Path) -> Result<GitOpResult>`
  - `ahead_behind(path: &Path) -> Result<(u32, u32)>`
  - `sha_exists_on_remote(path: &Path, sha: &str) -> Result<bool>` Check if a
    commit SHA exists on any remote branch (`git branch -r --contains <sha>`)
- [ ] Add `sync.rs` module with mandatory pre-flight:
  - `preflight_check(repos: &[&Path]) -> Result<Vec<PreflightResult>>` For each
    submodule: verify HEAD exists on remote, no dirty state, not in detached
    HEAD. Returns structured pass/fail per repo.
  - `sync_submodule_refs(parent_path: &Path) -> Result<Vec<GitOpResult>>` Runs
    `preflight_check` first — blocks if any repo fails. Updates parent's index
    only after all checks pass.
  - `sync_submodule_refs_force(parent_path: &Path) -> Result<Vec<GitOpResult>>`
    Bypasses pre-flight. Caller (`bin/toad`) must handle interactive
    confirmation before invoking this.
- [ ] Add `PreflightResult` struct to `toad-core` (MIT):
  - `repo: String`
  - `ready: bool`
  - `head_sha: String`
  - `pushed_to_remote: bool`
  - `is_clean: bool`
  - `is_detached: bool`
  - `message: String` — human-readable explanation if not ready
- [ ] Add unit tests for remote, sync, and pre-flight operations

### P5-4b: Output Consolidation Layer (`toad-git`, BSL-1.1)

- Ref: `§ Output Consolidation`
- [ ] All `toad-git` functions that execute git commands must capture
      stdout/stderr via `Command::new("git").output()` (not `.status()`) and
      return `GitOpResult` with both raw output and a summary
- [ ] Add helper function:
  - `run_git(path: &Path, args: &[&str]) -> Result<GitOpResult>` Central git
    command runner that captures output, builds summary, and populates
    `GitOpResult`. All other modules use this instead of calling
    `Command::new("git")` directly.
- [ ] Ensure error results include: which repo, what operation, the raw stderr,
      and an actionable hint (e.g., "run toad ggit pull -p X first")
- [ ] Add unit tests verifying output capture and summary generation

### P5-5: Branch Orchestration Data Models (`toad-core`, MIT)

- Ref: `§ Branch Orchestration & Lifecycle Tracking > Data Models`
- [ ] Add `BranchGroup` struct to `toad-core`:
  - `name: String` — the shared branch name (branch group identifier)
  - `repos: Vec<BranchGroupEntry>`
- [ ] Add `BranchGroupEntry` struct:
  - `repo: String`
  - `exists: bool`
  - `ahead: u32`, `behind: u32` — relative to target branch (e.g., development)
  - `merged: bool`
  - `pr_status: Option<PrStatus>`
  - `head_commit: Option<CommitInfo>`
- [ ] Add `PrStatus` enum:
  - `None` — no PR exists
  - `Open { number: u32, url: String }`
  - `Merged { number: u32, url: String }`
  - `Closed { number: u32, url: String }`
- [ ] Add unit tests for serialization/deserialization of branch tracking types

### P5-6: Expand `toad-git` — Branch Orchestration (`toad-git`, BSL-1.1)

- Ref: `§ Branch Orchestration & Lifecycle Tracking > Orchestration Logic`
- [ ] Add `branches.rs` module:
  - `list_branches_across(repos: &[&Path]) -> Result<Vec<BranchGroup>>` Collects
    all branch names across repos and groups them into a matrix.
  - `branch_group_status(repos: &[&Path], branch: &str) -> Result<BranchGroup>`
    Detailed status for a specific branch group (ahead/behind, merged, etc.)
- [ ] Add `merge_status.rs` module:
  - `check_merged(path: &Path, branch: &str, target: &str) -> Result<bool>`
    Check if `branch` has been merged into `target` in the given repo.
  - `merge_status_across(repos: &[&Path], branch: &str, target: &str) -> Result<Vec<(String, bool)>>`
    Check merge status across all repos.
- [ ] Add `align.rs` module:
  - `check_alignment(repos: &[&Path], branch: &str) -> Result<Vec<AlignmentReport>>`
    For each repo on the given branch: is it pushed, up-to-date, clean?
- [ ] Add commit linking support to `commit.rs`:
  - `commit_with_link(path: &Path, message: &str, group: &str, related: &[(String, String)]) -> Result<CommitInfo>`
    Appends `Part of:` and `Related:` trailers to the commit body.
- [ ] Add unit tests for branch orchestration, merge detection, and alignment

### P5-7: Expand `toad-git` — PR Status (Optional, `toad-git`, BSL-1.1)

- Ref: `§ Branch Orchestration & Lifecycle Tracking > Orchestration Logic`
- [ ] Add `pr.rs` module:
  - `get_pr_status(repo_url: &str, branch: &str, token: &str) -> Result<PrStatus>`
    Query GitHub API for PR status of a branch. Requires personal access token.
  - `get_pr_status_across(repos: &[(String, String)], branch: &str, token: &str) -> Result<Vec<(String, PrStatus)>>`
    PR status across all repos.
- [ ] Handle missing token gracefully — `PrStatus::None` with a note that GitHub
      API access is not configured
- [ ] Add unit tests with mocked API responses
- **Note:** This task is optional for v1.0.2. The core branch tracking (P5-6)
  works purely with local git data. PR status is a nice-to-have that can ship in
  a follow-up.

### P5-8: CLI — `toad ggit` Subcommand (`bin/toad`, MIT)

- Ref: `§ Command Surface` and `§ Additional Commands`
- [ ] Add `Ggit` variant to the CLI `Commands` enum in `main.rs`
- [ ] Add `GgitCommand` sub-enum with variants:
  - `Status { project: Option<String> }`
  - `Branch { name: String, project: Option<Vec<String>>, delete: bool }`
  - `Checkout { branch: String, project: Option<String> }`
  - `Add { all: bool, project: Option<String> }`
  - `Commit { message: String, project: Option<String>, link: bool }`
  - `Push { project: Option<String> }`
  - `Pull { project: Option<String> }`
  - `Sync { force: bool }`
  - `Preflight`
  - `Log { project: Option<String>, count: Option<usize> }`
  - `Diff { project: Option<String> }`
  - `Branches { group: Option<String> }`
  - `MergeStatus { branch: String }`
  - `Align { branch: String }`
  - `Pr { project: Option<String> }`
- [ ] Add global `--verbose` / `-v` flag to `Ggit` — when set, streams full raw
      git output per repo instead of consolidated summaries
- [ ] Implement project resolution: `--project <name>` resolves to the submodule
      path via the discovered `SubmoduleDetail` list
- [ ] Support multiple `-p` flags for commands that target multiple repos (e.g.,
      `toad ggit branch feat/x -p toad-core -p discovery`)
- [ ] When `--project` is omitted, iterate over all discovered submodules
- [ ] **Output consolidation:** Default output renders consolidated
      `GitOpResult` summaries (one line per repo). `-v` renders full raw git
      output with per-repo headers (`── repo-name ──`).
- [ ] **Pre-flight in sync:** `toad ggit sync` runs preflight check and blocks
      with structured report if any repo fails. Displays actionable fix commands
      per repo.
- [ ] **Force with confirmation:** `toad ggit sync --force` prompts for
      interactive `[y/N]` confirmation before bypassing preflight. AI agents
      must never auto-confirm this.
- [ ] `toad ggit preflight` renders structured pass/fail per repo
- [ ] `toad ggit branches` renders the cross-repo branch matrix with aligned
      columns and status indicators
- [ ] `toad ggit merge-status` renders per-repo merge state with PR info
- [ ] `toad ggit align` renders alignment report with clear pass/fail
- [ ] `toad ggit commit --link` appends group reference trailers
- [ ] Format output with existing Toad visual style (colored, structured)
- [ ] Add `--help` documentation for all `toad ggit` subcommands

### P5-9: Integration Testing & Dogfooding

- Ref: `§ Workflow Examples`
- [ ] Test cross-repo branch creation: `toad ggit branch feat/test-branch`
      creates branch in all submodules
- [ ] Test single-repo focus: `toad ggit status -p discovery` shows only
      discovery's status
- [ ] Test commit workflow: `toad ggit add --all -p toad-core` →
      `toad ggit commit -m "test" -p toad-core`
- [ ] Test sync: `toad ggit sync` updates parent's submodule references
- [ ] Test AI ergonomics: verify structured output is parseable (not just
      human-readable)
- [ ] Dogfood with Toad's own crate submodules through a real feature branch
      workflow
- [ ] Test branch orchestration: `toad ggit branches` shows cross-repo branch
      matrix
- [ ] Test branch group detail: `toad ggit branches --group feat/test-branch`
      shows per-repo status
- [ ] Test merge status: `toad ggit merge-status feat/test-branch` reports
      merged/not-merged per repo
- [ ] Test alignment check: `toad ggit align feat/test-branch` reports
      push/clean state per repo
- [ ] Test commit linking: `toad ggit commit -m "test" -p discovery --link`
      appends group trailers
- [ ] Test branch cleanup: `toad ggit branch -d feat/test-branch` deletes from
      all repos
- [ ] Full lifecycle dogfood: create branch → work → commit → push → verify
      alignment → merge → verify merge-status → cleanup → sync

---

## Phase 5b: Publishing Waterfall Script (Project-Specific)

> Ref: `§ Publishing Waterfall: Crates.io Release Automation`
>
> _This is NOT a `toad ggit` feature. It is project-specific automation for
> publishing Toad's crates to crates.io in the correct dependency order. Lives
> in `scripts/`, invoked manually as part of the release workflow._

### P5b-1: Dual Dependency Setup

- Ref: `§ Dual Dependency Strategy`
- [ ] Update all internal `Cargo.toml` dependency declarations to use the dual
      `version` + `path` pattern, e.g.,
      `toad-core = { version = "1.0.2", path = "../toad-core" }`
- [ ] Verify `cargo build` still works with path resolution locally
- [ ] Verify `cargo publish --dry-run` resolves the version field correctly for
      each crate

### P5b-2: Create `scripts/publish_waterfall.sh`

- Ref: `§ Solution: scripts/publish_waterfall.sh`
- [ ] Create `scripts/publish_waterfall.sh` with the following steps:
  1. Accept a version argument (e.g., `./publish_waterfall.sh 1.0.2`)
  2. Bump `version = "..."` in each crate's `Cargo.toml` (topological order)
  3. Bump dependency version references in downstream `Cargo.toml` files
  4. Run `cargo publish --dry-run` for each crate in order — abort on failure
  5. Run `cargo publish` for each crate in topological order:
     - Layer 0: `toad-core`
     - Layer 1: `toad-git`, `toad-manifest`, `scaffold`
     - Layer 2: `discovery`, `toad-ops`
     - Layer 3: `bin/toad`
  6. Between layers, poll `cargo search <crate>` until the new version appears
     in the index
  7. After all publishes succeed, commit the `Cargo.toml` version bumps
  8. Tag the release (`git tag v<version>`) in each submodule and the parent
     repo
- [ ] Add `--dry-run` flag that runs steps 1–4 without publishing
- [ ] Add error handling: if any publish fails mid-waterfall, report which
      crates succeeded and which remain, so the user can resume manually
- [ ] Make the script executable and add a usage header comment

### P5b-3: Integrate with Existing Release Workflow

- [ ] Update `conductor/workflow.md` SemVer Bump section to reference
      `scripts/publish_waterfall.sh` as the publish step
- [ ] Ensure `scripts/sync_version.sh` is called as part of the waterfall
      (README badge sync after version bump)
- [ ] Document the full release sequence in a comment at the top of the script:
      bump → dry-run → publish → commit → tag → sync README

---

## Phase 6: Context & Governance Updates

> Ref: `§ Governance: Licensing-Aware Development > AI Agent Context Updates`
>
> _Update all AI agent context files and conductor documents to encode the
> licensing boundaries. After this phase, any AI agent working on Toad will
> automatically respect the license split when placing new code._

### P6-1: Update `.gemini/GEMINI.md`

- Ref: `§ Governance > AI Agent Context Updates`
- [ ] Add a `## Licensing Architecture` section after the System Structure
      section
- [ ] Document the two license tiers: MIT (open contracts) and BSL-1.1
      (protected intelligence)
- [ ] List which crates are MIT and which are BSL-1.1
- [ ] Include the decision framework: data models → MIT, intelligence → BSL-1.1,
      CLI glue → MIT
- [ ] Include the dependency direction rule: MIT crates must never depend on
      BSL-1.1 crates (except the binary)
- [ ] Add a note that new crates default to BSL-1.1 unless they are pure
      contracts/types

### P6-2: Update `conductor/tech-stack.md`

- Ref: `§ Governance > AI Agent Context Updates`
- [ ] Update the Component Hierarchy to include license annotations:
  - `bin/toad` → `(MIT)`
  - `crates/toad-core` → `(MIT)`
  - `crates/scaffold` → `(MIT)`
  - `crates/discovery` → `(BSL-1.1)`
  - `crates/toad-git` → `(BSL-1.1)`
  - `crates/toad-manifest` → `(BSL-1.1)`
  - `crates/toad-ops` → `(BSL-1.1)`
- [ ] Add a new Design Principle: **Licensing-Aware Architecture** — new
      capabilities must be placed in the correct license tier based on the
      decision framework in `docs/releases/v1.0.2/evolution.md`
- [ ] Add a new Design Principle: **Dependency Direction** — MIT crates must
      never depend on BSL-1.1 crates; BSL-1.1 crates consume MIT contracts
- [ ] Note that each crate is now a separate git repo (submodule) with its own
      LICENSE file

### P6-3: Update `conductor/product.md`

- Ref: `§ Governance > AI Agent Context Updates`
- [ ] Add a new Core Goal: **Open Core Licensing** — maintain Toad as a
      legitimate open-source project (MIT CLI + core types) while protecting the
      intelligence layer under BSL-1.1
- [ ] Update the Vision paragraph to mention the open-core model

### P6-4: Update `conductor/product-guidelines.md`

- Ref: `§ Governance > AI Agent Context Updates`
- [ ] Add a new section: **5. Licensing & Architecture Boundaries**
- [ ] Document the dependency direction rule (with the ASCII diagram from
      evolution.md or a simplified version)
- [ ] Document the new crate decision framework
- [ ] Note that all new crate creation must include a LICENSE file and `license`
      field in `Cargo.toml` before any code is written

### P6-5: Update `.gemini/settings.json`

- [ ] Add `conductor/` to the context folders if not already present
- [ ] Ensure `docs/releases/` is accessible to Gemini for release planning
      context
- [ ] **Critical:** Add `CROSS_REPO_MAP.md` to the context files so Gemini reads
      it automatically on every new chat session. This is the primary mechanism
      ensuring AI agents start with a complete understanding of the inter-repo
      dependency graph, type flow, and license boundaries. Without this, every
      new session begins with the agent blind to the multi-repo architecture.

### P6-6: Cross-Repo Context Map (`toad-manifest`, BSL-1.1)

- Ref: `§ Governance > Cross-Repo Context Map: Solving AI Agent Navigation`
- [ ] Extend `toad manifest` to generate a Cross-Repo Context Map section
      containing:
  - **Dependency Graph** — parse `Cargo.toml` `[dependencies]` across all
    crates, output `crate (license) ← dependent (license)` edges
  - **Type Flow** — scan `pub struct` and `pub enum` in `toad-core`, trace `use`
    statements in other crates to determine where types are populated and
    displayed
  - **Call Chains** — document the function call path for each major `toad`
    command (status, ggit status, ggit sync, ggit branches, cw, create)
  - **Crate Responsibilities** — one-line summary per crate
  - **License Boundary** — MIT vs BSL-1.1 listing with the dependency direction
    rule
- [ ] Output the map as a section in the manifest's structured output
- [ ] Also write the map to `CROSS_REPO_MAP.md` at the workspace root so AI
      agents can read it without running `toad manifest`
- [ ] Regenerate `CROSS_REPO_MAP.md` every time `toad manifest` runs
- [ ] Add `CROSS_REPO_MAP.md` to `.gemini/GEMINI.md` as a referenced context
      file (so Gemini reads it on every session)
- [ ] Add `CROSS_REPO_MAP.md` to `.windsurf/` context if applicable
- [ ] Add unit tests verifying the dependency graph extraction and type flow
      scanning produce correct output for the current workspace

### P6-7: Validate AI Agent Navigation

- [ ] With the cross-repo map in place, verify that an AI agent can trace a bug
      from CLI to core type in ≤ 3 steps using only the map
- [ ] Verify the map stays accurate after adding a new type to `toad-core` and
      re-running `toad manifest`
- [ ] Verify the map includes all new v1.0.2 types (`RepoStatus`, `BranchGroup`,
      `GitOpResult`, `PreflightResult`, `CustomWorkflow`, `WorkflowRegistry`)

---

## Phase 7: Custom Workflows (`toad cw`)

> Ref: `§ Custom Workflows (toad cw)`
>
> _A generic extension mechanism that lets users register, manage, and invoke
> their own shell scripts as first-class `toad` subcommands. This keeps the core
> CLI language-agnostic while giving users a clean way to integrate
> project-specific automation (e.g., `toad cw release` for the publish waterfall
> script)._

### P7-1: Data Models — Workflow Registry (`toad-core`, MIT)

- Ref: `§ Custom Workflows > Data Models`
- [ ] Add `CustomWorkflow` struct to `toad-core`:
  - `description: String`
  - `script_path: PathBuf`
  - `registered_at: String`
  - `updated_at: String`
  - `last_run_at: Option<String>`
  - `last_exit_code: Option<i32>`
- [ ] Add `WorkflowRegistry` struct:
  - `workflows: HashMap<String, CustomWorkflow>`
  - `reserved_namespaces: Vec<String>`
- [ ] Add `load` and `save` methods for `WorkflowRegistry` targeting
      `~/.toad/custom_workflows.json`
- [ ] Add unit tests for serialization/deserialization and namespace collision
      detection

### P7-2: Reserved Namespace Authority (`toad-ops`, BSL-1.1)

- Ref: `§ Custom Workflows > Design Principles > Namespace protection`
- [ ] Add `reserved_command_names() -> Vec<&'static str>` function to `toad-ops`
      that returns all built-in `toad` command names (`status`, `do`, `ggit`,
      `create`, `home`, `stats`, `clean`, `tag`, `untag`, `cw`, `project`,
      `help`, `version`)
- [ ] This function is the **single source of truth** for namespace protection —
      `toad cw register` calls this, not the JSON cache
- [ ] On startup, sync the JSON `reserved_namespaces` cache from this function
      so external tools reading the JSON see the current list
- [ ] Add a unit test that compares `reserved_command_names()` output against
      the actual `Commands` enum variants in `bin/toad` — this test fails if a
      new command is added to the binary without updating the function

### P7-3: Workflow Execution Logic (`toad-ops`, BSL-1.1)

- Ref: `§ Custom Workflows > Execution Model`
- [ ] Add `custom_workflow.rs` module to `toad-ops`:
  - `register(registry: &mut WorkflowRegistry, name: &str, script_path: &Path, description: &str) -> Result<()>`
    Validates name (no namespace collision), script exists, is `.sh`, is
    executable. Adds to registry and saves.
  - `update(registry: &mut WorkflowRegistry, name: &str, script_path: Option<&Path>, description: Option<&str>) -> Result<()>`
    Updates an existing workflow's script path and/or description.
  - `delete(registry: &mut WorkflowRegistry, name: &str) -> Result<()>` Removes
    a workflow from the registry.
  - `run(registry: &mut WorkflowRegistry, name: &str, args: &[String]) -> Result<i32>`
    Looks up workflow, verifies script still exists, executes via
    `sh <script_path> [args...]`, streams stdout/stderr to terminal, updates
    `last_run_at` and `last_exit_code`, returns exit code.
  - `list(registry: &WorkflowRegistry) -> Vec<(&str, &CustomWorkflow)>` Returns
    all workflows sorted by name.
  - `info(registry: &WorkflowRegistry, name: &str) -> Result<&CustomWorkflow>`
    Returns detailed info for a single workflow.
- [ ] Validate script path on `register` and `run`:
  - Must be absolute path
  - Must exist
  - Must end in `.sh`
  - On `register`: warn if not executable, offer to `chmod +x`
  - On `run`: error if script has been moved/deleted since registration
- [ ] Add unit tests for register, update, delete, list, info, and validation
      edge cases (namespace collision, missing script, etc.)

### P7-4: CLI — `toad cw` Subcommand (`bin/toad`, MIT)

- Ref: `§ Custom Workflows > Command Surface`
- [ ] Add `Cw` variant to the CLI `Commands` enum in `main.rs`
- [ ] Add `CwCommand` sub-enum with variants:
  - `Run { name: String, args: Vec<String> }`
  - `Register { name: String, script_path: String, description: String }`
  - `Update { name: String, script: Option<String>, description: Option<String> }`
  - `Delete { name: String }`
  - `List`
  - `Info { name: String }`
- [ ] `toad cw <name> [args...]` dispatches to `Run` — the `name` is the first
      positional arg, remaining args are passed through
- [ ] `toad cw delete` prompts for `[y/N]` confirmation before removing
- [ ] `toad cw register` warns if script is not executable and offers to fix it
- [ ] `toad cw list` renders a formatted table with name, description, and
      script path (truncated if long)
- [ ] `toad cw info` renders full detail including timestamps and last run
      status
- [ ] Format output with existing Toad visual style (colored, structured)
- [ ] Add `--help` documentation for all `toad cw` subcommands
- [ ] Exit with the script's exit code when running a workflow

### P7-5: Integration Testing

- [ ] Test register → list → run → info → delete lifecycle
- [ ] Test namespace collision: `toad cw register status ...` should fail
- [ ] Test missing script: register with valid path, move the script,
      `toad cw run` should error with clear message
- [ ] Test argument passthrough: `toad cw release 1.0.2 --dry-run` passes
      `1.0.2` and `--dry-run` to the script as `$1` and `$2`
- [ ] Test update: change script path and description, verify `toad cw info`
      reflects the changes
- [ ] Test delete confirmation: `toad cw delete release` prompts before removing
- [ ] Dogfood: register `scripts/publish_waterfall.sh` as `toad cw release` and
      run it

---

## Cross-Cutting: Verification Checklist

> Run these checks after all phases are complete.

- [ ] `git clone --recurse-submodules <main-repo>` works from scratch
- [ ] `cargo build` succeeds in a fresh clone
- [ ] `cargo test` passes in a fresh clone
- [ ] Each crate repo has the correct LICENSE file
- [ ] Each crate's `Cargo.toml` has the correct `license` field
- [ ] `toad status` shows all crate repos as clean
- [ ] The main repo's root LICENSE is MIT
- [ ] No BSL-1.1 code exists directly in the main repo (only via submodules)
- [ ] `.gemini/GEMINI.md` contains the Licensing Architecture section
- [ ] `conductor/tech-stack.md` has license annotations on all crates
- [ ] `conductor/product-guidelines.md` has the Licensing & Architecture
      Boundaries section
- [ ] AI agents placing new code respect the license boundary (spot-check)
- [ ] `toad ggit status` returns structured output for all submodules
- [ ] `toad ggit status -p <name>` works for single-repo focus
- [ ] `toad ggit branch`, `commit`, `push` work end-to-end on a submodule
- [ ] `toad ggit sync` updates parent submodule references correctly
- [ ] All `toad ggit` subcommands have `--help` documentation
- [ ] `toad ggit branches` renders cross-repo branch matrix correctly
- [ ] `toad ggit branches --group <name>` shows per-repo detail with
      ahead/behind
- [ ] `toad ggit merge-status <branch>` reports merged/not-merged per repo
- [ ] `toad ggit align <branch>` reports alignment state per repo
- [ ] `toad ggit commit --link` appends group reference trailers to commit body
- [ ] `toad ggit branch -d <name>` deletes branch from all targeted repos
- [ ] No git operations exist outside `toad-git`:
      `grep -r 'Command::new("git")' crates/ --include="*.rs"` only hits
      `toad-git`
- [ ] `toad ggit sync` blocks when a submodule HEAD is not pushed to remote
      (ghost commit prevention)
- [ ] `toad ggit sync` blocks when a submodule has dirty/uncommitted changes
- [ ] `toad ggit sync --force` prompts for `[y/N]` confirmation before
      proceeding
- [ ] `toad ggit preflight` returns structured pass/fail per repo
- [ ] Default `toad ggit` output shows consolidated one-line-per-repo summaries
- [ ] `toad ggit -v` streams full raw git output with per-repo headers
- [ ] All `toad-git` functions use the central `run_git()` helper (no direct
      `Command::new("git")` calls outside of it)
- [ ] `toad cw register` rejects reserved namespace names
- [ ] `toad cw register` validates script exists, is `.sh`, and is executable
- [ ] `toad cw <name> [args...]` passes arguments through to the script
- [ ] `toad cw delete` prompts for confirmation
- [ ] `toad cw list` renders a formatted table of all registered workflows
- [ ] `toad cw info <name>` shows timestamps and last run status
- [ ] `~/.toad/custom_workflows.json` is created on first use with seeded
      reserved namespaces
- [ ] `toad cw release` works end-to-end with the publish waterfall script
- [ ] `scripts/check_license_boundary.sh` passes on the current workspace
- [ ] `scripts/check_license_boundary.sh` catches a simulated MIT→BSL violation
- [ ] Git pre-commit hook runs the license boundary check automatically
- [ ] `deny.toml` exists and `cargo deny check licenses` passes
- [ ] CI pipeline runs both enforcement layers (script first, cargo-deny second)
- [ ] `CROSS_REPO_MAP.md` exists at the workspace root and is up-to-date
- [ ] `toad manifest` regenerates `CROSS_REPO_MAP.md` with correct dependency
      graph, type flow, call chains, and license boundary
- [ ] `.gemini/GEMINI.md` references `CROSS_REPO_MAP.md` as a context file
- [ ] An AI agent can trace `toad status` from CLI → discovery → toad-git →
      toad-core using only the cross-repo map
- [ ] `toad project register` creates a named context with path and description
- [ ] `toad project switch` changes the active context for all commands
- [ ] `toad project current` shows the active context
- [ ] `toad project list` renders all contexts with active indicator
- [ ] `toad project delete` rejects deleting the active context
- [ ] Old config with only `home_pointer` auto-migrates to `default` context
- [ ] `toad status` resolves against the active context's path (not hardcoded)
- [ ] `TOAD_ROOT` env var still overrides the active context
- [ ] `scripts/dev_setup.sh` works from a fresh clone and registers `toad-dev`
- [ ] Switching between `toad-dev` and a user context changes which projects all
      commands see
- [ ] `~/.toad/contexts/<name>/` directory (with `shadows/` subdir) is created
      on `toad project register`
- [ ] `~/.toad/contexts/<name>/registry.json` is used (not
      `~/.toad/registry.json`)
- [ ] `~/.toad/contexts/<name>/shadows/MANIFEST.md` and `tags.json` are used
      (not `<workspace_root>/shadows/`)
- [ ] Old `~/.toad/registry.json` migrates to
      `~/.toad/contexts/default/registry.json`
- [ ] Old `<workspace_root>/shadows/` contents migrate to
      `~/.toad/contexts/default/shadows/`
- [ ] `toad project delete` removes the context's `~/.toad/contexts/<name>/` dir
      (including shadows)
- [ ] `toad home <path>` registers and switches context (backward compat)
