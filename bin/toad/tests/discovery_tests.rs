use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use std::process::Command as StdCommand;
use tempfile::tempdir;

#[test]
fn test_reveal_no_projects() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    fs::create_dir(dir.path().join("projects"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("reveal")
        .arg("test")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No projects found matching 'test'.",
        ));
    Ok(())
}

#[test]
fn test_reveal_with_project() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
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
    fs::write(dir.path().join(".toad-root"), "")?;
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
        .stdout(predicate::str::contains(
            "01/2 projects are HEALTHY & CLEAN",
        ))
        .stdout(predicate::str::contains(
            "1 projects have NEW GROWTH (UNTRACKED)",
        ))
        .stdout(predicate::str::contains("dirty-proj"));
    Ok(())
}

#[test]
fn test_sync() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("sync-proj"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("sync")
        .assert()
        .success()
        .stdout(predicate::str::contains("Scanning projects..."))
        .stdout(predicate::str::contains(
            "SUCCESS: Registry updated with 1 projects.",
        ));

    Ok(())
}

#[test]
fn test_reveal_cached() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let home = dir.path().join("fake-home");
    fs::create_dir(&home)?;
    let toad_dir = home.join(".toad");
    fs::create_dir(&toad_dir)?;

    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    let mut cmd_sync = cargo_bin_cmd!("toad");
    cmd_sync
        .env("HOME", &home)
        .current_dir(dir.path())
        .arg("sync")
        .assert()
        .success();

    let registry_path = toad_dir.join("registry.json");
    let mut registry_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&registry_path)?)?;

    let cached_project = serde_json::json!({
        "name": "cached-proj",
        "path": "/tmp/cached-proj",
        "stack": "Rust",
        "activity": "Active",
        "vcs_status": "Clean",
        "essence": "Cached project",
        "tags": ["#cached"],
        "taxonomy": ["#rust"],
        "artifact_dirs": ["target"],
        "sub_projects": []
    });
    registry_json["projects"]
        .as_array_mut()
        .unwrap()
        .push(cached_project);
    fs::write(&registry_path, serde_json::to_string(&registry_json)?)?;

    Ok(())
}

#[test]
fn test_reveal_staleness() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let home = dir.path().join("fake-home");
    fs::create_dir(&home)?;

    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    // 1. Create a project
    let proj_dir = projects_dir.join("temp-proj");
    fs::create_dir(&proj_dir)?;

    // 2. Sync to create cache
    let mut cmd_sync = cargo_bin_cmd!("toad");
    cmd_sync
        .env("HOME", &home)
        .current_dir(dir.path())
        .arg("sync")
        .assert()
        .success();

    // 3. Verify it's found (cached)
    let mut cmd_reveal1 = cargo_bin_cmd!("toad");
    cmd_reveal1
        .env("HOME", &home)
        .current_dir(dir.path())
        .arg("reveal")
        .arg("temp")
        .assert()
        .success()
        .stdout(predicate::str::contains("- temp-proj"));

    // 4. Delete the project
    fs::remove_dir(&proj_dir)?;

    // 5. Verify it's NOT found (re-scan triggered by fingerprint change)
    let mut cmd_reveal2 = cargo_bin_cmd!("toad");
    cmd_reveal2
        .env("HOME", &home)
        .current_dir(dir.path())
        .arg("reveal")
        .arg("temp")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "No projects found matching 'temp'.",
        ));

    Ok(())
}
