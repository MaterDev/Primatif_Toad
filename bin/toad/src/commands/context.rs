use anyhow::Result;
use colored::*;
use toad_core::{Workspace, ProjectAtlas};
use crate::commands::utils::resolve_projects;

pub fn handle(workspace: &Workspace, task: Option<String>, inspire: Option<String>, project_name: Option<String>, compare_name: Option<String>, synthesis: bool) -> Result<()> {
    println!("{}", "--- 🌊 TOAD SITUATION REPORT ---".blue().bold());

    let projects = resolve_projects(workspace)?;
    let atlas_path = workspace.atlas_path();
    let atlas: ProjectAtlas = if atlas_path.exists() {
        let content = std::fs::read_to_string(atlas_path)?;
        serde_json::from_str(&content)?
    } else {
        ProjectAtlas::default()
    };

    if synthesis {
        let report = toad_manifest::generate_synthesis(&projects, &atlas);
        println!("\n{}", report);
    } else if let Some(c_name) = compare_name {
        if let Some(p_name) = project_name {
            generate_comparison_report(&projects, &p_name, &c_name);
        } else {
            println!("{} Specify a primary project with --project to compare against.", "ERROR:".red().bold());
        }
    } else if let Some(t) = task {
        generate_task_report(&projects, &atlas, &t, project_name.as_deref());
    } else if let Some(i) = inspire {
        generate_inspiration_report(&projects, &atlas, &i);
    } else {
        println!("{} Provide a --task, --inspire query, or --compare to generate a report.", "INFO:".blue().bold());
    }

    Ok(())
}

fn generate_comparison_report(projects: &[toad_core::ProjectDetail], a: &str, b: &str) {
    let proj_a = projects.iter().find(|p| p.name == a);
    let proj_b = projects.iter().find(|p| p.name == b);

    match (proj_a, proj_b) {
        (Some(p1), Some(p2)) => {
            let preflight = toad_ops::migration::compare_projects(p1, p2);
            println!("{} Comparing {} → {}", "ANALYSIS:".blue().bold(), a.cyan(), b.cyan());
            println!("Compatibility Score: {}%", match preflight.compatibility_score {
                s if s > 80 => format!("{}", s).green(),
                s if s > 50 => format!("{}", s).yellow(),
                s => format!("{}", s).red(),
            });

            if !preflight.mismatches.is_empty() {
                println!("\n{} Potential Hurdles:", "RISKS:".red().bold());
                for m in preflight.mismatches {
                    println!("  - {}", m.yellow());
                }
            }

            if !preflight.matching_capabilities.is_empty() {
                println!("\n{} Shared Patterns:", "SYNERGY:".green().bold());
                for m in preflight.matching_capabilities {
                    println!("  - {}", m.dimmed());
                }
            }
        }
        _ => {
            println!("{} One or both projects not found in registry.", "ERROR:".red().bold());
        }
    }
}

fn generate_task_report(projects: &[toad_core::ProjectDetail], atlas: &ProjectAtlas, task: &str, target_project: Option<&str>) {
    println!("{} Building tailored context for task: {}", "ANALYZING:".blue().bold(), task);
    
    if let Some(name) = target_project {
        if let Some(p) = projects.iter().find(|p| p.name == name) {
            println!("\n{} {}", "TARGET:".green().bold(), name);
            println!("Stack: {}", p.stack.cyan());
            println!("Roles: {}", p.dna.roles.join(", ").dimmed());
            
            // Look for relevant capabilities in other projects
            let task_lower = task.to_lowercase();
            let mut relevant_examples = Vec::new();
            
            for (p_name, dna) in &atlas.dna_map {
                if p_name == name { continue; }
                
                let matches_task = dna.capabilities.iter().any(|c| task_lower.contains(&c.to_lowercase()))
                    || dna.structural_patterns.iter().any(|sp| task_lower.contains(&sp.to_lowercase()));
                
                if matches_task {
                    relevant_examples.push(p_name);
                }
            }

            if !relevant_examples.is_empty() {
                println!("\n{} These projects already implement similar patterns:", "INSPIRATION:".yellow().bold());
                for ex in relevant_examples {
                    println!("  - {}", ex.cyan());
                }
            }
        }
    } else {
        println!("{} Specify a project with --project to get deeper insights.", "TIP:".yellow().bold());
    }
}

fn generate_inspiration_report(projects: &[toad_core::ProjectDetail], _atlas: &ProjectAtlas, query: &str) {
    println!("{} Searching for structural patterns matching: {}", "QUERY:".blue().bold(), query);
    
    let query_lower = query.to_lowercase();
    let mut matches = Vec::new();

    for p in projects {
        let dna = &p.dna;
        let matches_dna = dna.roles.iter().any(|r| r.to_lowercase().contains(&query_lower))
            || dna.capabilities.iter().any(|c| c.to_lowercase().contains(&query_lower))
            || dna.structural_patterns.iter().any(|sp| sp.to_lowercase().contains(&query_lower));
        
        if matches_dna {
            matches.push(p);
        }
    }

    if matches.is_empty() {
        println!("No structural patterns found matching '{}'.", query);
    } else {
        println!("\n{} Matches found in the following projects:", "SUCCESS:".green().bold());
        for p in matches {
            println!("  - {} ({})", p.name.cyan(), p.stack);
            if !p.dna.capabilities.is_empty() {
                println!("    Capabilities: {}", p.dna.capabilities.join(", ").dimmed());
            }
        }
    }
}
