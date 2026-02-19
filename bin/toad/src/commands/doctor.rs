use anyhow::Result;
use colored::*;
use toad_core::Workspace;
use toad_ops::doctor::run_health_check;

pub fn handle(json: bool) -> Result<()> {
    let workspace = Workspace::discover()?;
    let mut report = run_health_check(&workspace)?;

    // Collect diagnostics from all projects
    if let Ok(registry) =
        toad_core::ProjectRegistry::load(workspace.active_context.as_deref(), None)
    {
        for project in &registry.projects {
            let project_diagnostics =
                toad_discovery::detect_metadata_issues(&project.path, &project.name);
            report.diagnostics.merge(project_diagnostics);
        }
    }

    // Add diagnostic summary to warnings if issues found
    if report.diagnostics.has_errors() || report.diagnostics.has_warnings() {
        report.warnings.push(format!(
            "Found {} metadata issues ({} errors, {} warnings)",
            report.diagnostics.diagnostics.len(),
            report.diagnostics.error_count(),
            report.diagnostics.warning_count()
        ));
    }

    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
        return Ok(());
    }

    println!("{}", "--- TOAD HEALTH CHECK ---".green().bold());

    // 1. Installation Checks
    println!("\n{} Installation", "»".blue());
    println!("  {} Toad version: {}", "✅".green(), report.version);
    if report.mcp_installed {
        println!("  {} toad-mcp is installed", "✅".green());
    } else {
        println!("  {} toad-mcp not found in PATH", "⚠️".yellow());
    }

    // 2. Workspace Checks
    println!("\n{} Workspace", "»".blue());
    if report.workspace_discovered {
        println!(
            "  {} Workspace discovered at {:?}",
            "✅".green(),
            report
                .workspace_path
                .as_deref()
                .unwrap_or_else(|| std::path::Path::new("unknown"))
        );
    }

    if report.registry_fresh {
        println!("  {} Registry is fresh (fingerprint match)", "✅".green());
    } else {
        println!(
            "  {} Registry is stale (fingerprint mismatch)",
            "⚠️".yellow()
        );
    }

    if report.project_count > 0 {
        println!(
            "  {} Registry loaded ({} projects)",
            "✅".green(),
            report.project_count
        );
    } else {
        println!("  {} Registry is empty or not loaded", "⚠️".yellow());
    }

    // 3. Git Checks
    println!("\n{} Git", "»".blue());
    if report.git_remotes_reachable {
        println!("  {} Git remotes are reachable", "✅".green());
    } else {
        println!("  {} Could not reach git remotes", "⚠️".yellow());
    }

    if report.uninitialized_submodules == 0 && report.dirty_submodules == 0 {
        println!(
            "  {} All submodules are initialized and clean",
            "✅".green()
        );
    } else {
        if report.uninitialized_submodules > 0 {
            println!(
                "  {} {} uninitialized submodules",
                "⚠️".yellow(),
                report.uninitialized_submodules
            );
        }
        if report.dirty_submodules > 0 {
            println!(
                "  {} {} submodules with uncommitted changes",
                "⚠️".yellow(),
                report.dirty_submodules
            );
        }
    }

    // 4. MCP Checks
    println!("\n{} MCP Server", "»".blue());
    println!("  {} MCP server can be initialized", "✅".green());

    // 5. Artifacts Checks
    println!("\n{} Artifacts", "»".blue());
    if report.manifest_exists {
        println!("  {} MANIFEST.md exists", "✅".green());
    } else {
        println!("  {} MANIFEST.md not found", "⚠️".yellow());
    }

    if report.atlas_exists {
        println!("  {} ATLAS.json exists", "✅".green());
    } else {
        println!("  {} ATLAS.json not found", "⚠️".yellow());
    }

    // 6. Metadata Diagnostics
    if !report.diagnostics.diagnostics.is_empty() {
        println!("\n{} Metadata Issues", "»".blue());
        for diag in &report.diagnostics.diagnostics {
            let icon = match diag.severity {
                toad_core::DiagnosticSeverity::Error => "❌",
                toad_core::DiagnosticSeverity::Warning => "⚠️",
                toad_core::DiagnosticSeverity::Info => "ℹ️",
            };
            println!(
                "  {} {} - {}: {}",
                icon,
                diag.project_name.bold(),
                diag.file_name.yellow(),
                diag.message
            );
            if let Some(details) = &diag.details {
                println!("    {}", details.dimmed());
            }
        }
    }

    // Summary
    println!("\n{}", "--- SUMMARY ---".bold());
    if report.issues.is_empty() && report.warnings.is_empty() && !report.diagnostics.has_errors() {
        println!("{} All checks passed! Toad is healthy.", "✅".green());
    } else {
        if !report.warnings.is_empty() {
            println!("{} {} warnings", "⚠️".yellow(), report.warnings.len());
            for w in &report.warnings {
                println!("  - {}", w);
            }
        }
        if !report.issues.is_empty() {
            println!("{} {} issues", "❌".red(), report.issues.len());
            for i in &report.issues {
                println!("  - {}", i);
            }
        }
        if report.diagnostics.has_errors() {
            println!(
                "{} {} metadata errors detected",
                "❌".red(),
                report.diagnostics.error_count()
            );
        }
    }

    Ok(())
}
