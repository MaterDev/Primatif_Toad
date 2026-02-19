use crate::cli::AnalyzeCommand;
use crate::commands::utils;
use anyhow::Result;
use colored::*;
use toad_core::Workspace;

pub fn handle(workspace: &Workspace, subcommand: &AnalyzeCommand, json: bool) -> Result<()> {
    let projects = utils::resolve_projects(workspace)?;

    match subcommand {
        AnalyzeCommand::Deps { query } => {
            let targets = utils::filter_projects(projects, query.as_deref(), None);
            let graph = toad_ops::analytics::analyze_dependencies(&targets)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&graph)?);
            } else {
                render_deps(&graph);
            }
        }
        AnalyzeCommand::Velocity { days, query } => {
            let targets = utils::filter_projects(projects, query.as_deref(), None);
            if targets.is_empty() {
                println!("No projects found matching query.");
                return Ok(());
            }

            if json {
                let mut results = std::collections::HashMap::new();
                for p in &targets {
                    let velocity = toad_ops::analytics::analyze_velocity(&p.path, *days)?;
                    results.insert(p.name.clone(), velocity);
                }
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                println!("{}", "--- DEVELOPMENT VELOCITY ---".blue().bold());
                for p in &targets {
                    let velocity = toad_ops::analytics::analyze_velocity(&p.path, *days)?;
                    render_velocity(&p.name, &velocity);
                }
            }
        }
        AnalyzeCommand::Debt { query } => {
            let targets = utils::filter_projects(projects, query.as_deref(), None);
            if json {
                let mut results = std::collections::HashMap::new();
                for p in &targets {
                    let debt = toad_ops::analytics::analyze_debt(&p.path)?;
                    results.insert(p.name.clone(), debt);
                }
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                println!("{}", "--- TECHNICAL DEBT ANALYSIS ---".blue().bold());
                for p in &targets {
                    let debt = toad_ops::analytics::analyze_debt(&p.path)?;
                    render_debt(&p.name, &debt);
                }
            }
        }
        AnalyzeCommand::Health { query } => {
            let targets = utils::filter_projects(projects, query.as_deref(), None);
            if json {
                let mut results = std::collections::HashMap::new();
                for p in &targets {
                    let health = toad_ops::analytics::calculate_health_score(p)?;
                    results.insert(p.name.clone(), health);
                }
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                println!("{}", "--- PROJECT HEALTH SCORES ---".blue().bold());
                for p in &targets {
                    let health = toad_ops::analytics::calculate_health_score(p)?;
                    render_health(&p.name, &health);
                }

                let insights = toad_ops::analytics::generate_insights(&targets)?;
                if !insights.is_empty() {
                    println!(
                        "
{}",
                        "--- INSIGHTS & RECOMMENDATIONS ---".yellow().bold()
                    );
                    for insight in insights {
                        println!(
                            "{} {} ({})",
                            "💡".yellow(),
                            insight.title.bold(),
                            insight.severity.dimmed()
                        );
                        println!("   {}", insight.description);
                        println!("   {} {}", "→".blue(), insight.action_item.cyan());
                    }
                }
            }
        }
        AnalyzeCommand::Trends { days: _ } => {
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "status": "planned", "message": "Historical trends analysis requires persistent state (v1.2.0 planned)." })
                );
            } else {
                println!("Historical trends analysis requires persistent state (v1.2.0 planned).");
            }
        }
        AnalyzeCommand::Patterns => {
            if json {
                println!(
                    "{}",
                    serde_json::json!({ "status": "planned", "message": "Cross-project pattern analysis (Coming soon)." })
                );
            } else {
                println!("Cross-project pattern analysis (Coming soon).");
            }
        }
        AnalyzeCommand::Submodules => {
            if json {
                let mut results = Vec::new();
                for p in &projects {
                    for sub in &p.submodules {
                        results.push(serde_json::json!({
                            "project": p.name,
                            "submodule": sub.name,
                            "initialized": sub.initialized,
                            "vcs_status": sub.vcs_status,
                        }));
                    }
                }
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                println!("{}", "--- SUBMODULE HEALTH ---".blue().bold());
                for p in &projects {
                    if !p.submodules.is_empty() {
                        println!(
                            "
{} {}:",
                            "📦".blue(),
                            p.name.bold()
                        );
                        for sub in &p.submodules {
                            let status = if sub.initialized {
                                "✅ Aligned"
                            } else {
                                "⚠️ Uninitialized"
                            };
                            println!("  {} {} - {}", "»".dimmed(), sub.name, status.cyan());
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn render_deps(graph: &toad_core::DependencyGraph) {
    println!("{}", "--- DEPENDENCY ANALYSIS ---".blue().bold());

    println!(
        "
{}",
        "Critical Path (Most Depended Upon):".yellow()
    );
    for (i, name) in graph.critical_path.iter().take(5).enumerate() {
        if let Some(node) = graph.nodes.get(name) {
            println!(
                "  {}. {} ({} dependents)",
                i + 1,
                name.bold(),
                node.dependents.len()
            );
        }
    }

    if !graph.circular_dependencies.is_empty() {
        println!(
            "
{}",
            "Circular Dependencies Detected:".red().bold()
        );
        for cycle in &graph.circular_dependencies {
            println!("  {} {}", "×".red(), cycle.join(" -> ").cyan());
        }
    } else {
        println!(
            "
{}",
            "Health: ✅ No circular dependencies detected".green()
        );
    }

    if !graph.orphaned_projects.is_empty() {
        println!(
            "
{}",
            "Orphaned Projects:".dimmed()
        );
        for name in &graph.orphaned_projects {
            println!("  - {}", name);
        }
    }
}

fn render_velocity(name: &str, v: &toad_core::VelocityMetrics) {
    println!(
        "
{} {}:",
        "🚀".green(),
        name.bold()
    );
    println!("  Commits: {}", v.commit_count.to_string().yellow());
    println!(
        "  Churn:   +{} -{} lines",
        v.lines_added.to_string().green(),
        v.lines_removed.to_string().red()
    );
    if !v.active_contributors.is_empty() {
        println!("  Team:    {}", v.active_contributors.join(", ").dimmed());
    }
}

fn render_debt(name: &str, d: &toad_core::DebtIndicators) {
    println!(
        "
{} {}:",
        "⚠️".yellow(),
        name.bold()
    );
    println!("  Debt Score: {}/10", d.debt_score.to_string().cyan());
    println!(
        "  Comments:   {} TODOs, {} FIXMEs, {} HACKs",
        d.todo_count.to_string().yellow(),
        d.fixme_count.to_string().red(),
        d.hack_count.to_string().magenta()
    );
    if !d.large_files.is_empty() {
        println!("  Large Files (>700 lines):");
        for f in &d.large_files {
            println!("    - {}", f.dimmed());
        }
    }
}

fn render_health(name: &str, h: &toad_core::HealthScore) {
    let status_color = if h.total > 80 {
        "green"
    } else if h.total > 50 {
        "yellow"
    } else {
        "red"
    };
    println!(
        "
{} {}: {}/100",
        "🏥".color(status_color),
        name.bold(),
        h.total.to_string().color(status_color).bold()
    );
    println!(
        "  VCS:   {}/20  Tests: {}/20  Docs: {}/15",
        h.vcs_cleanliness, h.test_coverage, h.documentation
    );
    println!(
        "  Pulse: {}/15  Deps:  {}/15  Qual: {}/15",
        h.activity, h.dependencies, h.code_quality
    );
}
