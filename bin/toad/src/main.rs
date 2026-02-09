use anyhow::Result;
use clap::Parser;
use colored::*;
use std::fs;
use std::path::PathBuf;
use toad_core::Workspace;

mod cli;
mod commands;
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
        Commands::Create { name, dry_run, yes } => {
            commands::create::handle(&workspace, name, *dry_run, *yes)?;
        }
        Commands::Reveal { query, tag } => {
            commands::reveal::handle(&workspace, query.clone(), tag.clone())?;
        }
        Commands::Status { query, tag } => {
            commands::status::handle(&workspace, query.clone(), tag.clone())?;
        }
        Commands::Stats { query, tag, all } => {
            commands::stats::handle(&workspace, query.clone(), tag.clone(), *all)?;
        }
        Commands::Home { path, yes } => {
            commands::home::handle(discovered, path.clone(), *yes)?;
        }
        Commands::Do {
            command,
            query,
            tag,
            yes,
            dry_run,
            fail_fast,
        } => {
            commands::do_cmd::handle(
                &workspace,
                command.clone(),
                query.clone(),
                tag.clone(),
                *yes,
                *dry_run,
                *fail_fast,
            )?;
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
        }
        Commands::Skill { subcommand } => {
            commands::skill::handle(subcommand, &workspace)?;
        }
        Commands::Sync => {
            commands::sync::handle(&workspace)?;
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
            commands::clean::handle(
                &workspace,
                query.clone(),
                tag.clone(),
                tier.clone(),
                *yes,
                *dry_run,
            )?;
        }
        Commands::Docs => {
            commands::docs::handle(VERSION)?;
        }
        Commands::Project { subcommand } => {
            commands::project::handle(subcommand)?;
        }
        Commands::Ggit { subcommand } => {
            commands::ggit::handle(subcommand, &workspace)?;
        }
        Commands::Cw { subcommand } => {
            commands::cw::handle(subcommand)?;
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
