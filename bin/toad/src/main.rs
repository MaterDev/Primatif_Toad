use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use scaffold::{create_project, open_in_editor, ProjectConfig};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "toad")]
#[command(about = "Code Control Plane CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project directory
    Create {
        /// Name of the project
        name: String,
    },
    /// List all available commands
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create { name } => {
            // ... existing code ...
            // In a real Mac-agnostic setup, we resolve the relative projects folder
            // based on the location of the binary or a workspace root.
            // For now, we assume execution from the root of the Code directory.
            let root_dir = PathBuf::from("projects");
            
            let config = ProjectConfig {
                name,
                root_dir: root_dir.clone(),
            };

            create_project(config)?;

            // Offer to open in editor
            println!("\nWould you like to open this project? [v]scode, [w]indsurf, or [n]o");
            print!("> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim().to_lowercase();

            match choice.as_str() {
                "v" | "vscode" => open_in_editor(name, &root_dir, "vscode")?,
                "w" | "windsurf" => open_in_editor(name, &root_dir, "windsurf")?,
                _ => println!("Skipping editor launch."),
            }
        }
        Commands::List => {
            let mut cmd = Cli::command();
            cmd.print_help()?;
        }
    }

    Ok(())
}