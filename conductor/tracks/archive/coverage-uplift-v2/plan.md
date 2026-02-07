# Track-016: Coverage Uplift (Phase 2)

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** In-Progress

## Objective

Increase workspace-wide code coverage from 67.24% to the project target of >80%.
Focus on the high-impact areas identified in `bin/toad` and `toad-core`.

## Deliverables



### Phase 1: CLI Command Coverage (`bin/toad`)

- [x] **Task: Strategy Command Tests**

    - [x] Write Tests: Add `test_strategy_flow` to `cli_tests.rs` covering list, info, add, and remove.

- [x] **Task: Clean Command Tests**

    - [x] Write Tests: Add tests for real (non-dry-run) cleaning and multi-project batch cleaning.

- [x] **Task: Error Handling & Bootstrap Tests**

    - [x] Write Tests: Verify behavior when no workspace is found for both bootstrap and non-bootstrap commands.



### Phase 2: Core Logic Coverage (`toad-core`)

- [x] **Task: Workspace Discovery Tests**

    - [x] Write Tests: Mock environment variables and filesystem structures to test all 3 tiers of `Workspace::discover`.

- [x] **Task: Strategy Registry Edge Cases**

    - [x] Write Tests: Test loading of invalid TOML and shadowing behavior in `StrategyRegistry`.



### Phase 3: Ops & Discovery Refinements

- [x] **Task: Audit Logging Verification**

    - [x] Write Tests: Verify that `toad do` actually writes to the `ops.log` file.

- [x] **Task: Discovery Snapshot Integrity**

    - [x] Write Tests: Verify `scan_all_projects` behavior with custom strategies and hybrid project detection.



## Verification

- [x] Run `just coverage` and confirm total coverage is >80%.

- [x] Full `just qa` pass.


