use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::tempdir;

#[test]
fn test_version() {
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("TOAD CONTROL"));
}

#[test]
fn test_list() {
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage: toad <COMMAND>"));
}

#[test]
fn test_reveal_no_projects() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::create_dir(dir.path().join("projects"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("reveal")
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains("No projects found."));
    Ok(())
}

#[test]
fn test_reveal_with_project() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("my-cool-project"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("reveal")
        .arg("cool")
        .assert()
        .success()
        .stdout(predicate::str::contains("- my-cool-project"));
    Ok(())
}

#[test]
fn test_status_mixed() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    // 1. Clean Project
    let clean_path = projects_dir.join("clean-proj");
    fs::create_dir(&clean_path)?;
    StdCommand::new("git")
        .arg("init")
        .current_dir(&clean_path)
        .output()?;

    // 2. Dirty Project
    let dirty_path = projects_dir.join("dirty-proj");
    fs::create_dir(&dirty_path)?;
    StdCommand::new("git")
        .arg("init")
        .current_dir(&dirty_path)
        .output()?;
    fs::write(dirty_path.join("README.md"), "hello")?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("status")
        .assert()
        .success()
        .stdout(predicate::str::contains("01/2 projects are CLEAN"))
        .stdout(predicate::str::contains("1 projects have NEW FILES"))
        .stdout(predicate::str::contains("dirty-proj"));
    Ok(())
}

#[test]
fn test_do_dry_run_ish() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("proj-a"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    // Use -y to skip confirmation
    cmd.current_dir(dir.path())
        .arg("do")
        .arg("echo 'running'")
        .arg("-q")
        .arg("proj-a")
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::contains("Processing proj-a... OK"))
        .stdout(predicate::str::contains("1 Succeeded"));
    Ok(())
}

#[test]
fn test_do_multiple_parallel() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    for i in 0..10 {
        fs::create_dir(projects_dir.join(format!("proj-{}", i)))?;
    }

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("do")
        .arg("echo 'hi'")
        .arg("-q")
        .arg("proj")
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::contains("10 Succeeded"));
    Ok(())
}

#[test]
fn test_docs() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("docs")
        .assert()
        .success()
        .stdout(predicate::str::contains("SUCCESS: Documentation updated"));

    assert!(dir.path().join("docs").join("CLI.md").exists());
    Ok(())
}

#[test]
fn test_manifest() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::create_dir(dir.path().join("projects"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("manifest")
        .assert()
        .success()
        .stdout(predicate::str::contains("SUCCESS: Manifest updated"));

    assert!(dir.path().join("shadows").join("MANIFEST.md").exists());
    Ok(())
}

#[test]
fn test_stale_context_warning() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    // Create a manifest with an old fingerprint
    let shadows_dir = dir.path().join("shadows");
    fs::create_dir(&shadows_dir)?;
    // Use 0 to guarantee a mismatch since real fingerprints are non-zero when projects exist
    fs::write(shadows_dir.join("MANIFEST.md"), "**Fingerprint:** `0`")?;

    // Create a project to change the actual fingerprint
    fs::create_dir(projects_dir.join("new-proj"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Context is stale"));
    Ok(())
}

#[test]
fn test_create_dry_run() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::create_dir(dir.path().join("projects"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("create")
        .arg("new-project")
        .arg("--dry-run")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "[Dry Run] Would create project directory",
        ));

    assert!(!dir.path().join("projects").join("new-project").exists());
    Ok(())
}

#[test]
fn test_stats_basic() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    let proj_path = projects_dir.join("test-stats");
    fs::create_dir(&proj_path)?;
    fs::write(proj_path.join("Cargo.toml"), "")?;
    fs::write(proj_path.join("main.rs"), "0".repeat(100))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("stats")
        .assert()
        .success()
        .stdout(predicate::str::contains("--- ECOSYSTEM ANALYTICS ---"))
        .stdout(predicate::str::contains("test-stats"));
    Ok(())
}
