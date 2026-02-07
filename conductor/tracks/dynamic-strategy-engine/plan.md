# Track-015: Dynamic Strategy Engine

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** In-Progress

## Objective

Transform Toad into a data-driven orchestration engine where language/stack
support is modular, user-definable, and manageable via the CLI. This provides
the metadata required for Unified Taxonomy and Toad Clean, effectively creating
a "Stack Support Plugin System".

## Code Review Findings

- **Detection:** `discovery::strategies` is hardcoded and returns the first
  match only. Needs to support multiple matches for hybrid projects.
- **Taxonomy:** `ProjectDetail` separates `stack` (enum) and `hashtags` (vec).
  These should be unified into a single dynamic taxonomy.
- **Operations:** `toad-ops::stats` hardcodes artifact directories (e.g.,
  `target`, `node_modules`). This must be moved to the strategy manifest.
- **Reporting:** `toad-manifest` has a fixed table structure that doesn't
  account for dynamic stacks.

## Deliverables

### Phase 1: Core Data Models & Serialization

- [x] **Task: Define `StackStrategy` Model**

  - [x] Write Tests: Verify `StackStrategy` can be serialized/deserialized
        to/from TOML.

  - [x] Implement: Create `StackStrategy` struct in `toad-core` (name,
        match_files, artifacts, tags, priority).

- [x] **Task: Refactor `ProjectDetail` & Taxonomy**

  - [x] Write Tests: Ensure `ProjectDetail` can store a unified `taxonomy` list.

  - [x] Implement: Add `taxonomy` to `ProjectDetail` and prepare for
        `stack`/`hashtags` deprecation.

### Phase 2: The Strategy Registry & Loader

- [x] **Task: Implement Strategy Loader**

  - [x] Write Tests: Verify loader correctly merges `builtin` and `custom` (at
        `~/.toad/strategies/`) strategies.

  - [x] Implement: Create the loader in `toad-core`. It should load TOML files.

- [x] **Task: Strategy Resolution**

  - [x] Write Tests: Verify priority-based resolution (custom beats builtin).

  - [x] Implement: Logic to handle multiple matches (cumulative tags).

### Phase 3: CLI Management (`toad strategy`)

- [x] **Task: Implement `toad strategy list`**

  - [x] Implement: List all active strategies with their metadata.

- [x] **Task: Implement `toad strategy add`**

  - [x] Implement: Command with flags and basic interactive prompts to generate
        a new strategy TOML.

- [x] **Task: Implement `toad strategy info <name>`**

  - [x] Implement: Show details of a specific strategy.

### Phase 4: Migration & Integration

- [x] **Task: Migrate Hardcoded Logic**

  - [x] Implement: Convert Rust, Go, Node, Python, and Monorepo logic into
        `builtin` TOML files.

  - [x] Implement: Refactor `discovery` and `toad-ops::stats` to use the dynamic
        engine.

- [x] **Task: Manifest Update**

  - [x] Implement: Update `toad-manifest` to render the unified taxonomy.

### Phase 5: Documentation & User Guidance

- [x] **Task: Update User Guide & README**

  - [x] Implement: Add section on "Stack Support Plugin System" to `README.md`.

  - [x] Implement: Add detailed "Developing Stack Plugins" guide to
        `USER_GUIDE.md`.

- [x] **Task: CLI Directives**

  - [x] Implement: Ensure `toad strategy --help` and related commands provide
        clear guidance on the plugin model.

### Phase 6: Cleanup

- [x] **Task: Final De-stiffening**

  - [x] Delete `crates/discovery/src/strategies.rs`.

  - [x] Remove `ProjectStack` enum and old fields.

### Phase 7: Refinements and Performance Optimization

- [x] **Task: Performance Tuning**

  - [x] Implement `HashSet` lookup for artifact detection in `toad-ops`.

- [x] **Task: Robustness & Safety**

  - [x] Sanitize strategy filenames in `toad strategy add`.

  - [x] Implement strategy shadowing (custom TOML files replace built-ins).

  - [x] Improve Wails default strategy with frontend artifacts.

  - [x] Make monorepo discovery more flexible (substring matching).

## Verification

- [x] `toad strategy add --name "Elixir" --match "mix.exs" --clean "deps,_build"`
      and verify it is detected.

- [x] Run `toad reveal` on a project with a custom strategy.

- [x] Full `just qa` pass.
