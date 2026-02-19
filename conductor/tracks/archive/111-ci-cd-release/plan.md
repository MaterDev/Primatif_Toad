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

- [x] Create `.github/workflows/` directory
- [x] Create `ci.yml` workflow file
- [x] Configure test job with matrix (Ubuntu, macOS)
- [x] Configure lint job (clippy, rustfmt, markdownlint, dprint)
- [x] Configure license boundary check job
- [x] Configure build job
- [x] Add cargo caching for faster builds

### Phase 2: Create Release Workflow

- [x] Create `release.yml` workflow file
- [x] Configure release creation on version tags
- [x] Configure multi-platform binary builds
- [x] Add binary stripping for smaller artifacts
- [x] Configure asset uploads to GitHub releases

### Phase 3: Update Documentation

- [x] Add CI status badge to README.md
- [x] Create CONTRIBUTING.md with development workflow
- [x] Document CI requirements for PRs

### Phase 4: Testing

- [x] Push to dev branch and verify CI runs
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
