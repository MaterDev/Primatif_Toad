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
