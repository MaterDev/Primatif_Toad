use anyhow::Result;
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use toad_core::{ToadResult, Workspace};

pub fn handle(workspace_discovered: ToadResult<Workspace>, path: Option<String>) -> Result<()> {
    let mut config =
        toad_core::GlobalConfig::load(None)?.unwrap_or_else(|| toad_core::GlobalConfig {
            home_pointer: PathBuf::from("."),
            active_context: None,
            project_contexts: std::collections::HashMap::new(),
        });

    if let Some(new_path) = path {
        let p = PathBuf::from(new_path);
        if !p.exists() {
            anyhow::bail!("Path does not exist: {:?}", p);
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

        let ctx_shadows = toad_core::GlobalConfig::context_dir(&name, None)?.join("shadows");
        fs::create_dir_all(&ctx_shadows)?;

        config.save(None)?;
        println!(
            "{} Anchor updated and registered as context '{}' at: {:?}",
            "SUCCESS:".green().bold(),
            name,
            abs_path
        );
    } else {
        match &workspace_discovered {
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
    Ok(())
}
