# v1.1.0 "Deep Croak" — PR Review Tracker

> Tracks all pull requests for the v1.1.0 release cycle (dev → main). **Merge
> order:** Submodule PRs first, then the parent Hub PR last.

---

## Legend

- `[ ]` Not reviewed
- `[~]` Review in progress
- `[x]` Approved / Merged

---

## Review Plan

Reviews are conducted in dependency order (leaves first, consumers last) so that
any issue found upstream is caught before reviewing its dependents.

### Phase 1: Foundation (no internal dependencies)

1. **toad-core** — Data models, traits, config. Everything depends on this.
2. **toad-scaffold** — Project bootstrapping. No internal consumers.

### Phase 2: Intelligence Layer (depends on toad-core)

1. **toad-discovery** — Scanning & intelligence engine.
2. **toad-git** — Git operations & VCS intelligence.
3. **toad-manifest** — Context generation & tiered prompts.
4. **toad-ops** — Batch operations, analytics, safety.

### Phase 3: Binaries (consume all library crates)

1. **toad-mcp** — MCP server. Largest diff; depends on core + discovery +
   manifest + ops.
2. **bin/toad** — CLI binary (if changes present in this cycle).

### Phase 4: Parent Hub (submodule refs + governance)

1. **Primatif_Toad** — Submodule pointers, license docs, deny.toml, scripts,
   README, conductor docs.

### Per-Module Checklist

For each module, the reviewer verifies:

- [x] Diff reviewed for correctness (logic, edge cases, error handling)
- [x] No unintended file changes or leftover debug code
- [x] LICENSE file present and matches expected license (MIT or BUSL-1.1)
- [x] `Cargo.toml` license field matches boundary
- [x] README links back to parent project
- [x] No secrets, credentials, or sensitive data exposed
- [x] Public API changes are backward-compatible or intentionally breaking
- [x] License boundary respected (MIT crates do not depend on BUSL crates)

---

## PR Index

### MIT Crates

| Status | Repo          | PR  | Link                                                   |
| :----: | :------------ | :-: | :----------------------------------------------------- |
|  [~]   | toad-core     | #2  | [PR](https://github.com/Primatif/toad-core/pull/2)     |
|  [x]   | toad-scaffold | #2  | [PR](https://github.com/Primatif/toad-scaffold/pull/2) |

### BUSL-1.1 Crates

| Status | Repo           | PR  | Link                                                    |
| :----: | :------------- | :-: | :------------------------------------------------------ |
|  [~]   | toad-discovery | #2  | [PR](https://github.com/Primatif/toad-discovery/pull/2) |
|  [~]   | toad-git       | #2  | [PR](https://github.com/Primatif/toad-git/pull/2)       |
|  [x]   | toad-manifest  | #2  | [PR](https://github.com/Primatif/toad-manifest/pull/2)  |
|  [~]   | toad-ops       | #2  | [PR](https://github.com/Primatif/toad-ops/pull/2)       |
|  [~]   | toad-mcp       | #1  | [PR](https://github.com/Primatif/toad-mcp/pull/1)       |

### Parent Hub

| Status | Repo          | PR  | Link                                                   |
| :----: | :------------ | :-: | :----------------------------------------------------- |
|  [~]   | Primatif_Toad | #4  | [PR](https://github.com/Primatif/Primatif_Toad/pull/4) |

---

## Review Results (Round 2 Consolidation)

### Phase 1: Foundation

#### toad-core (#2)

Status: Reviewed — issues found.

- [HIGH] TOAD_ROOT override ignored when global config exists; `projects_dir`
  can follow the active context instead of the explicit env override.
  (src/workspace.rs:58-67)
- [MEDIUM] Legacy artifact migration uses `fs::rename`, which fails across
  filesystems; migration can error if legacy workspaces live on a different
  volume. Also in the shadows migration loop. (src/config.rs:182-196)
- **[LOW] `estimate_tokens` approximation is brittle.** (src/utils.rs:2) The
  characters/4 rule is highly inaccurate for code (which has many
  symbols/newlines). While fine for a heuristic, it should be explicitly
  documented as an approximation.
- [HIGH] `truncate_by_tokens` contains an unterminated string literal, so the
  crate will not compile. (src/utils.rs:10-12)
- [HIGH] `Workspace::discover` uses `TOAD_HOME` env var, but
  `GlobalConfig::config_dir` uses `TOAD_CONFIG_DIR`. This inconsistency causes
  discovery to mismatch config loading if only one is set. (src/workspace.rs:41
  vs src/config.rs:82)
- [MEDIUM] `Workspace::discover` L75: `fs::canonicalize` on `env_home` will fail
  if the directory does not exist yet, preventing Toad from initializing into a
  new target path set via environment variable.

#### toad-scaffold (#2)

Status: Reviewed — no issues found.

### Phase 2: Intelligence Layer

#### toad-discovery (#2)

Status: Reviewed — issues found.

- [HIGH] Submodule DNA detection uses `root.join(sub.path)` instead of the
  resolved submodule path, so DNA can be derived from a non-existent directory
  when `projects_dir` is `.../projects`. (src/scanner.rs:289-292)
- **[LOW] Character-aware truncation efficiency.** (src/detection.rs:40) Using
  `.chars().take(n).collect()` inside a loop is $O(N)$. For high-volume
  scanning, this is less efficient than checked byte slicing.
- [MEDIUM] Hub/pond classification checks `.gitmodules` under `projects_dir`, so
  hub workspaces with `projects/` are reported as `Pond` instead of `Hub`.
  (src/reports.rs:241-246)
- [HIGH] `scan_all_projects` adds submodules to the `details` list twice: once
  during the `codebase_root` scan (L276) and again if found during the
  `projects_dir` scan (L318). This causes duplicate entries in the manifest and
  registry.

#### toad-git (#2)

Status: Reviewed — issues found.

- [HIGH] `preflight_check` now hard-codes `is_aligned` and `unpushed_count`,
  dropping alignment and unpushed commit detection. This can green-light unsafe
  syncs. (src/sync.rs:18-23)
- [MEDIUM] Submodule status treats `U` (merge conflict) as clean because only
  `'+'` is mapped to dirty. (src/submodule.rs:86-89)
- **[LOW] `align_submodule` implementation is a dummy pull.** (src/align.rs:10)
  The spec implies alignment to the _parent's index_, but the code just runs
  `git pull`.
- [MEDIUM] `check_status` now returns `Untracked` for any repository that
  contains untracked files, even if there are staged/modified files; this masks
  dirty working trees. (src/status.rs:23-27)
- [MEDIUM] `has_unmerged_changes` now checks for merge-conflict markers instead
  of upstream divergence, changing the meaning of the API from “unpushed
  changes” to “conflict files”. (src/merge_status.rs:10-16)

#### toad-manifest (#2)

Status: Reviewed — issues found.

- [MEDIUM] Per-project `CONTEXT.md` generation is still a stub. It currently
  calls `generate_markdown` which produces a single-row table instead of the
  project deep-dive required by the "Deep Croak" spec. (src/lib.rs:214)

#### toad-ops (#2)

Status: Reviewed — issues found.

- [HIGH] `distribute_skills` now only supports three vendors and writes to
  `.windsurf/`, `.cursor/`, `.gemini/` without vendor-specific paths (e.g.
  `.windsurf/rules`, `.gemini/skills`, `.cursorrules`). This breaks existing
  integrations and drops custom vendor mappings. (src/workflow.rs:37-53)
- **[HIGH] REGRESSION: Busy-loop timeout implementation.** (src/shell.rs:25-45)
  `run_in_dir` now uses a manual `loop` with `thread::sleep` and `try_wait`
  rather than using the `wait-timeout` crate (which is still in `Cargo.toml`).
- [MEDIUM] `compare_projects` can underflow the score (e.g., large mismatches)
  and then casts a negative to `u8`, producing high scores instead of clamping
  at 0. (src/migration.rs:16-55)

### Phase 3: Binaries

#### toad-mcp (#1)

Status: Reviewed — issues found.

- [HIGH] `list_contexts` moves `ctx.path` out of a borrowed value, which will
  not compile. Use `ctx.path.clone()` to serialize. (src/server.rs:333-337)
- **[MEDIUM] `NoParams` serializability.** (src/server.rs:72) Adding an empty
  `NoParams` struct without fields can sometimes cause issues with certain
  JSON-RPC parsers that expect an object `{}`.

### Phase 4: Parent Hub

#### Primatif_Toad (#4)

Status: Reviewed — issues found.

- [HIGH] `ggit` submodule operations build paths with
  `workspace.projects_dir.join(&sub.path)` instead of `p.path.join(&sub.path)`.
  For any project outside the hub root, this points at the wrong directory and
  can run git commands against the wrong repo.
  (bin/toad/src/commands/ggit.rs:119-127, 172-179, 271-274)
- **[MEDIUM] UI Glitch: Nothing to commit reported as FAIL.** `ggit commit`
  reports `FAIL` if git returns 1 (nothing to commit). This should be mapped to
  `OK` or `UP TO DATE`.
- [HIGH] `CHANGELOG.md` has been heavily truncated to move details to archive
  files. While good for AI token budgets, it makes the project root less
  informative for human contributors. (CHANGELOG.md)

---

## Cross-Module Concerns

- **Version Drift:** Hub `Cargo.lock` shows some submodules on `dev` and others
  on `main`. Ensure all submodule branches are unified to `dev` before main
  merge.
- **DNA Field Initialization:** New `dna` field in `ProjectDetail` broke tests
  in `toad-core`, `toad-manifest`, and `toad-ops`. (Fixed in current session,
  but worth noting for PR review).

---

## Post-Merge Verification

- [ ] All submodule PRs merged (Phase 1 → 2 → 3 order)
- [ ] Parent Hub PR merged (Phase 4)
- [ ] `git submodule update --remote` on main confirms alignment
- [ ] `cargo check` passes on main branch
- [ ] `just check` passes on main branch
- [ ] `scripts/check_license_boundary.sh` passes on main branch
