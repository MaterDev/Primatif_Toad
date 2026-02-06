# Track-015: Dynamic Strategy Engine

## Status
- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** In-Progress

## Objective
Transform Toad into a data-driven orchestration engine where language/stack support is modular, user-definable, and manageable via the CLI. This provides the metadata required for Unified Taxonomy and Toad Clean, effectively creating a "Stack Support Plugin System".

## Code Review Findings
- **Detection:** `discovery::strategies` is hardcoded and returns the first match only. Needs to support multiple matches for hybrid projects.
- **Taxonomy:** `ProjectDetail` separates `stack` (enum) and `hashtags` (vec). These should be unified into a single dynamic taxonomy.
- **Operations:** `toad-ops::stats` hardcodes artifact directories (e.g., `target`, `node_modules`). This must be moved to the strategy manifest.
- **Reporting:** `toad-manifest` has a fixed table structure that doesn't account for dynamic stacks.

## Deliverables

### Phase 1: Core Data Models & Serialization
- [ ] **Task: Define `StackStrategy` Model**
    - [ ] Write Tests: Verify `StackStrategy` can be serialized/deserialized to/from TOML.
    - [ ] Implement: Create `StackStrategy` struct in `toad-core` (name, match_files, artifacts, tags, priority).
- [ ] **Task: Refactor `ProjectDetail` & Taxonomy**
    - [ ] Write Tests: Ensure `ProjectDetail` can store a unified `taxonomy` list.
    - [ ] Implement: Add `taxonomy` to `ProjectDetail` and prepare for `stack`/`hashtags` deprecation.

### Phase 2: The Strategy Registry & Loader
- [ ] **Task: Implement Strategy Loader**
    - [ ] Write Tests: Verify loader correctly merges `builtin` and `custom` (at `~/.toad/strategies/`) strategies.
    - [ ] Implement: Create the loader in `toad-core`. It should load TOML files.
- [ ] **Task: Strategy Resolution**
    - [ ] Write Tests: Verify priority-based resolution (custom beats builtin).
    - [ ] Implement: Logic to handle multiple matches (cumulative tags).

### Phase 3: CLI Management (`toad strategy`)
- [ ] **Task: Implement `toad strategy list`**
    - [ ] Implement: List all active strategies with their metadata.
- [ ] **Task: Implement `toad strategy add`**
    - [ ] Implement: Command with flags and basic interactive prompts to generate a new strategy TOML.
- [ ] **Task: Implement `toad strategy info <name>`**
    - [ ] Implement: Show details of a specific strategy.

### Phase 4: Migration & Integration
- [ ] **Task: Migrate Hardcoded Logic**
    - [ ] Implement: Convert Rust, Go, Node, Python, and Monorepo logic into `builtin` TOML files.
    - [ ] Implement: Refactor `discovery` and `toad-ops::stats` to use the dynamic engine.
- [ ] **Task: Manifest Update**
    - [ ] Implement: Update `toad-manifest` to render the unified taxonomy.

### Phase 5: Documentation & User Guidance
- [ ] **Task: Update User Guide & README**
    - [ ] Implement: Add section on "Stack Support Plugin System" to `README.md`.
    - [ ] Implement: Add detailed "Developing Stack Plugins" guide to `USER_GUIDE.md`.
- [ ] **Task: CLI Directives**
    - [ ] Implement: Ensure `toad strategy --help` and related commands provide clear guidance on the plugin model.

### Phase 6: Cleanup
- [ ] **Task: Final De-stiffening**
    - [ ] Delete `crates/discovery/src/strategies.rs`.
    - [ ] Remove `ProjectStack` enum and old fields.

## Verification
- [ ] `toad strategy add --name "Elixir" --match "mix.exs" --clean "deps,_build"` and verify it is detected.
- [ ] Run `toad reveal` on a project with a custom strategy.
- [ ] Full `just qa` pass.