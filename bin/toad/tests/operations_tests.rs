use assert_cmd::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_do_dry_run_ish() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
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
    fs::write(dir.path().join(".toad-root"), "")?;
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
fn test_do_destructive_abort() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("proj-danger"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    // This command contains a destructive pattern but we won't provide 'PROCEED'
    cmd.current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
        .arg("do")
        .arg("rm -rf /")
        .arg("-q")
        .arg("proj-danger")
        .write_stdin("n\n") // Abort
        .assert()
        .success()
        .stdout(predicate::str::contains("Aborted"));

    Ok(())
}

#[test]
fn test_do_fail_fast() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("proj-1"))?;
    fs::create_dir(projects_dir.join("proj-2"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
        .arg("do")
        .arg("false") // Will fail
        .arg("-q")
        .arg("proj")
        .arg("-f") // Fail fast
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::contains("FAIL"));

    Ok(())
}

#[test]
fn test_clean_dry_run() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    // Setup strategies
    let config_dir = dir.path().join(".toad");
    fs::create_dir_all(config_dir.join("strategies/builtin"))?;
    toad_core::strategy::StrategyRegistry::install_defaults(
        &config_dir.join("strategies/builtin"),
    )?;

    let proj_path = projects_dir.join("test-clean");
    fs::create_dir(&proj_path)?;
    fs::write(proj_path.join("Cargo.toml"), "")?;
    let target_dir = proj_path.join("target");
    fs::create_dir(&target_dir)?;
    fs::write(target_dir.join("junk"), "0".repeat(100))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
        .arg("clean")
        .arg("--dry-run")
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "--- 🌊 POND HYGIENE PRE-FLIGHT ---",
        ))
        .stdout(predicate::str::contains("test-clean"))
        .stdout(predicate::str::contains("100 B"));

    assert!(target_dir.exists());
    Ok(())
}

#[test]
fn test_clean_activity_tier() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    // Setup strategies
    let config_dir = dir.path().join(".toad");
    fs::create_dir_all(config_dir.join("strategies/builtin"))?;
    toad_core::strategy::StrategyRegistry::install_defaults(
        &config_dir.join("strategies/builtin"),
    )?;

    // 1. Active project (newly created)
    let active_path = projects_dir.join("active-proj");
    fs::create_dir(&active_path)?;
    fs::write(active_path.join("Cargo.toml"), "")?;
    fs::create_dir(active_path.join("target"))?;

    // 2. Cold project (old mtime)
    let cold_path = projects_dir.join("cold-proj");
    fs::create_dir(&cold_path)?;
    fs::write(cold_path.join("Cargo.toml"), "")?;
    fs::create_dir(cold_path.join("target"))?;
    let cold_time =
        std::time::SystemTime::now() - std::time::Duration::from_secs(10 * 24 * 60 * 60);
    filetime::set_file_mtime(&cold_path, filetime::FileTime::from_system_time(cold_time))?;

    // Clean only cold projects
    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
        .arg("clean")
        .arg("--tier")
        .arg("cold")
        .arg("--dry-run")
        .arg("-y")
        .assert()
        .success()
        .stdout(predicate::str::contains("cold-proj"))
        .stdout(predicate::str::contains("active-proj").not());

    Ok(())
}

#[test]
fn test_clean_real_execution() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;

    // Setup strategies
    let config_dir = dir.path().join(".toad");
    fs::create_dir_all(config_dir.join("strategies/builtin"))?;
    toad_core::strategy::StrategyRegistry::install_defaults(
        &config_dir.join("strategies/builtin"),
    )?;

    let proj_path = projects_dir.join("test-clean-real");
    fs::create_dir(&proj_path)?;
    fs::write(proj_path.join("Cargo.toml"), "")?;
    let target_dir = proj_path.join("target");
    fs::create_dir(&target_dir)?;
    fs::write(target_dir.join("artifact"), "data")?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
        .arg("clean")
        .arg("-y") // Skip confirmation
        .assert()
        .success()
        .stdout(predicate::str::contains("■ 1 Succeeded"));

    assert!(!target_dir.exists());
    assert!(proj_path.exists());
    Ok(())
}

#[test]
fn test_stats_basic() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
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

#[test]
fn test_stats_all() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("stats-proj"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
        .arg("stats")
        .arg("--all")
        .assert()
        .success()
        .stdout(predicate::str::contains("TOP 1 OFFENDERS"));

    Ok(())
}
