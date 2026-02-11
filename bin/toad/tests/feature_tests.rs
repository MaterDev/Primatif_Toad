use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_create_dry_run() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
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
fn test_docs() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("docs")
        .assert()
        .success()
        .stdout(predicate::str::contains("SUCCESS: Documentation updated"));

    assert!(dir
        .path()
        .join("docs")
        .join("guides")
        .join("CLI.md")
        .exists());
    Ok(())
}

#[test]
fn test_manifest() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let root = fs::canonicalize(dir.path())?;
    fs::write(root.join(".toad-root"), "")?;
    let config_dir = root.join(".toad");
    fs::create_dir_all(&config_dir)?;
    fs::create_dir(root.join("projects"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(&root)
        .env("TOAD_ROOT", &root)
        .env("TOAD_CONFIG_DIR", &config_dir)
        .arg("skill")
        .arg("sync")
        .assert()
        .success()
        .stdout(predicate::str::contains("SYNCHRONIZING AI SKILLS"));

    // In v1.1.0, shadows are per-context under the config dir
    let manifest_in_context = config_dir.join("contexts/default/shadows/MANIFEST.md");
    let manifest_legacy = root.join("shadows/MANIFEST.md");
    assert!(
        manifest_in_context.exists() || manifest_legacy.exists(),
        "MANIFEST.md not found at {:?} or {:?}",
        manifest_in_context,
        manifest_legacy
    );
    Ok(())
}

#[test]
fn test_strategy_flow() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let home = dir.path().join("fake-home");
    fs::create_dir(&home)?;

    // 1. List (should show built-ins after auto-install)
    let mut cmd_list = cargo_bin_cmd!("toad");
    cmd_list
        .env("HOME", &home)
        .current_dir(dir.path())
        .arg("strategy")
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rust"))
        .stdout(predicate::str::contains("NodeJS"));

    // 2. Add
    let mut cmd_add = cargo_bin_cmd!("toad");
    cmd_add
        .env("HOME", &home)
        .current_dir(dir.path())
        .arg("strategy")
        .arg("add")
        .arg("Zig")
        .arg("-m")
        .arg("build.zig")
        .arg("-t")
        .arg("#zig")
        .assert()
        .success()
        .stdout(predicate::str::contains("Strategy 'Zig' added"));

    // 3. Info
    let mut cmd_info = cargo_bin_cmd!("toad");
    cmd_info
        .env("HOME", &home)
        .current_dir(dir.path())
        .arg("strategy")
        .arg("info")
        .arg("Zig")
        .assert()
        .success()
        .stdout(predicate::str::contains("Name: Zig"))
        .stdout(predicate::str::contains("Matches: build.zig"));

    // 4. Remove
    let mut cmd_rm = cargo_bin_cmd!("toad");
    cmd_rm
        .env("HOME", &home)
        .current_dir(dir.path())
        .arg("strategy")
        .arg("remove")
        .arg("Zig")
        .assert()
        .success()
        .stdout(predicate::str::contains("Strategy 'Zig' removed"));

    Ok(())
}
