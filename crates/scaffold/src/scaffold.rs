// SPDX-License-Identifier: MIT
//! Logic for scaffolding new projects within the Code Control Plane.

use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Configuration for creating a new project.
pub struct ProjectConfig<'a> {
    /// The name of the project (becomes the directory name).
    pub name: &'a str,
    /// The root directory where projects are stored (e.g., "projects/").
    pub root_dir: PathBuf,
    /// If true, performs a simulation without modifying the filesystem.
    pub dry_run: bool,
}

/// Creates a new project directory with standard boilerplate.
///
/// This includes:
/// - A `docs/` directory.
/// - A `README.md` file.
/// - A `.gitignore` file.
pub fn create_project(config: ProjectConfig) -> Result<PathBuf> {
    let project_path = config.root_dir.join(config.name);

    if project_path.exists() {
        bail!("Project directory already exists: {:?}", project_path);
    }

    if config.dry_run {
        println!(
            "[Dry Run] Would create project directory: {:?}",
            project_path
        );
        println!("[Dry Run] Would create directories: docs/");
        println!("[Dry Run] Would write files: README.md, .gitignore");
        return Ok(project_path);
    }

    println!("Creating project: {}", config.name);

    // 1. Create directories
    fs::create_dir_all(project_path.join("docs"))
        .context("Failed to create project directories")?;

    // 2. Write README.md
    let readme_content = format!("# {}\n\n## Overview", config.name);
    fs::write(project_path.join("README.md"), readme_content)
        .context("Failed to write README.md")?;

    // 3. Write .gitignore
    let gitignore_content = "target/\n.DS_Store\n.env\n";
    fs::write(project_path.join(".gitignore"), gitignore_content)
        .context("Failed to write .gitignore")?;

    println!("Project created successfully at: {:?}", project_path);

    Ok(project_path)
}

/// Launches a specified editor in the project directory.
///
/// Supported editors:
/// - `vscode` (launches `code`)
/// - `windsurf` (launches `windsurf`)
pub fn open_in_editor(project_name: &str, root_dir: &Path, editor: &str) -> Result<()> {
    let project_path = root_dir.join(project_name);

    let command = match editor {
        "vscode" => "code",
        "windsurf" => "windsurf",
        _ => bail!("Unknown editor: {}", editor),
    };

    println!("Opening in {}...", editor);

    let status = Command::new(command)
        .arg(".")
        .current_dir(project_path)
        .status()
        .context(format!("Failed to launch {}. Is it in your PATH?", command))?;

    if !status.success() {
        bail!("Editor launch failed");
    }

    Ok(())
}
