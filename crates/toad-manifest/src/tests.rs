use super::generate_markdown;
use std::path::PathBuf;
use toad_core::{ActivityTier, ProjectDetail, ProjectStack, VcsStatus};

#[test]
fn test_generate_markdown_basic() {
    let projects = vec![ProjectDetail {
        name: "test-proj".to_string(),
        path: PathBuf::from("projects/test-proj"),
        stack: ProjectStack::Rust,
        activity: ActivityTier::Active,
        vcs_status: VcsStatus::Clean,
        essence: Some("Test essence".to_string()),
        hashtags: vec!["#test".to_string()],
        tags: vec!["#test".to_string()],
        sub_projects: Vec::new(),
    }];

    let md = generate_markdown(&projects, 12345);
    assert!(md.contains("**Fingerprint:** `12345`"));
    assert!(md.contains(
        "| **`test-proj`** | `Rust` | 🔥 Active | ✅ Clean | Test essence | `[#test]` |"
    ));
}

#[test]
fn test_generate_markdown_empty() {
    let md = generate_markdown(&[], 0);
    assert!(md.contains("**Fingerprint:** `0`"));
    assert!(md.contains("| Project | Stack | Activity | VCS | Essence (Extractive) | Hashtags |"));
}

#[test]
fn test_generate_markdown_escaping() {
    let projects = vec![ProjectDetail {
        name: "escape-test".to_string(),
        path: PathBuf::from("projects/escape-test"),
        stack: ProjectStack::Generic,
        activity: ActivityTier::Archive,
        vcs_status: VcsStatus::None,
        essence: Some("Pipe | [Link] https://google.com".to_string()),
        hashtags: vec![],
        tags: vec![],
        sub_projects: Vec::new(),
    }];

    let md = generate_markdown(&projects, 999);
    // Pipe should be escaped
    assert!(md.contains("Pipe \\|"));
    // Brackets should be escaped
    assert!(md.contains("\\[Link\\]"));
    // URL should be escaped
    assert!(md.contains("https:\\\\/google.com"));
    // No hashtags case
    assert!(md.contains("Pending harvest..."));
}

#[test]
fn test_generate_markdown_truncation() {
    let long_essence = "A".repeat(200);
    let projects = vec![ProjectDetail {
        name: "long-proj".to_string(),
        path: PathBuf::from("projects/long-proj"),
        stack: ProjectStack::Go,
        activity: ActivityTier::Cold,
        vcs_status: VcsStatus::Dirty,
        essence: Some(long_essence),
        hashtags: vec!["#go".to_string()],
        tags: vec!["#go".to_string()],
        sub_projects: Vec::new(),
    }];

    let md = generate_markdown(&projects, 1);
    // Should be truncated at 100 chars (actually 97 + "...")
    assert!(md.contains("..."));
    // Find the essence part in the row
    let line = md.lines().find(|l| l.contains("long-proj")).unwrap();
    let parts: Vec<&str> = line.split('|').collect();
    let essence_part = parts[5].trim();
    assert_eq!(essence_part.len(), 100);
    assert!(essence_part.ends_with("..."));
}
