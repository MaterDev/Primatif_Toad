# v1.1.0 "Deep Croak" — Deep Code Review

## Overall Verdict

Solid foundation and correct architectural direction, with several spec
deviations and a couple of architectural violations that should be fixed
**before MCP server work**.

- The crate boundary discipline is good.
- The global-home / context architecture is largely correct.
- The remaining risk is concentrated in a small number of command handlers that
  still behave like “terminal apps” instead of reusable library surfaces.

---

## Critical Issues (Fix Before MCP)

### 1) `toad do` violates the No-Print + No-stdin mandates (BLK-1, BLK-4)

File:

- `bin/toad/src/commands/do_cmd.rs`

Findings:

- Prints directly via `println!` throughout the handler.
- Reads stdin directly for confirmations.
- Returns `Result<()>` instead of a structured result type.
- Has no `--json` output path.

Why this matters:

- MCP tools must call the same underlying logic **without a terminal**.
- Direct printing and stdin reads make this logic non-reusable for a server.

Required change:

- Move business logic into a library (likely `toad-ops`) that:
  - Accepts confirmation as an input parameter (`confirmed: bool`)
  - Returns a structured report (e.g., `BatchOperationReport`)
- Keep printing and prompting in `main.rs` / `ui.rs` only.

### 2) `toad clean` has the same violations as `toad do`

File:

- `bin/toad/src/commands/clean.rs`

Findings:

- Prints directly.
- Reads stdin directly.
- Returns `Result<()>`.
- Progress/reporting is created inside command logic.

Required change:

- Same fix pattern as `toad do`: extract library logic returning a structured
  report and accept confirmation as a parameter.

### 3) `status` and `stats` ignore query/tag filtering in the data layer

Files:

- `bin/toad/src/commands/status.rs`
- `bin/toad/src/commands/stats.rs`

Findings:

- `status.rs` ignores `_query` and `_tag` (filtering is done only in `ui.rs`).
- `stats.rs` ignores `_query`, `_tag`, `_all` (filtering is done only in
  `ui.rs`).

Why this matters:

- `--json` output becomes untrustworthy because it returns unfiltered full
  datasets.
- MCP tools need filtering semantics at the data/service layer.

Required change:

- Implement filtering in discovery/ops layer (or in command handlers before
  returning structured data), not in the UI formatting layer.

---

## Moderate Issues

### 4) Fingerprint parsing is fragile and duplicated

Files:

- `bin/toad/src/main.rs`
- `bin/toad/src/commands/manifest.rs`

Finding:

- The “stored fingerprint” is extracted by parsing `MANIFEST.md` for a line
  containing `**Fingerprint:**`.

Why this matters:

- This is “machine state derived from human output.”
- The fingerprint already exists in `registry.json`
  (`ProjectRegistry.fingerprint`).

Recommendation:

- Use `ProjectRegistry::load(...).fingerprint` as the authoritative stored
  fingerprint.
- Centralize fingerprint load/compare logic in one place.

### 5) `manifest --check` uses `std::process::exit(1)`

File:

- `bin/toad/src/commands/manifest.rs`

Why this matters:

- `exit()` bypasses normal error propagation and makes reuse (MCP/server)
  harder.

Recommendation:

- Return a typed error (ideally `ToadError` variant) or a structured “stale”
  result that the binary translates into an exit code.

### 6) Per-project `CONTEXT.md` isn’t a “deep dive” yet

File:

- `bin/toad/src/commands/manifest.rs`

Finding:

- `CONTEXT.md` is currently generated via
  `generate_markdown(&[p.clone()], ...)`, which is just a single-row manifest
  table.

Spec mismatch:

- The spec’s progressive disclosure intends `CONTEXT.md` to be a project-level
  briefing (~4k tokens) including essence, stack details, key files, taxonomy,
  and later DNA.

Recommendation:

- Implement a dedicated
  `generate_project_context_md(project: &ProjectDetail, ...) -> String`.

### 7) `init-context` missing spec’d flags

File:

- `bin/toad/src/cli.rs`

Finding:

- `init-context` implements only `--force`.

Spec wants:

- `--dry-run`
- `--project <name>`
- `--no-sync`

Recommendation:

- Add the flags to CLI surface and thread them through to the underlying
  generation pipeline.

### 8) `Workspace::discover()` still has upward `.toad-root` search (spec said remove)

File:

- `crates/toad-core/src/workspace.rs`

Finding:

- The implementation includes a Tier 4 upward search to find `.toad-root` and
  auto-migrate.

Note:

- This is arguably a reasonable backward compatibility behavior, but it
  conflicts with the explicit Phase 0 directive to remove upward discovery.

Recommendation:

- Decide deliberately:
  - If keeping: document it as a compatibility layer and ensure it can’t mask
    missing global setup.
  - If removing: implement the 3-tier logic exactly as spec.

---

## Strengths / What’s Done Well

### 9) No-Print mandate is mostly honored in library crates

Finding:

- Library crates appear clean of `println!` usage (printing is concentrated in
  `bin/toad`), which is the correct direction.

### 10) `ProgressReporter` trait is clean and correctly placed

Files:

- `crates/toad-core/src/ui.rs` (trait + `NoOpReporter`)
- `bin/toad/src/ui.rs` (`IndicatifReporter` impl)

This correctly enables headless execution (MCP) and terminal UI without
coupling.

### 11) `ToadError` typed error surface exists and is reasonable

File:

- `crates/toad-core/src/error.rs`

Covers the expected “core failure states” and supports library error returns.

### 12) Schema-first contract is broadly satisfied

Finding:

- Core structs and report types reviewed derive `Serialize + Deserialize`.
- Global `--json` exists and works for many commands.

### 13) Embedded strategies + custom overrides work

File:

- `crates/toad-core/src/strategy.rs`

Default embedded strategies and `~/.toad/strategies/custom` overrides are
implemented and tested.

### 14) CHANGELOG diffing works and is capped

File:

- `crates/toad-discovery/src/reports.rs`

Diff logic correctly emits Added/Removed/Modified changes and caps history to 50
entries.

### 15) License/dependency boundaries appear respected

Finding:

- `toad-core` (MIT) does not depend on BSL crates.
- BSL crates depend on `toad-core` and other BSL crates as expected.

### 16) File size discipline looks good

Finding:

- No reviewed source file exceeds the 700 LOC guideline.

---

## Tasks.md Completion Audit (High-Level)

- **BLK-1 No-Print**: partial (blocked by `do` and `clean`)
- **BLK-4 No-stdin in lib logic**: not complete (blocked by `do` and `clean`)
- **P1/P2**: mostly complete (fingerprint `!=`, changelog, token mitigation
  present)
- **P3**: largely complete (agents, llms.txt, system prompt) but `CONTEXT.md`
  depth is not met, and `init-context` flags are missing

---

## Recommended Actions Before MCP (Priority Order)

### Must-fix (blocks MCP)

1. Refactor `do` and `clean` to return structured results and remove direct
   printing/stdin reads from command logic.
2. Move filtering semantics for `status`/`stats` into the data/service layer so
   JSON output is truthful.
3. Stop parsing fingerprints from `MANIFEST.md`; use `ProjectRegistry` as
   authoritative.
4. Remove `std::process::exit(1)` from `manifest --check` and return
   typed/structured results.

### Should-fix (quality + spec alignment)

1. Implement a real project-level `CONTEXT.md` generator (“deep dive”) rather
   than a one-row manifest.
2. Add `init-context` flags required by spec (`--dry-run`, `--project`,
   `--no-sync`).
3. Decide explicitly whether the upward `.toad-root` search stays as
   compatibility or is removed to match spec.

---

## Bottom Line

The “Deep Croak” architecture is largely in place, and most work aligns with the
vision of Toad as an AI-native context oracle. The remaining work is primarily
about finishing the **library-first / server-friendly** refactor for the
remaining terminal-centric commands and tightening spec fidelity so MCP can
reuse the same core logic safely.
