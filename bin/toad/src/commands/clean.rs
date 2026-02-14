use crate::commands::utils::{normalize_tag, resolve_projects};
use anyhow::Result;
use toad_core::{Workspace, BatchCleanReport, ProjectDetail, ProgressReporter};
use toad_ops::clean::execute_batch_clean;

pub fn handle(
    workspace: &Workspace,
    query: Option<String>,
    tag: Option<String>,
    tier: Option<String>,
    reporter: &dyn ProgressReporter,
) -> Result<(Vec<ProjectDetail>, BatchCleanReport)> {
    let projects = resolve_projects(workspace)?;

    let targets: Vec<_> = projects
        .into_iter()
        .filter(|p| {
            let name_match = match query {
                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                None => true,
            };
            let tag_match = match tag {
                Some(ref t) => {
                    let target = normalize_tag(t);
                    p.tags.contains(&target)
                }
                None => true,
            };
            let tier_match = match tier {
                Some(ref tr) => {
                    let tier_str = p.activity.to_string().to_lowercase();
                    tier_str.contains(&tr.to_lowercase())
                }
                None => true,
            };
            name_match && tag_match && tier_match && !p.artifact_dirs.is_empty()
        })
        .collect();

    if targets.is_empty() {
        return Ok((Vec::new(), BatchCleanReport {
            results: Vec::new(),
            total_reclaimed: 0,
            success_count: 0,
            fail_count: 0,
        }));
    }

    let report = execute_batch_clean(&targets, reporter);

    Ok((targets, report))
}
