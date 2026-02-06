# Spec: Unified Taxonomy & Manifest Refactor

## Overview
Current metadata detection is redundant and fragmented across the discovery engine and the manifest generator. This track collapses all categorical data into a single "Ingredients" taxonomy based on raw evidence found on the disk and custom user tags.

## Requirements
1.  **Unified Model:** Replace `ProjectStack` and `hashtags` in `ProjectDetail` with a single `taxonomy: Vec<String>`.
2.  **Exhaustive Discovery:** Refactor `detect_stack` to `detect_ingredients`, which runs every strategy and collects all matches (no more early returns).
3.  **SSOT (Single Source of Truth):** The `toad-manifest` crate must be a "dumb reporter"—it simply formats the taxonomy provided by the discovery engine.
4.  **Preserved Features:** Keep extractive `essence` (README lines), fingerprinting, and table layout in the manifest.
5.  **Multi-Stack Support:** Correcty report hybrid projects (e.g., a project with both `Cargo.toml` and `package.json` gets both `#rust` and `#node` tags).

## Design
- **Core:** Update `ProjectDetail` struct.
- **Discovery:** Change strategies to return `Vec<String>` ingredients.
- **Manifest:** Simplify table generation to use the unified taxonomy column.
