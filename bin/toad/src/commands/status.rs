use anyhow::Result;
use toad_core::{StatusReport, Workspace};

pub fn handle(workspace: &Workspace, _query: Option<String>, _tag: Option<String>) -> Result<StatusReport> {
    let report = toad_discovery::generate_status_report(workspace)?;
    // Note: Filtering logic should move to toad-discovery or be handled in main.rs before UI
    Ok(report)
}
