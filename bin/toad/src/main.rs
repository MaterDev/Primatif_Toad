use anyhow::{bail, Result};
use clap::{CommandFactory, Parser, Subcommand};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use toad_core::{TagRegistry, VcsStatus, Workspace};
use toad_discovery::scan_all_projects;
use toad_ops::stats::{calculate_project_stats, format_size};
use toad_scaffold::{create_project, open_in_editor, ProjectConfig};

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

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
    },
    /// Scan projects and report Git status
    Status {
        /// Optional query to filter projects
        query: Option<String>,

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
    },
    /// Ecosystem health and disk usage analytics
    Stats {
        /// Optional query to filter projects
        query: Option<String>,

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,

        /// Show details for all matching projects
        #[arg(long, short = 'a')]
        all: bool,
    },
    /// Manage the global Toad workspace anchor
    Home {
        /// Set a new absolute path for the Toad home
        path: Option<String>,
    },
    /// Execute a shell command across projects matching a query
    Do {
        /// Command to execute
        command: String,

        /// Query to filter projects
        #[arg(long, short = 'q')]
        query: String,

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,

        /// Simulate the action without executing
        #[arg(long, short = 'd')]
        dry_run: bool,

        /// Halt the entire batch if a single project fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Assign a tag to projects
    Tag {
        /// Project name (optional if using filters)
        project: Option<String>,
        /// Tag name
        tag: Option<String>,

        /// Filter by name query
        #[arg(long, short = 'q')]
        query: Option<String>,

        /// Filter by existing tag
        #[arg(long, short = 't')]
        filter_tag: Option<String>,

        /// Automatically assign tags based on detected stacks
        #[arg(long)]
        harvest: bool,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Remove a tag from projects
    Untag {
        /// Project name (optional if using filters)
        project: Option<String>,
        /// Tag name
        tag: Option<String>,

        /// Filter by name query
        #[arg(long, short = 'q')]
        query: Option<String>,

        /// Filter by existing tag
        #[arg(long, short = 't')]
        filter_tag: Option<String>,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Manage and synchronize AI agent skills
    Skill {
        #[command(subcommand)]
        subcommand: SkillCommand,
    },
    /// Synchronize the project registry cache
    Sync,
    /// Manage language/stack strategies
    Strategy {
        #[command(subcommand)]
        subcommand: StrategyCommands,
    },
    /// Reclaim disk space by removing build artifacts
    Clean {
        /// Optional query to filter projects
        query: Option<String>,

        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,

        /// Filter by activity tier (active, cold, archive)
        #[arg(long)]
        tier: Option<String>,

        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,

        /// Simulate the action without deleting
        #[arg(long, short = 'd')]
        dry_run: bool,
    },
    /// Generate programmatic CLI documentation (Markdown)
    Docs,
    /// Manage project contexts (register, switch, list)
    Project {
        #[command(subcommand)]
        subcommand: ProjectCommand,
    },
    /// Multi-repo Git orchestration
    Ggit {
        #[command(subcommand)]
        subcommand: GgitCommand,
    },
    /// Custom workflows and script orchestration
    Cw {
        #[command(subcommand)]
        subcommand: CwCommand,
    },
    /// List all available commands
    List,
    /// Display version information and the Toad banner
    Version,
}

#[derive(Subcommand)]
enum SkillCommand {
    /// Synchronize all skills (Blueprint, CLI, Manifest) to AI vendors
    Sync,
    /// List distributed skills and registered vendors
    List,
}

#[derive(Subcommand)]
enum CwCommand {
    /// Execute a custom workflow script
    Run {
        /// Name of the workflow
        name: String,
        /// Arguments to pass to the script
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
    /// Register a new custom workflow
    Register {
        /// Name of the workflow
        name: String,
        /// Path to the script
        script: String,
        /// Optional description
        #[arg(long, short = 'd')]
        description: Option<String>,
    },
    /// List all registered custom workflows
    List,
    /// Show detailed info for a custom workflow
    Info {
        /// Name of the workflow
        name: String,
    },
    /// Remove a registered custom workflow
    Delete {
        /// Name of the workflow
        name: String,
    },
}

#[derive(Subcommand)]
enum GgitCommand {
    /// Show consolidated Git status across repositories
    Status {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
    },
    /// Commit changes across repositories
    Commit {
        /// Commit message
        #[arg(long, short = 'm')]
        message: String,
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Automatically commit the Hub root if submodules are changed (Cascade)
        #[arg(long, short = 'c')]
        cascade: bool,
        /// Halt the entire batch if a single repo fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Push changes across repositories
    Push {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Halt the entire batch if a single repo fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Pull changes across repositories
    Pull {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Halt the entire batch if a single repo fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Switch branches across repositories
    Checkout {
        /// Branch name
        branch: String,
        /// Create the branch if it doesn't exist
        #[arg(long, short = 'b')]
        create: bool,
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Halt the entire batch if a single repo fails
        #[arg(long, short = 'f')]
        fail_fast: bool,
    },
    /// Synchronize and align repositories (safe multi-repo update)
    Sync {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Skip pre-flight safety checks
        #[arg(long, short = 'f')]
        force: bool,
    },
    /// List all branches across repositories
    Branches {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
        /// Show remote branches
        #[arg(long, short = 'r')]
        all: bool,
    },
    /// Force-align submodules to Hub root expectations
    Align {
        /// Optional query to filter projects
        #[arg(long, short = 'q')]
        query: Option<String>,
        /// Filter by tag
        #[arg(long, short = 't')]
        tag: Option<String>,
    },
}

#[derive(Subcommand)]
enum ProjectCommand {
    /// Register a new project context
    Register {
        /// Name of the context
        name: String,
        /// Absolute path to the workspace root
        path: String,
        /// Optional description
        #[arg(long, short = 'd')]
        description: Option<String>,
        /// Explicitly set the context type (hub, pond, generic)
        #[arg(long, short = 't', value_enum)]
        context_type: Option<ContextTypeChoice>,
        /// AI Vendors to sync memory to (comma-separated: windsurf,cursor,gemini)
        #[arg(long, short = 'a')]
        ai: Option<String>,
    },
    /// Switch the active project context
    Switch {
        /// Name of the context
        name: String,
    },
    /// Show the currently active context
    Current,
    /// List all registered contexts
    List,
    /// Update an existing context
    Update {
        /// Name of the context
        name: String,
        /// New path for the context
        #[arg(long, short = 'p')]
        path: Option<String>,
        /// New description
        #[arg(long, short = 'd')]
        description: Option<String>,
        /// New context type
        #[arg(long, short = 't', value_enum)]
        context_type: Option<ContextTypeChoice>,
        /// Update AI Vendors
        #[arg(long, short = 'a')]
        ai: Option<String>,
    },
    /// Remove a registered context
    Delete {
        /// Name of the context
        name: String,
        /// Skip confirmation prompt
        #[arg(long, short = 'y')]
        yes: bool,
    },
    /// Show detailed info for a context
    Info {
        /// Name of the context
        name: String,
    },
}

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum ContextTypeChoice {
    Hub,
    Pond,
    Generic,
}

impl From<ContextTypeChoice> for toad_core::ContextType {
    fn from(choice: ContextTypeChoice) -> Self {
        match choice {
            ContextTypeChoice::Hub => toad_core::ContextType::Hub,
            ContextTypeChoice::Pond => toad_core::ContextType::Pond,
            ContextTypeChoice::Generic => toad_core::ContextType::Generic,
        }
    }
}

impl From<toad_core::ContextType> for ContextTypeChoice {
    fn from(t: toad_core::ContextType) -> Self {
        match t {
            toad_core::ContextType::Hub => ContextTypeChoice::Hub,
            toad_core::ContextType::Pond => ContextTypeChoice::Pond,
            toad_core::ContextType::Generic => ContextTypeChoice::Generic,
        }
    }
}

#[derive(Subcommand)]
enum StrategyCommands {
    /// List all active strategies
    List,
    /// Add a new custom strategy
    Add {
        /// Name of the strategy (e.g., Elixir)
        name: String,
        /// Files that identify this stack (comma-separated, e.g., mix.exs)
        #[arg(long, short = 'm')]
        match_files: String,
        /// Build artifacts to clean (comma-separated, e.g., deps,_build)
        #[arg(long, short = 'c')]
        artifacts: Option<String>,
        /// Tags to auto-assign (comma-separated, e.g., #elixir)
        #[arg(long, short = 't')]
        tags: Option<String>,
        /// Priority for matching (higher = earlier check)
        #[arg(long, default_value = "10")]
        priority: i32,
    },
    /// Show details of a specific strategy
    Info {
        /// Name of the strategy
        name: String,
    },
    /// Remove a custom strategy
    Remove {
        /// Name of the strategy
        name: String,
    },
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

    // --- Context Discovery ---
    let discovered = Workspace::discover();

    // Commands that don't require a valid workspace
    let is_bootstrap = matches!(
        &cli.command,
        Commands::Home { .. } | Commands::Version | Commands::List | Commands::Docs
    );

    let workspace = match &discovered {
        Ok(ws) => ws.clone(),
        Err(e) => {
            if is_bootstrap {
                Workspace::with_root(PathBuf::from("."), None, None)
            } else {
                println!("{} {}", "ERROR:".red().bold(), e);
                return Ok(());
            }
        }
    };

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
                            "{} Context is stale. Run 'toad skill sync' to re-sync intuition.",
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
        Commands::Reveal { query, tag } => {
            println!("Searching for projects matching '{}'...", query);

            let registry =
                toad_core::ProjectRegistry::load(workspace.active_context.as_deref(), None)
                    .unwrap_or_default();
            let current_fp = workspace.get_fingerprint().unwrap_or(0);

            let projects = if registry.fingerprint == current_fp && !registry.projects.is_empty() {
                registry.projects
            } else {
                let p = scan_all_projects(&workspace)?;
                // Update cache in the background (or foreground here for simplicity)
                let new_registry = toad_core::ProjectRegistry {
                    fingerprint: current_fp,
                    projects: p.clone(),
                    last_sync: std::time::SystemTime::now(),
                };
                let _ = new_registry.save(workspace.active_context.as_deref(), None);
                p
            };

            let matches: Vec<_> = projects
                .into_iter()
                .filter(|p| {
                    let name_match = p.name.to_lowercase().contains(&query.to_lowercase());
                    let tag_match = match tag {
                        Some(t) => {
                            let target = if t.starts_with('#') {
                                t.clone()
                            } else {
                                format!("#{}", t)
                            };
                            p.tags.contains(&target)
                        }
                        None => true,
                    };
                    name_match && tag_match
                })
                .collect();

            if matches.is_empty() {
                println!("No projects found.");
            } else {
                for project in matches {
                    let tags_display = if project.tags.is_empty() {
                        String::new()
                    } else {
                        format!(" {}", project.tags.join(" ").dimmed())
                    };
                    println!("- {}{}", project.name, tags_display);
                }
            }
        }
        Commands::Status { query, tag } => {
            println!("{}", "--- ECOSYSTEM HEALTH SCAN ---".green().bold());
            let projects = scan_all_projects(&workspace)?;
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

                if let Some(t) = &tag {
                    let target = if t.starts_with('#') {
                        t.clone()
                    } else {
                        format!("#{}", t)
                    };
                    if !project.tags.contains(&target) {
                        continue;
                    }
                }

                total_matching += 1;

                match project.vcs_status {
                    VcsStatus::Dirty => dirty.push(project.name.clone()),
                    VcsStatus::Untracked => untracked.push(project.name.clone()),
                    VcsStatus::Clean => clean_count += 1,
                    VcsStatus::None => no_repo_count += 1,
                }

                // Submodule Display logic during scan
                println!(
                    "{} {} ({}) {}",
                    "»".blue(),
                    project.name.bold(),
                    project.stack.dimmed(),
                    project.vcs_status
                );

                for sub in project.submodules {
                    total_matching += 1;
                    match sub.vcs_status {
                        VcsStatus::Dirty => dirty.push(format!("{} -> {}", project.name, sub.name)),
                        VcsStatus::Untracked => {
                            untracked.push(format!("{} -> {}", project.name, sub.name))
                        }
                        VcsStatus::Clean => clean_count += 1,
                        VcsStatus::None => no_repo_count += 1,
                    }

                    let status_indicator = if sub.initialized {
                        format!("{}", sub.vcs_status)
                    } else {
                        "⭕ Uninit".to_string()
                    };

                    let alignment = if sub.initialized {
                        if sub.expected_commit == sub.actual_commit {
                            " (aligned)".dimmed()
                        } else {
                            " (drifted)".red().bold()
                        }
                    } else {
                        "".normal()
                    };

                    println!(
                        "  {} {} ({}) {} {}",
                        "└─".dimmed(),
                        sub.name.cyan(),
                        sub.stack.dimmed(),
                        status_indicator,
                        alignment
                    );
                }
            }

            if total_matching == 0 {
                println!("No projects found.");
                return Ok(());
            }

            // --- UX Optimization: Summary View ---
            println!("\n{}", "--- SUMMARY ---".green().bold());
            if clean_count > 0 {
                println!(
                    "{} {:02}/{} projects are {}",
                    "🪷".green(),
                    clean_count,
                    total_matching,
                    "HEALTHY & CLEAN".green().bold()
                );
            }

            if no_repo_count > 0 {
                println!(
                    "{} {:02}/{} projects are {}",
                    "🌾".yellow(),
                    no_repo_count,
                    total_matching,
                    "OUTSIDE THE TOAD POND (UNTRACKED)".yellow()
                );
            }

            // --- Dirty Promotion ---
            if !untracked.is_empty() {
                println!(
                    "\n{} {} projects have {}",
                    "🌿".green(),
                    untracked.len(),
                    "NEW GROWTH (UNTRACKED)".green().bold()
                );
                for name in untracked {
                    println!("  {} {}", "»".green(), name);
                }
            }

            if !dirty.is_empty() {
                println!(
                    "\n{} {} projects have {}",
                    "⚠️".red(),
                    dirty.len(),
                    "PENDING CHANGES (DIRTY)".red().bold()
                );
                for name in dirty {
                    println!("  {} {}", "»".red(), name);
                }
            }
            println!("\n{}", "--- SCAN COMPLETE ---".green());
        }
        Commands::Stats { query, tag, all } => {
            println!("{}", "--- ECOSYSTEM ANALYTICS ---".green().bold());

            let registry =
                toad_core::ProjectRegistry::load(workspace.active_context.as_deref(), None)
                    .unwrap_or_default();
            let current_fp = workspace.get_fingerprint().unwrap_or(0);

            let projects = if registry.fingerprint == current_fp && !registry.projects.is_empty() {
                registry.projects
            } else {
                let p = scan_all_projects(&workspace)?;
                let new_registry = toad_core::ProjectRegistry {
                    fingerprint: current_fp,
                    projects: p.clone(),
                    last_sync: std::time::SystemTime::now(),
                };
                let _ = new_registry.save(workspace.active_context.as_deref(), None);
                p
            };

            let matching: Vec<_> = projects
                .into_iter()
                .filter(|p| {
                    let name_match = match query {
                        Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                        None => true,
                    };
                    let tag_match = match tag {
                        Some(ref t) => {
                            let target = if t.starts_with('#') {
                                t.clone()
                            } else {
                                format!("#{}", t)
                            };
                            p.tags.contains(&target)
                        }
                        None => true,
                    };
                    name_match && tag_match
                })
                .collect();

            if matching.is_empty() {
                println!("No projects found.");
                return Ok(());
            }

            println!("Analyzing {} projects...", matching.len());
            let pb = ProgressBar::new(matching.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.green/black}] {pos}/{len}",
                    )?
                    .progress_chars("■-"),
            );

            let mut results: Vec<_> = matching
                .into_par_iter()
                .map(|p| {
                    let artifact_set: std::collections::HashSet<&str> =
                        p.artifact_dirs.iter().map(|s| s.as_str()).collect();
                    let stats = calculate_project_stats(&p.path, &artifact_set);
                    pb.inc(1);
                    (p, stats)
                })
                .collect();

            pb.finish_and_clear();

            // Sort by size descending
            results.sort_by(|a, b| b.1.total_bytes.cmp(&a.1.total_bytes));

            let total_ecosystem_bytes: u64 = results.iter().map(|(_, s)| s.total_bytes).sum();
            let total_artifact_bytes: u64 = results.iter().map(|(_, s)| s.artifact_bytes).sum();

            println!(
                "{} Total Usage: {} ({} Artifacts)",
                "■".green(),
                format_size(total_ecosystem_bytes).bold(),
                format_size(total_artifact_bytes).dimmed()
            );

            let limit = if *all { results.len() } else { 10 };
            let display_count = std::cmp::min(results.len(), limit);

            println!(
                "\n{}",
                format!("TOP {} OFFENDERS", display_count).yellow().bold()
            );

            for (p, stats) in results.iter().take(display_count) {
                let size_str = format_size(stats.total_bytes);

                // Color coding
                let color_size = if stats.total_bytes > 1024 * 1024 * 1024 {
                    size_str.red().bold()
                } else if stats.total_bytes > 200 * 1024 * 1024 {
                    size_str.yellow()
                } else {
                    size_str.green()
                };

                // Bloat bar
                let bar_width = 20;
                let bloat_blocks = ((stats.bloat_index / 100.0) * bar_width as f64) as usize;
                let source_blocks = bar_width - bloat_blocks;

                let bar = format!(
                    "{}{}",
                    "■".repeat(source_blocks).white(),
                    "■".repeat(bloat_blocks).dimmed()
                );

                println!(
                    "{: <20} | {: >10} | [{}] {:.0}% bloat ({})",
                    p.name.bold(),
                    color_size,
                    bar,
                    stats.bloat_index,
                    p.activity
                );
            }

            if !*all && results.len() > 10 {
                println!(
                    "\n... and {} more. Use --all to see full list.",
                    results.len() - 10
                );
            }
        }
        Commands::Home { path } => {
            let mut config =
                toad_core::GlobalConfig::load(None)?.unwrap_or_else(|| toad_core::GlobalConfig {
                    home_pointer: PathBuf::from("."),
                    active_context: None,
                    project_contexts: std::collections::HashMap::new(),
                });

            if let Some(new_path) = path {
                let p = PathBuf::from(new_path);
                if !p.exists() {
                    bail!("Path does not exist: {:?}", p);
                }
                let abs_path = fs::canonicalize(p)?;
                if !abs_path.join(".toad-root").exists() {
                    println!(
                        "{} Path does not contain a '.toad-root' marker.",
                        "WARNING:".yellow().bold()
                    );
                    print!("Initialize as a new Toad home? [y/N]: ");
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    if !input.trim().to_lowercase().starts_with('y') {
                        println!("Aborted.");
                        return Ok(());
                    }
                    let marker_content = "# Primatif Toad Workspace Root\n# This file identifies this directory as a Toad Control Plane home.\n# Do not delete this file if you want the 'toad' CLI to recognize this workspace.\n";
                    fs::write(abs_path.join(".toad-root"), marker_content)?;
                }

                // Register as context
                let name = if config.project_contexts.is_empty() {
                    "default".to_string()
                } else {
                    abs_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("new-home")
                        .to_string()
                };

                config.home_pointer = abs_path.clone();
                config.project_contexts.insert(
                    name.clone(),
                    toad_core::ProjectContext {
                        path: abs_path.clone(),
                        description: Some("Registered via 'toad home'".to_string()),
                        context_type: if abs_path.join(".gitmodules").exists() {
                            toad_core::ContextType::Hub
                        } else {
                            toad_core::ContextType::Generic
                        },
                        ai_vendors: Vec::new(),
                        registered_at: std::time::SystemTime::now(),
                    },
                );
                config.active_context = Some(name.clone());

                // Create per-context storage
                let ctx_shadows =
                    toad_core::GlobalConfig::context_dir(&name, None)?.join("shadows");
                fs::create_dir_all(&ctx_shadows)?;

                config.save(None)?;
                println!(
                    "{} Anchor updated and registered as context '{}' at: {:?}",
                    "SUCCESS:".green().bold(),
                    name,
                    abs_path
                );
            } else {
                match &discovered {
                    Ok(ws) => {
                        let context_info = if let Some(name) = &ws.active_context {
                            format!(" (context: {})", name.bold())
                        } else {
                            String::new()
                        };
                        println!(
                            "{} Current Toad Home{}: {:?}",
                            "ACTIVE:".green().bold(),
                            context_info,
                            ws.root
                        );
                    }
                    Err(e) => {
                        println!("{} {}", "ORPHANED:".red().bold(), e);
                        println!("Use 'toad home <path>' to anchor this system.");
                    }
                }
            }
        }
        Commands::Do {
            command,
            query,
            tag,
            yes,
            dry_run,
            fail_fast,
        } => {
            println!("{}", "--- BATCH OPERATION PREFLIGHT ---".blue().bold());

            let registry =
                toad_core::ProjectRegistry::load(workspace.active_context.as_deref(), None)
                    .unwrap_or_default();
            let current_fp = workspace.get_fingerprint().unwrap_or(0);

            let projects = if registry.fingerprint == current_fp && !registry.projects.is_empty() {
                registry.projects
            } else {
                println!(
                    "{} Registry is stale/missing. Performing one-time scan...",
                    "INFO:".blue()
                );
                let p = scan_all_projects(&workspace)?;
                let new_registry = toad_core::ProjectRegistry {
                    fingerprint: current_fp,
                    projects: p.clone(),
                    last_sync: std::time::SystemTime::now(),
                };
                let _ = new_registry.save(workspace.active_context.as_deref(), None);
                p
            };

            let targets: Vec<_> = projects
                .into_iter()
                .filter(|p| {
                    let name_match = p.name.to_lowercase().contains(&query.to_lowercase());
                    let tag_match = match tag {
                        Some(t) => {
                            let target = if t.starts_with('#') {
                                t.clone()
                            } else {
                                format!("#{}", t)
                            };
                            p.tags.contains(&target)
                        }
                        None => true,
                    };
                    name_match && tag_match
                })
                .collect();

            if targets.is_empty() {
                println!("No projects found matching '{}'.", query);
                return Ok(());
            }

            println!("Found {} target(s):", targets.len());
            for project in &targets {
                let path_display = if *dry_run {
                    format!(" ({:?})", project.path)
                } else {
                    String::new()
                };
                let tags_display = if project.tags.is_empty() {
                    String::new()
                } else {
                    format!(" {}", project.tags.join(" ").dimmed())
                };
                println!(
                    "  {} {}{}{}",
                    "»".blue(),
                    project.name,
                    tags_display,
                    path_display
                );
            }
            println!("\nCommand: {}", command.yellow().bold());

            // --- Safety Guardrails: Destructive Command Detection ---
            if toad_ops::safety::is_destructive(command) {
                println!(
                    "\n{} This command is potentially {}",
                    "WARNING:".yellow().bold(),
                    "DESTRUCTIVE".red().bold()
                );
                print!("Please type 'PROCEED' to confirm: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if input.trim() != "PROCEED" {
                    println!("Aborted.");
                    return Ok(());
                }
            } else if !*yes && !*dry_run {
                print!("\nExecute on {} projects? [y/N]: ", targets.len());
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            if *dry_run {
                println!("\n{}", "--- DRY RUN COMPLETE ---".green().bold());
                return Ok(());
            }

            println!("\n{}", "--- EXECUTING BATCH ---".blue().bold());
            let pb = ProgressBar::new(targets.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.green/black}] {pos}/{len} ({eta})",
                    )?
                    .progress_chars("■-"),
            );

            let failed = AtomicBool::new(false);
            let results: Vec<_> = targets
                .par_iter()
                .map(|project| {
                    if *fail_fast && failed.load(Ordering::Relaxed) {
                        return (project.name.clone(), None);
                    }

                    let res = toad_ops::shell::run_in_dir(
                        &project.path,
                        command,
                        Duration::from_secs(30),
                    );

                    if let Ok(ref op_res) = res {
                        if op_res.exit_code != 0 {
                            failed.store(true, Ordering::Relaxed);
                        }
                    } else {
                        failed.store(true, Ordering::Relaxed);
                    }

                    pb.inc(1);
                    (project.name.clone(), Some(res))
                })
                .collect();

            pb.finish_and_clear();

            let mut success_count = 0;
            let mut fail_count = 0;
            let mut skip_count = 0;

            for (name, outcome) in results {
                match outcome {
                    Some(Ok(res)) => {
                        print!("Processing {}... ", name);
                        if res.exit_code == 0 {
                            println!("{}", "OK".green());
                            success_count += 1;
                        } else {
                            println!("{} (Code: {})", "FAIL".red(), res.exit_code);
                            if res.timed_out {
                                println!("  {}", "Timed out after 30s".yellow());
                            }
                            if !res.stderr.is_empty() {
                                println!("{}", res.stderr.dimmed());
                            }
                            fail_count += 1;
                        }
                    }
                    Some(Err(e)) => {
                        print!("Processing {}... ", name);
                        println!("{} (Error: {})", "ERROR".red(), e);
                        fail_count += 1;
                    }
                    None => {
                        skip_count += 1;
                    }
                }
            }

            println!("\n{}", "--- BATCH COMPLETE ---".blue().bold());
            println!(
                "{} {} Succeeded | {} {} Failed{}",
                "■".green(),
                success_count,
                "■".red(),
                fail_count,
                if skip_count > 0 {
                    format!(" | {} {} Skipped", "■".yellow(), skip_count)
                } else {
                    String::new()
                }
            );

            // --- Audit Logging ---
            let entry = toad_ops::audit::AuditEntry {
                timestamp: chrono::Local::now().to_rfc3339(),
                command: command.to_string(),
                target_count: targets.len(),
                success_count,
                fail_count,
                skip_count,
                user: whoami::username().unwrap_or_else(|_| "unknown".to_string()),
            };
            if let Err(e) = toad_ops::audit::log_operation(entry) {
                println!(
                    "{} Failed to write to audit log: {}",
                    "WARNING:".yellow(),
                    e
                );
            }
        }
        Commands::Tag {
            project,
            tag,
            query,
            filter_tag,
            harvest,
            yes,
        } => {
            let mut tag_reg = TagRegistry::load(&workspace.tags_path())?;

            let registry =
                toad_core::ProjectRegistry::load(workspace.active_context.as_deref(), None)
                    .unwrap_or_default();
            let current_fp = workspace.get_fingerprint().unwrap_or(0);

            let projects = if registry.fingerprint == current_fp && !registry.projects.is_empty() {
                registry.projects
            } else {
                let p = scan_all_projects(&workspace)?;
                let new_registry = toad_core::ProjectRegistry {
                    fingerprint: current_fp,
                    projects: p.clone(),
                    last_sync: std::time::SystemTime::now(),
                };
                let _ = new_registry.save(workspace.active_context.as_deref(), None);
                p
            };

            let mut targets = Vec::new();

            // 1. Logic for --harvest
            if *harvest {
                println!("{} Harvesting stack tags...", "INFO:".blue().bold());
                for p in projects {
                    let stack_tag = p.stack.to_lowercase();
                    tag_reg.add_tag(&p.name, &stack_tag);
                    targets.push(p.name);
                }
            }
            // 2. Logic for filters (MUST come before specific project logic)
            else if query.is_some() || filter_tag.is_some() {
                // If filters are used, the first positional argument (project)
                // is actually the tag name we want to assign.
                let t_name = match (tag, project) {
                    (Some(t), _) => Some(t),
                    (None, Some(p)) => Some(p),
                    (None, None) => None,
                };

                if let Some(t_name) = t_name {
                    let matching: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match filter_tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if matching.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    println!("Found {} target(s):", matching.len());
                    for p in &matching {
                        println!("  {} {}", "»".blue(), p.name);
                    }

                    if !*yes {
                        print!("\nAssign tag '{}' to these projects? [y/N]: ", t_name);
                        io::stdout().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        if !input.trim().to_lowercase().starts_with('y') {
                            println!("Aborted.");
                            return Ok(());
                        }
                    }

                    for p in matching {
                        tag_reg.add_tag(&p.name, t_name);
                        targets.push(p.name);
                    }
                } else {
                    bail!("Must provide a tag name to assign.");
                }
            }
            // 3. Logic for specific project
            else if let Some(p_name) = project {
                if let Some(t_name) = tag {
                    tag_reg.add_tag(p_name, t_name);
                    targets.push(p_name.clone());
                } else {
                    bail!("Must provide a tag name.");
                }
            } else {
                bail!("Must provide a project name or use filters (--query, --tag, --harvest).");
            }

            if let Err(e) = tag_reg.save(&workspace.tags_path()) {
                println!("{} Failed to save tags: {}", "ERROR:".red().bold(), e);
                return Err(e);
            }

            println!(
                "{} Processed {} projects.",
                "SUCCESS:".green().bold(),
                targets.len()
            );
        }
        Commands::Untag {
            project,
            tag,
            query,
            filter_tag,
            yes,
        } => {
            let mut tag_reg = TagRegistry::load(&workspace.tags_path())?;

            let registry =
                toad_core::ProjectRegistry::load(workspace.active_context.as_deref(), None)
                    .unwrap_or_default();
            let current_fp = workspace.get_fingerprint().unwrap_or(0);

            let projects = if registry.fingerprint == current_fp && !registry.projects.is_empty() {
                registry.projects
            } else {
                let p = scan_all_projects(&workspace)?;
                let new_registry = toad_core::ProjectRegistry {
                    fingerprint: current_fp,
                    projects: p.clone(),
                    last_sync: std::time::SystemTime::now(),
                };
                let _ = new_registry.save(workspace.active_context.as_deref(), None);
                p
            };

            let mut targets = Vec::new();

            // 1. Logic for filters (MUST come before specific project logic)
            if query.is_some() || filter_tag.is_some() {
                // If filters are used, the first positional argument (project)
                // is actually the tag name we want to remove.
                let t_name = match (tag, project) {
                    (Some(t), _) => Some(t),
                    (None, Some(p)) => Some(p),
                    (None, None) => None,
                };

                if let Some(t_name) = t_name {
                    let matching: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match filter_tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if matching.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    println!("Found {} target(s):", matching.len());
                    for p in &matching {
                        println!("  {} {}", "»".blue(), p.name);
                    }

                    if !*yes {
                        print!("\nRemove tag '{}' from these projects? [y/N]: ", t_name);
                        io::stdout().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        if !input.trim().to_lowercase().starts_with('y') {
                            println!("Aborted.");
                            return Ok(());
                        }
                    }

                    for p in matching {
                        tag_reg.remove_tag(&p.name, t_name);
                        targets.push(p.name);
                    }
                } else {
                    bail!("Must provide a tag name to remove.");
                }
            }
            // 2. Logic for specific project
            else if let Some(p_name) = project {
                if let Some(t_name) = tag {
                    tag_reg.remove_tag(p_name, t_name);
                    targets.push(p_name.clone());
                } else {
                    bail!("Must provide a tag name to remove.");
                }
            } else {
                bail!("Must provide a project name or use filters (--query, --tag).");
            }

            if let Err(e) = tag_reg.save(&workspace.tags_path()) {
                println!("{} Failed to save tags: {}", "ERROR:".red().bold(), e);
                return Err(e);
            }

            println!(
                "{} Processed {} projects.",
                "SUCCESS:".green().bold(),
                targets.len()
            );
        }
        Commands::Skill { subcommand } => {
            match subcommand {
                SkillCommand::Sync => {
                    println!("{}", "--- SYNCHRONIZING AI SKILLS ---".green().bold());
                    let fingerprint = workspace.get_fingerprint()?;
                    let projects = scan_all_projects(&workspace)?;

                    // 1. Generate Manifest (Internal Shadow)
                    println!("Updating Semantic Manifest (Shadow)...");
                    let manifest_md = toad_manifest::generate_markdown(&projects, fingerprint);
                    workspace.ensure_shadows()?;
                    fs::write(workspace.manifest_path(), manifest_md)?;

                    // 2. Generate Blueprint Skill
                    println!("Generating Agnostic Architectural Blueprint...");
                    let blueprint = toad_manifest::generate_blueprint(&projects);

                    // 3. Generate CLI Reference Skill
                    println!("Generating Toad CLI Reference Skill...");
                    let mut cmd = Cli::command();
                    let help = cmd.render_help().to_string();
                    let cli_skill = toad_manifest::generate_cli_skill(&help);

                    // Distribution
                    let mut distributed = false;
                    if let Some(name) = &workspace.active_context {
                        if let Ok(Some(config)) = toad_core::GlobalConfig::load(None) {
                            if let Some(ctx) = config.project_contexts.get(name) {
                                if !ctx.ai_vendors.is_empty() {
                                    println!(
                                        "Distributing skills to AI vendors: {}...",
                                        ctx.ai_vendors.join(", ")
                                    );
                                    let skills = vec![
                                        ("toad-blueprint".to_string(), blueprint.clone()),
                                        ("toad-cli".to_string(), cli_skill.clone()),
                                    ];
                                    let synced = toad_ops::workflow::distribute_skills(
                                        &workspace.root,
                                        &ctx.ai_vendors,
                                        skills,
                                    )?;
                                    for path in synced {
                                        println!("  {} Sync: {:?}", "»".green(), path);
                                    }
                                    distributed = true;
                                }
                            }
                        }
                    }

                    if !distributed {
                        // Fallback to agnostic blueprint and cli skill at root
                        let blueprint_path = workspace.root.join("toad-blueprint.md");
                        fs::write(&blueprint_path, &blueprint)?;
                        println!(
                            "{} Agnostic blueprint updated at root: {:?}",
                            "SUCCESS:".green().bold(),
                            blueprint_path
                        );

                        let cli_path = workspace.root.join("toad-cli.md");
                        fs::write(&cli_path, &cli_skill)?;
                        println!(
                            "{} CLI reference skill updated at root: {:?}",
                            "SUCCESS:".green().bold(),
                            cli_path
                        );
                    }
                    println!(
                        "\n{} AI Agent memory is now synchronized.",
                        "SUCCESS:".green().bold()
                    );
                }
                SkillCommand::List => {
                    println!(
                        "{}",
                        "--- REGISTERED AI VENDORS & SKILLS ---".green().bold()
                    );
                    if let Some(name) = &workspace.active_context {
                        if let Ok(Some(config)) = toad_core::GlobalConfig::load(None) {
                            if let Some(ctx) = config.project_contexts.get(name) {
                                if ctx.ai_vendors.is_empty() {
                                    println!("No AI vendors registered for context '{}'.", name);
                                } else {
                                    println!("{:<15} VENDOR", "SLOT");
                                    println!("{:-<15} {:-<20}", "", "");
                                    for vendor in &ctx.ai_vendors {
                                        println!("{:<15} {}", "Skills Slot", vendor.bold());
                                    }
                                }
                            }
                        }
                    } else {
                        println!("No active context found.");
                    }

                    println!("\n--- ACTIVE TOAD SKILLS ---");
                    println!("- toad-blueprint: Architectural & dependency map.");
                    println!("- toad-cli: High-density command reference.");
                }
            }
        }
        Commands::Sync => {
            println!("Scanning projects...");
            let pb = ProgressBar::new_spinner();
            pb.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.green} [{elapsed_precise}] {msg}")?,
            );
            pb.set_message("Discovering projects on disk...");
            pb.enable_steady_tick(Duration::from_millis(100));

            let fingerprint = workspace.get_fingerprint()?;
            let projects = scan_all_projects(&workspace)?;

            pb.set_message("Saving to registry...");
            let registry = toad_core::ProjectRegistry {
                fingerprint,
                projects,
                last_sync: std::time::SystemTime::now(),
            };
            registry.save(workspace.active_context.as_deref(), None)?;

            pb.finish_and_clear();
            println!(
                "{} Registry synchronized ({} projects found).",
                "SUCCESS:".green().bold(),
                registry.projects.len()
            );
        }
        Commands::Strategy { subcommand } => {
            let registry = toad_core::strategy::StrategyRegistry::load()?;

            match subcommand {
                StrategyCommands::List => {
                    println!("{}", "--- ACTIVE STACK STRATEGIES ---".green().bold());
                    for strategy in &registry.strategies {
                        println!(
                            "🌿 {: <10} {} (Priority: {})",
                            strategy.name.bold(),
                            format!("[{}]", strategy.match_files.join(", ")).dimmed(),
                            strategy.priority
                        );
                    }
                }
                StrategyCommands::Info { name } => {
                    let strategy = registry
                        .strategies
                        .iter()
                        .find(|s| s.name.to_lowercase() == name.to_lowercase());
                    if let Some(s) = strategy {
                        println!("{}: {}", "Name".bold(), s.name);
                        println!("{}: {}", "Priority".bold(), s.priority);
                        println!("{}: {}", "Matches".bold(), s.match_files.join(", "));
                        println!("{}: {}", "Artifacts".bold(), s.artifacts.join(", "));
                        println!("{}: {}", "Auto-Tags".bold(), s.tags.join(", "));
                    } else {
                        bail!("Strategy '{}' not found.", name);
                    }
                }
                StrategyCommands::Add {
                    name,
                    match_files,
                    artifacts,
                    tags,
                    priority,
                } => {
                    let new_strategy = toad_core::StackStrategy {
                        name: name.clone(),
                        match_files: match_files
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect(),
                        artifacts: artifacts
                            .clone()
                            .unwrap_or_default()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect(),
                        tags: tags
                            .clone()
                            .unwrap_or_default()
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect(),
                        priority: *priority,
                    };

                    let custom_dir =
                        toad_core::GlobalConfig::config_dir(None)?.join("strategies/custom");
                    fs::create_dir_all(&custom_dir)?;

                    let mut safe_name = name
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                        .collect::<String>();
                    safe_name.truncate(64);
                    let filename = format!("{}.toml", safe_name);
                    let path = custom_dir.join(filename);
                    let content = toml::to_string(&new_strategy)?;
                    fs::write(&path, content)?;

                    println!(
                        "{} Strategy '{}' added and saved to {:?}",
                        "SUCCESS:".green().bold(),
                        name,
                        path
                    );
                }
                StrategyCommands::Remove { name } => {
                    let custom_dir =
                        toad_core::GlobalConfig::config_dir(None)?.join("strategies/custom");
                    let mut safe_name = name
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
                        .collect::<String>();
                    safe_name.truncate(64);
                    let filename = format!("{}.toml", safe_name);
                    let path = custom_dir.join(filename);

                    if path.exists() {
                        fs::remove_file(&path)?;
                        println!("{} Strategy '{}' removed.", "SUCCESS:".green().bold(), name);
                    } else {
                        bail!("Custom strategy '{}' not found or is a built-in.", name);
                    }
                }
            }
        }
        Commands::Clean {
            query,
            tag,
            tier,
            yes,
            dry_run,
        } => {
            println!("{}", "--- 🌊 POND HYGIENE PRE-FLIGHT ---".blue().bold());

            let registry =
                toad_core::ProjectRegistry::load(workspace.active_context.as_deref(), None)
                    .unwrap_or_default();
            let current_fp = workspace.get_fingerprint().unwrap_or(0);

            let projects = if registry.fingerprint == current_fp && !registry.projects.is_empty() {
                registry.projects
            } else {
                let p = scan_all_projects(&workspace)?;
                let new_registry = toad_core::ProjectRegistry {
                    fingerprint: current_fp,
                    projects: p.clone(),
                    last_sync: std::time::SystemTime::now(),
                };
                let _ = new_registry.save(workspace.active_context.as_deref(), None);
                p
            };

            let targets: Vec<_> = projects
                .into_iter()
                .filter(|p| {
                    let name_match = match query {
                        Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                        None => true,
                    };
                    let tag_match = match tag {
                        Some(ref t) => {
                            let target = if t.starts_with('#') {
                                t.clone()
                            } else {
                                format!("#{}", t)
                            };
                            p.tags.contains(&target)
                        }
                        None => true,
                    };
                    let tier_match = match tier {
                        Some(ref tr) => {
                            let tier_str = p.activity.to_string().to_lowercase();
                            tier_str.contains(&tr.to_lowercase())
                        }
                        None => true,
                    };
                    name_match && tag_match && tier_match && !p.artifact_dirs.is_empty()
                })
                .collect();

            if targets.is_empty() {
                println!("No projects found matching filters with artifacts to clean.");
                return Ok(());
            }

            println!("Found {} project(s) to clean:", targets.len());
            let mut total_potential_savings = 0;

            for project in &targets {
                let artifact_set: std::collections::HashSet<&str> =
                    project.artifact_dirs.iter().map(|s| s.as_str()).collect();
                let stats = calculate_project_stats(&project.path, &artifact_set);
                total_potential_savings += stats.artifact_bytes;

                println!(
                    "  {} {} ({}) -> {}",
                    "»".blue(),
                    project.name.bold(),
                    project.stack.dimmed(),
                    format_size(stats.artifact_bytes).yellow()
                );
                for artifact in &project.artifact_dirs {
                    if project.path.join(artifact).exists() {
                        println!("    {} {}", "└─".dimmed(), artifact.dimmed());
                    }
                }
            }

            println!(
                "\n{} Potential Savings: {}",
                "🌿".green(),
                format_size(total_potential_savings).bold().green()
            );

            if !*yes && !*dry_run {
                print!("\nProceed with cleaning? [y/N]: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            if *dry_run {
                println!("\n{}", "--- 🌊 DRY RUN COMPLETE ---".green().bold());
                return Ok(());
            }

            println!("\n{}", "--- 🧹 CLEANING POND ---".blue().bold());
            let pb = ProgressBar::new(targets.len() as u64);
            pb.set_style(
                ProgressStyle::default_bar()
                    .template(
                        "{spinner:.green} [{elapsed_precise}] [{bar:40.green/black}] {pos}/{len} ({eta})",
                    )?
                    .progress_chars("■-"),
            );

            let results: Vec<_> = targets
                .par_iter()
                .map(|project| {
                    let res = toad_ops::clean::clean_project(
                        &project.path,
                        &project.artifact_dirs,
                        false,
                    );
                    pb.inc(1);
                    (project.name.clone(), res)
                })
                .collect();

            pb.finish_and_clear();

            let mut success_count = 0;
            let mut fail_count = 0;
            let mut total_reclaimed = 0;

            for (name, outcome) in results {
                match outcome {
                    Ok(res) => {
                        if res.errors.is_empty() {
                            success_count += 1;
                        } else {
                            println!("{} Issues cleaning {}:", "WARNING:".yellow(), name);
                            for err in res.errors {
                                println!("  - {}", err.red());
                            }
                            fail_count += 1;
                        }
                        total_reclaimed += res.bytes_reclaimed;
                    }
                    Err(e) => {
                        println!("{} Critical error cleaning {}: {}", "ERROR:".red(), name, e);
                        fail_count += 1;
                    }
                }
            }

            println!(
                "\n{} Successfully cleaned {} projects.",
                "🪷".green(),
                success_count
            );
            if fail_count > 0 {
                println!("{} Failed to clean {} projects.", "⚠️".red(), fail_count);
            }
            println!(
                "{} Total Space Reclaimed: {}",
                "🌿".green(),
                format_size(total_reclaimed).bold().green()
            );
        }
        Commands::Docs => {
            println!("Generating programmatic CLI documentation...");
            let mut cmd = Cli::command();
            let help = cmd.render_help().to_string();

            let mut output = String::new();
            output.push_str("# Toad CLI Reference\n\n");
            output.push_str(&format!("> **Version:** `v{}`  \n", VERSION));
            output.push_str("> **Generated by:** `toad docs`  \n");
            output.push_str("> **Last Updated:** 2026-02-06  \n\n");
            output.push_str("```text\n");
            output.push_str(&help);
            output.push_str("\n```\n");

            let docs_path = PathBuf::from("docs/guides/CLI.md");
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
        Commands::Project { subcommand } => {
            let mut config =
                toad_core::GlobalConfig::load(None)?.unwrap_or_else(|| toad_core::GlobalConfig {
                    home_pointer: PathBuf::from("."),
                    active_context: None,
                    project_contexts: std::collections::HashMap::new(),
                });

            match subcommand {
                ProjectCommand::Register {
                    name,
                    path,
                    description,
                    context_type,
                    ai,
                } => {
                    let abs_path = fs::canonicalize(PathBuf::from(path))?;
                    if !abs_path.exists() {
                        bail!("Path does not exist: {:?}", abs_path);
                    }

                    if config.project_contexts.contains_key(name) {
                        bail!("Context '{}' already exists.", name);
                    }

                    let detected_type = if let Some(t) = context_type {
                        toad_core::ContextType::from(*t)
                    } else if abs_path.join(".gitmodules").exists() {
                        toad_core::ContextType::Hub
                    } else if abs_path.join("projects").exists() {
                        toad_core::ContextType::Pond
                    } else {
                        toad_core::ContextType::Generic
                    };

                    let ai_vendors = if let Some(a) = ai {
                        a.split(',').map(|s| s.trim().to_string()).collect()
                    } else {
                        Vec::new()
                    };

                    let ctx = toad_core::ProjectContext {
                        path: abs_path.clone(),
                        description: description.clone(),
                        context_type: detected_type,
                        ai_vendors,
                        registered_at: std::time::SystemTime::now(),
                    };

                    config.project_contexts.insert(name.clone(), ctx);

                    // Create per-context storage
                    let ctx_shadows =
                        toad_core::GlobalConfig::context_dir(name, None)?.join("shadows");
                    fs::create_dir_all(&ctx_shadows)?;

                    config.save(None)?;
                    println!(
                        "{} Context '{}' ({}) registered at {:?}",
                        "SUCCESS:".green().bold(),
                        name,
                        detected_type,
                        abs_path
                    );
                }
                ProjectCommand::Switch { name } => {
                    if !config.project_contexts.contains_key(name) {
                        bail!("Context '{}' not found.", name);
                    }
                    config.active_context = Some(name.clone());
                    config.save(None)?;
                    println!(
                        "{} Switched to context '{}'",
                        "SUCCESS:".green().bold(),
                        name
                    );
                }
                ProjectCommand::Current => {
                    if let Some(name) = &config.active_context {
                        if let Some(ctx) = config.project_contexts.get(name) {
                            println!(
                                "{} Active context: {} ({})",
                                "ACTIVE:".green().bold(),
                                name.bold(),
                                ctx.context_type
                            );
                            println!("  Path:        {:?}", ctx.path);
                            if let Some(desc) = &ctx.description {
                                println!("  Description: {}", desc);
                            }
                            if !ctx.ai_vendors.is_empty() {
                                println!("  AI Vendors:  {}", ctx.ai_vendors.join(", "));
                            }
                        }
                    } else {
                        println!(
                            "{} No active context. Using legacy home: {:?}",
                            "LEGACY:".yellow().bold(),
                            config.home_pointer
                        );
                    }
                }
                ProjectCommand::List => {
                    println!("{}", "--- REGISTERED PROJECT CONTEXTS ---".green().bold());
                    if config.project_contexts.is_empty() {
                        println!("No contexts registered.");
                    } else {
                        // Header
                        println!(
                            "{:<15} {:<10} {:<15} {:<40} ACTIVE",
                            "NAME", "TYPE", "VENDORS", "PATH"
                        );
                        println!("{:-<15} {:-<10} {:-<15} {:-<40} {:-<6}", "", "", "", "", "");

                        let mut names: Vec<_> = config.project_contexts.keys().collect();
                        names.sort();

                        for name in names {
                            let ctx = config.project_contexts.get(name).unwrap();
                            let active = if config.active_context.as_ref() == Some(name) {
                                "✅"
                            } else {
                                ""
                            };
                            let vendors = if ctx.ai_vendors.is_empty() {
                                "-".to_string()
                            } else {
                                ctx.ai_vendors.join(",")
                            };
                            println!(
                                "{:<15} {:<10} {:<15} {:<40?} {:^6}",
                                name.bold(),
                                ctx.context_type.to_string(),
                                vendors,
                                ctx.path,
                                active
                            );
                        }
                    }
                }
                ProjectCommand::Update {
                    name,
                    path,
                    description,
                    context_type,
                    ai,
                } => {
                    let ctx = config
                        .project_contexts
                        .get_mut(name)
                        .ok_or_else(|| anyhow::anyhow!("Context '{}' not found.", name))?;
                    if let Some(p) = path {
                        let abs_path = fs::canonicalize(PathBuf::from(p))?;
                        ctx.path = abs_path;
                    }
                    if let Some(d) = description {
                        ctx.description = Some(d.clone());
                    }
                    if let Some(t) = context_type {
                        ctx.context_type = toad_core::ContextType::from(*t);
                    }
                    if let Some(a) = ai {
                        ctx.ai_vendors = a.split(',').map(|s| s.trim().to_string()).collect();
                    }
                    config.save(None)?;
                    println!("{} Context '{}' updated.", "SUCCESS:".green().bold(), name);
                }
                ProjectCommand::Delete { name, yes } => {
                    if config.active_context.as_ref() == Some(name) {
                        println!(
                            "{} Cannot delete the active context. Switch to another context first.",
                            "ERROR:".red().bold()
                        );
                        return Ok(());
                    }

                    if !config.project_contexts.contains_key(name) {
                        bail!("Context '{}' not found.", name);
                    }

                    if !*yes {
                        print!(
                            "Are you sure you want to delete context '{}' and all its cached data? [y/N]: ",
                            name
                        );
                        io::stdout().flush()?;
                        let mut input = String::new();
                        io::stdin().read_line(&mut input)?;
                        if !input.trim().to_lowercase().starts_with('y') {
                            println!("Aborted.");
                            return Ok(());
                        }
                    }

                    config.project_contexts.remove(name);

                    // Remove per-context storage
                    let ctx_dir = toad_core::GlobalConfig::context_dir(name, None)?;
                    if ctx_dir.exists() {
                        fs::remove_dir_all(&ctx_dir)?;
                    }

                    config.save(None)?;
                    println!("{} Context '{}' removed.", "SUCCESS:".green().bold(), name);
                }
                ProjectCommand::Info { name } => {
                    let ctx = config
                        .project_contexts
                        .get(name)
                        .ok_or_else(|| anyhow::anyhow!("Context '{}' not found.", name))?;
                    println!("{}: {}", "Name".bold(), name);
                    println!("{}: {}", "Type".bold(), ctx.context_type);
                    println!("{}: {:?}", "Path".bold(), ctx.path);
                    println!(
                        "{}: {}",
                        "Description".bold(),
                        ctx.description.as_deref().unwrap_or("-")
                    );
                    println!("{}: {:?}", "Registered".bold(), ctx.registered_at);
                    let active = if config.active_context.as_ref() == Some(name) {
                        "Yes"
                    } else {
                        "No"
                    };
                    println!("{}: {}", "Active".bold(), active);
                }
            }
        }
        Commands::Ggit { subcommand } => {
            let registry =
                toad_core::ProjectRegistry::load(workspace.active_context.as_deref(), None)
                    .unwrap_or_default();
            let current_fp = workspace.get_fingerprint().unwrap_or(0);

            let projects = if registry.fingerprint == current_fp && !registry.projects.is_empty() {
                registry.projects
            } else {
                scan_all_projects(&workspace)?
            };

            match subcommand {
                GgitCommand::Status { query, tag } => {
                    println!("{}", "--- MULTI-REPO GIT STATUS ---".green().bold());

                    let targets: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if targets.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    // For Hub contexts, we also want to show submodule status as peers
                    let mut all_repos = Vec::new();
                    for p in targets {
                        // The primary project
                        all_repos.push((p.name.clone(), p.path.clone(), p.vcs_status.clone()));

                        // Its submodules
                        for sub in p.submodules {
                            all_repos.push((
                                format!("{} > {}", p.name, sub.name),
                                workspace.root.join(&sub.path),
                                sub.vcs_status.clone(),
                            ));
                        }
                    }

                    // Header
                    println!("{:<40} {:<15} BRANCH", "REPOSITORY", "STATUS");
                    println!("{:-<40} {:-<15} {:-<20}", "", "", "");

                    for (name, path, status) in all_repos {
                        let branch = toad_git::branch::current_branch(&path).unwrap_or_default();
                        println!("{:<40} {:<15} {}", name.bold(), status, branch.cyan());
                    }
                }
                GgitCommand::Commit {
                    message,
                    query,
                    tag,
                    cascade,
                    fail_fast,
                } => {
                    println!("{}", "--- MULTI-REPO GIT COMMIT ---".blue().bold());

                    let targets: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if targets.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    let mut results = Vec::new();
                    let mut submodule_failed = false;
                    let mut submodule_changed = false;

                    for p in &targets {
                        let mut project_sub_failed = false;
                        // 1. Commit submodules first
                        for sub in &p.submodules {
                            let sub_path = workspace.root.join(&sub.path);
                            if toad_git::commit::is_dirty(&sub_path)? {
                                println!("Committing submodule: {}", sub.name.cyan());
                                let res = toad_git::commit::commit(&sub_path, message, &sub.name)?;
                                if !res.success {
                                    submodule_failed = true;
                                    project_sub_failed = true;
                                } else {
                                    submodule_changed = true;
                                }
                                results.push(res);

                                if *fail_fast && submodule_failed {
                                    break;
                                }
                            }
                        }

                        if *fail_fast && submodule_failed {
                            break;
                        }

                        // 2. Commit the project itself (Block if its own submodules failed)
                        if project_sub_failed {
                            println!(
                                "{} Skipping project {} because its submodules failed.",
                                "WARN:".yellow(),
                                p.name.cyan()
                            );
                            continue;
                        }

                        if toad_git::commit::is_dirty(&p.path)? {
                            println!("Committing project: {}", p.name.cyan());
                            let res = toad_git::commit::commit(&p.path, message, &p.name)?;
                            if !res.success && *fail_fast {
                                results.push(res);
                                break;
                            }
                            results.push(res);
                        }
                    }

                    // 3. Cascade to Hub Root if requested (Block if ANY submodule failed)
                    if *cascade && submodule_changed {
                        if submodule_failed {
                            println!(
                                "{} Aborting Hub Root cascade because one or more submodules failed.",
                                "ERROR:".red().bold()
                            );
                        } else {
                            let root_path = &workspace.root;
                            if toad_git::commit::is_dirty(root_path)? {
                                println!("Cascading commit to Hub Root...");
                                let res = toad_git::commit::commit(root_path, message, "Hub Root")?;
                                results.push(res);
                            }
                        }
                    }

                    // Summary
                    println!("\n--- COMMIT SUMMARY ---");
                    let mut any_fail = false;
                    for res in results {
                        let status = if res.success {
                            "OK".green()
                        } else {
                            any_fail = true;
                            "FAIL".red()
                        };
                        println!("{:<30} {}", res.project_name.bold(), status);
                        if !res.success {
                            println!("  Error: {}", res.stderr.dimmed());
                        }
                    }

                    if any_fail {
                        std::process::exit(1);
                    }
                }
                GgitCommand::Push {
                    query,
                    tag,
                    fail_fast,
                } => {
                    println!("{}", "--- MULTI-REPO GIT PUSH ---".blue().bold());
                    let targets: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if targets.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    let mut results = Vec::new();
                    let mut any_sub_failed = false;

                    for p in targets {
                        let mut project_sub_failed = false;
                        // Push submodules first
                        for sub in p.submodules {
                            let sub_path = workspace.root.join(&sub.path);
                            println!("Pushing submodule: {}", sub.name.cyan());
                            let res = toad_git::remote::push(&sub_path, &sub.name, None, None)?;
                            if !res.success {
                                any_sub_failed = true;
                                project_sub_failed = true;
                            }
                            results.push(res);

                            if *fail_fast && any_sub_failed {
                                break;
                            }
                        }

                        if *fail_fast && any_sub_failed {
                            break;
                        }

                        // Push project (Block if its own submodules failed)
                        if project_sub_failed {
                            println!(
                                "{} Skipping push for project {} because its submodules failed.",
                                "WARN:".yellow(),
                                p.name.cyan()
                            );
                            continue;
                        }

                        println!("Pushing project: {}", p.name.cyan());
                        let res = toad_git::remote::push(&p.path, &p.name, None, None)?;
                        if !res.success && *fail_fast {
                            results.push(res);
                            break;
                        }
                        results.push(res);
                    }

                    // Summary
                    println!("\n--- PUSH SUMMARY ---");
                    let mut any_fail = false;
                    for res in results {
                        let status = if res.success {
                            "OK".green()
                        } else {
                            any_fail = true;
                            "FAIL".red()
                        };
                        println!("{:<30} {}", res.project_name.bold(), status);
                    }

                    if any_fail {
                        std::process::exit(1);
                    }
                }
                GgitCommand::Pull {
                    query,
                    tag,
                    fail_fast,
                } => {
                    println!("{}", "--- MULTI-REPO GIT PULL ---".blue().bold());
                    let targets: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if targets.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    let mut results = Vec::new();
                    let mut any_failed = false;

                    for p in targets {
                        // Pull project first
                        println!("Pulling project: {}", p.name.cyan());
                        let res = toad_git::remote::pull(&p.path, &p.name)?;
                        let project_failed = !res.success;
                        if project_failed {
                            any_failed = true;
                        }
                        results.push(res);

                        if project_failed {
                            if *fail_fast {
                                break;
                            }
                            println!(
                                "{} Skipping submodules for project {} because project pull failed.",
                                "WARN:".yellow(),
                                p.name.cyan()
                            );
                            continue;
                        }

                        // Pull submodules
                        for sub in p.submodules {
                            let sub_path = workspace.root.join(&sub.path);
                            println!("Pulling submodule: {}", sub.name.cyan());
                            let res = toad_git::remote::pull(&sub_path, &sub.name)?;
                            if !res.success {
                                any_failed = true;
                            }
                            results.push(res);

                            if *fail_fast && any_failed {
                                break;
                            }
                        }

                        if *fail_fast && any_failed {
                            break;
                        }
                    }

                    // Summary
                    println!("\n--- PULL SUMMARY ---");
                    let mut any_fail = false;
                    for res in results {
                        let status = if res.success {
                            "OK".green()
                        } else {
                            any_fail = true;
                            "FAIL".red()
                        };
                        println!("{:<30} {}", res.project_name.bold(), status);
                    }

                    if any_fail {
                        std::process::exit(1);
                    }
                }
                GgitCommand::Checkout {
                    branch,
                    create,
                    query,
                    tag,
                    fail_fast,
                } => {
                    println!("{}", "--- MULTI-REPO GIT CHECKOUT ---".blue().bold());
                    let targets: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if targets.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    let mut results = Vec::new();
                    let mut any_failed = false;

                    for p in targets {
                        // 1. Checkout project
                        println!("Checking out {} in project: {}...", branch.cyan(), p.name.cyan());
                        let res = toad_git::branch::checkout(&p.path, &branch, &p.name, *create)?;
                        if !res.success {
                            any_failed = true;
                        }
                        results.push(res);

                        if *fail_fast && any_failed {
                            break;
                        }

                        // 2. Checkout submodules
                        for sub in p.submodules {
                            let sub_path = workspace.root.join(&sub.path);
                            println!("Checking out {} in submodule: {}...", branch.cyan(), sub.name.cyan());
                            let sub_res = toad_git::branch::checkout(&sub_path, &branch, &sub.name, *create)?;
                            if !sub_res.success {
                                any_failed = true;
                            }
                            results.push(sub_res);

                            if *fail_fast && any_failed {
                                break;
                            }
                        }

                        if *fail_fast && any_failed {
                            break;
                        }
                    }

                    // Summary
                    println!("\n--- CHECKOUT SUMMARY ---");
                    let mut any_fail = false;
                    for res in results {
                        let status = if res.success {
                            "OK".green()
                        } else {
                            any_fail = true;
                            "FAIL".red()
                        };
                        println!("{:<30} {}", res.project_name.bold(), status);
                    }

                    if any_fail {
                        std::process::exit(1);
                    }
                }
                GgitCommand::Sync { query, tag, force } => {
                    println!("{}", "--- ECOSYSTEM SYNC & ALIGN ---".blue().bold());

                    let targets: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if targets.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    let mut preflight_results = Vec::new();
                    let mut any_issues = false;

                    // 1. Pre-flight Check
                    println!("Running safety checks...");
                    for p in &targets {
                        // Project check
                        let res = toad_git::sync::preflight_check(&p.path, &p.name, None, None)?;
                        if !res.issues.is_empty() {
                            any_issues = true;
                        }
                        preflight_results.push(res);

                        // Submodule checks
                        for sub in &p.submodules {
                            let sub_path = workspace.root.join(&sub.path);
                            let sub_res = toad_git::sync::preflight_check(
                                &sub_path,
                                &format!("{} > {}", p.name, sub.name),
                                Some(&p.path),
                                Some(&sub.path),
                            )?;
                            if !sub_res.issues.is_empty() {
                                any_issues = true;
                            }
                            preflight_results.push(sub_res);
                        }
                    }

                    if any_issues && !*force {
                        println!("\n{} Safety checks failed:", "ERROR:".red().bold());
                        for res in preflight_results {
                            if !res.issues.is_empty() {
                                println!("  » {}:", res.project_name.cyan());
                                for issue in res.issues {
                                    println!("    - {}", issue.yellow());
                                }
                            }
                        }
                        println!(
                            "\nUse {} to skip safety checks (Dangerous).",
                            "--force".bold()
                        );
                        std::process::exit(1);
                    }

                    // 2. Perform Sync
                    println!("\nSynchronizing repositories...");
                    let mut results = Vec::new();
                    for p in targets {
                        // Pull project
                        println!("Updating project: {}", p.name.cyan());
                        let res = toad_git::remote::pull(&p.path, &p.name)?;
                        results.push(res);

                        // Sync submodules
                        if !p.submodules.is_empty() {
                            println!("Aligning submodules for {}...", p.name.cyan());
                            // git submodule update --init --recursive
                            let sub_res = toad_git::run_git(
                                &p.path,
                                &["submodule", "update", "--init", "--recursive"],
                                &format!("{} (submodules)", p.name),
                            )?;
                            results.push(sub_res);
                        }
                    }

                    // Summary
                    println!("\n--- SYNC SUMMARY ---");
                    let mut any_fail = false;
                    for res in results {
                        let status = if res.success {
                            "OK".green()
                        } else {
                            any_fail = true;
                            "FAIL".red()
                        };
                        println!("{:<40} {}", res.project_name.bold(), status);
                        if !res.success {
                            println!("  Error: {}", res.stderr.dimmed());
                        }
                    }

                    if any_fail {
                        std::process::exit(1);
                    }
                }
                GgitCommand::Branches { query, tag, all } => {
                    println!("{}", "--- MULTI-REPO BRANCH LIST ---".green().bold());

                    let targets: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if targets.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    for p in targets {
                        println!("\n{} {}", "»".blue(), p.name.bold());

                        // Local branches
                        let local = toad_git::branches::list_local_branches(&p.path)?;
                        for b in local {
                            let current_marker = if b.is_current { "*" } else { " " };
                            let color_name = if b.is_current {
                                b.name.green().bold()
                            } else {
                                b.name.normal()
                            };
                            println!("  {} {}", current_marker.green(), color_name);
                        }

                        // Remote branches if requested
                        if *all {
                            let remote = toad_git::branches::list_remote_branches(&p.path)?;
                            for b in remote {
                                println!("    {} {}", "remote:".dimmed(), b.name.red());
                            }
                        }

                        // Submodules
                        for sub in p.submodules {
                            let sub_path = workspace.root.join(&sub.path);
                            let sub_branch =
                                toad_git::branch::current_branch(&sub_path).unwrap_or_default();
                            println!(
                                "  {} {} ({})",
                                "└─".dimmed(),
                                sub.name.cyan(),
                                sub_branch.dimmed()
                            );
                        }
                    }
                }
                GgitCommand::Align { query, tag } => {
                    println!("{}", "--- SUBMODULE ALIGNMENT ---".blue().bold());

                    let targets: Vec<_> = projects
                        .into_iter()
                        .filter(|p| {
                            let name_match = match query {
                                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                                None => true,
                            };
                            let tag_match = match tag {
                                Some(ref t) => {
                                    let target = if t.starts_with('#') {
                                        t.clone()
                                    } else {
                                        format!("#{}", t)
                                    };
                                    p.tags.contains(&target)
                                }
                                None => true,
                            };
                            name_match && tag_match
                        })
                        .collect();

                    if targets.is_empty() {
                        println!("No projects found matching filters.");
                        return Ok(());
                    }

                    let mut results = Vec::new();
                    for p in targets {
                        if p.submodules.is_empty() {
                            continue;
                        }

                        println!("Aligning submodules for {}...", p.name.cyan());
                        for sub in p.submodules {
                            let res =
                                toad_git::align::align_submodule(&p.path, &sub.path, &sub.name)?;
                            results.push(res);
                        }
                    }

                    // Summary
                    println!("\n--- ALIGNMENT SUMMARY ---");
                    for res in results {
                        let status = if res.success {
                            "OK".green()
                        } else {
                            "FAIL".red()
                        };
                        println!("{:<30} {}", res.project_name.bold(), status);
                    }
                }
            }
        }
        Commands::Cw { subcommand } => {
            let mut registry = toad_core::WorkflowRegistry::load(None)?;

            match subcommand {
                CwCommand::Register {
                    name,
                    script,
                    description,
                } => {
                    let script_path = fs::canonicalize(PathBuf::from(script))?;
                    toad_ops::workflow::register_workflow(
                        &mut registry,
                        name.clone(),
                        script_path.clone(),
                        description.clone(),
                    )?;
                    registry.save(None)?;
                    println!(
                        "{} Workflow '{}' registered at {:?}",
                        "SUCCESS:".green().bold(),
                        name,
                        script_path
                    );
                }
                CwCommand::Run { name, args } => {
                    let workflow = registry
                        .workflows
                        .get(&name.to_lowercase())
                        .ok_or_else(|| anyhow::anyhow!("Workflow '{}' not found.", name))?;

                    println!(
                        "{} Executing workflow: {}...",
                        "INFO:".blue().bold(),
                        name.cyan()
                    );
                    let exit_code = toad_ops::workflow::run_workflow(workflow, args)?;
                    std::process::exit(exit_code);
                }
                CwCommand::List => {
                    println!("{}", "--- REGISTERED CUSTOM WORKFLOWS ---".green().bold());
                    if registry.workflows.is_empty() {
                        println!("No workflows registered.");
                    } else {
                        println!("{:<15} {:<40} DESCRIPTION", "NAME", "SCRIPT");
                        println!("{:-<15} {:-<40} {:-<30}", "", "", "");

                        let mut names: Vec<_> = registry.workflows.keys().collect();
                        names.sort();

                        for name in names {
                            let wf = registry.workflows.get(name).unwrap();
                            let desc = wf.description.as_deref().unwrap_or("-");
                            println!("{:<15} {:<40?} {}", name.bold(), wf.script_path, desc);
                        }
                    }
                }
                CwCommand::Info { name } => {
                    let wf = registry
                        .workflows
                        .get(&name.to_lowercase())
                        .ok_or_else(|| anyhow::anyhow!("Workflow '{}' not found.", name))?;

                    println!("{}: {}", "Name".bold(), wf.name);
                    println!("{}: {:?}", "Script".bold(), wf.script_path);
                    println!(
                        "{}: {}",
                        "Description".bold(),
                        wf.description.as_deref().unwrap_or("-")
                    );
                    println!("{}: {:?}", "Registered".bold(), wf.registered_at);
                }
                CwCommand::Delete { name } => {
                    if registry.workflows.remove(&name.to_lowercase()).is_some() {
                        registry.save(None)?;
                        println!("{} Workflow '{}' removed.", "SUCCESS:".green().bold(), name);
                    } else {
                        bail!("Workflow '{}' not found.", name);
                    }
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
