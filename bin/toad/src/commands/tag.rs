use crate::commands::utils::{normalize_tag, resolve_projects};
use anyhow::{bail, Result};
use colored::*;
use std::io::{self, Write};
use toad_core::{TagRegistry, Workspace};

pub fn handle_tag(
    workspace: &Workspace,
    project: Option<String>,
    tag: Option<String>,
    query: Option<String>,
    filter_tag: Option<String>,
    harvest: bool,
    yes: bool,
) -> Result<()> {
    let mut tag_reg = TagRegistry::load(&workspace.tags_path())?;
    let projects = resolve_projects(workspace)?;

    let mut targets = Vec::new();

    if harvest {
        println!("{} Harvesting stack tags...", "INFO:".blue().bold());
        for p in projects {
            let stack_tag = p.stack.to_lowercase();
            tag_reg.add_tag(&p.name, &stack_tag);
            targets.push(p.name.clone());

            for sub in p.submodules {
                let sub_stack_tag = sub.stack.to_lowercase();
                tag_reg.add_tag(&sub.name, &sub_stack_tag);
                targets.push(sub.name.clone());
            }
        }
    } else if query.is_some() || filter_tag.is_some() {
        let t_name = match (tag, project) {
            (Some(t), _) => Some(t),
            (None, Some(p)) => Some(p),
            (None, None) => None,
        };

        if let Some(t_name) = t_name {
            let matching: Vec<_> = projects
                .into_iter()
                .filter(|p| {
                    let name_match = match query {
                        Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                        None => true,
                    };
                    let tag_match = match filter_tag {
                        Some(ref t) => {
                            let target = normalize_tag(t);
                            p.tags.contains(&target)
                        }
                        None => true,
                    };
                    name_match && tag_match
                })
                .collect();

            if matching.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }

            println!("Found {} target(s):", matching.len());
            for p in &matching {
                println!("  {} {}", "»".blue(), p.name);
            }

            if !yes {
                print!("\nAssign tag '{}' to these projects? [y/N]: ", t_name);
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            for p in matching {
                tag_reg.add_tag(&p.name, &t_name);
                targets.push(p.name);
            }
        } else {
            bail!("Must provide a tag name to assign.");
        }
    } else if let Some(p_name) = project {
        if let Some(t_name) = tag {
            tag_reg.add_tag(&p_name, &t_name);
            targets.push(p_name);
        } else {
            bail!("Must provide a tag name.");
        }
    } else {
        bail!("Must provide a project name or use filters (--query, --tag, --harvest).");
    }

    if let Err(e) = tag_reg.save(&workspace.tags_path()) {
        println!("{} Failed to save tags: {}", "ERROR:".red().bold(), e);
        return Err(e.into());
    }
    println!(
        "{} Processed {} projects.",
        "SUCCESS:".green().bold(),
        targets.len()
    );
    Ok(())
}

pub fn handle_untag(
    workspace: &Workspace,
    project: Option<String>,
    tag: Option<String>,
    query: Option<String>,
    filter_tag: Option<String>,
    yes: bool,
) -> Result<()> {
    let mut tag_reg = TagRegistry::load(&workspace.tags_path())?;
    let projects = resolve_projects(workspace)?;

    let mut targets = Vec::new();

    if query.is_some() || filter_tag.is_some() {
        let t_name = match (tag, project) {
            (Some(t), _) => Some(t),
            (None, Some(p)) => Some(p),
            (None, None) => None,
        };

        if let Some(t_name) = t_name {
            let matching: Vec<_> = projects
                .into_iter()
                .filter(|p| {
                    let name_match = match query {
                        Some(ref q) => p.name.to_lowercase().contains(&q.to_lowercase()),
                        None => true,
                    };
                    let tag_match = match filter_tag {
                        Some(ref t) => {
                            let target = normalize_tag(t);
                            p.tags.contains(&target)
                        }
                        None => true,
                    };
                    name_match && tag_match
                })
                .collect();

            if matching.is_empty() {
                println!("No projects found matching filters.");
                return Ok(());
            }

            println!("Found {} target(s):", matching.len());
            for p in &matching {
                println!("  {} {}", "»".blue(), p.name);
            }

            if !yes {
                print!("\nRemove tag '{}' from these projects? [y/N]: ", t_name);
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Aborted.");
                    return Ok(());
                }
            }

            for p in matching {
                tag_reg.remove_tag(&p.name, &t_name);
                targets.push(p.name);
            }
        } else {
            bail!("Must provide a tag name to remove.");
        }
    } else if let Some(p_name) = project {
        if let Some(t_name) = tag {
            tag_reg.remove_tag(&p_name, &t_name);
            targets.push(p_name);
        } else {
            bail!("Must provide a tag name to remove.");
        }
    } else {
        bail!("Must provide a project name or use filters (--query, --tag).");
    }

    if let Err(e) = tag_reg.save(&workspace.tags_path()) {
        println!("{} Failed to save tags: {}", "ERROR:".red().bold(), e);
        return Err(e.into());
    }
    println!(
        "{} Processed {} projects.",
        "SUCCESS:".green().bold(),
        targets.len()
    );
    Ok(())
}
