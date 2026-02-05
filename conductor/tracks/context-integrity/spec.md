# Spec: Context Integrity (Deep Fingerprinting)

## Overview

The current fingerprinting logic in `toad-core` is too shallow, relying only on
the `mtime` of the root `projects/` directory. This track implements a
multi-level hashing strategy to ensure the AI context ("Shadows") accurately
reflects the state of the managed ecosystem.

## Requirements

- **Deep Detection:** Detect changes to individual project metadata
  (structurally significant files).
- **Deletion Awareness:** Detect when projects are removed.
- **Zero-Latency Feel:** Must execute in under 50ms for 100+ projects to avoid
  slowing down CLI startup.
- **Deterministic:** Fingerprint must be consistent across different machine
  reboots if the filesystem hasn't changed.

## Design

1. **Level 1: Root Scan:** Get the `mtime` of the `projects/` directory.
2. **Level 2: Project Metadata:** Iterate through all first-level subdirectories
   in `projects/`.
3. **Level 3: Structural Files:** For each project, stat the following
   "High-Value" files if they exist:
   - `Cargo.toml`, `Cargo.lock`
   - `package.json`, `package-lock.json`, `pnpm-lock.yaml`, `yarn.lock`
   - `go.mod`, `go.sum`, `go.work`
   - `pyproject.toml`, `requirements.txt`, `poetry.lock`
   - `README.md`, `README.markdown`, `readme.md`
   - `Justfile`, `.gitignore`
   - `.git/index` (Detects local commits/stages)
4. **Aggregation:** Combine all `mtime` values and the sorted list of project
   names into a single `u64` using a fast non-cryptographic hash (like `SeaHash`
   or simple XOR with rotation).
