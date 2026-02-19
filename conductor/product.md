# Product Definition: Primatif Toad

## Vision

**Primatif Toad** is an AI-Native Local-Ops Platform and Developer CLI designed
to manage a modular ecosystem of independent projects. It acts as a **Context
Oracle**, transforming raw repository data into high-fidelity, machine-readable
metadata for AI agents (Gemini, Claude, local LLMs).

## Core Goals

1. **Global Context Architecture:** Decouple Toad metadata from the user's
   source repositories. All context lives in a central `~/.toad/` home for total
   portability.
2. **Context Engineering:** Generate high-density, tiered context
   (`MANIFEST.md`, `context.json`, `llms.txt`, `AGENTS.md`) specifically
   optimized for AI context windows.
3. **Live Oracle (MCP):** Provide a Model Context Protocol (MCP) server so
   agents can query ecosystem health and patterns in real-time.
4. **Multi-Repo Orchestration:** Provide first-class group-git operations
   (`toad ggit`) and parallel execution (`toad do`) across distributed submodule
   ecosystems.
5. **Structural DNA Mapping:** Automatically detect architectural patterns (Data
   Layer, API Surface, Auth Patterns) to facilitate cross-repo synthesis.
6. **Named Project Contexts:** Switch between isolated workspace roots (Hub or
   Pond modes) with zero context drift.
7. **Quality & Safety:** Centralize quality gates and enforce transactional
   safety across all managed codebases.

## Core Roles

- **Toad status:** Health checks across the ecosystem.
- **Toad do:** Bulk shell operations across multiple projects.
- **Toad reveal:** Deep discovery of project capabilities.
- **Toad manifest:** Context generation for AI orchestration.
- **Scaffold:** Standardized project bootstrapping.
