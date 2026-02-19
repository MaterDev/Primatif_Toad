use anyhow::Result;
use std::collections::HashMap;
use toad_core::{DiagnosticReport, ProjectRegistry, StatusReport, Workspace};

pub fn handle(
    workspace: &Workspace,
    query: Option<String>,
    tag: Option<String>,
) -> Result<(StatusReport, DiagnosticReport)> {
    let report =
        toad_discovery::generate_status_report(workspace, query.as_deref(), tag.as_deref())?;

    // Load registry to get full project details with paths
    let registry = ProjectRegistry::load(workspace.active_context.as_deref(), None)?;

    // Build a hashmap for O(1) lookups
    let project_map: HashMap<_, _> = registry.projects.iter().map(|p| (&p.name, p)).collect();

    // Collect diagnostics for all projects in the report
    let mut diagnostics = DiagnosticReport::new();
    for status_project in &report.projects {
        if let Some(full_project) = project_map.get(&status_project.name) {
            let project_diagnostics =
                toad_discovery::detect_metadata_issues(&full_project.path, &full_project.name);
            diagnostics.merge(project_diagnostics);
        }
    }

    Ok((report, diagnostics))
}
