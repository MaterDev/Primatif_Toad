# How to use Conductor (Solo-Dev Guide)

Conductor is your **AI Project Manager**. It manages the complexity of the
project so you can focus on writing code.

## 1. Starting a Session

When you start a session with Gemini, simply say:

> "Read the conductor index and tell me what's next."

Gemini will read `conductor/index.md`, find the active track, and look at the
`plan.md` to give you your immediate next step.

## 2. Creating a New Feature (A "Track")

When you have a new idea:

1. Ask Gemini to "Initialize a new Track for [Feature Name]."
2. Gemini will:
   - Add it to `conductor/tracks.md`.
   - Create a directory `conductor/tracks/[id]/`.
   - Create a `spec.md` (Design) and `plan.md` (Checklist).
3. Review the `spec.md` to ensure the AI understood your vision.

## 3. Working on a Track

As you work:

- The AI will use the `plan.md` as its to-do list.
- After every successful change, the AI will update the `plan.md` with `[x]` for
  completed tasks.
- If you get interrupted, the AI will leave a "Next Step" note in the `plan.md`.

## 4. Complexity Management

If a task feels too big, ask Gemini to "Refine the plan." It will break down the
current track into smaller, more manageable sub-tasks.

## 5. Definition of Done

A track is finished when:

- All items in `plan.md` are checked.
- `just` quality gates pass.
- A high-context commit is made referencing the Track ID.
- The Track is moved to "Archived/Completed" in `tracks.md`.

## 6. Keeping the Brain Sharp

Periodically ask Gemini to "Verify the Tech Stack and Workflow." It will audit
the current codebase against your established rules and suggest fixes if things
are getting messy.

## 7. Disk Hygiene (Cleaning the Pond)

As your ecosystem grows, build artifacts (like `node_modules`, `target/`, or
`.venv`) can consume significant disk space. Toad provides a safe, data-driven
way to reclaim this space.

### Cleaning Projects

- `toad clean`: Scans projects and identifies build artifacts based on their
  detected stack (e.g., `target` for Rust).
- `toad clean --dry-run`: Preview what will be deleted and how much space will
  be reclaimed without actually removing any files.
- `toad clean --tier cold`: Clean only projects that haven't been touched in
  over 7 days.
- `toad clean --yes`: Skip the confirmation prompt (use with caution!).

Toad includes a "Reserved Paths" safety layer that prevents it from ever
cleaning critical files like `.git`, `src`, or configuration manifests, even if
a custom strategy is misconfigured.
