use crate::ui::IndicatifReporter;
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use toad_core::Workspace;
use toad_discovery::sync_registry;

pub fn handle(workspace: &Workspace) -> Result<()> {
    println!("Scanning projects...");
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner().template("{spinner:.green} [{elapsed_precise}] {msg}")?,
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let reporter = IndicatifReporter { pb };
    let count = sync_registry(workspace, &reporter)?;

    println!(
        "{} Registry updated with {} projects.",
        "SUCCESS:".green().bold(),
        count
    );
    Ok(())
}
