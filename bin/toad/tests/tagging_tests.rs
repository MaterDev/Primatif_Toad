use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_tagging_flow() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let root = fs::canonicalize(dir.path())?;
    fs::write(root.join(".toad-root"), "")?;
    let config_dir = root.join(".toad");
    fs::create_dir_all(&config_dir)?;
    let projects_dir = root.join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("tag-proj"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(&root)
        .env("TOAD_ROOT", &root)
        .env("TOAD_CONFIG_DIR", &config_dir)
        .arg("tag")
        .arg("tag-proj")
        .arg("active")
        .assert()
        .success()
        .stdout(predicate::str::contains("Processed 1 projects."));

    let mut cmd_reveal = cargo_bin_cmd!("toad");
    cmd_reveal
        .current_dir(&root)
        .env("TOAD_ROOT", &root)
        .env("TOAD_CONFIG_DIR", &config_dir)
        .arg("reveal")
        .arg("tag")
        .assert()
        .success()
        .stdout(predicate::str::contains("#active"));

    Ok(())
}

#[test]
fn test_untag_flow() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let root = fs::canonicalize(dir.path())?;
    fs::write(root.join(".toad-root"), "")?;
    let config_dir = root.join(".toad");
    fs::create_dir_all(&config_dir)?;
    let projects_dir = root.join("projects");
    fs::create_dir(&projects_dir)?;
    let proj_path = projects_dir.join("untag-proj");
    fs::create_dir(&proj_path)?;

    // 1. Tag it
    let mut cmd_tag = cargo_bin_cmd!("toad");
    cmd_tag
        .current_dir(&root)
        .env("TOAD_ROOT", &root)
        .env("TOAD_CONFIG_DIR", &config_dir)
        .arg("tag")
        .arg("untag-proj")
        .arg("temp")
        .assert()
        .success();

    // 2. Untag it
    let mut cmd_untag = cargo_bin_cmd!("toad");
    cmd_untag
        .current_dir(&root)
        .env("TOAD_ROOT", &root)
        .env("TOAD_CONFIG_DIR", &config_dir)
        .arg("untag")
        .arg("untag-proj")
        .arg("temp")
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::contains("Processed 1 projects"));

    Ok(())
}

#[test]
fn test_tag_harvest() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let config_dir = dir.path().join(".toad");
    fs::create_dir_all(&config_dir)?;

    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    // Create a Rust project
    let rust_path = projects_dir.join("rust-harvest");
    fs::create_dir(&rust_path)?;
    fs::write(rust_path.join("Cargo.toml"), "")?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .env("TOAD_CONFIG_DIR", &config_dir)
        .env("TOAD_ROOT", dir.path())
        .arg("tag")
        .arg("--harvest")
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::contains("Processed 1 projects"));

    let mut cmd_reveal = cargo_bin_cmd!("toad");
    cmd_reveal
        .current_dir(dir.path())
        .env("TOAD_CONFIG_DIR", &config_dir)
        .env("TOAD_ROOT", dir.path())
        .arg("reveal")
        .arg("rust")
        .assert()
        .success()
        .stdout(predicate::str::contains("#rust"));

    Ok(())
}

#[test]
fn test_tag_untag_filters() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let root = fs::canonicalize(dir.path())?;
    fs::write(root.join(".toad-root"), "")?;
    let config_dir = root.join(".toad");
    fs::create_dir_all(&config_dir)?;
    let projects_dir = root.join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("filter-a"))?;
    fs::create_dir(projects_dir.join("filter-b"))?;

    // 1. Tag by query
    let mut cmd_tag = cargo_bin_cmd!("toad");
    cmd_tag
        .current_dir(&root)
        .env("TOAD_ROOT", &root)
        .env("TOAD_CONFIG_DIR", &config_dir)
        .arg("tag")
        .arg("-q")
        .arg("filter")
        .arg("filtered")
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::contains("Processed 2 projects"));

    // 2. Untag by query
    let mut cmd_untag = cargo_bin_cmd!("toad");
    cmd_untag
        .current_dir(&root)
        .env("TOAD_ROOT", &root)
        .env("TOAD_CONFIG_DIR", &config_dir)
        .arg("untag")
        .arg("-q")
        .arg("filter-a")
        .arg("filtered")
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::contains("Processed 1 projects"));

    Ok(())
}
