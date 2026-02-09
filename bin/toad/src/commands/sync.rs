use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::{Duration, SystemTime};
use toad_core::{ProjectRegistry, Workspace};
use toad_discovery::scan_all_projects;

pub fn handle(workspace: &Workspace) -> Result<()> {
    println!("Scanning projects...");
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner().template("{spinner:.green} [{elapsed_precise}] {msg}")?,
    );
    pb.set_message("Discovering projects on disk...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let fingerprint = workspace.get_fingerprint()?;
    let projects = scan_all_projects(workspace)?;

    pb.set_message("Saving to registry...");
    let registry = ProjectRegistry {
        fingerprint,
        projects,
        last_sync: SystemTime::now(),
    };
    registry.save(workspace.active_context.as_deref(), None)?;

    pb.finish_with_message("SUCCESS: Registry synchronized.");
    println!(
        "{} Registry updated with {} projects.",
        "SUCCESS:".green().bold(),
        registry.projects.len()
    );
    Ok(())
}
