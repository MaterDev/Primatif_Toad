use crate::ui;
use anyhow::Result;
use toad_core::Workspace;

pub fn handle(workspace: &Workspace, query: Option<String>, tag: Option<String>) -> Result<()> {
    let report = toad_discovery::generate_status_report(workspace)?;
    ui::format_status_report(&report, query.as_deref(), tag.as_deref());
    Ok(())
}
