# Track-012: Macro Tagging & Global Context

## Status

- **Priority:** High
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** In-Progress

## Objective

Production-harden the CLI's context discovery and implement macro-level metadata
management.

## Deliverables

- [x] **Phase 1: The Anchor Infrastructure** [commit: fe68718]
  - [x] Create `.toad-root` in the repository root. [commit: fe68718]
  - [ ] Implement `GlobalConfig` registry in `toad-core`
        (`~/.toad/config.json`).
  - [x] Implement `Workspace::discover()` with 3-tier priority logic. [commit: fe68718]
- [x] **Phase 2: Global CLI Refactor** [commit: fe68718]
  - [x] Create `toad home` command to manage the workspace pointer. [commit: fe68718]
  - [ ] Update every subcommand to use absolute paths from the discovered
        workspace.
  - [ ] Add `just install` recipe to make `toad` globally available in `$PATH`.
- [x] **Phase 3: Macro Tagging** [commit: fe68718]
  - [ ] Refactor `toad tag` to support bulk operations via `--query` and
        `--stack`.
  - [x] Implement "Preflight" visualization for bulk tagging. [commit: fe68718]
  - [x] Add `toad tag --harvest` for automated stack-based tagging. [commit: fe68718]
- [x] **Verification** [commit: fe68718]
  - [x] Verify `toad status` works from a foreign directory (e.g., `/tmp`). [commit: fe68718]
  - [x] Verify `toad home <path>` correctly validates and updates pointer. [commit: fe68718]
  - [x] Perform SemVer bump. [commit: fe68718]
