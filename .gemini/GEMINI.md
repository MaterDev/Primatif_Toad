# Gemini Context: Primatif_Toad

## Identity & Purpose
- **Role:** This is **Primatif_Toad** (CLI: `toad`), the **Toad Control** system for local development.
- **Goal:** Provide a portable, Mac-agnostic control plane to manage, audit, and manipulate software projects in the `projects/` directory.
- **Protocol:** This project uses the **Conductor** extension for solo-dev orchestration.

---

## 🧠 System Brain (Conductor)
At the start of every session, you MUST read the **Conductor Index** to understand the current state, active tracks, and next steps.

1.  **Index:** `conductor/index.md` (Root of all knowledge)
2.  **Vision:** `conductor/product.md` (What we are building)
3.  **Architecture:** `conductor/tech-stack.md` (How we build it)
4.  **Workflow:** `conductor/workflow.md` (How we work & QA gates)
5.  **Tracks:** `conductor/tracks.md` (What's in progress)

---

## 🛠️ Operational Protocol (Universal File Resolution)
You must adhere to the **Universal File Resolution Protocol** (defined in `conductor/USER_GUIDE.md`) to manage tracks and state.

- **Solo-Dev Flow:** You are the **AI Project Manager**. Your job is to track atomic progress in `plan.md` files so the user doesn't have to manage the complexity.
- **Modular Codebase:** Strictly follow the crate-based architecture defined in `tech-stack.md`.
- **Quality Gates:** Never consider a task finished until `just fmt`, `just lint`, and `just test` pass.
- **Commit History:** Follow the structured, high-context format (Motivation, Implementation, Impact, Context) to ensure the Git history serves as a readable long-term memory for future AI agents.

## System Structure
- `bin/toad`: **The Interface.** Main CLI application (`toad`).
- `crates/`: **The Capabilities.** (Modular logic layers)
- `projects/`: **The Target.** Managed repositories (ignored by Git).
- `conductor/`: **The Orchestrator.** Project state, tracks, and orchestration rules.
- `shadows/`: **The Context.** AI-specific metadata and context maps (ignored by Git).