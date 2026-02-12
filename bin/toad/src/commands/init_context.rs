use anyhow::Result;
use colored::*;
use toad_core::Workspace;
use crate::ui::IndicatifReporter;

pub fn handle(workspace: &Workspace, force: bool) -> Result<()> {
    println!("{}", "--- INITIALIZING TOAD CONTEXT ---".green().bold());
    
    if force {
        println!("{} Force flag set. Clearing old registry if it exists...", "INFO:".blue().bold());
        let registry_path = toad_core::ProjectRegistry::registry_path(workspace.active_context.as_deref(), None)?;
        if registry_path.exists() {
            let _ = std::fs::remove_file(registry_path);
        }
    }

    // 1. Sync Registry
    let reporter = IndicatifReporter::spinner()?;
    reporter.pb.set_message("Discovering projects and analyzing stacks...");
    toad_discovery::sync_registry(workspace, &reporter)?;
    
    // 2. Generate Manifest and Tiered Prompts
    crate::commands::manifest::handle(workspace, false, false, false)?;

    println!("
{}", "--- INITIALIZATION COMPLETE ---".green().bold());
    println!("Toad is now ready to orchestrate your AI agents.");
    println!("Entry point: {:?}", workspace.shadows_dir.join("llms.txt"));
    
    Ok(())
}
