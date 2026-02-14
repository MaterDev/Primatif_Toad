use crate::cli::CwCommand;
use anyhow::{bail, Result};
use colored::*;
use std::fs;
use std::path::PathBuf;
use toad_core::WorkflowRegistry;

pub fn handle(subcommand: &CwCommand) -> Result<()> {
    let mut registry = WorkflowRegistry::load(None)?;

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
    Ok(())
}
