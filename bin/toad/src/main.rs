use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use colored::*;
use discovery::{find_projects, scan_all_projects};
use rayon::prelude::*;
use scaffold::{create_project, open_in_editor, ProjectConfig};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use toad_core::{VcsStatus, Workspace};

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
    /// Scan projects and report Git status
    Status {
        /// Optional query to filter projects
        query: Option<String>,
    },
    /// Execute a shell command across projects matching a query
    Do {
        /// Command to execute
        command: String,

        /// Query to filter projects
        #[arg(long, short = 'q')]
        query: String,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Generate a project manifest for AI context (Shadow)
    Manifest,
    /// Generate programmatic CLI documentation (Markdown)
    Docs,
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
    let workspace = Workspace::new();

    // --- Context Health Check ---
    let manifest_path = workspace.manifest_path();
    if manifest_path.exists() {
        if let Ok(content) = fs::read_to_string(&manifest_path) {
            if let Some(line) = content.lines().find(|l| l.contains("**Fingerprint:**")) {
                let stored_fp = line
                    .split('`')
                    .nth(1)
                    .unwrap_or_default()
                    .parse::<u64>()
                    .unwrap_or_default();

                if let Ok(current_fp) = workspace.get_fingerprint() {
                    if current_fp > stored_fp {
                        println!(
                            "{} Context is stale. Run 'toad manifest' to re-sync intuition.",
                            "WARNING:".yellow().bold()
                        );
                    }
                }
            }
        }
    }

    match &cli.command {
        Commands::Create { name, dry_run } => {
            let config = ProjectConfig {
                name,
                root_dir: workspace.projects_dir.clone(),
                dry_run: *dry_run,
            };

            create_project(config)?;

            if *dry_run {
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
        }
        Commands::Reveal { query } => {
            println!("Searching for projects matching '{}'...", query);
            let matches = find_projects(&workspace.projects_dir, query, 30)?;

            if matches.is_empty() {
                println!("No projects found.");
            } else {
                for project in matches {
                    println!("- {}", project);
                }
            }
        }
        Commands::Status { query } => {
            println!("{}", "--- ECOSYSTEM HEALTH SCAN ---".green().bold());
            let projects = scan_all_projects(&workspace.projects_dir)?;
            let mut dirty = Vec::new();
            let mut untracked = Vec::new();
            let mut clean_count = 0;
            let mut no_repo_count = 0;
            let mut total_matching = 0;

            for project in projects {
                if let Some(q) = &query {
                    if !project.name.to_lowercase().contains(&q.to_lowercase()) {
                        continue;
                    }
                }

                total_matching += 1;

                match project.vcs_status {
                    VcsStatus::Dirty => dirty.push(project.name),
                    VcsStatus::Untracked => untracked.push(project.name),
                    VcsStatus::Clean => clean_count += 1,
                    VcsStatus::None => no_repo_count += 1,
                }
            }

            if total_matching == 0 {
                println!("No projects found.");
                return Ok(());
            }

            // --- UX Optimization: Summary View ---
            if clean_count > 0 {
                println!(
                    "{} {:02}/{} projects are {}",
                    "■".green(),
                    clean_count,
                    total_matching,
                    "CLEAN".green().bold()
                );
            }

            if no_repo_count > 0 {
                println!(
                    "{} {:02}/{} projects are {}",
                    "■".yellow(),
                    no_repo_count,
                    total_matching,
                    "UNTRACKED BY TOAD".yellow()
                );
            }

            // --- Dirty Promotion ---
            if !untracked.is_empty() {
                println!(
                    "\n{} {} projects have {}",
                    "⚡".blue(),
                    untracked.len(),
                    "NEW FILES".blue().bold()
                );
                for name in untracked {
                    println!("  {} {}", "»".blue(), name);
                }
            }

            if !dirty.is_empty() {
                println!(
                    "\n{} {} projects have {}",
                    "⚠️".red(),
                    dirty.len(),
                    "PENDING CHANGES".red().bold()
                );
                for name in dirty {
                    println!("  {} {}", "»".red(), name);
                }
            }
            println!("\n{}", "--- SCAN COMPLETE ---".green());
        }
        Commands::Do {
            command,
            query,
            yes,
        } => {
            println!("{}", "--- BATCH OPERATION PREFLIGHT ---".blue().bold());
            let projects = scan_all_projects(&workspace.projects_dir)?;
            let targets: Vec<_> = projects
                .into_iter()
                .filter(|p| p.name.to_lowercase().contains(&query.to_lowercase()))
                .collect();

            if targets.is_empty() {
                println!("No projects found matching '{}'.", query);
                return Ok(());
            }

            println!("Found {} target(s):", targets.len());
            for project in &targets {
                println!("  {} {}", "»".blue(), project.name);
            }
            println!("\nCommand: {}", command.yellow().bold());

            if !*yes {
                print!("\nExecute on {} projects? [y/N]: ", targets.len());
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            println!("\n{}", "--- EXECUTING BATCH ---".blue().bold());
            let results: Vec<_> = targets
                .into_par_iter()
                .map(|project| {
                    let res = toad_ops::shell::run_in_dir(&project.path, command);
                    (project.name, res)
                })
                .collect();

            let mut success_count = 0;
            let mut fail_count = 0;

            for (name, outcome) in results {
                print!("Processing {}... ", name);
                match outcome {
                    Ok(res) => {
                        if res.exit_code == 0 {
                            println!("{}", "OK".green());
                            success_count += 1;
                        } else {
                            println!("{} (Code: {})", "FAIL".red(), res.exit_code);
                            if !res.stderr.is_empty() {
                                println!("{}", res.stderr.dimmed());
                            }
                            fail_count += 1;
                        }
                    }
                    Err(e) => {
                        println!("{} (Error: {})", "ERROR".red(), e);
                        fail_count += 1;
                    }
                }
            }

            println!("\n{}", "--- BATCH COMPLETE ---".blue().bold());
            println!(
                "{} {} Succeeded | {} {} Failed",
                "■".green(),
                success_count,
                "■".red(),
                fail_count
            );
        }
        Commands::Manifest => {
            println!("Generating project manifest (Shadow Context)...");
            let fingerprint = workspace.get_fingerprint()?;
            let projects = scan_all_projects(&workspace.projects_dir)?;

            let output = toad_manifest::generate_markdown(&projects, fingerprint);

            workspace.ensure_shadows()?;
            let manifest_path = workspace.manifest_path();

            fs::write(&manifest_path, output)?;

            println!(
                "{} Manifest updated at: {:?}",
                "SUCCESS:".green().bold(),
                manifest_path
            );
        }
        Commands::Docs => {
            println!("Generating programmatic CLI documentation...");
            let mut cmd = Cli::command();
            let help = cmd.render_help().to_string();

            let mut output = String::new();
            output.push_str("# Toad CLI Reference\n\n");
            output.push_str("> **Generated by:** `toad docs`  \n");
            output.push_str("> **Last Updated:** 2026-02-04  \n\n");
            output.push_str("```text\n");
            output.push_str(&help);
            output.push_str("\n```\n");

            let docs_path = PathBuf::from("docs/CLI.md");
            if let Some(parent) = docs_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&docs_path, output)?;
            println!(
                "{} Documentation updated at: {:?}",
                "SUCCESS:".green().bold(),
                docs_path
            );
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
