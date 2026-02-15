use crate::ui::IndicatifReporter;
use anyhow::Result;
use colored::*;
use toad_core::Workspace;

pub fn handle(
    workspace: &Workspace,
    force: bool,
    dry_run: bool,
    project: Option<String>,
    no_sync: bool,
) -> Result<()> {
    println!("{}", "--- INITIALIZING TOAD CONTEXT ---".green().bold());

    if dry_run {
        println!(
            "{} DRY RUN: No files will be modified.",
            "INFO:".blue().bold()
        );
    }

    if force && !dry_run {
        println!(
            "{} Force flag set. Clearing old registry if it exists...",
            "INFO:".blue().bold()
        );
        let registry_path =
            toad_core::ProjectRegistry::registry_path(workspace.active_context.as_deref(), None)?;
        if registry_path.exists() {
            let _ = std::fs::remove_file(registry_path);
        }
    }

    // 1. Sync Registry
    if !no_sync && !dry_run {
        let reporter = IndicatifReporter::spinner()?;
        reporter
            .pb
            .set_message("Discovering projects and analyzing stacks...");
        toad_discovery::sync_registry(workspace, &reporter)?;
    } else if no_sync {
        println!(
            "{} Skipping registry synchronization.",
            "INFO:".blue().bold()
        );
    }

    // 2. Generate Manifest and Tiered Prompts
    if !dry_run {
        crate::commands::manifest::handle(workspace, false, false, false, project.as_deref())?;
    }

    if dry_run {
        println!("\n{}", "--- DRY RUN COMPLETE ---".green().bold());
    } else {
        println!("\n{}", "--- INITIALIZATION COMPLETE ---".green().bold());
        println!("Toad is now ready to orchestrate your AI agents.");
        println!("Entry point: {:?}", workspace.shadows_dir.join("llms.txt"));
    }

    Ok(())
}
