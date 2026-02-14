use anyhow::Result;
use std::time::SystemTime;
use toad_core::{ProjectDetail, ProjectRegistry, Workspace};
use toad_discovery::scan_all_projects;

pub fn resolve_projects(workspace: &Workspace) -> Result<Vec<ProjectDetail>> {
    let registry =
        ProjectRegistry::load(workspace.active_context.as_deref(), None).unwrap_or_default();
    let current_fp = workspace.get_fingerprint().unwrap_or(0);

    if registry.fingerprint == current_fp && !registry.projects.is_empty() {
        Ok(registry.projects)
    } else {
        let projects = scan_all_projects(workspace)?;
        let new_registry = ProjectRegistry {
            fingerprint: current_fp,
            projects: projects.clone(),
            last_sync: SystemTime::now(),
        };
        let _ = new_registry.save(workspace.active_context.as_deref(), None);
        Ok(projects)
    }
}

pub fn normalize_tag(t: &str) -> String {
    if t.starts_with('#') {
        t.to_string()
    } else {
        format!("#{}", t)
    }
}

pub fn filter_projects(
    projects: Vec<ProjectDetail>,
    query: Option<&str>,
    tag: Option<&str>,
) -> Vec<ProjectDetail> {
    projects
        .into_iter()
        .filter(|p| {
            let name_match = match query {
                Some(q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                None => true,
            };
            let tag_match = match tag {
                Some(t) => {
                    let target = normalize_tag(t);
                    p.tags.contains(&target)
                }
                None => true,
            };
            name_match && tag_match
        })
        .collect()
}
