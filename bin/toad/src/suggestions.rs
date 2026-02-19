use std::collections::HashMap;

/// Command alias mappings for common user guesses
pub fn get_command_aliases() -> HashMap<&'static str, &'static str> {
    let mut aliases = HashMap::new();

    // Common aliases
    aliases.insert("unregister", "delete");
    aliases.insert("remove", "delete");
    aliases.insert("rm", "delete");
    aliases.insert("del", "delete");
    aliases.insert("ls", "list");
    aliases.insert("search", "reveal");
    aliases.insert("find", "reveal");
    aliases.insert("check", "status");
    aliases.insert("health", "doctor");
    aliases.insert("diagnose", "doctor");
    aliases.insert("cleanup", "clean");
    aliases.insert("purge", "clean");
    aliases.insert("git", "ggit");
    aliases.insert("workflow", "cw");
    aliases.insert("workflows", "cw");

    aliases
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    #[allow(clippy::needless_range_loop)]
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                0
            } else {
                1
            };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }

    matrix[len1][len2]
}

/// Suggest similar commands based on Levenshtein distance
pub fn suggest_command(input: &str, valid_commands: &[&str]) -> Option<String> {
    let input_lower = input.to_lowercase();

    // Check aliases first
    let aliases = get_command_aliases();
    if let Some(canonical) = aliases.get(input_lower.as_str()) {
        return Some(format!("Did you mean '{}'?", canonical));
    }

    // Find closest match using Levenshtein distance
    let mut best_match: Option<(&str, usize)> = None;

    for &cmd in valid_commands {
        let distance = levenshtein_distance(&input_lower, cmd);

        // Only suggest if distance is small (likely a typo)
        if distance <= 2 {
            if let Some((_, best_dist)) = best_match {
                if distance < best_dist {
                    best_match = Some((cmd, distance));
                }
            } else {
                best_match = Some((cmd, distance));
            }
        }
    }

    best_match.map(|(cmd, _)| format!("Did you mean '{}'?", cmd))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("cat", "hat"), 1); // One substitution: c->h
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("", "test"), 4);
        assert_eq!(levenshtein_distance("same", "same"), 0);
    }

    #[test]
    fn test_command_aliases() {
        let aliases = get_command_aliases();
        assert_eq!(aliases.get("unregister"), Some(&"delete"));
        assert_eq!(aliases.get("ls"), Some(&"list"));
        assert_eq!(aliases.get("search"), Some(&"reveal"));
    }

    #[test]
    fn test_suggest_command() {
        let valid = vec!["status", "stats", "reveal", "sync", "clean"];

        // Alias match
        assert!(suggest_command("search", &valid).is_some());

        // Typo match
        assert!(suggest_command("statu", &valid).unwrap().contains("status"));
        assert!(suggest_command("revael", &valid)
            .unwrap()
            .contains("reveal"));

        // No match for very different input
        assert!(suggest_command("xyz", &valid).is_none());
    }
}
