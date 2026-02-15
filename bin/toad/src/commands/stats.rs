use crate::commands::utils::resolve_projects;
use anyhow::Result;
use toad_core::{AnalyticsReport, Workspace};

pub fn handle(
    workspace: &Workspace,
    query: Option<String>,
    tag: Option<String>,
    _all: bool,
) -> Result<AnalyticsReport> {
    let projects = resolve_projects(workspace)?;
    let report =
        toad_ops::stats::generate_analytics_report(&projects, query.as_deref(), tag.as_deref());
    Ok(report)
}
