use anyhow::Result;
use toad_core::{ProgressReporter, Workspace};
use toad_discovery::sync_registry;

pub fn handle(workspace: &Workspace, reporter: &dyn ProgressReporter) -> Result<usize> {
    let count = sync_registry(workspace, reporter)?;
    Ok(count)
}
