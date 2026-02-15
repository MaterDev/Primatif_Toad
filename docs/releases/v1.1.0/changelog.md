# Changelog: v1.1.0 "Deep Croak"

**Release Date:** 2026-02-15

## Overview

v1.1.0 "Deep Croak" transforms Toad from a metadata scanner into a portable, AI-native **Context Oracle**. This release introduces the Model Context Protocol (MCP), deep structural intelligence (DNA mapping), and tiered context engineering to provide AI agents with zero-latency vision across complex multi-repo ecosystems.

---

## High-Level Summaries

### 🔌 Model Context Protocol (MCP)
Implemented a full MCP server (`toad-mcp`) that exposes Toad's intelligence directly to AI agents. 
- **12+ Specialized Tools:** Project listing, DNA extraction, semantic search, context switching, and ecosystem health.
- **Standardized Discovery:** Enables seamless integration with Cursor, Windsurf, and other MCP-compatible IDEs.

### 🧬 Pattern Intel & DNA Mapping
Toad now performs deep structural analysis to identify the "DNA" of a project.
- **Role Detection:** Automatically identifies "Data Layer", "API Surface", "CLI", and "Tests".
- **Capability Mapping:** Detects features like "Dockerized", "Async/Tokio", "Serialization", and framework usage.
- **Ecosystem Indexing:** Generates `ATLAS.json` for rapid cross-project pattern matching.

### 🌊 Context Engineering
Introduced a tiered metadata architecture for progressive disclosure of context.
- **Tiered Prompts:** `llms.txt` → `SYSTEM_PROMPT.md` → `MANIFEST.md`.
- **Architectural Blueprints:** Generates high-fidelity component graphs and dependency maps.
- **Project Briefings:** Enhanced `CONTEXT.md` with entry points, lifecycle data, and operational intelligence.

### 🛡️ Multi-Repo & Core Hardening
Refined the foundation for robust performance across heterogeneous filesystems.
- **Group-Git (ggit) Evolution:** Atomic commits, multi-repo status, and safe submodule alignment.
- **VCS Safety Guardrails:** Restored preflight checks for unpushed commits and SHA drift.
- **Cross-Platform Migration:** Safe migration of legacy Toad artifacts across volume boundaries (Unix & Windows).

### 🤖 AI Vendor Expansion
Toad now speaks the language of the entire modern AI landscape.
- **10+ Vendors Supported:** Native skill distribution for Windsurf, Cursor, Copilot, Cline, PearAI, and more.
- **Custom Mapping:** Support for `vendor:path` specifications for tailor-made AI environments.

---

## Detailed Changes

### toad-core
- **Workspace Discovery:** Fixed `TOAD_ROOT` priority and standardized env var discovery tiers.
- **Path Resolution:** Ensured `safe_canonicalize` returns absolute paths against CWD.
- **Migration:** Implemented `move_xdev` with symlink preservation for cross-device migrations.

### toad-git
- **Status Reporting:** Corrected `Dirty` vs `Untracked` priority mapping.
- **Preflight Checks:** Added detection for unpushed commits and submodule SHA mismatch.
- **Divergence API:** Refactored `has_unmerged_changes` to check upstream ahead/behind status.

### toad-ops
- **Performance:** Replaced busy-loops with `wait-timeout` for efficient task orchestration.
- **Skill Distribution:** Expanded vendor list and added custom path mapping support.
- **Safety:** Added underflow protection to project compatibility scoring.

### toad-discovery
- **DNA Detection:** Fixed submodule path building for accurate metadata extraction.
- **Project Scanning:** Refined Hub/Pond detection and eliminated duplicate submodule results.

### toad-manifest
- **Metadata Generation:** Enhanced project-level briefings with entry points and development lifecycle info.
- **Architectural Vision:** Improved blueprint generation for complex dependency graphs.

---

## 🐸 The Vision: "Deep Croak"
With v1.1.0, Toad moves beyond simple file tracking. It now understands *what* a project is, *how* it works, and *where* it fits in the larger ecosystem, making it the definitive source of truth for AI-native software engineering.
