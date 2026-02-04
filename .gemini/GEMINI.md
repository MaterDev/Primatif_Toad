# Gemini Context: Code Root

## Identity & Purpose
- **Role:** This is the root `Code` directory for user `jakehclark`.
- **Goal:** Manage local development environment, scripts, and context without interfering with individual project repositories.
- **Strategy:** "Overlay" Git repository. The root Git repo ignores all subdirectories by default (`*`), whitelisting only admin tools (`scripts/`, `work_orders/`, `GEMINI.md`).

## System Structure
- `scripts/`: Shared "DevOps" scripts for local management.
- `work_orders/`: Plans and logs for major structural changes or complex tasks.
- `GEMINI.md`: This context file.

## Conventions
- **No Conflict:** Do not track sub-repositories. Use strict `.gitignore` rules.
- **Safety:** Always verify paths before moving or deleting.
