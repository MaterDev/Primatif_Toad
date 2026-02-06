# Spec: Toad Sync (Registry Cache)

## Overview

Scanning the filesystem for 100+ projects on every CLI call is inefficient. This
track implements a cached registry to make discovery instant.

## Requirements

- **Performance:** `toad reveal` should become O(1) or O(log N) lookup.
- **Resilience:** The cache must be able to detect "Ghost" projects (deleted
  from disk) and trigger a re-sync.
- **Manual Control:** Explicit `toad sync` command to force a refresh.

## Design

1. Store project metadata in `~/.toad/registry.json`.
2. CLI checks `registry.json` first.
3. If registry is missing or stale (fingerprint mismatch), trigger background or
   foreground sync.
