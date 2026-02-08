// SPDX-License-Identifier: MIT
use super::*;
use std::fs;
use std::sync::Mutex;
use tempfile::tempdir;

static DISCOVERY_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_workspace_paths() {
    let root = PathBuf::from("/tmp/toad");
    let ws = Workspace::with_root(root.clone(), None, None);
    assert_eq!(ws.projects_dir, root.join("projects"));
    assert_eq!(ws.shadows_dir, root.join("shadows"));
    assert_eq!(ws.manifest_path(), root.join("shadows").join("MANIFEST.md"));
}

#[test]
fn test_ensure_shadows() -> Result<()> {
    let dir = tempdir()?;
    let ws = Workspace::with_root(dir.path().to_path_buf(), None, None);

    assert!(!ws.shadows_dir.exists());
    ws.ensure_shadows()?;
    assert!(ws.shadows_dir.exists());
    Ok(())
}

#[test]
fn test_get_fingerprint() -> Result<()> {
    let dir = tempdir()?;
    let ws = Workspace::with_root(dir.path().to_path_buf(), None, None);

    // Should fail if projects dir doesn't exist
    assert!(ws.get_fingerprint().is_err());

    fs::create_dir(&ws.projects_dir)?;
    let fp1 = ws.get_fingerprint()?;
    assert!(fp1 > 0);

    // Create a project
    let proj_dir = ws.projects_dir.join("test-proj");
    fs::create_dir(&proj_dir)?;

    // Explicitly set mtime to be different from root
    let future = filetime::FileTime::from_system_time(
        SystemTime::now() + std::time::Duration::from_secs(10),
    );
    filetime::set_file_mtime(&proj_dir, future)?;

    let fp2 = ws.get_fingerprint()?;
    assert_ne!(fp1, fp2, "Fingerprint should change when project is added");

    // Add a high-value file
    let readme_path = proj_dir.join("README.md");
    fs::write(&readme_path, "hello")?;

    let even_further = filetime::FileTime::from_system_time(
        SystemTime::now() + std::time::Duration::from_secs(20),
    );
    filetime::set_file_mtime(&readme_path, even_further)?;

    let fp3 = ws.get_fingerprint()?;
    assert_ne!(
        fp2, fp3,
        "Fingerprint should change when README is added/modified"
    );

    Ok(())
}

#[test]
fn test_fingerprint_performance() -> Result<()> {
    let dir = tempdir()?;
    let ws = Workspace::with_root(dir.path().to_path_buf(), None, None);
    fs::create_dir(&ws.projects_dir)?;

    // Create 100 projects with 5 high-value files each
    for i in 0..100 {
        let proj_dir = ws.projects_dir.join(format!("proj-{}", i));
        fs::create_dir(&proj_dir)?;
        fs::write(proj_dir.join("README.md"), "test")?;
        fs::write(proj_dir.join("Cargo.toml"), "test")?;
        fs::write(proj_dir.join("package.json"), "test")?;
        fs::write(proj_dir.join(".gitignore"), "test")?;
        fs::create_dir(proj_dir.join(".git"))?;
        fs::write(proj_dir.join(".git/index"), "test")?;
    }

    let start = std::time::Instant::now();
    let _fp = ws.get_fingerprint()?;
    let duration = start.elapsed();

    println!("Fingerprinting 100 projects took: {:?}", duration);
    // Should be under 100ms (raised from 50ms for CI variance)
    assert!(
        duration.as_millis() < 100,
        "Fingerprinting too slow: {:?}",
        duration
    );
    Ok(())
}

#[test]
fn test_stack_strategy_serialization() -> Result<()> {
    let strategy = StackStrategy {
        name: "Rust".to_string(),
        match_files: vec!["Cargo.toml".to_string()],
        artifacts: vec!["target".to_string()],
        tags: vec!["#rust".to_string()],
        priority: 10,
    };

    let toml = toml::to_string(&strategy)?;
    assert!(toml.contains("name = \"Rust\""));
    assert!(toml.contains("match_files = [\"Cargo.toml\"]"));

    let loaded: StackStrategy = toml::from_str(&toml)?;
    assert_eq!(strategy, loaded);
    Ok(())
}

#[test]
fn test_strategy_registry_install_and_load() -> Result<()> {
    let dir = tempdir()?;
    let builtin_dir = dir.path().join("builtin");
    fs::create_dir(&builtin_dir)?;

    crate::strategy::StrategyRegistry::install_defaults(&builtin_dir)?;

    let strategies = crate::strategy::StrategyRegistry::load_from_dir(&builtin_dir)?;
    assert!(strategies.len() >= 5);

    let rust = strategies
        .iter()
        .find(|s| s.name == "Rust")
        .expect("Rust strategy missing");
    assert_eq!(rust.match_files, vec!["Cargo.toml".to_string()]);
    assert_eq!(rust.artifacts, vec!["target".to_string()]);

    Ok(())
}

#[test]
fn test_project_registry_serialization() -> Result<()> {
    let mut registry = ProjectRegistry::default();
    registry.fingerprint = 12345;
    registry.projects.push(ProjectDetail {
        name: "test-proj".to_string(),
        path: PathBuf::from("/tmp/test-proj"),
        stack: "Rust".to_string(),
        activity: ActivityTier::Active,
        vcs_status: VcsStatus::Clean,
        essence: Some("A test project".to_string()),
        tags: vec!["#tag1".to_string()],
        taxonomy: vec!["#rust".to_string(), "#test".to_string()],
        artifact_dirs: vec!["target".to_string()],
        sub_projects: vec![],
    });

    // Mock the config dir for testing
    let dir = tempdir()?;
    let registry_path = dir.path().join("registry.json");

    // So we just test manual save/load logic using the same serde logic
    let content = serde_json::to_string_pretty(&registry)?;
    fs::write(&registry_path, content)?;

    let loaded_content = fs::read_to_string(&registry_path)?;
    let loaded: ProjectRegistry = serde_json::from_str(&loaded_content)?;

    assert_eq!(loaded.fingerprint, 12345);
    assert_eq!(loaded.projects.len(), 1);
    assert_eq!(loaded.projects[0].name, "test-proj");

    Ok(())
}

#[test]
fn test_workspace_discovery_tiers() -> Result<()> {
    let _lock = DISCOVERY_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let dir = tempdir()?;
    let root = dir.path();
    fs::write(root.join(".toad-root"), "")?;

    let original_cwd = std::env::current_dir().unwrap_or_else(|_| fs::canonicalize(root).unwrap());
    let original_root = std::env::var("TOAD_ROOT").ok();

    // 1. Env Var tier
    unsafe {
        std::env::set_var("TOAD_ROOT", root.to_str().unwrap());
    }
    let ws = Workspace::discover()?;
    assert_eq!(ws.root, fs::canonicalize(root)?);
    unsafe {
        std::env::remove_var("TOAD_ROOT");
    }

    // 2. Upward search tier
    let sub = root.join("a/b/c");
    fs::create_dir_all(&sub)?;
    std::env::set_current_dir(&sub)?;
    let ws = Workspace::discover()?;
    assert_eq!(ws.root, fs::canonicalize(root)?);

    // 3. Global config tier
    // We can't easily mock home_dir without more complex crates,
    // but we can test the logic if we were in a non-workspace dir
    let other_dir = tempdir()?;
    std::env::set_current_dir(other_dir.path())?;
    assert!(Workspace::discover().is_err());

    // Restore global state
    let _ = std::env::set_current_dir(&original_cwd);
    if let Some(val) = original_root {
        unsafe {
            std::env::set_var("TOAD_ROOT", val);
        }
    }

    Ok(())
}

#[test]
fn test_global_config_persistence() -> Result<()> {
    let dir = tempdir()?;
    let base_dir = dir.path().to_path_buf();

    let config = GlobalConfig {
        home_pointer: PathBuf::from("/tmp/fake"),
        active_context: Some("test".to_string()),
        project_contexts: {
            let mut m = std::collections::HashMap::new();
            m.insert(
                "test".to_string(),
                ProjectContext {
                    path: PathBuf::from("/tmp/fake"),
                    description: None,
                    registered_at: SystemTime::now(),
                },
            );
            m
        },
    };
    config.save(Some(&base_dir))?;

    let loaded = GlobalConfig::load(Some(&base_dir))?.expect("Config should be loaded");
    assert_eq!(loaded.home_pointer, PathBuf::from("/tmp/fake"));
    assert_eq!(loaded.active_context, Some("test".to_string()));
    assert!(loaded.project_contexts.contains_key("test"));

    Ok(())
}

#[test]
fn test_global_config_migration() -> Result<()> {
    let dir = tempdir()?;
    let base_dir = dir.path().to_path_buf();

    // Manually write legacy JSON (simulate old format)
    let legacy_json = serde_json::json!({
        "home_pointer": "/tmp/legacy"
    });
    fs::write(
        GlobalConfig::config_path(Some(&base_dir))?,
        serde_json::to_string_pretty(&legacy_json)?,
    )?;

    // Create a legacy registry at the old location
    fs::write(base_dir.join("registry.json"), "{}")?;

    let loaded =
        GlobalConfig::load(Some(&base_dir))?.expect("Config should be migrated and loaded");
    assert_eq!(loaded.active_context, Some("default".to_string()));
    assert_eq!(loaded.home_pointer, PathBuf::from("/tmp/legacy"));
    assert!(loaded.project_contexts.contains_key("default"));
    assert_eq!(
        loaded.project_contexts.get("default").unwrap().path,
        PathBuf::from("/tmp/legacy")
    );

    // Verify registry migration
    assert!(
        GlobalConfig::context_dir("default", Some(&base_dir))?
            .join("registry.json")
            .exists()
    );
    assert!(!base_dir.join("registry.json").exists());

    Ok(())
}

#[test]
fn test_project_registry_context_paths() -> Result<()> {
    let dir = tempdir()?;
    let base_dir = dir.path().to_path_buf();

    // 1. Global path
    let global_path = ProjectRegistry::registry_path(None, Some(&base_dir))?;
    assert_eq!(global_path, base_dir.join("registry.json"));

    // 2. Context path
    let context_path = ProjectRegistry::registry_path(Some("test-ctx"), Some(&base_dir))?;
    assert_eq!(
        context_path,
        base_dir.join("contexts/test-ctx/registry.json")
    );

    Ok(())
}

#[test]
fn test_workspace_context_shadows() -> Result<()> {
    let dir = tempdir()?;
    let root = dir.path().join("work");
    let base_dir = dir.path().join("config");
    fs::create_dir(&root)?;
    fs::create_dir(&base_dir)?;

    // 1. No context
    let ws1 = Workspace::with_root(root.clone(), None, Some(&base_dir));
    assert_eq!(ws1.shadows_dir, root.join("shadows"));

    // 2. With context
    let ws2 = Workspace::with_root(root.clone(), Some("ctx-a".to_string()), Some(&base_dir));
    assert_eq!(ws2.shadows_dir, base_dir.join("contexts/ctx-a/shadows"));

    Ok(())
}

#[test]
fn test_artifact_migration_logic() -> Result<()> {
    let dir = tempdir()?;
    let base_dir = dir.path().to_path_buf();
    let root_dir = dir.path().join("my-toad-home");
    fs::create_dir(&root_dir)?;

    // Setup legacy files
    let legacy_registry = base_dir.join("registry.json");
    fs::write(&legacy_registry, "registry content")?;

    let legacy_shadows = root_dir.join("shadows");
    fs::create_dir(&legacy_shadows)?;
    fs::write(legacy_shadows.join("tags.json"), "tags content")?;
    fs::write(legacy_shadows.join("MANIFEST.md"), "manifest content")?;

    let config = GlobalConfig {
        home_pointer: root_dir.clone(),
        active_context: Some("default".to_string()),
        project_contexts: std::collections::HashMap::new(),
    };

    config.migrate_legacy_artifacts(Some(&base_dir))?;

    // Verify registry moved
    let new_registry = base_dir.join("contexts/default/registry.json");
    assert!(new_registry.exists());
    assert_eq!(fs::read_to_string(new_registry)?, "registry content");
    assert!(!legacy_registry.exists());

    // Verify shadows moved
    let new_shadows = base_dir.join("contexts/default/shadows");
    assert!(new_shadows.exists());
    assert_eq!(
        fs::read_to_string(new_shadows.join("tags.json"))?,
        "tags content"
    );
    assert_eq!(
        fs::read_to_string(new_shadows.join("MANIFEST.md"))?,
        "manifest content"
    );
    assert!(!legacy_shadows.exists());

    Ok(())
}

#[test]
fn test_display_impls() {
    assert_eq!(format!("{}", ActivityTier::Active), "🔥 Active");
    assert_eq!(format!("{}", ActivityTier::Cold), "❄️ Cold");
    assert_eq!(format!("{}", ActivityTier::Archive), "🗄️ Archive");

    assert_eq!(format!("{}", VcsStatus::Clean), "✅ Clean");
    assert_eq!(format!("{}", VcsStatus::Dirty), "⚠️ Dirty");
    assert_eq!(format!("{}", VcsStatus::Untracked), "❓ Untracked");
    assert_eq!(format!("{}", VcsStatus::None), "N/A");
}

#[test]
fn test_workspace_discovery_env_var() -> Result<()> {
    let _lock = DISCOVERY_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let dir = tempdir()?;
    let root = dir.path().to_path_buf();

    let original_root = std::env::var("TOAD_ROOT").ok();
    unsafe {
        std::env::set_var("TOAD_ROOT", root.to_str().unwrap());
    }

    let ws = Workspace::discover()?;
    assert_eq!(ws.root, fs::canonicalize(&root)?);

    if let Some(val) = original_root {
        unsafe {
            std::env::set_var("TOAD_ROOT", val);
        }
    } else {
        unsafe {
            std::env::remove_var("TOAD_ROOT");
        }
    }
    Ok(())
}

#[test]
#[cfg_attr(tarpaulin, ignore)]
fn test_workspace_discovery_upward() -> Result<()> {
    let _lock = DISCOVERY_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let dir = tempdir()?;
    let root_path = dir.path().to_path_buf();
    let root = fs::canonicalize(&root_path)?;
    fs::write(root.join(".toad-root"), "marker")?;

    let sub = root.join("depth/1/2/3");
    fs::create_dir_all(&sub)?;

    // Save TOAD_ROOT and clear it so discover() doesn't short-circuit via env var
    let original_root = std::env::var("TOAD_ROOT").ok();
    unsafe {
        std::env::remove_var("TOAD_ROOT");
    }

    // current_dir() can fail if another test deleted the cwd, so recover first
    let original_cwd = std::env::current_dir().unwrap_or_else(|_| root.clone());

    // Use a scope to ensure Workspace::discover happens while CWD is set
    let ws_res = {
        std::env::set_current_dir(&sub)?;
        let res = Workspace::discover();
        let _ = std::env::set_current_dir(&original_cwd);
        res
    };

    // Restore TOAD_ROOT
    if let Some(val) = original_root {
        unsafe {
            std::env::set_var("TOAD_ROOT", val);
        }
    }

    let ws = ws_res?;
    assert_eq!(fs::canonicalize(ws.root)?, root);

    // Keep dir alive until the end
    drop(dir);
    Ok(())
}

#[test]
fn test_global_config_active_path() -> Result<()> {
    let dir = tempdir()?;
    let root1 = fs::canonicalize(dir.path())?.join("root1");
    let root2 = fs::canonicalize(dir.path())?.join("root2");
    fs::create_dir(&root1)?;
    fs::create_dir(&root2)?;

    let mut config = GlobalConfig {
        home_pointer: root1.clone(),
        active_context: None,
        project_contexts: std::collections::HashMap::new(),
    };

    // 1. Fallback to home_pointer
    assert_eq!(config.active_path()?, root1);

    // 2. Use active context
    config.project_contexts.insert(
        "ctx2".to_string(),
        ProjectContext {
            path: root2.clone(),
            description: None,
            registered_at: SystemTime::now(),
        },
    );
    config.active_context = Some("ctx2".to_string());
    assert_eq!(config.active_path()?, root2);

    Ok(())
}

#[test]
fn test_workspace_new_and_ensure() -> Result<()> {
    let _lock = DISCOVERY_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let dir = tempdir()?;
    let base_dir = fs::canonicalize(dir.path())?;

    // Set env vars so discover() works and uses our temp dir
    let original_config = std::env::var("TOAD_CONFIG_DIR").ok();
    let original_root = std::env::var("TOAD_ROOT").ok();
    unsafe {
        std::env::set_var("TOAD_CONFIG_DIR", base_dir.to_str().unwrap());
        std::env::set_var("TOAD_ROOT", base_dir.to_str().unwrap());
    }

    // Create a dummy config so discover doesn't bail
    let config = GlobalConfig {
        home_pointer: base_dir.clone(),
        active_context: Some("default".to_string()),
        project_contexts: {
            let mut m = std::collections::HashMap::new();
            m.insert(
                "default".to_string(),
                ProjectContext {
                    path: base_dir.clone(),
                    description: None,
                    registered_at: SystemTime::now(),
                },
            );
            m
        },
    };
    config.save(Some(&base_dir))?;

    let ws = Workspace::new();
    assert_eq!(fs::canonicalize(&ws.root)?, base_dir);
    assert!(ws.shadows_dir.starts_with(&base_dir));

    ws.ensure_shadows()?;
    assert!(ws.shadows_dir.exists());

    // Restore env vars
    if let Some(val) = original_config {
        unsafe {
            std::env::set_var("TOAD_CONFIG_DIR", val);
        }
    } else {
        unsafe {
            std::env::remove_var("TOAD_CONFIG_DIR");
        }
    }
    if let Some(val) = original_root {
        unsafe {
            std::env::set_var("TOAD_ROOT", val);
        }
    } else {
        unsafe {
            std::env::remove_var("TOAD_ROOT");
        }
    }
    Ok(())
}

#[test]
fn test_project_registry_context_save() -> Result<()> {
    let dir = tempdir()?;
    let base_dir = dir.path().to_path_buf();

    let registry = ProjectRegistry::default();
    registry.save(Some("my-ctx"), Some(&base_dir))?;

    let target = base_dir.join("contexts/my-ctx/registry.json");
    assert!(target.exists());

    let loaded = ProjectRegistry::load(Some("my-ctx"), Some(&base_dir))?;
    assert_eq!(loaded.fingerprint, 0);

    Ok(())
}
