use crate::cli::ProjectCommand;
use anyhow::{bail, Result};
use colored::*;
use std::fs;
use std::io::{self, Write};
use toad_core::{ContextType, GlobalConfig, ProjectContext};

pub fn handle(subcommand: &ProjectCommand) -> Result<()> {
    let mut config = GlobalConfig::load(None)?.unwrap_or_else(|| GlobalConfig {
        home_pointer: std::path::PathBuf::from("."),
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
            let abs_path = fs::canonicalize(std::path::PathBuf::from(path))?;
            if !abs_path.exists() {
                bail!("Path does not exist: {:?}", abs_path);
            }

            if config.project_contexts.contains_key(name) {
                bail!("Context '{}' already exists.", name);
            }

            let detected_type = if let Some(t) = context_type {
                ContextType::from(*t)
            } else if abs_path.join(".gitmodules").exists() {
                ContextType::Hub
            } else if abs_path.join("projects").exists() {
                ContextType::Pond
            } else {
                ContextType::Generic
            };

            let ai_vendors: Vec<String> = if let Some(a) = ai {
                a.split(',').map(|s| s.trim().to_string()).collect()
            } else {
                Vec::new()
            };

            let ctx = ProjectContext {
                path: abs_path.clone(),
                description: description.clone(),
                context_type: detected_type,
                ai_vendors,
                registered_at: std::time::SystemTime::now(),
            };

            config.project_contexts.insert(name.clone(), ctx);

            // Create per-context storage
            let ctx_shadows = GlobalConfig::context_dir(name, None)?.join("shadows");
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
                let abs_path = fs::canonicalize(std::path::PathBuf::from(p))?;
                ctx.path = abs_path;
            }
            if let Some(d) = description {
                ctx.description = Some(d.to_string());
            }
            if let Some(t) = context_type {
                ctx.context_type = ContextType::from(*t);
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

            let ctx_dir = GlobalConfig::context_dir(name, None)?;
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

    Ok(())
}
