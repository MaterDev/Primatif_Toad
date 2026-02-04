# Operational Conventions (State Machine)

To ensure consistency across multiple AI sessions, we use a structured Work
Order system as our "State Machine."

## 1. Work Order Lifecycle

Every task must be tracked in `docs/work_orders/`.

- **Status: Pending** - Not started.
- **Status: In-Progress** - Currently being worked on by an agent.
- **Status: Completed** - Verified and finished.

## 2. Session Protocol

When starting a new session, the agent should:

1. Read `docs/work_orders/` to find the latest "In-Progress" or the next
   "Pending" task.
2. Announce which Work Order it is adopting.
3. Upon completion, update the Work Order to `Status: Completed` and list the
   changes made.

## 3. Tooling Workflow

- **Pre-check:** Run `just fmt` or `just lint` before starting.

- **Verification:** Every logic change requires running `just test`.

- **Documentation Integrity:** Before marking a task as "Completed" or proposing
  a commit, you MUST:

  1. Update `README.md` if the user-facing CLI or architecture changed.

  2. Update `.gemini/GEMINI.md` if the architectural rules or context changed.

  3. Ensure the "Success Criteria" in the Work Order are physically verified.

- **Finalization:** Update the `.gemini/SESSIONS.md` with a fresh receipt if
  significant progress was made.

## 4. Modular Development Rules

- **Decoupling:** New logic must live in a dedicated crate in `crates/` if it
  serves a distinct purpose (e.g., `toad-ai`, `toad-git`).

- **Core Dependency:** All platform crates must depend on `toad-core` for shared
  types.

- **Thin CLI:** The `bin/toad` package must remain a thin orchestration layer.
  Logic belongs in crates.

- **Strategy Pattern:** Use traits/strategies for extensible features like stack
  detection or agentic prompts.

- **Test Separation:** Implementation files should stay lean. Move unit tests to
  a separate module or file (e.g., `mod tests;` in `lib.rs` with logic in
  `src/tests.rs`).

## 5. Session Management

- **Persistence:** Significant milestones must be recorded in
  `.gemini/SESSIONS.md`.

- **Formatting:** Entries must be dated (YYYY-MM-DD) and include a clear
  **Goal**, **Accomplishments**, and **Next Steps**.

- **Context Pruning:** To maintain high token efficiency, the active log should
  only contain recent context. Move older entries to a `docs/archive/sessions/`
  directory once the log exceeds 10 entries.

- **Auto-Loading:** The `SESSIONS.md` file must be whitelisted in
  `.gemini/settings.json` to ensure immediate agent awareness.
