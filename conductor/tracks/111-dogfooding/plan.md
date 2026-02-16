# Plan: Dogfooding Improvements (111-dogfooding)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 40 minutes
- **Target:** v1.1.0 (improves development workflow)
- **Priority:** P2 (High value, not blocking)

---

## Tasks

### Phase 1: Apply Tags

- [ ] Tag core crates: `toad tag toad-core --tag core --yes`
- [ ] Tag intelligence crates: `toad-discovery`, `toad-manifest`
- [ ] Tag orchestration crates: `toad-git`, `toad-ops`
- [ ] Tag interface crates: `Primatif_Toad`, `bin/toad-mcp`
- [ ] Tag utility crates: `toad-scaffold`
- [ ] Verify tags: `toad status --tag core`

### Phase 2: Register Workflows

- [ ] Create `scripts/workflows/` directory
- [ ] Create `qa.sh` workflow
- [ ] Create `release-check.sh` workflow
- [ ] Create `update-docs.sh` workflow
- [ ] Make scripts executable: `chmod +x scripts/workflows/*.sh`
- [ ] Register workflows: `toad cw register <name> <path>`
- [ ] Test workflows: `toad cw run qa`

### Phase 3: Add Git Hook

- [ ] Create `scripts/git-hooks/post-commit`
- [ ] Add auto-refresh logic
- [ ] Update `Justfile` to install post-commit hook
- [ ] Run `just setup-hooks` to install
- [ ] Test: make a commit and verify manifest refreshes

### Phase 4: MCP Configuration

- [ ] Document MCP setup in README or `.windsurf/`
- [ ] Test MCP server with Toad workspace
- [ ] Verify tools work for querying Toad's ecosystem

### Phase 5: Documentation

- [ ] Add dogfooding section to README
- [ ] Document workflow commands
- [ ] Document tag taxonomy
- [ ] Add examples of using Toad to develop Toad

---

## Acceptance Criteria

- All submodules tagged with appropriate taxonomy
- Common workflows registered and working
- Git hook auto-refreshes manifest after commits
- MCP server configured for development
- Documentation includes dogfooding examples
