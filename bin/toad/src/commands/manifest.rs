use anyhow::Result;
use colored::*;
use std::fs;
use toad_core::{ProjectRegistry, Workspace};
use toad_discovery::scan_all_projects;

pub fn handle(workspace: &Workspace, json_flag: bool, check: bool) -> Result<()> {
    let current_fp = workspace.get_fingerprint()?;
    
    // Check staleness logic
    let mut stored_fp = 0;
    let manifest_path = workspace.manifest_path();
    if manifest_path.exists() {
        if let Ok(content) = fs::read_to_string(&manifest_path) {
            if let Some(line) = content.lines().find(|l| l.contains("**Fingerprint:**")) {
                stored_fp = line
                    .split('`')
                    .nth(1)
                    .unwrap_or_default()
                    .parse::<u64>()
                    .unwrap_or_default();
            }
        }
    }

    let is_stale = current_fp != stored_fp;

    if check {
        if is_stale {
            println!("{} Context is stale. (Stored: {}, Current: {})", "STALE:".yellow().bold(), stored_fp, current_fp);
            std::process::exit(1);
        } else {
            println!("{} Context is up to date.", "FRESH:".green().bold());
            return Ok(());
        }
    }

    if !json_flag {
        println!("{}", "--- GENERATING SEMANTIC MANIFEST ---".green().bold());
    }

    let projects = scan_all_projects(workspace)?;
    
    // 1. Save Markdown Manifest
    let manifest_md = toad_manifest::generate_markdown(&projects, current_fp);
    workspace.ensure_shadows()?;
    fs::write(workspace.manifest_path(), manifest_md)?;

    // 2. Save context.json (The structured side-effect)
    let registry = ProjectRegistry {
        fingerprint: current_fp,
        projects: projects.clone(),
        last_sync: std::time::SystemTime::now(),
    };
    let registry_json = serde_json::to_string_pretty(&registry)?;
    fs::write(workspace.context_json_path(), &registry_json)?;

    if json_flag {
        println!("{}", registry_json);
    } else {
        println!(
            "{} Manifest and structured context updated ({} projects).",
            "SUCCESS:".green().bold(),
            projects.len()
        );
        println!("  - Markdown: {:?}", workspace.manifest_path());
        println!("  - JSON:     {:?}", workspace.context_json_path());
    }

    Ok(())
}
