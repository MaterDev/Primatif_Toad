# Spec: Toad Tag (Taxonomy)

## Overview

A tagging system allows developers to group and filter projects by technology,
status, or custom categories without modifying the project's filesystem
structure. This provides a multi-dimensional way to manage a large project
ecosystem.

## Requirements

1. **Local Metadata Registry:** Store tags in a central `tags.json` file within
   the `shadows/` directory (not inside the projects).
2. **Simple CLI API:**
   - `toad tag <project> <tag>`: Assign a tag.
   - `toad untag <project> <tag>`: Remove a tag.
3. **Ubiquitous Filtering:** Support a `--tag <name>` flag across `status`,
   `reveal`, `do`, and `stats`.
4. **Auto-Harvesting:** Automatically detect and apply stack-based tags (e.g.,
   `#rust`, `#node`, `#go`) during discovery.
5. **Visual Integration:** Display tags in project lists and summaries with a
   consistent `#tag` prefix.

## Design

- **Storage:** `shadows/tags.json` mapping project names to sets of tags.
- **Model:** Update `ProjectDetail` in `toad-core` to include a
  `tags: Vec<String>` field.
- **Persistence:** Implement a `TagRegistry` struct in `toad-core` with
  load/save capabilities.
- **Discovery:** Merge persistent tags from `tags.json` with procedurally
  detected hashtags in `discovery::scan_all_projects`.
