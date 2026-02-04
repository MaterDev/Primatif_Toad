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
- **Finalization:** Update the `.gemini/GEMINI.md` "Receipt for current session"
  if the project architecture changed.
