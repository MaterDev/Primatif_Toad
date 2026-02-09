use crate::commands::utils::resolve_projects;
use crate::ui;
use anyhow::Result;
use toad_core::Workspace;

pub fn handle(
    workspace: &Workspace,
    query: Option<String>,
    tag: Option<String>,
    all: bool,
) -> Result<()> {
    let projects = resolve_projects(workspace)?;
    let report = toad_ops::stats::generate_analytics_report(&projects);
    ui::format_analytics_report(&report, query.as_deref(), tag.as_deref(), all);
    Ok(())
}
