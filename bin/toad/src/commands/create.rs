use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use toad_core::Workspace;
use toad_scaffold::{create_project, open_in_editor, ProjectConfig};

pub fn handle(workspace: &Workspace, name: &str, dry_run: bool) -> Result<()> {
    let project_path = workspace.projects_dir.join(name);

    if dry_run {
        println!(
            "{} Would create project directory: {:?}",
            "[Dry Run]".yellow().bold(),
            project_path
        );
        println!("{} Would create directories: docs/", "[Dry Run]".yellow());
        println!(
            "{} Would write files: README.md, .gitignore",
            "[Dry Run]".yellow()
        );
        println!("{} Would initialize Git repository", "[Dry Run]".yellow());
        return Ok(());
    }

    let config = ProjectConfig {
        name,
        root_dir: workspace.projects_dir.clone(),
        dry_run,
    };

    println!("Creating project: {}", name);
    create_project(config)?;
    println!("Project created successfully at: {:?}", project_path);

    println!("\nWould you like to open this project? [v]scode, [w]indsurf, or [n]o");
    print!("> ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice = input.trim().to_lowercase();

    match choice.as_str() {
        "v" | "vscode" => open_in_editor(name, &workspace.projects_dir, "vscode")?,
        "w" | "windsurf" => open_in_editor(name, &workspace.projects_dir, "windsurf")?,
        _ => println!("Skipping editor launch."),
    }

    Ok(())
}
