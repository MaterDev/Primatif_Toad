use anyhow::Result;
use clap::Parser;
use colored::*;
use std::path::PathBuf;
use toad_core::Workspace;

mod cli;
mod commands;
mod suggestions;
mod ui;

use cli::{Cli, Commands};

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            // Check if this is an unknown subcommand error
            let error_msg = e.to_string();
            if error_msg.contains("unrecognized subcommand") || error_msg.contains("invalid subcommand") {
                // Extract the attempted command from the error message
                if let Some(attempted) = extract_attempted_command(&error_msg) {
                    let valid_commands = get_valid_commands();
                    if let Some(suggestion) = suggestions::suggest_command(&attempted, &valid_commands) {
                        eprintln!("{}", e);
                        eprintln!("\n{} {}", "Hint:".yellow().bold(), suggestion);
                        std::process::exit(2);
                    }
                }
            }
            // For other errors, use default clap error handling
            e.exit();
        }
    };

    // --- Context Discovery ---
    let discovered = Workspace::discover();

    // Commands that don't require a valid workspace
    let is_bootstrap = matches!(
        &cli.command,
        Commands::Home { .. }
            | Commands::Version
            | Commands::List
            | Commands::Docs
            | Commands::Doctor
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

    // --- Context Health Check & Auto-Sync ---
    let is_manifest_cmd = matches!(&cli.command, Commands::Manifest { .. });
    let stored_fp = workspace.stored_fingerprint();

    if let Ok(current_fp) = workspace.get_fingerprint() {
        if current_fp != stored_fp && !is_manifest_cmd && !is_bootstrap {
            if cli.no_sync {
                println!(
                    "{} Context is stale. Run 'toad manifest' to re-sync intuition.",
                    "WARNING:".yellow().bold()
                );
            } else {
                // Opportunistic Refresh
                if !cli.json {
                    println!(
                        "{} Intuition is stale. Refreshing context...",
                        "INFO:".blue().bold()
                    );
                }
                commands::manifest::handle(&workspace, false, false, true, None)?;
                if !cli.json {
                    println!("{} Context synchronized.", "SUCCESS:".green().bold());
                }
            }
        }
    }

    match &cli.command {
        Commands::Create { name, dry_run, yes } => {
            commands::create::handle(&workspace, name, *dry_run, *yes)?;
        }
        Commands::Reveal { query, tag } => {
            let result = commands::reveal::handle(&workspace, query.clone(), tag.clone())?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                ui::format_search_results(&result);
            }
        }
        Commands::Status { query, tag } => {
            let (result, diagnostics) = commands::status::handle(&workspace, query.clone(), tag.clone())?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                ui::format_status_report(&result, &diagnostics, query.as_deref(), tag.as_deref());
            }
        }
        Commands::Stats { query, tag, all } => {
            let result = commands::stats::handle(&workspace, query.clone(), tag.clone(), *all)?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                ui::format_analytics_report(&result, query.as_deref(), tag.as_deref(), *all);
            }
        }
        Commands::Home { path, yes } => {
            let result = commands::home::handle(discovered, path.clone(), *yes)?;
            match result {
                Some(report) if report.is_new && !report.already_registered => {
                    // This means we need confirmation
                    println!(
                        "{} Path {:?} does not contain a '.toad-root' marker.",
                        "WARNING:".yellow().bold(),
                        report.path
                    );
                    print!("Initialize as a new Toad home? [y/N]: ");
                    use std::io::{self, Write};
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    if input.trim().to_lowercase().starts_with('y') {
                        // Re-run with yes=true
                        let confirmed_result = commands::home::handle(
                            toad_core::Workspace::discover(), // re-discover
                            path.clone(),
                            true,
                        )?;
                        if cli.json {
                            println!("{}", serde_json::to_string_pretty(&confirmed_result)?);
                        } else {
                            ui::format_home_report(confirmed_result);
                        }
                    } else {
                        println!("Aborted.");
                    }
                }
                result => {
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&result)?);
                    } else {
                        ui::format_home_report(result);
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
            if !cli.json {
                println!("{}", "--- BATCH OPERATION PREFLIGHT ---".blue().bold());
            }

            let projects = commands::utils::resolve_projects(&workspace)?;
            let targets =
                commands::utils::filter_projects(projects, Some(query.as_str()), tag.as_deref());

            if targets.is_empty() {
                if cli.json {
                    println!(
                        "{}",
                        serde_json::json!({ "status": "error", "message": format!("No projects found matching '{}'", query) })
                    );
                } else {
                    println!("No projects found matching '{}'.", query);
                }
                return Ok(());
            }

            if !cli.json {
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
            }

            // --- Safety Guardrails ---
            let mut mismatched = Vec::new();
            for p in &targets {
                if toad_ops::safety::is_stack_mismatch(command, &p.stack) {
                    mismatched.push(p.name.clone());
                }
            }

            if !*yes && !*dry_run {
                use std::io::{self, Write};
                if !mismatched.is_empty() {
                    println!(
                        "\n{} Command stack mismatch detected for: {}",
                        "WARNING:".yellow().bold(),
                        mismatched.join(", ").cyan()
                    );
                    println!(
                        "You are running a stack-specific command on projects that don't match."
                    );
                    print!("Please type 'PROCEED' to confirm: ");
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    if input.trim() != "PROCEED" {
                        println!("Aborted.");
                        return Ok(());
                    }
                } else if toad_ops::safety::is_destructive(command) {
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
                } else {
                    print!("\nExecute on {} projects? [y/N]: ", targets.len());
                    io::stdout().flush()?;
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    if !input.trim().to_lowercase().starts_with('y') {
                        println!("Aborted.");
                        return Ok(());
                    }
                }
            }

            if *dry_run {
                if !cli.json {
                    println!("\n{}", "--- DRY RUN COMPLETE ---".green().bold());
                } else {
                    println!(
                        "{}",
                        serde_json::json!({ "status": "dry_run", "targets": targets.len() })
                    );
                }
                return Ok(());
            }

            if !cli.json {
                println!("\n{}", "--- EXECUTING BATCH ---".blue().bold());
            }

            let report = toad_ops::execute_batch_operation(&targets, command, *fail_fast);

            if cli.json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                ui::format_batch_report(&report);
            }

            // Post-mutation auto-sync
            if !dry_run && !cli.no_sync {
                commands::manifest::handle(&workspace, false, false, true, None)?;
            }

            // Audit Log
            let _ = commands::do_cmd::log_audit(targets.len(), command.clone(), &report);
        }
        Commands::Tag {
            project,
            tag,
            query,
            filter_tag,
            harvest,
            yes,
        } => {
            commands::tag::handle_tag(
                &workspace,
                project.clone(),
                tag.clone(),
                query.clone(),
                filter_tag.clone(),
                *harvest,
                *yes,
            )?;
            if !cli.no_sync {
                commands::manifest::handle(&workspace, false, false, true, None)?;
            }
        }
        Commands::Untag {
            project,
            tag,
            query,
            filter_tag,
            yes,
        } => {
            commands::tag::handle_untag(
                &workspace,
                project.clone(),
                tag.clone(),
                query.clone(),
                filter_tag.clone(),
                *yes,
            )?;
            if !cli.no_sync {
                commands::manifest::handle(&workspace, false, false, true, None)?;
            }
        }
        Commands::Skill { subcommand } => {
            commands::skill::handle(subcommand, &workspace)?;
        }
        Commands::Sync => {
            if cli.json {
                let reporter = toad_core::NoOpReporter;
                let count = commands::sync::handle(&workspace, &reporter)?;
                println!(
                    "{}",
                    serde_json::json!({
                        "status": "success",
                        "projects_synchronized": count
                    })
                );
            } else {
                let reporter = ui::IndicatifReporter::spinner()?;
                reporter.pb.set_message("Scanning projects...");
                let count = commands::sync::handle(&workspace, &reporter)?;
                ui::format_sync_report(count);
            }
        }
        Commands::Strategy { subcommand } => {
            commands::strategy::handle(subcommand)?;
        }
        Commands::Clean {
            query,
            tag,
            tier,
            yes,
            dry_run,
        } => {
            if !cli.json {
                println!("{}", "--- 🌊 POND HYGIENE PRE-FLIGHT ---".blue().bold());
            }

            // For pre-flight, we need to calculate potential savings
            let projects = commands::utils::resolve_projects(&workspace)?;
            let targets =
                commands::utils::filter_projects(projects, query.as_deref(), tag.as_deref());
            let targets: Vec<_> = targets
                .into_iter()
                .filter(|p| {
                    let tier_match = match tier {
                        Some(ref tr) => {
                            let tier_str = p.activity.to_string().to_lowercase();
                            tier_str.contains(&tr.to_lowercase())
                        }
                        None => true,
                    };
                    tier_match && !p.artifact_dirs.is_empty()
                })
                .collect();

            if targets.is_empty() {
                if cli.json {
                    println!(
                        "{}",
                        serde_json::json!({ "status": "error", "message": "No projects found matching filters with artifacts to clean" })
                    );
                } else {
                    println!("No projects found matching filters with artifacts to clean.");
                }
                return Ok(());
            }

            if !cli.json {
                println!("Found {} project(s) to clean:", targets.len());
                let mut total_potential_savings = 0;

                for project in &targets {
                    let artifact_set: std::collections::HashSet<&str> =
                        project.artifact_dirs.iter().map(|s| s.as_str()).collect();
                    let stats =
                        toad_ops::stats::calculate_project_stats(&project.path, &artifact_set);
                    total_potential_savings += stats.artifact_bytes;

                    println!(
                        "  {} {} ({}) -> {}",
                        "»".blue(),
                        project.name.bold(),
                        project.stack.dimmed(),
                        toad_ops::stats::format_size(stats.artifact_bytes).yellow()
                    );
                }

                println!(
                    "\n{} Potential Savings: {}",
                    "🌿".green(),
                    toad_ops::stats::format_size(total_potential_savings)
                        .bold()
                        .green()
                );
            }

            if !*yes && !*dry_run {
                use std::io::{self, Write};
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
                if !cli.json {
                    println!("\n{}", "--- 🌊 DRY RUN COMPLETE ---".green().bold());
                } else {
                    println!(
                        "{}",
                        serde_json::json!({ "status": "dry_run", "targets": targets.len() })
                    );
                }
                return Ok(());
            }

            let report = if cli.json {
                let reporter = toad_core::NoOpReporter;
                commands::clean::handle(
                    &workspace,
                    query.clone(),
                    tag.clone(),
                    tier.clone(),
                    &reporter,
                )?
            } else {
                println!("\n{}", "--- 🧹 CLEANING POND ---".blue().bold());
                let pb = indicatif::ProgressBar::new(targets.len() as u64);
                pb.set_style(
                    indicatif::ProgressStyle::default_bar()
                        .template(
                            "{spinner:.green} [{elapsed_precise}] [{bar:40.green/black}] {pos}/{len} ({eta})",
                        )?
                        .progress_chars("■-"),
                );
                let reporter = ui::IndicatifReporter { pb };
                commands::clean::handle(
                    &workspace,
                    query.clone(),
                    tag.clone(),
                    tier.clone(),
                    &reporter,
                )?
            };

            if cli.json {
                println!("{}", serde_json::to_string_pretty(&report.1)?);
            } else {
                ui::format_clean_report(&report.1);
            }

            if !dry_run && !cli.no_sync {
                commands::manifest::handle(&workspace, false, false, true, None)?;
            }
        }
        Commands::Docs => {
            commands::docs::handle(VERSION)?;
        }
        Commands::Project { subcommand } => {
            commands::project::handle(subcommand)?;
        }
        Commands::Ggit { subcommand } => {
            commands::ggit::handle(subcommand, &workspace, cli.json)?;
        }
        Commands::Cw { subcommand } => {
            commands::cw::handle(subcommand)?;
        }
        Commands::Manifest { json, check } => {
            commands::manifest::handle(&workspace, *json, *check, false, None)?;
        }
        Commands::Doctor => {
            commands::doctor::handle(cli.json)?;
        }
        Commands::Analyze { subcommand } => {
            commands::analyze::handle(&workspace, subcommand, cli.json)?;
        }
        Commands::Context {
            task,
            inspire,
            project,
            compare,
            synthesis,
        } => {
            commands::context::handle(
                &workspace,
                task.clone(),
                inspire.clone(),
                project.clone(),
                compare.clone(),
                *synthesis,
            )?;
        }
        Commands::InitContext {
            force,
            dry_run,
            project,
            no_sync,
        } => {
            commands::init_context::handle(
                &workspace,
                *force,
                *dry_run,
                project.clone(),
                *no_sync,
            )?;
        }
        Commands::List => {
            use clap::CommandFactory;
            let mut cmd = Cli::command();
            cmd.print_help()?;
        }
        Commands::Version => {
            print_banner();
        }
    }

    Ok(())
}

/// Extract the attempted command from a clap error message
fn extract_attempted_command(error_msg: &str) -> Option<String> {
    // Try to extract from "unrecognized subcommand 'xyz'"
    if let Some(start) = error_msg.find("'") {
        if let Some(end) = error_msg[start + 1..].find("'") {
            return Some(error_msg[start + 1..start + 1 + end].to_string());
        }
    }
    None
}

/// Get list of valid top-level commands
fn get_valid_commands() -> Vec<&'static str> {
    vec![
        "create", "reveal", "status", "tag", "untag", "clean", "sync", "manifest",
        "atlas", "doctor", "project", "cw", "strategy", "ggit", "home", "version",
        "list", "docs",
    ]
}
