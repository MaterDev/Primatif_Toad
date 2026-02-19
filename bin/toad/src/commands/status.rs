use anyhow::Result;
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
    
    // Collect diagnostics for all projects in the report
    let mut diagnostics = DiagnosticReport::new();
    for status_project in &report.projects {
        // Find the full project detail from registry
        if let Some(full_project) = registry.projects.iter().find(|p| p.name == status_project.name) {
            let project_diagnostics =
                toad_discovery::detect_metadata_issues(&full_project.path, &full_project.name);
            diagnostics.merge(project_diagnostics);
        }
    }
    
    Ok((report, diagnostics))
}
