// SPDX-License-Identifier: MIT
use assert_cmd::cargo_bin_cmd;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_project_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let base_dir = dir.path().join(".toad");
    let work_dir = dir.path().join("work1");
    let work_dir2 = dir.path().join("work2");
    fs::create_dir(&work_dir)?;
    fs::create_dir(&work_dir2)?;
    fs::create_dir(&base_dir)?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.env("TOAD_CONFIG_DIR", base_dir.to_str().unwrap());

    // 1. Register
    cmd.args([
        "project",
        "register",
        "unique1",
        work_dir.to_str().unwrap(),
        "-d",
        "Test Context 1",
    ])
    .assert()
    .success();

    // 2. List
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.env("TOAD_CONFIG_DIR", base_dir.to_str().unwrap());
    cmd.args(["project", "list"])
        .assert()
        .success()
        .stdout(predicates::str::contains("unique1"))
        .stdout(predicates::str::contains("work1"));

    // 3. Switch
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.env("TOAD_CONFIG_DIR", base_dir.to_str().unwrap());
    cmd.args(["project", "switch", "unique1"])
        .assert()
        .success();

    // 4. Current
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.env("TOAD_CONFIG_DIR", base_dir.to_str().unwrap());
    cmd.args(["project", "current"])
        .assert()
        .success()
        .stdout(predicates::str::contains("unique1"))
        .stdout(predicates::str::contains("work1"));

    // 5. Update
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.env("TOAD_CONFIG_DIR", base_dir.to_str().unwrap());
    cmd.args(["project", "update", "unique1", "-d", "Updated description"])
        .assert()
        .success();

    // 6. Delete (requires interaction, so we skip or mock)
    // We'll skip real delete for now or implement it if possible via stdin

    Ok(())
}

#[test]
fn test_home_shortcut() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let base_dir = dir.path().join(".toad");
    let work_dir = dir.path().join("my-home");
    fs::create_dir(&work_dir)?;
    fs::create_dir(&base_dir)?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.env("TOAD_CONFIG_DIR", base_dir.to_str().unwrap());

    // toad home <path> (auto-register default)
    cmd.args(["home", work_dir.to_str().unwrap()])
        .write_stdin(
            "y
",
        ) // In case it asks to initialize
        .assert()
        .success();

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.env("TOAD_CONFIG_DIR", base_dir.to_str().unwrap());
    cmd.args(["project", "current"])
        .assert()
        .success()
        .stdout(predicates::str::contains("my-home"));

    Ok(())
}
