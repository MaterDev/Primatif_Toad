// SPDX-License-Identifier: BUSL-1.1
//! VCS-specific safety guardrails.

/// Checks if a command string contains potentially destructive Git patterns.
pub fn is_destructive_git_command(command: &str) -> bool {
    let dangerous_git_patterns = [
        "reset --hard",
        "push -f",
        "push --force",
        "clean -f",
        "clean -df",
        "clean -xdf",
    ];

    let cmd_lower = command.to_lowercase();
    dangerous_git_patterns
        .iter()
        .any(|pattern| cmd_lower.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_destructive_git() {
        assert!(is_destructive_git_command("git reset --hard HEAD"));
        assert!(is_destructive_git_command("GIT PUSH -F ORIGIN"));
        assert!(is_destructive_git_command("git clean -xdf"));
        assert!(!is_destructive_git_command("git pull"));
        assert!(!is_destructive_git_command("git checkout main"));
    }
}
