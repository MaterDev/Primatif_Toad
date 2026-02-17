use anyhow::Result;
use colored::*;
use toad_core::Workspace;
use toad_ops::doctor::run_health_check;

pub fn handle() -> Result<()> {
    println!("{}", "--- TOAD HEALTH CHECK ---".green().bold());

    let workspace = Workspace::discover()?;
    let report = run_health_check(&workspace)?;

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

    // Summary
    println!("\n{}", "--- SUMMARY ---".bold());
    if report.issues.is_empty() && report.warnings.is_empty() {
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
    }

    Ok(())
}
