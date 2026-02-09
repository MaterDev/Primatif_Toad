use crate::ui;
use anyhow::Result;
use toad_core::Workspace;

pub fn handle(workspace: &Workspace, query: String, tag: Option<String>) -> Result<()> {
    let results = toad_discovery::search_projects(workspace, &query, tag.as_deref())?;
    ui::format_search_results(&results);
    Ok(())
}
