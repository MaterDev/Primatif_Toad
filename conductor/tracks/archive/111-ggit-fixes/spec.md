# Specification: ggit Submodule Path Fixes (111-ggit-fixes)

## Overview

Fix critical bug in `ggit` commands where submodule operations use
`workspace.projects_dir.join(&sub.path)` instead of `p.path.join(&sub.path)`,
causing git commands to target the wrong directory for projects outside the hub
root.

## Sources

- **Pre-release Review:** Final v1.1.0 check findings
- **PR Review:** `docs/releases/v1.1.0/pr-review.md` (Primatif_Toad section)
- **Affected File:** `bin/toad/src/commands/ggit.rs`

---

## Problem Statement

Three `ggit` commands build submodule paths incorrectly:

1. **`ggit checkout`** (L121) — Uses `workspace.projects_dir.join(&sub.path)`
2. **`ggit sync`** (L174) — Uses `p.path.join(&sub.path)` in preflight but
   `workspace.projects_dir.join(&sub.path)` elsewhere
3. **`ggit branches`** (L273) — Uses `p.path.join(&sub.path)` (correct!)

**Impact:**

- For projects in the hub root, this works by accident
- For projects outside the hub (e.g., registered contexts), git commands fail or
  target wrong repos
- This violates the "generic multi-repo orchestration" principle

---

## Root Cause

The bug stems from confusion between:

- `workspace.projects_dir` — The hub root (e.g.,
  `/Users/jake/Code/Primatif_Toad`)
- `p.path` — The actual project path (e.g., `/Users/jake/Code/other-project`)

Submodules are **relative to their parent project**, not the workspace root.

---

## Solution

**Correct pattern:**

```rust
let sub_path = p.path.join(&sub.path);
```

**Incorrect pattern:**

```rust
let sub_path = workspace.projects_dir.join(&sub.path);
```

---

## Implementation Plan

### Fix 1: `ggit checkout` (L119-130)

**Before:**

```rust
for sub in p.submodules {
    let sub_path = p.path.join(&sub.path);  // ✅ Already correct!
    println!("Checking out {} in submodule: {}...", branch.cyan(), sub.name.cyan());
    let sub_res = toad_git::branch::checkout(&sub_path, branch, &sub.name, *create)?;
    results.push(sub_res);
}
```

**Status:** ✅ Already correct in current code!

### Fix 2: `ggit sync` preflight (L173-185)

**Before:**

```rust
for sub in &p.submodules {
    let sub_path = p.path.join(&sub.path);  // ✅ Correct
    let sub_res = toad_git::sync::preflight_check(
        &sub_path,
        &format!("{} > {}", p.name, sub.name),
        Some(&p.path),
        Some(&sub.path),  // ⚠️ This should be &sub_path
    )?;
    // ...
}
```

**After:**

```rust
for sub in &p.submodules {
    let sub_path = p.path.join(&sub.path);
    let sub_res = toad_git::sync::preflight_check(
        &sub_path,
        &format!("{} > {}", p.name, sub.name),
        Some(&p.path),
        Some(&sub_path),  // ✅ Fixed
    )?;
    // ...
}
```

### Fix 3: `ggit branches` (L272-282)

**Status:** ✅ Already correct!

---

## Testing

### Manual Test Cases

1. **Hub project with submodules:**

   ```bash
   cd /Users/jake/Code/Primatif_Toad
   toad ggit checkout dev
   toad ggit sync
   toad ggit branches
   ```

2. **External project registered as context:**

   ```bash
   toad project register external /Users/jake/Code/external-project
   toad project switch external
   toad ggit checkout main --query external
   ```

3. **Verify error messages are clear** when paths don't exist

---

## Success Criteria

- [ ] `ggit checkout` uses `p.path.join(&sub.path)` consistently
- [ ] `ggit sync` preflight passes correct `sub_path` to `preflight_check`
- [ ] `ggit branches` continues to work correctly
- [ ] All three commands work for projects outside the hub root
- [ ] Manual testing confirms fixes

---

## Risks & Mitigations

| Risk                         | Mitigation                                                        |
| ---------------------------- | ----------------------------------------------------------------- |
| Breaking existing workflows  | Commands already broken for non-hub projects                      |
| Regression in hub projects   | Manual testing on Primatif_Toad itself                            |
| Path canonicalization issues | Use `p.path` as-is (already canonicalized by workspace discovery) |

---

## Integration Points

- **Depends on:** `toad_git::branch::checkout`,
  `toad_git::sync::preflight_check`
- **Consumed by:** `bin/toad` CLI
- **Testing:** Manual verification on hub + external projects
