# Specification: Governance & AI Navigation (v1.0.2 Phase 6)

> **Source of Truth:** This track is derived from the release planning
> documents. All design decisions and task details live there.
>
> - Design: `docs/releases/v1.0.2/evolution.md` § Governance, § Cross-Repo Context Map
> - Tasks: `docs/releases/v1.0.2/tasks.md` § Phase 6

## Overview
Implement the automated enforcement of license boundaries and generate the cross-repo context maps necessary for AI agents to navigate the distributed architecture.

## Requirements
- `toad manifest` extension to generate `CROSS_REPO_MAP.md` with dependency graph, type flow, call chains, crate responsibilities, and license boundaries.
- Automated detection of MIT → BUSL-1.1 dependency violations.
- Update `.gemini/GEMINI.md` with licensing architecture section.
- Update `conductor/tech-stack.md` with license annotations on all crates.
- Update `conductor/product-guidelines.md` with licensing & architecture boundaries.
- **Critical:** Register `CROSS_REPO_MAP.md` in `.gemini/settings.json` so every new AI chat session starts with the inter-repo dependency graph.
- Validate AI agent navigation using only the cross-repo map.

## References
- [Evolution Doc](../../../docs/releases/v1.0.2/evolution.md) — § Governance, § Cross-Repo Context Map
- [Task Breakdown](../../../docs/releases/v1.0.2/tasks.md) — § Phase 6
