use crate::cli::GgitCommand;
use crate::commands::utils::{filter_projects, resolve_projects};
use crate::ui;
use anyhow::Result;
use colored::*;
use toad_core::Workspace;

pub fn handle(subcommand: &GgitCommand, workspace: &Workspace) -> Result<()> {
    let projects = resolve_projects(workspace)?;

    match subcommand {
        GgitCommand::Status { query, tag } => {
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }
            let report = toad_git::generate_multi_repo_status(&targets)?;
            ui::format_multi_repo_status(&report);
        }
        GgitCommand::Commit {
            message,
            query,
            tag,
            cascade,
            fail_fast,
        } => {
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }

            let mut report =
                toad_git::execute_multi_repo_commit(&targets, message, *cascade, *fail_fast)?;

            if *cascade {
                let mut submodule_changed = false;
                let mut submodule_failed = false;
                for res in &report.results {
                    if res.success && res.project_name != "Hub Root" {
                        submodule_changed = true;
                    }
                    if !res.success && res.project_name != "Hub Root" {
                        submodule_failed = true;
                    }
                }

                if submodule_changed
                    && !submodule_failed
                    && toad_git::commit::is_dirty(&workspace.projects_dir)?
                {
                    let res = toad_git::commit::commit(&workspace.projects_dir, message, "Hub Root")?;
                    report.results.push(res);
                }
            }

            ui::format_multi_repo_git_report(&report);
            if report.results.iter().any(|r| !r.success) && *fail_fast {
                std::process::exit(1);
            }
        }
        GgitCommand::Push {
            query,
            tag,
            fail_fast,
        } => {
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }
            let report = toad_git::execute_multi_repo_push(&targets, *fail_fast)?;
            ui::format_multi_repo_git_report(&report);
            if report.results.iter().any(|r| !r.success) && *fail_fast {
                std::process::exit(1);
            }
        }
        GgitCommand::Pull {
            query,
            tag,
            fail_fast,
        } => {
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }
            let report = toad_git::execute_multi_repo_pull(&targets, *fail_fast)?;
            ui::format_multi_repo_git_report(&report);
            if report.results.iter().any(|r| !r.success) && *fail_fast {
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
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }

            let mut results = Vec::new();
            for p in targets {
                println!(
                    "Checking out {} in project: {}...",
                    branch.cyan(),
                    p.name.cyan()
                );
                let res = toad_git::branch::checkout(&p.path, branch, &p.name, *create)?;
                results.push(res);

                for sub in p.submodules {
                    let sub_path = workspace.projects_dir.join(&sub.path);
                    println!(
                        "Checking out {} in submodule: {}...",
                        branch.cyan(),
                        sub.name.cyan()
                    );
                    let sub_res =
                        toad_git::branch::checkout(&sub_path, branch, &sub.name, *create)?;
                    results.push(sub_res);
                }

                if *fail_fast && results.iter().any(|r| !r.success) {
                    break;
                }
            }

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
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }

            // Restore detailed preflight output
            println!("Running safety checks...");
            let mut preflight_results = Vec::new();
            let mut any_issues = false;

            for p in &targets {
                let res = toad_git::sync::preflight_check(&p.path, &p.name, None, None)?;
                if !res.issues.is_empty() {
                    any_issues = true;
                }
                preflight_results.push(res);

                for sub in &p.submodules {
                    let sub_path = workspace.projects_dir.join(&sub.path);
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

            println!("\nSynchronizing repositories...");
            // Use run_git for the recursive sync to match old behavior
            let mut results = Vec::new();
            for p in targets {
                println!("Updating project: {}", p.name.cyan());
                let res = toad_git::remote::pull(&p.path, &p.name)?;
                results.push(res);

                if !p.submodules.is_empty() {
                    println!("Aligning submodules for {}...", p.name.cyan());
                    let sub_res = toad_git::run_git(
                        &p.path,
                        &["submodule", "update", "--init", "--recursive"],
                        &format!("{} (submodules)", p.name),
                    )?;
                    results.push(sub_res);
                }
            }

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
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }

            for p in targets {
                println!("\n{} {}", "»".blue(), p.name.bold());

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

                if *all {
                    let remote = toad_git::branches::list_remote_branches(&p.path)?;
                    for b in remote {
                        println!("    {} {}", "remote:".dimmed(), b.name.red());
                    }
                }

                for sub in p.submodules {
                    let sub_path = workspace.projects_dir.join(&sub.path);
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
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
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
                    let res = toad_git::align::align_submodule(&p.path, &sub.path, &sub.name)?;
                    results.push(res);
                }
            }

            println!("\n--- ALIGNMENT SUMMARY ---");
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
    }

    Ok(())
}
