use anyhow::Result;
use toad_core::{SearchResult, Workspace};

pub fn handle(workspace: &Workspace, query: String, tag: Option<String>) -> Result<SearchResult> {
    let results = toad_discovery::search_projects(workspace, &query, tag.as_deref())?;
    Ok(results)
}
