#[cfg(test)]
mod tests {
    use super::super::generate_markdown;
    use std::path::PathBuf;
    use toad_core::{ActivityTier, ProjectDetail, ProjectStack, VcsStatus};

    #[test]
    fn test_generate_markdown() {
        let projects = vec![ProjectDetail {
            name: "test-proj".to_string(),
            path: PathBuf::from("projects/test-proj"),
            stack: ProjectStack::Rust,
            activity: ActivityTier::Active,
            vcs_status: VcsStatus::Clean,
            essence: Some("Test essence".to_string()),
            hashtags: vec!["#test".to_string()],
            sub_projects: Vec::new(),
        }];

        let md = generate_markdown(&projects, 12345);
        assert!(md.contains("**Fingerprint:** `12345`"));
        assert!(md.contains(
            "| **`test-proj`** | `Rust` | 🔥 Active | ✅ Clean | Test essence | `[#test]` |"
        ));
    }
}
