use anyhow::Result;
use std::io::{self, Write};
use toad_core::Workspace;
use toad_scaffold::{create_project, open_in_editor, ProjectConfig};

pub fn handle(workspace: &Workspace, name: &str, dry_run: bool) -> Result<()> {
    let config = ProjectConfig {
        name,
        root_dir: workspace.projects_dir.clone(),
        dry_run,
    };

    create_project(config)?;

    if dry_run {
        return Ok(());
    }

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
