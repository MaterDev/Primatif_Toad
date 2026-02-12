use anyhow::Result;
use toad_core::{StatusReport, Workspace};

pub fn handle(workspace: &Workspace, query: Option<String>, tag: Option<String>) -> Result<StatusReport> {
    let report = toad_discovery::generate_status_report(workspace, query.as_deref(), tag.as_deref())?;
    Ok(report)
}
