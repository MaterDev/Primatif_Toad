use crate::commands::utils::{normalize_tag, resolve_projects};
use crate::ui;
use anyhow::Result;
use colored::*;
use std::io::{self, Write};
use toad_core::Workspace;

pub fn handle(
    workspace: &Workspace,
    command: String,
    query: String,
    tag: Option<String>,
    yes: bool,
    dry_run: bool,
    fail_fast: bool,
) -> Result<()> {
    println!("{}", "--- BATCH OPERATION PREFLIGHT ---".blue().bold());

    let projects = resolve_projects(workspace)?;

    let targets: Vec<_> = projects
        .into_iter()
        .filter(|p| {
            let name_match = p.name.to_lowercase().contains(&query.to_lowercase());
            let tag_match = match tag {
                Some(ref t) => {
                    let target = normalize_tag(t);
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
        let path_display = if dry_run {
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

    // --- Safety Guardrails ---
    if !yes && !dry_run {
        if toad_ops::safety::is_destructive(&command) {
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

    if dry_run {
        println!("\n{}", "--- DRY RUN COMPLETE ---".green().bold());
        return Ok(());
    }

    println!("\n{}", "--- EXECUTING BATCH ---".blue().bold());
    let report = toad_ops::execute_batch_operation(&targets, &command, fail_fast);
    ui::format_batch_report(&report);

    // --- Audit Logging ---
    let entry = toad_ops::audit::AuditEntry {
        timestamp: chrono::Local::now().to_rfc3339(),
        command: command.clone(),
        target_count: targets.len(),
        success_count: report.success_count,
        fail_count: report.fail_count,
        skip_count: report.skip_count,
        user: whoami::username().unwrap_or_else(|_| "unknown".to_string()),
    };
    if let Err(e) = toad_ops::audit::log_operation(entry) {
        println!(
            "{} Failed to write to audit log: {}",
            "WARNING:".yellow(),
            e
        );
    }

    Ok(())
}
