use crate::cli::StrategyCommands;
use anyhow::{bail, Result};
use colored::*;
use std::fs;
use toad_core::strategy::StrategyRegistry;
use toad_core::{GlobalConfig, StackStrategy};

pub fn handle(subcommand: &StrategyCommands) -> Result<()> {
    let registry = StrategyRegistry::load()?;

    match subcommand {
        StrategyCommands::List => {
            println!("{}", "--- ACTIVE STACK STRATEGIES ---".green().bold());
            for strategy in &registry.strategies {
                println!(
                    "🌿 {: <10} {} (Priority: {})",
                    strategy.name.bold(),
                    format!("[{}]", strategy.match_files.join(", ")).dimmed(),
                    strategy.priority
                );
            }
        }
        StrategyCommands::Info { name } => {
            let strategy = registry
                .strategies
                .iter()
                .find(|s| s.name.to_lowercase() == name.to_lowercase());
            if let Some(s) = strategy {
                println!("{}: {}", "Name".bold(), s.name);
                println!("{}: {}", "Priority".bold(), s.priority);
                println!("{}: {}", "Matches".bold(), s.match_files.join(", "));
                println!("{}: {}", "Artifacts".bold(), s.artifacts.join(", "));
                println!("{}: {}", "Auto-Tags".bold(), s.tags.join(", "));
            } else {
                bail!("Strategy '{}' not found.", name);
            }
        }
        StrategyCommands::Add {
            name,
            match_files,
            artifacts,
            tags,
            priority,
        } => {
            let match_files_vec: Vec<String> = match_files
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            let artifacts_vec: Vec<String> = artifacts
                .clone()
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let tags_vec: Vec<String> = tags
                .clone()
                .unwrap_or_default()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();

            let new_strategy = StackStrategy {
                name: name.clone(),
                match_files: match_files_vec,
                artifacts: artifacts_vec,
                tags: tags_vec,
                priority: *priority,
            };

            let custom_dir = GlobalConfig::config_dir(None)?.join("strategies/custom");
            fs::create_dir_all(&custom_dir)?;

            let mut safe_name = name
                .to_lowercase()
                .chars()
                .filter(|c: &char| c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect::<String>();
            safe_name.truncate(64);
            let filename = format!("{}.toml", safe_name);
            let path = custom_dir.join(filename);
            let content = toml::to_string(&new_strategy)?;
            fs::write(&path, content)?;

            println!(
                "{} Strategy '{}' added and saved to {:?}",
                "SUCCESS:".green().bold(),
                name,
                path
            );
        }
        StrategyCommands::Remove { name, yes } => {
            if !yes {
                use std::io::{self, Write};
                print!("Remove strategy '{}'? [y/N]: ", name);
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("Aborted. (Use --yes to skip confirmation)");
                    return Ok(());
                }
            }
            
            let custom_dir = GlobalConfig::config_dir(None)?.join("strategies/custom");
            let mut safe_name = name
                .to_lowercase()
                .chars()
                .filter(|c: &char| c.is_alphanumeric() || *c == '_' || *c == '-')
                .collect::<String>();
            safe_name.truncate(64);
            let filename = format!("{}.toml", safe_name);
            let path = custom_dir.join(filename);

            if path.exists() {
                fs::remove_file(&path)?;
                println!("{} Strategy '{}' removed.", "SUCCESS:".green().bold(), name);
            } else {
                bail!("Custom strategy '{}' not found or is a built-in.", name);
            }
        }
    }
    Ok(())
}
