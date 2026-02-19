# Plan: Diagnostic Resilience (v1.1.2-diagnostic-resilience)

> **Spec:** [./spec.md](./spec.md)

---

## Tasks

### Phase 1: Core Detection

- [ ] Update strategy matching logic to differentiate between "No Match" and
      "Parse Failure".
- [ ] Store parsing errors in the `ProjectDetail` or a transient diagnostic
      cache.

### Phase 2: User Reporting

- [ ] Update `toad status` to show a ⚠️ symbol next to projects with malformed
      metadata.
- [ ] Add a "Metadata Health" section to `toad doctor`.

---

## Acceptance Criteria

- Intentionally breaking a `Cargo.toml` results in a clear warning in
  `toad doctor`.
- `toad status` reports the project as malformed rather than silently defaulting
  to "Generic".
