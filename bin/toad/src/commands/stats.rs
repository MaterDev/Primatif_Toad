use crate::commands::utils::resolve_projects;
use anyhow::Result;
use toad_core::{AnalyticsReport, Workspace};

pub fn handle(
    workspace: &Workspace,
    _query: Option<String>,
    _tag: Option<String>,
    _all: bool,
) -> Result<AnalyticsReport> {
    let projects = resolve_projects(workspace)?;
    let report = toad_ops::stats::generate_analytics_report(&projects);
    Ok(report)
}
