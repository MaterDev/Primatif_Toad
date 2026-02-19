# QA Report: Primatif_Toad v1.1.1

## Executive Summary

This QA session validated the core functionality of the `toad` CLI and
`toad-mcp` integration. Most features are working as expected, with a few
usability quirks identified in the `tag` and `create` commands. The MCP server
is functioning correctly and providing accurate ecosystem context.

## Feature Matrix

| Feature Category    | Status   | Notes                                                              |
| :------------------ | :------- | :----------------------------------------------------------------- |
| **Discovery**       | ✅ Pass  | `list`, `reveal`, `status` work correctly.                         |
| **Analysis**        | ✅ Pass  | `analyze` commands provide deep insights.                          |
| **Management**      | ⚠️ Issues | `tag` and `create` require explicit flags for non-interactive use. |
| **MCP Integration** | ✅ Pass  | All tested tools return valid JSON data.                           |
| **System Health**   | ✅ Pass  | `doctor` and license checks pass.                                  |

## Critical Issues & Observations

### Usability

1. **Non-Interactive Flags:** The `toad tag` command requires `--yes` to bypass
   confirmation, which is not immediately obvious from the error message.
   Similarly, `toad create` uses `--yes` instead of `--no-interactive`.
   - _Recommendation:_ Standardize on a single flag (e.g., `--yes` or `--force`)
     for all commands and update error messages to suggest it.
2. **Context Switching:** Switching contexts is seamless, but deleting a context
   requires manual confirmation even when scripting, unless a force flag is
   implemented (needs verification).
3. **Command Discovery:** `toad project unregister` was a reasonable guess but
   incorrect; `toad project delete` is the actual command. An alias or "did you
   mean" suggestion for `unregister` would be helpful.

### Performance

- **Manifest Generation:** `toad manifest` is reasonably fast but can slow down
  with large repositories.
- **Status Checks:** `toad status` is efficient.

## Regression Test Plan (Future)

To prevent regressions, we should implement the following automated tests:

1. **CLI Integration Tests:** A script that runs through the "Test Suite 3"
   scenario automatically, using the `qa_sandbox`.
2. **MCP E2E Tests:** A test suite that queries the MCP server and validates the
   JSON schema of the responses.
3. **Help Text Validation:** Ensure all commands have clear, consistent help
   text and examples.

## Deep-Dive Findings (Phase 2)

### 1. Skill & Strategy Engines

- **Skill Sync:** `toad skill sync` is highly robust. It successfully
  regenerates the blueprint, CLI docs, and MCP reference and distributes them to
  the AI vendor slots.
- **Strategy Inspection:** `toad strategy list` and `info` work as intended. The
  priority-based detection logic is transparent and verifiable.

### 2. Error Resilience & "Silent Failures"

- **Metadata Corruption:** During error injection (invalid TOML), Toad did not
  crash. Instead, it gracefully degraded the project to a "Generic" stack.
- **Observation:** While graceful degradation prevents system crashes, it can
  lead to "silent failures" where a Rust project is no longer recognized as
  such, and features like `artifact removal` (target dir) stop working for that
  project.
- _Recommendation:_ Add a "Parsing Error" warning to `toad doctor` or
  `toad status` when a strategy-matching file (like `Cargo.toml`) exists but
  cannot be parsed.

### 3. Submodule Awareness

- **Context Refresh:** The "Intuition is stale" auto-refresh logic is triggered
  frequently and works reliably to keep the registry in sync with the
  filesystem.

## Conclusion

Release v1.1.1 is stable and ready for use. The identified usability issues are
minor and can be addressed in future updates. The robust analysis capabilities
are a significant asset.
