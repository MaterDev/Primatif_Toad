// SPDX-License-Identifier: BUSL-1.1
//! Safety guardrails for destructive bulk operations.

/// Checks if a command string contains potentially destructive patterns.
///
/// Returns `true` if the command is considered "dangerous".
pub fn is_destructive(command: &str) -> bool {
    // 1. Check generic destructive patterns
    let dangerous_patterns = ["rm ", "delete", "force", "prune", "drop ", "truncate"];

    let cmd_lower = command.to_lowercase();
    if dangerous_patterns
        .iter()
        .any(|pattern| cmd_lower.contains(pattern))
    {
        return true;
    }

    // 2. Delegate Git-specific checks to the pulse layer
    toad_git::safety::is_destructive_git_command(command)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_destructive() {
        assert!(is_destructive("rm -rf ."));
        assert!(is_destructive("git reset --hard HEAD"));
        assert!(is_destructive("GIT PUSH -F ORIGIN"));
        assert!(!is_destructive("git pull"));
        assert!(!is_destructive("ls -la"));
    }
}
