# Spec: Toad Tag (Taxonomy)

## Overview

A tagging system allows developers to group projects by technology, status, or
category (e.g., "archive", "rust", "active") without renaming folders.

## Requirements

- **Local Registry:** Tags must be stored in a local JSON registry, not in the
  projects themselves.
- **Searchable:** `toad reveal` and `toad status` must support filtering by
  tags.
- **CLI Managed:** Commands to add, remove, and list tags for projects.

## Design

1. Store tags in `~/.toad/tags.json`.
2. Add `toad tag <project> <tag>` and `toad untag <project> <tag>` commands.
3. Update discovery logic to merge tags into project metadata.
