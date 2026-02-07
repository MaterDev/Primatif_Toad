# Track-008: Toad Tag (Taxonomy)

## Status

- **Priority:** Medium
- **Owner:** Gemini (Solo-Dev Support)
- **Status:** Completed

## Objective

Implement a lightweight project tagging system for multi-dimensional ecosystem
organization and filtering.

## Deliverables

- [x] **Infrastructure: Tag Registry** [commit: c8132d3]
  - [x] Create `TagRegistry` in `toad-core`. [commit: c8132d3]
  - [x] Implement JSON persistence in `shadows/tags.json`. [commit: c8132d3]
- [x] **Discovery Integration** [commit: c8132d3]
  - [x] Update `ProjectDetail` model to include tags. [commit: c8132d3]
  - [x] Refactor discovery to merge procedurally detected tags with persistent
        ones.
- [x] **CLI API: Tag Management** [commit: c8132d3]
  - [x] Implement `toad tag` command. [commit: c8132d3]
  - [x] Implement `toad untag` command. [commit: c8132d3]
- [x] **CLI API: Filtering** [commit: c8132d3]
  - [x] Add `--tag` filter to `reveal`, `status`, `do`, and `stats`. [commit:
        c8132d3]
- [x] **Verification** [commit: c8132d3]
  - [x] Verify tag persistence after CLI calls. [commit: c8132d3]
  - [x] Perform SemVer bump. [commit: c8132d3]
