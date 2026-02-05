# Product Guidelines: Primatif Toad

## 1. Visual Identity & Aesthetic

Primatif Toad follows a **Retro-Atari / Cybernetic** aesthetic. The goal is to
provide a "terminal arcade" feel that makes development feel tactile and
high-stakes.

- **Primary Color:** "Toad Green" (Standard ANSI Green).
- **Art Style:** High-block ASCII art for banners and major status changes.
- **Tone:** Technical, direct, and slightly futuristic.

## 2. CLI Design Standards

To maintain the "Solo-Dev Smooth Flow," the CLI must be its own best manual.

- **Self-Documentation:** Every subcommand must implement a robust `--help`
  flag.
- **Discovery:** The `toad list` command is the central directory for all
  capabilities.
- **Feedback:** Success and error states should be visually distinct (e.g.,
  using block characters or specific ASCII patterns).

## 3. Documentation & Context Strategy

Documentation is not just for humans; it is the "Long-term Memory" for AI
agents.

- **Single Source of Truth:** All project standards, workflows, and technical
  specs live in the `conductor/` directory.
- **Track-Based History:** Major features are documented as "Tracks" with
  specific implementation plans that act as a live state machine.
- **Context Integrity:** Employs multi-level mtime fingerprinting to ensure the
  AI's view of the `projects/` directory is always synchronized with the actual
  filesystem.

## 4. Engineering Tone

- **Concise:** Favor high information density over conversational filler.
- **Action-Oriented:** Documentation and CLI output should focus on "What's
  next?" and "How do I fix this?".
