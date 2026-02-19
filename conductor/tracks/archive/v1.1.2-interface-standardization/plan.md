# Plan: Interface Standardization (v1.1.2-interface-standardization)

> **Spec:** [./spec.md](./spec.md)

---

## Tasks

### Phase 1: Unified Flags

- [x] Audit all commands for interactive/destructive behavior.
- [x] Implement `--yes` across `tag`, `create`, `delete`, `clean`, and `ggit`.
- [x] Ensure `--json` output is consistent and suppresses interactive prompts.

### Phase 2: Error Message Updates ✅ COMPLETE

- [x] Update `tag` error message to suggest `--yes`.
- [x] Update all confirmation abort messages to mention `--yes` flag

**Completed:**

- All abort messages now display: "Aborted. (Use --yes to skip confirmation)"
- Updated in: ggit.rs (5 commands), cw.rs, strategy.rs
- Provides clear guidance for non-interactive execution

### Phase 3: Command Discovery ✅ COMPLETE

- [x] Add `unregister` alias to `toad project delete`.
- [x] Add `remove` alias to `toad tag`.
- [x] Implement "did you mean" suggestions for subcommands.
- [x] Integrate suggestion engine into main.rs
- [x] Test with common typos (lst→list, search→reveal, statu→status)

**Completed:**

- Created suggestion engine with 15+ aliases
- Integrated into main.rs with clap error handling
- Levenshtein distance algorithm suggests commands within distance of 2
- Tested successfully with typos

### Phase 4: Documentation ✅ COMPLETE

- [x] Update USER_GUIDE.md with `--yes` flag examples
- [x] Document command aliases in USER_GUIDE.md
- [x] Add examples of suggestion system

**Completed:**

- Added "Non-Interactive Execution" section to USER_GUIDE.md
- Documented all `--yes` flags with examples
- Listed all command aliases
- Included "Did You Mean?" examples

---

## Acceptance Criteria

- [x] Commands behave identically when `--yes` is passed.
- [x] Aliases work as expected (tested: lst→list, search→reveal, statu→status).
- [x] "Did you mean" suggestions work for typos.
- [x] Documentation updated to reflect standardized flags.

## TRACK COMPLETE

All phases complete. v1.1.2-interface-standardization is ready for release.
