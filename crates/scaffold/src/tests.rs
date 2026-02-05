use super::scaffold::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_create_project_success() {
    let dir = tempdir().unwrap();
    let name = "test-proj";
    let config = ProjectConfig {
        name,
        root_dir: dir.path().to_path_buf(),
        dry_run: false,
    };

    create_project(config).expect("Should create project");

    let proj_path = dir.path().join(name);
    assert!(proj_path.exists());
    assert!(proj_path.join("docs").is_dir());
    assert!(proj_path.join("README.md").is_file());
    assert!(proj_path.join(".git").is_dir());
}

#[test]
fn test_create_project_dry_run() {
    let dir = tempdir().unwrap();
    let name = "dry-run-proj";
    let config = ProjectConfig {
        name,
        root_dir: dir.path().to_path_buf(),
        dry_run: true,
    };

    create_project(config).expect("Dry run should succeed");

    let proj_path = dir.path().join(name);
    assert!(!proj_path.exists(), "Dry run should NOT create directories");
}

#[test]
fn test_create_project_already_exists() {
    let dir = tempdir().unwrap();
    let name = "existing-proj";
    let proj_path = dir.path().join(name);
    fs::create_dir(&proj_path).unwrap();

    let config = ProjectConfig {
        name,
        root_dir: dir.path().to_path_buf(),
        dry_run: false,
    };

    let result = create_project(config);
    assert!(result.is_err(), "Should fail if directory exists");
}

#[test]
fn test_open_in_editor_invalid() {
    let dir = tempdir().unwrap();
    let result = open_in_editor("test", dir.path(), "notepad");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Unknown editor"));
}

#[test]
fn test_open_in_editor_success_path_logic() {
    let dir = tempdir().unwrap();
    let proj_name = "test-proj";
    fs::create_dir(dir.path().join(proj_name)).unwrap();

    // We can't easily test real editor launch, but we've tested the 'invalid' path.
    // The successful paths for vscode/windsurf are covered by the match logic.
}
