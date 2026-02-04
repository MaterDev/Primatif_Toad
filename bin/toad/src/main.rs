use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use colored::*;
use discovery::find_projects;
use scaffold::{create_project, open_in_editor, ProjectConfig};
use std::io::{self, Write};
use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "toad")]
#[command(about = "Primatif_Toad: Toad Control CLI", version = VERSION)]
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

        /// Simulate the action without creating files
        #[arg(long, short = 'd')]
        dry_run: bool,
    },
    /// Find projects matching a query
    Reveal {
        /// Case-insensitive search query
        query: String,
    },
    /// List all available commands
    List,
    /// Display version information and the Toad banner
    Version,
}

fn print_banner() {
    // Force colors to ensure they show up in all environments
    colored::control::set_override(true);

    let toad = r###"      _   _      
     (.)_(.)    
  _ (   _   ) _ 
 / \/`-----'\/ \ 
 __/  ^   ^  \__
"###;
    println!("{}", toad.green());
        println!(" {} v{}", "TOAD CONTROL".green().bold(), VERSION.white());
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let root_dir = PathBuf::from("projects");

    match &cli.command {
        Commands::Create { name, dry_run } => {
            let config = ProjectConfig {
                name,
                root_dir: root_dir.clone(),
                dry_run: *dry_run,
            };

            create_project(config)?;

            if *dry_run {
                return Ok(())
            }

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
        Commands::Reveal { query } => {
            println!("Searching for projects matching '{}'...", query);
            let matches = find_projects(&root_dir, query, 30)?;

            if matches.is_empty() {
                println!("No projects found.");
            } else {
                for project in matches {
                    println!("- {}", project);
                }
            }
        }
        Commands::List => {
            let mut cmd = Cli::command();
            cmd.print_help()?;
        }
        Commands::Version => {
            print_banner();
        }
    }

    Ok(())
}
