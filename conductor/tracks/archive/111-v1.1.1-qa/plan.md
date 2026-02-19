# Plan: v1.1.1 QA Testing (111-v1.1.1-qa)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 1 hour
- **Target:** v1.1.1
- **Priority:** P1 (Blocking for final release sign-off)

---

## Tasks

### Phase 1: Environment Setup

- [x] Create QA release directory: `docs/releases/v1.1.1/`
- [x] Draft initial test plan: `docs/releases/v1.1.1/qa_test_plan.md`

### Phase 2: Hub Inspection (Non-Destructive)

- [x] Run `toad doctor` on main workspace.
- [x] Run `analyze health` across all submodules.
- [x] Run `analyze debt` to identify high-risk files.
- [x] Run `analyze deps` to verify critical path.

### Phase 3: Sandbox Validation (Destructive)

- [x] Generate sandbox: `scripts/sandbox/mksandbox.sh -c 5 -o qa_sandbox`
- [x] Register and switch to sandbox context.
- [x] Test `toad tag` with `--yes` flag.
- [x] Test `toad create` with `--yes` flag.
- [x] Test `toad sync` and `toad manifest`.
- [x] Cleanup: delete context and sandbox directory.

### Phase 4: MCP Verification

- [x] Verify `get_active_context`.
- [x] Verify `list_projects` and `get_ecosystem_status`.
- [x] Verify `analyze_debt` and `analyze_dependencies`.

### Phase 5: Documentation & Sign-off

- [x] Generate final QA Report: `docs/releases/v1.1.1/qa_report.md`
- [x] Define future regression strategy.
- [x] Perform Deep-Dive validation (Skills, Strategies, Error Injection).
- [ ] Final user review of results.

---

## Acceptance Criteria

- Full coverage of CLI subcommands.
- Verified isolation of sandbox testing.
- Documented usability findings (non-interactive flags).
- Healthy state confirmed by `toad doctor`.
