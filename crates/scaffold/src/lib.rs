use anyhow::{Context, Result, bail};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct ProjectConfig<'a> {
    pub name: &'a str,
    pub root_dir: PathBuf,
}

pub fn create_project(config: ProjectConfig) -> Result<()> {
    let project_path = config.root_dir.join(config.name);

    if project_path.exists() {
        bail!("Project directory already exists: {:?}", project_path);
    }

    println!("Creating project: {}", config.name);

    // 1. Create directories
    fs::create_dir_all(project_path.join("docs"))
        .context("Failed to create project directories")?;

    // 2. Write README.md
    let readme_content = format!(
        "# {}\n\n## Overview\nThis project was scaffolded by the Code Control Plane.\n\n## Documentation\nSee [docs/](docs/) for more details.",
        config.name
    );
    fs::write(project_path.join("README.md"), readme_content)
        .context("Failed to write README.md")?;

    // 3. Write .gitignore
    let gitignore_content = "target/\n.DS_Store\n.env\n";
    fs::write(project_path.join(".gitignore"), gitignore_content)
        .context("Failed to write .gitignore")?;

    // 4. Git Init
    init_git(&project_path).context("Failed to initialize git repository")?;

    println!("Project created successfully at: {:?}", project_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_project_structure() {
        let dir = tempdir().unwrap();
        let name = "test-proj";
        let config = ProjectConfig {
            name,
            root_dir: dir.path().to_path_buf(),
        };

        create_project(config).expect("Should create project");

        let proj_path = dir.path().join(name);
        assert!(proj_path.exists());
        assert!(proj_path.join("docs").is_dir());
        assert!(proj_path.join("README.md").is_file());
        assert!(proj_path.join(".gitignore").is_file());
        assert!(proj_path.join(".git").is_dir());
    }
}

fn init_git(path: &Path) -> Result<()> {
    let status = Command::new("git")
        .arg("init")
        .current_dir(path)
        .status()?;

    if !status.success() {
        bail!("Git init failed with status: {}", status);
    }
    Ok(())
}

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