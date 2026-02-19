use anyhow::Result;
use colored::*;
use std::fs;
use toad_core::{ToadError, Workspace};

pub fn handle(
    workspace: &Workspace,
    json_flag: bool,
    check: bool,
    quiet: bool,
    project_filter: Option<&str>,
) -> Result<()> {
    let current_fp = workspace.get_fingerprint()?;
    let stored_fp = workspace.stored_fingerprint();

    let is_stale = current_fp != stored_fp;

    if check {
        if is_stale {
            return Err(ToadError::Discovery(format!(
                "Context is stale. (Stored: {}, Current: {})",
                stored_fp, current_fp
            ))
            .into());
        } else {
            if !quiet {
                println!("{} Context is up to date.", "FRESH:".green().bold());
            }
            return Ok(());
        }
    }

    if !json_flag && !quiet {
        println!("{}", "--- GENERATING SEMANTIC MANIFEST ---".green().bold());
    }

    let config = toad_core::GlobalConfig::load(None)?.unwrap_or_default();

    // Perform sync which handles projects discovery, changelog, and registry saving
    let reporter = toad_core::NoOpReporter;
    toad_discovery::sync_registry(workspace, &reporter)?;

    // Load the projects we just saved
    let registry = toad_core::ProjectRegistry::load(workspace.active_context.as_deref(), None)?;

    let projects: Vec<_> = registry
        .projects
        .iter()
        .filter(|p| {
            if let Some(f) = project_filter {
                p.name.to_lowercase().contains(&f.to_lowercase())
            } else {
                true
            }
        })
        .cloned()
        .collect();

    // 1. Save Markdown Manifest
    let manifest_md = toad_manifest::generate_markdown(
        &projects,
        current_fp,
        Some(config.budget.ecosystem_tokens),
    );
    workspace.ensure_shadows()?;
    fs::write(workspace.manifest_path(), manifest_md)?;

    // 2. Save context.json (redundant with registry.json but kept for specialized tooling)
    let registry_json = serde_json::to_string_pretty(&registry)?;
    fs::write(workspace.context_json_path(), &registry_json)?;

    // 3. Generate Tiered Prompts
    if !quiet && !json_flag {
        println!("Generating Tiered Prompts (llms.txt, SYSTEM_PROMPT.md)...");
    }

    // SYSTEM_PROMPT.md
    let system_prompt =
        toad_manifest::generate_system_prompt(&projects, Some(config.budget.ecosystem_tokens));
    fs::write(
        workspace.shadows_dir.join("SYSTEM_PROMPT.md"),
        system_prompt,
    )?;

    // llms.txt
    let llms_txt = toad_manifest::generate_llms_txt(&projects);
    fs::write(workspace.shadows_dir.join("llms.txt"), llms_txt)?;

    // Per-project files
    for p in &projects {
        let proj_shadow_dir = workspace.shadows_dir.join(&p.name);
        fs::create_dir_all(&proj_shadow_dir)?;

        // AGENTS.md
        let agents_md = toad_manifest::generate_agents_md(p);
        fs::write(proj_shadow_dir.join("AGENTS.md"), agents_md)?;

        // CONTEXT.md (Project-level deep dive)
        let context_md =
            toad_manifest::generate_project_context_md(p, Some(config.budget.project_tokens));
        fs::write(proj_shadow_dir.join("CONTEXT.md"), context_md)?;
    }

    if json_flag {
        println!("{}", registry_json);
    } else if !quiet {
        println!(
            "{} Agent Interface and tiered prompts updated ({} projects).",
            "SUCCESS:".green().bold(),
            projects.len()
        );
        println!(
            "  - Entry Point: {:?}",
            workspace.shadows_dir.join("llms.txt")
        );
        println!("  - JSON Context: {:?}", workspace.context_json_path());
    }

    Ok(())
}
