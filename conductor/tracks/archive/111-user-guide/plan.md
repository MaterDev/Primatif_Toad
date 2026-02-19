# Plan: User Guide Updates (111-user-guide)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 45 minutes
- **Target:** v1.1.0 (documentation completeness)
- **Priority:** P1 (Critical for adoption)

---

## Tasks

### Phase 1: Create USER_GUIDE.md

- [ ] Create `USER_GUIDE.md` at repo root
- [ ] Write Quick Start section (5-minute setup)
- [ ] Write Core Concepts section
  - Workspace Discovery
  - Project Registry
  - Context Engineering
  - DNA Patterns
- [ ] Write Common Workflows section
  - Multi-Repo Git Operations
  - Bulk Operations
  - Context Management
  - AI Integration
- [ ] Write Advanced Topics section
  - MCP Server
  - Stack Strategies
  - Custom Workflows
- [ ] Write Troubleshooting section
- [ ] Add Reference links

### Phase 2: Update MCP Guide

- [ ] Update `docs/guides/MCP.md`
- [ ] Document all 16 tools in "Available Tools"
- [ ] Add "Common Workflows" section
- [ ] Update troubleshooting tips

### Phase 3: Update CLI Guide

- [ ] Verify `docs/guides/CLI.md` is current
- [ ] Add link to USER_GUIDE.md at top
- [ ] Run `toad docs` to regenerate if needed

### Phase 4: Update README

- [ ] Update "What's New in v1.1.0" section
- [ ] Add prominent link to USER_GUIDE.md
- [ ] Verify quick start commands are accurate
- [ ] Update feature highlights

### Phase 5: Testing

- [ ] Follow quick start guide manually
- [ ] Verify all links work
- [ ] Check for broken references
- [ ] Run markdownlint

---

## Acceptance Criteria

- USER_GUIDE.md provides comprehensive getting-started guide
- All v1.1.0 features are documented
- Quick start is tested and works
- Links between guides are correct
- Documentation passes markdownlint
