use crate::commands::utils::{normalize_tag, resolve_projects};
use crate::ui::IndicatifReporter;
use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, Write};
use toad_core::Workspace;
use toad_ops::clean::execute_batch_clean;
use toad_ops::stats::{calculate_project_stats, format_size};

pub fn handle(
    workspace: &Workspace,
    query: Option<String>,
    tag: Option<String>,
    tier: Option<String>,
    yes: bool,
    dry_run: bool,
) -> Result<()> {
    println!("{}", "--- 🌊 POND HYGIENE PRE-FLIGHT ---".blue().bold());

    let projects = resolve_projects(workspace)?;

    let targets: Vec<_> = projects
        .into_iter()
        .filter(|p| {
            let name_match = match query {
                Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                None => true,
            };
            let tag_match = match tag {
                Some(ref t) => {
                    let target = normalize_tag(t);
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

    if !yes && !dry_run {
        print!("\nProceed with cleaning? [y/N]: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            println!("Aborted.");
            return Ok(());
        }
    }

    if dry_run {
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

    let reporter = IndicatifReporter { pb };
    let report = execute_batch_clean(&targets, &reporter);

    for (name, outcome) in report.results {
        match outcome {
            Ok(res) => {
                if !res.errors.is_empty() {
                    println!("{} Issues cleaning {}:", "WARNING:".yellow(), name);
                    for err in res.errors {
                        println!("  - {}", err.red());
                    }
                }
            }
            Err(e) => {
                println!("{} Critical error cleaning {}: {}", "ERROR:".red(), name, e);
            }
        }
    }

    println!("\n{}", "--- CLEANING COMPLETE ---".blue().bold());
    println!(
        "{} {} Succeeded | {} {} Failed",
        "■".green(),
        report.success_count,
        "■".red(),
        report.fail_count
    );
    println!(
        "{} Total Reclaimed: {}",
        "🌿".green(),
        format_size(report.total_reclaimed).bold().green()
    );

    Ok(())
}
