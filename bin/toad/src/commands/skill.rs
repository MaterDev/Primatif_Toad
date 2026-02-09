use crate::cli::{Cli, SkillCommand};
use anyhow::Result;
use clap::CommandFactory;
use colored::*;
use std::fs;
use toad_core::{GlobalConfig, Workspace};
use toad_discovery::scan_all_projects;

pub fn handle(subcommand: &SkillCommand, workspace: &Workspace) -> Result<()> {
    match subcommand {
        SkillCommand::Sync => {
            println!("{}", "--- SYNCHRONIZING AI SKILLS ---".green().bold());
            let fingerprint = workspace.get_fingerprint()?;
            let projects = scan_all_projects(workspace)?;

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
                if let Ok(Some(config)) = GlobalConfig::load(None) {
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
                if let Ok(Some(config)) = GlobalConfig::load(None) {
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
    Ok(())
}
