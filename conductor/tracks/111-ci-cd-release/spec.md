# Specification: CI/CD & Release Automation (111-ci-cd-release)

## Overview

Add GitHub Actions CI/CD pipeline and release automation to ensure code quality, prevent regressions, and streamline the v1.1.0 release process. Critical for production readiness and future development velocity.

## Sources

- **Current State:** No `.github/workflows/` directory exists
- **Justfile:** Existing QA commands (`just check`, `just qa`)
- **Pre-release Review:** Final v1.1.0 check findings

---

## Problem Statement

**Critical Gap:** Toad has no automated CI/CD pipeline.

Current issues:
- ❌ No automated testing on PRs
- ❌ No automated quality checks (clippy, fmt, markdownlint)
- ❌ No automated license boundary verification
- ❌ No release automation
- ❌ Manual verification required for every change

**Risk:** Without CI/CD, regressions can slip into production, and releases require manual coordination across 8 repositories.

---

## Goals

1. **Automated Testing** — Run tests on every PR and push
2. **Quality Gates** — Enforce clippy, formatting, and linting
3. **License Verification** — Automated boundary checks
4. **Release Automation** — Streamline multi-repo releases
5. **Documentation Checks** — Verify docs are up-to-date

---

## Non-Goals

- Deployment automation (Toad is a CLI tool, not a service)
- Performance benchmarking (future enhancement)
- Security scanning (future enhancement)

---

## Architecture Decisions

### AD-1: GitHub Actions

Use GitHub Actions for CI/CD because:
- Native GitHub integration
- Free for public repos
- Mature Rust ecosystem support
- Easy to configure

### AD-2: Multi-Job Pipeline

Separate jobs for different concerns:
1. **Test** — Run `cargo test --workspace`
2. **Lint** — Run clippy, rustfmt, markdownlint
3. **License** — Run `scripts/check_license_boundary.sh`
4. **Build** — Verify `cargo build` succeeds

### AD-3: Matrix Testing

Test on multiple platforms:
- Ubuntu (primary)
- macOS (secondary)
- Windows (optional, future)

### AD-4: Release Workflow

Separate workflow for releases:
- Triggered by version tags (`v*`)
- Builds release binaries
- Creates GitHub release
- Publishes to crates.io (future)

---

## Implementation Plan

### Phase 1: Create CI Workflow (20 min)

**File:** `.github/workflows/ci.yml`

```yaml
name: CI

on:
  push:
    branches: [main, dev]
  pull_request:
    branches: [main, dev]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test Suite
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
        rust: [stable]
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ matrix.rust }}
      
      - name: Cache cargo registry
        uses: actions/cache@v3
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo index
        uses: actions/cache@v3
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Cache cargo build
        uses: actions/cache@v3
        with:
          path: target
          key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Run tests
        run: cargo test --workspace --verbose

  lint:
    name: Linting
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      
      - name: Check formatting
        run: cargo fmt --all -- --check
      
      - name: Run clippy
        run: cargo clippy --workspace -- -D warnings
      
      - name: Install markdownlint
        run: npm install -g markdownlint-cli
      
      - name: Check markdown
        run: markdownlint "**/*.md"
      
      - name: Install dprint
        run: cargo install dprint
      
      - name: Check dprint
        run: dprint check

  license:
    name: License Boundary Check
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive
      
      - name: Verify license boundaries
        run: ./scripts/check_license_boundary.sh

  build:
    name: Build
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Build
        run: cargo build --workspace --verbose
      
      - name: Build MCP server
        run: cargo build -p toad-mcp --verbose
```

### Phase 2: Create Release Workflow (15 min)

**File:** `.github/workflows/release.yml`

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

env:
  CARGO_TERM_COLOR: always

jobs:
  create-release:
    name: Create Release
    runs-on: ubuntu-latest
    outputs:
      upload_url: ${{ steps.create_release.outputs.upload_url }}
    steps:
      - name: Create Release
        id: create_release
        uses: actions/create-release@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tag_name: ${{ github.ref }}
          release_name: Release ${{ github.ref }}
          draft: false
          prerelease: false

  build-release:
    name: Build Release
    needs: create-release
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact_name: toad
            asset_name: toad-linux-amd64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact_name: toad
            asset_name: toad-macos-amd64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact_name: toad
            asset_name: toad-macos-arm64
    steps:
      - uses: actions/checkout@v4
        with:
          submodules: recursive
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Build release binary
        run: cargo build --release --target ${{ matrix.target }} -p toad
      
      - name: Strip binary
        if: matrix.os != 'windows-latest'
        run: strip target/${{ matrix.target }}/release/${{ matrix.artifact_name }}
      
      - name: Upload Release Asset
        uses: actions/upload-release-asset@v1
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          upload_url: ${{ needs.create-release.outputs.upload_url }}
          asset_path: target/${{ matrix.target }}/release/${{ matrix.artifact_name }}
          asset_name: ${{ matrix.asset_name }}
          asset_content_type: application/octet-stream
```

### Phase 3: Add Status Badges to README (5 min)

**File:** `README.md`

Add badges after the version badge:

```markdown
[![Version: v1.1.0](https://img.shields.io/badge/version-v1.1.0-green.svg)](Cargo.toml)
[![CI](https://github.com/Primatif/Primatif_Toad/workflows/CI/badge.svg)](https://github.com/Primatif/Primatif_Toad/actions)
[![Coverage: >80%](https://img.shields.io/badge/coverage-%3E80%25-brightgreen.svg)](Justfile)
```

### Phase 4: Add CONTRIBUTING.md (10 min)

**File:** `CONTRIBUTING.md`

```markdown
# Contributing to Toad

## Development Setup

1. Clone with submodules:
   ```bash
   git clone --recursive https://github.com/Primatif/Primatif_Toad.git
   cd Primatif_Toad
   ```

2. Run setup:
   ```bash
   just setup
   ```

3. Install Toad:
   ```bash
   just install
   ```

## Before Submitting a PR

Run the full QA suite:

```bash
just qa
```

This runs:
- `cargo test --workspace`
- `cargo clippy --workspace -- -D warnings`
- `cargo fmt --all -- --check`
- `markdownlint "**/*.md"`
- `dprint check`
- License boundary verification

## CI/CD

All PRs must pass:
- ✅ Tests on Ubuntu and macOS
- ✅ Clippy with no warnings
- ✅ Formatting checks
- ✅ Markdown linting
- ✅ License boundary verification

## License

See [LICENSE](LICENSE) for details on the MIT/BUSL-1.1 dual licensing model.
```

---

## Success Criteria

- [ ] CI workflow runs on every PR and push
- [ ] All quality checks automated
- [ ] Release workflow creates binaries for Linux and macOS
- [ ] Status badges in README
- [ ] CONTRIBUTING.md guides contributors
- [ ] CI passes on current codebase

---

## Integration Points

- **Depends on:** Existing `Justfile` commands
- **Consumed by:** GitHub Actions
- **Testing:** Push to dev branch and verify CI runs

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| CI takes too long | Use caching for cargo dependencies |
| Flaky tests | Ensure tests are deterministic |
| License check fails | Fix boundaries before enabling CI |
| macOS runner costs | Only run on main/dev branches |

---

## Future Enhancements

- Windows support
- Automated crates.io publishing
- Performance benchmarking
- Security scanning (cargo-audit)
- Coverage reporting
