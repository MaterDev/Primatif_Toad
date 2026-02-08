// SPDX-License-Identifier: BUSL-1.1
use anyhow::{bail, Result};
use std::path::Path;
use std::process::Command;

/// Initializes a git repository in the given directory.
pub fn init_repo(path: &Path) -> Result<()> {
    let status = Command::new("git").arg("init").current_dir(path).status()?;

    if !status.success() {
        bail!("Git init failed with status: {}", status);
    }
    Ok(())
}
