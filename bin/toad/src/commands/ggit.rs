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
                    && toad_git::commit::is_dirty(&workspace.root)?
                {
                    let res = toad_git::commit::commit(&workspace.root, message, "Hub Root")?;
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
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }
            let report =
                toad_git::execute_multi_repo_checkout(&targets, branch, *create, *fail_fast)?;
            ui::format_multi_repo_git_report(&report);
            if report.results.iter().any(|r| !r.success) && *fail_fast {
                std::process::exit(1);
            }
        }
        GgitCommand::Sync { query, tag, force } => {
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }
            match toad_git::execute_multi_repo_sync(&targets, *force, false) {
                Ok(report) => ui::format_multi_repo_git_report(&report),
                Err(e) => {
                    println!("\n{} Safety checks failed:", "ERROR:".red().bold());
                    println!("{}", e.to_string().yellow());
                    println!(
                        "\nUse {} to skip safety checks (Dangerous).",
                        "--force".bold()
                    );
                    std::process::exit(1);
                }
            }
        }
        GgitCommand::Branches { query, tag, all } => {
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }
            let groups = toad_git::generate_multi_repo_branch_report(&targets, *all)?;
            ui::format_multi_repo_branch_report(&groups);
        }
        GgitCommand::Align { query, tag } => {
            let targets = filter_projects(projects, query.as_deref(), tag.as_deref());
            if targets.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }
            let report = toad_git::execute_multi_repo_align(&targets, false)?;
            ui::format_multi_repo_git_report(&report);
        }
    }

    Ok(())
}
