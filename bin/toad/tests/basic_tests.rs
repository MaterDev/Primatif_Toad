use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
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
        .stdout(predicate::str::contains("Usage: toad [OPTIONS] <COMMAND>"));
}

#[test]
fn test_uninitialized_behavior() {
    let dir = tempdir().unwrap();
    // No .toad-root marker here

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .env("HOME", dir.path()) // Redirect HOME to temp dir
        .env_remove("TOAD_ROOT")
        .env_remove("TOAD_CONFIG_DIR")
        .arg("status")
        .assert()
        .success() // Should succeed now (zero-config)
        .stdout(predicate::str::contains("--- ECOSYSTEM HEALTH SCAN ---"));
}

#[test]
fn test_bootstrap_no_workspace() {
    let dir = tempdir().unwrap();
    // Bootstrap command 'version' should work even without a workspace
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("TOAD CONTROL"));
}

#[test]
fn test_stale_context_warning() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
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
        .env_remove("TOAD_CONFIG_DIR")
        .env("HOME", dir.path()) // Ensure it doesn't find real ~/.toad
        .env("TOAD_ROOT", dir.path())
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Context is stale"));
    Ok(())
}
