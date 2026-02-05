# Track-012: Macro Tagging & Global Context

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** In-Progress

## Objective

Production-harden the CLI's context discovery and implement macro-level metadata
management.

## Deliverables

- [ ] **Phase 1: The Anchor Infrastructure**
  - [ ] Create `.toad-root` in the repository root.
  - [ ] Implement `GlobalConfig` registry in `toad-core`
        (`~/.toad/config.json`).
  - [ ] Implement `Workspace::discover()` with 3-tier priority logic.
- [ ] **Phase 2: Global CLI Refactor**
  - [ ] Create `toad home` command to manage the workspace pointer.
  - [ ] Update every subcommand to use absolute paths from the discovered
        workspace.
  - [ ] Add `just install` recipe to make `toad` globally available in `$PATH`.
- [ ] **Phase 3: Macro Tagging**
  - [ ] Refactor `toad tag` to support bulk operations via `--query` and
        `--stack`.
  - [ ] Implement "Preflight" visualization for bulk tagging.
  - [ ] Add `toad tag --harvest` for automated stack-based tagging.
- [ ] **Verification**
  - [ ] Verify `toad status` works from a foreign directory (e.g., `/tmp`).
  - [ ] Verify `toad home <path>` correctly validates and updates pointer.
  - [ ] Perform SemVer bump.
