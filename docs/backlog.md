# Toad Control Backlog

These 5 features are identified as **Essential** to evolving Primatif_Toad from a scaffolding tool into a comprehensive Control Plane.

## 1. Toad Status (The Health Check)
**Command:** `toad status [--all | --query <str>]`
- **Description:** Scans projects to report their Git status.
- **Benefit:** Instantly answer "Which of my 70 projects have uncommitted changes?" without manually checking each one.
- **Implementation:** Extend `crates/git-ops` to use `git2` or shell out to check for dirty working trees.

## 2. Toad Do (Bulk Operations)
**Command:** `toad do <command> --query <str>`
- **Description:** Execute a shell command across multiple projects matching a query.
- **Benefit:** Massive time saver for maintenance.
    - *Example:* `toad do "git pull" --query primatif` (Update all Primatif repos).
    - *Example:* `toad do "npm install" --query web` (Update dependencies).

## 3. Toad Clean (Disk Hygiene)
**Command:** `toad clean [--dry-run]`
- **Description:** deeply cleans project artifacts to free up disk space.
- **Benefit:** Rust `target/` and Node `node_modules/` folders can take up gigabytes. This command would find and remove them in inactive projects.
- **Implementation:** `crates/maintenance` logic to identify build artifacts.

## 4. Toad Tag (Taxonomy)
**Command:** `toad tag <project> <tag>` / `toad reveal --tag <tag>`
- **Description:** A lightweight tagging system (using a local JSON registry) to categorize projects beyond just their names.
- **Benefit:** Allows grouping projects by technology ("rust", "react") or status ("active", "archive") regardless of their folder name.

## 5. Toad Sync (Registry Cache)
**Command:** `toad sync`
- **Description:** explicitly scans the filesystem and builds/updates a local `registry.json` cache.
- **Benefit:**
    - **Speed:** `toad reveal` becomes instant (O(1) lookup vs O(N) disk scan).
    - **Resilience:** Detects "Ghost" projects (in registry but deleted from disk) and cleans them up.
