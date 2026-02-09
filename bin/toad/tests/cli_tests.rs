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
    fs::write(dir.path().join(".toad-root"), "")?;
    fs::create_dir(dir.path().join("projects"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("skill")
        .arg("sync")
        .assert()
        .success()
        .stdout(predicate::str::contains("SYNCHRONIZING AI SKILLS"));

    assert!(dir.path().join("shadows").join("MANIFEST.md").exists());
    Ok(())
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
        .arg("version")
        .assert()
        .success()
        .stdout(predicate::str::contains("Context is stale"));
    Ok(())
}

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
fn test_tagging_flow() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("tag-proj"))?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .arg("tag")
        .arg("tag-proj")
        .arg("active")
        .assert()
        .success()
        .stdout(predicate::str::contains("Processed 1 projects."));

    let mut cmd_reveal = cargo_bin_cmd!("toad");
    cmd_reveal
        .current_dir(dir.path())
        .arg("reveal")
        .arg("tag")
        .assert()
        .success()
        .stdout(predicate::str::contains("#active"));

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

#[test]
fn test_error_no_workspace() {
    let dir = tempdir().unwrap();
    // No .toad-root marker here

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
        .env("HOME", dir.path()) // Redirect HOME to temp dir
        .env_remove("TOAD_ROOT")
        .env_remove("TOAD_CONFIG_DIR")
        .arg("status")
        .assert()
        .success() // Should print error message and exit cleanly (Ok(()))
        .stdout(predicate::str::contains("ERROR: Toad workspace not found"));
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
fn test_tag_harvest() -> Result<(), Box<dyn std::error::Error>> {
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

    // Create a Rust project
    let rust_path = projects_dir.join("rust-harvest");
    fs::create_dir(&rust_path)?;
    fs::write(rust_path.join("Cargo.toml"), "")?;

    let mut cmd = cargo_bin_cmd!("toad");
    cmd.current_dir(dir.path())
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
        .env("TOAD_ROOT", dir.path())
        .arg("reveal")
        .arg("rust")
        .assert()
        .success()
        .stdout(predicate::str::contains("#rust"));

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
fn test_untag_flow() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;
    let proj_path = projects_dir.join("untag-proj");
    fs::create_dir(&proj_path)?;

    // 1. Tag it
    let mut cmd_tag = cargo_bin_cmd!("toad");
    cmd_tag
        .current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
        .arg("tag")
        .arg("untag-proj")
        .arg("temp")
        .assert()
        .success();

    // 2. Untag it
    let mut cmd_untag = cargo_bin_cmd!("toad");
    cmd_untag
        .current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
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

#[test]
fn test_tag_untag_filters() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    fs::write(dir.path().join(".toad-root"), "")?;
    let projects_dir = dir.path().join("projects");
    fs::create_dir(&projects_dir)?;
    fs::create_dir(projects_dir.join("filter-a"))?;
    fs::create_dir(projects_dir.join("filter-b"))?;

    // 1. Tag by query
    let mut cmd_tag = cargo_bin_cmd!("toad");
    cmd_tag
        .current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
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
        .current_dir(dir.path())
        .env("TOAD_ROOT", dir.path())
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
