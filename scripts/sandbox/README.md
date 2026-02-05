# Toad Sandbox Generator 🐸

This utility generates repeatable, high-fidelity mock project environments for
testing and verifying bulk operations within the Toad Control Plane.

## Purpose

Bulk operations like `toad do` are powerful and potentially destructive. This
generator allows developers to test complex shell commands in a safe, isolated
directory before running them against production repositories.

## Usage

Run the main script from the `scripts/sandbox` directory:

```bash
./mksandbox.sh [options]
```

### Options

- `-c, --count N`: Total number of mock projects to generate (Default: 10).
- `-o, --output DIR`: Name of the parent directory for the sandbox (Default:
  `test_sandbox`).
- `-s, --stacks LIST`: Comma-separated list of tech stacks to include.
  Supported: `rust`, `node`, `go`, `generic`.
- `-d, --depth N`: Nesting depth of the project files. Useful for testing
  recursive discovery (Default: 1).
- `-t, --tar`: Automatically archive the resulting sandbox into a `.tar.gz`
  file.

### Examples

**1. Fast local test:**

```bash
./mksandbox.sh -c 5 -o my_test
```

**2. High-volume tech mix:**

```bash
./mksandbox.sh --count 100 --stacks "rust,node" --depth 2 --tar
```

## Internal Structure

- `mksandbox.sh`: Main entry point and argument parser.
- `parts/project_gen.sh`: Core logic for generating individual project files
  based on tech stack.
- `parts/archive_gen.sh`: Logic for tarball compression.

---

_Hop safely, little toads!_ 🐸
