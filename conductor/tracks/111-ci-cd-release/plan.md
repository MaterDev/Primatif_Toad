# Plan: CI/CD & Release Automation (111-ci-cd-release)

> **Spec:** [./spec.md](./spec.md)

---

## Timeline

- **Estimated Effort:** 50 minutes
- **Target:** v1.1.0 (critical for production readiness)
- **Priority:** P0 (Blocks safe releases)

---

## Tasks

### Phase 1: Create CI Workflow

- [ ] Create `.github/workflows/` directory
- [ ] Create `ci.yml` workflow file
- [ ] Configure test job with matrix (Ubuntu, macOS)
- [ ] Configure lint job (clippy, rustfmt, markdownlint, dprint)
- [ ] Configure license boundary check job
- [ ] Configure build job
- [ ] Add cargo caching for faster builds

### Phase 2: Create Release Workflow

- [ ] Create `release.yml` workflow file
- [ ] Configure release creation on version tags
- [ ] Configure multi-platform binary builds
- [ ] Add binary stripping for smaller artifacts
- [ ] Configure asset uploads to GitHub releases

### Phase 3: Update Documentation

- [ ] Add CI status badge to README.md
- [ ] Create CONTRIBUTING.md with development workflow
- [ ] Document CI requirements for PRs

### Phase 4: Testing

- [ ] Push to dev branch and verify CI runs
- [ ] Fix any CI failures
- [ ] Verify all jobs pass
- [ ] Test release workflow with test tag (optional)

---

## Acceptance Criteria

- CI workflow runs on every PR and push to main/dev
- All quality checks pass (tests, clippy, fmt, markdown)
- License boundary verification automated
- Release workflow creates binaries for Linux and macOS
- Documentation updated with CI badges and contribution guide
