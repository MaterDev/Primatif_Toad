# Plan: Interface Standardization & Command Discovery (v1.1.2-interface-standardization)

> **Spec:** [./spec.md](./spec.md)

---

## Tasks

### Phase 1: Flag Standardization

- [ ] Audit all commands for interactive/destructive behavior.
- [ ] Implement `--yes` across `tag`, `create`, `delete`, `clean`, and `ggit`.
- [ ] Ensure `--json` output is consistent and suppresses interactive prompts.

### Phase 2: Error Message Updates

- [ ] Update `tag` error message to suggest `--yes`.
- [ ] Update `create` error message for existing directories to be more
      descriptive.

### Phase 3: Command Discovery

- [ ] Add `unregister` alias to `toad project delete`.
- [ ] Add `remove` alias to `toad tag`.
- [ ] Research/Implement "did you mean" suggestions for subcommands.

---

## Acceptance Criteria

- Commands behave identically when `--yes` is passed.
- Aliases work as expected.
- Documentation updated to reflect standardized flags.
