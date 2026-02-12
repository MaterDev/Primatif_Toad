use colored::*;
use toad_core::{
    AnalyticsReport, BatchCleanReport, BatchOperationReport, MultiRepoGitReport,
    MultiRepoStatusReport, ProgressReporter, SearchResult, StatusReport, VcsStatus,
};
use toad_ops::stats::format_size;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub struct IndicatifReporter {
    pub pb: ProgressBar,
}

impl IndicatifReporter {
    pub fn spinner() -> Result<Self, anyhow::Error> {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner().template("{spinner:.green} [{elapsed_precise}] {msg}")?,
        );
        pb.enable_steady_tick(Duration::from_millis(100));
        Ok(Self { pb })
    }
}

pub fn format_sync_report(count: usize) {
    println!(
        "{} Registry updated with {} projects.",
        "SUCCESS:".green().bold(),
        count
    );
}

pub fn format_clean_report(report: &BatchCleanReport) {
    for (name, outcome) in &report.results {
        match outcome {
            Ok(res) => {
                if !res.errors.is_empty() {
                    println!("{} Issues cleaning {}:", "WARNING:".yellow(), name);
                    for err in &res.errors {
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
}

impl ProgressReporter for IndicatifReporter {
    fn set_message(&self, msg: &str) {
        self.pb.set_message(msg.to_string());
    }
    fn inc(&self, delta: u64) {
        self.pb.inc(delta);
    }
    fn set_length(&self, len: u64) {
        self.pb.set_length(len);
    }
    fn finish(&self) {
        self.pb.finish_and_clear();
    }
    fn finish_with_message(&self, msg: &str) {
        self.pb.finish_with_message(msg.to_string());
    }
}

use crate::commands::home::HomeReport;

pub fn format_home_report(report: Option<HomeReport>) {
    match report {
        Some(r) => {
            if r.already_registered {
                println!(
                    "{} Toad home is active at: {:?}",
                    "ACTIVE:".green().bold(),
                    r.path
                );
                println!("Context: {}", r.name.bold());
            } else if r.is_new {
                println!(
                    "{} Initialized and registered new Toad home at: {:?}",
                    "SUCCESS:".green().bold(),
                    r.path
                );
            }
        }
        None => {
            println!(
                "{} No Toad home anchored.",
                "ORPHANED:".red().bold()
            );
            println!("Use 'toad home <path>' to anchor this system.");
        }
    }
}

pub fn format_multi_repo_git_report(report: &MultiRepoGitReport) {
    println!("\n{}", format!("--- {} ---", report.title).blue().bold());

    if report.results.is_empty() {
        println!("No operations were performed.");
        return;
    }

    for res in &report.results {
        let status = if res.success {
            "OK".green()
        } else {
            "FAIL".red()
        };
        println!("{:<30} {}", res.project_name.bold(), status);
        if !res.success && !res.stderr.is_empty() {
            println!("  Error: {}", res.stderr.dimmed());
        }
    }
}

pub fn format_multi_repo_status(report: &MultiRepoStatusReport) {
    println!("{}", "--- MULTI-REPO GIT STATUS ---".green().bold());
    println!("{:<40} {:<15} BRANCH", "REPOSITORY", "STATUS");
    println!("{:-<40} {:-<15} {:-<20}", "", "", "");

    for item in &report.items {
        let status = format!("{}", item.status);
        println!(
            "{:<40} {:<15} {}",
            item.name.bold(),
            status,
            item.branch.cyan()
        );
    }
}

pub fn format_batch_report(report: &BatchOperationReport) {
    println!("\n{}", "--- BATCH COMPLETE ---".blue().bold());

    for res in &report.results {
        print!("Processing {}... ", res.project_name);
        if res.exit_code == 0 {
            println!("{}", "OK".green());
        } else if res.stderr == "Skipped due to previous failure" {
            println!("{}", "SKIPPED".yellow());
        } else {
            println!("{} (Code: {})", "FAIL".red(), res.exit_code);
            if res.timed_out {
                println!("  {}", "Timed out after 30s".yellow());
            }
            if !res.stderr.is_empty() {
                println!("{}", res.stderr.dimmed());
            }
        }
    }

    println!(
        "\n{} {} Succeeded | {} {} Failed{}",
        "■".green(),
        report.success_count,
        "■".red(),
        report.fail_count,
        if report.skip_count > 0 {
            format!(" | {} {} Skipped", "■".yellow(), report.skip_count)
        } else {
            String::new()
        }
    );
}

pub fn format_search_results(results: &SearchResult) {
    if results.matches.is_empty() {
        println!("No projects found matching '{}'.", results.query);
        return;
    }

    println!("Searching for projects matching '{}'...", results.query);
    for project in &results.matches {
        let tags_display = if project.tags.is_empty() {
            String::new()
        } else {
            format!(" {}", project.tags.join(" ").dimmed())
        };
        println!("- {}{}", project.name, tags_display);
    }
}

pub fn format_analytics_report(
    report: &AnalyticsReport,
    _query: Option<&str>,
    _tag: Option<&str>,
    all: bool,
) {
    println!("{}", "--- ECOSYSTEM ANALYTICS ---".green().bold());

    if report.offenders.is_empty() {
        println!("No projects found matching filters.");
        return;
    }

    println!(
        "{} Total Usage: {} ({} Artifacts)",
        "■".green(),
        format_size(report.total_usage).bold(),
        format_size(report.total_artifacts).dimmed()
    );

    let limit = if all { report.offenders.len() } else { 10 };
    let display_count = std::cmp::min(report.offenders.len(), limit);

    println!(
        "\n{}",
        format!("TOP {} OFFENDERS", display_count).yellow().bold()
    );

    for p in report.offenders.iter().take(display_count) {
        let size_str = format_size(p.total_size);

        let color_size = if p.total_size > 1024 * 1024 * 1024 {
            size_str.red().bold()
        } else if p.total_size > 200 * 1024 * 1024 {
            size_str.yellow()
        } else {
            size_str.green()
        };

        let bar_width = 20;
        let filled = ((p.bloat_percentage / 100.0) * bar_width as f64).round() as usize;
        let empty = bar_width - filled;
        let bar = format!(
            "[{}{}]",
            "■".repeat(filled).red(),
            "-".repeat(empty).dimmed()
        );

        println!(
            "{:<20} | {:>10} | {} {:.0}% bloat ({})",
            p.name.bold(),
            color_size,
            bar,
            p.bloat_percentage,
            p.activity
        );
    }
}

pub fn format_status_report(report: &StatusReport, _query: Option<&str>, _tag: Option<&str>) {
    println!("{}", "--- ECOSYSTEM HEALTH SCAN ---".green().bold());

    let mut dirty = Vec::new();
    let mut untracked = Vec::new();
    let mut clean_count = 0;
    let mut no_repo_count = 0;
    let total_matching = report.projects.len();

    for p in &report.projects {
        match p.vcs_status {
            VcsStatus::Dirty => dirty.push(p.name.clone()),
            VcsStatus::Untracked => untracked.push(p.name.clone()),
            VcsStatus::Clean => clean_count += 1,
            VcsStatus::None => no_repo_count += 1,
        }

        println!(
            "{} {} ({}) {}",
            "»".blue(),
            p.name.bold(),
            p.stack.dimmed(),
            p.vcs_status
        );

        for issue in &p.issues {
            let issue_msg: &str = issue;
            println!("  {} {}", "└─".dimmed(), issue_msg.yellow());
        }
    }

    if total_matching == 0 {
        println!("No projects found matching filters.");
        return;
    }

    println!("\n{}", "--- SUMMARY ---".green().bold());
    if clean_count > 0 {
        println!(
            "{} {:02}/{} projects are {}",
            "🪷".green(),
            clean_count,
            total_matching,
            "HEALTHY & CLEAN".green().bold()
        );
    }

    if no_repo_count > 0 {
        println!(
            "{} {:02}/{} projects are {}",
            "🌾".yellow(),
            no_repo_count,
            total_matching,
            "OUTSIDE THE TOAD POND (UNTRACKED)".yellow()
        );
    }

    if !untracked.is_empty() {
        println!(
            "\n{} {} projects have {}",
            "🌿".green(),
            untracked.len(),
            "NEW GROWTH (UNTRACKED)".green().bold()
        );
        for name in untracked {
            println!("  {} {}", "»".green(), name);
        }
    }

    if !dirty.is_empty() {
        println!(
            "\n{} {} projects have {}",
            "⚠️".red(),
            dirty.len(),
            "PENDING CHANGES (DIRTY)".red().bold()
        );
        for name in dirty {
            println!("  {} {}", "»".red(), name);
        }
    }
    println!("\n{}", "--- SCAN COMPLETE ---".green());
}
