use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toad_core::{ContextType, GlobalConfig, ProjectContext, ToadResult, Workspace};

#[derive(Debug, Serialize, Deserialize)]
pub struct HomeReport {
    pub path: PathBuf,
    pub name: String,
    pub is_new: bool,
    pub already_registered: bool,
}

pub fn handle(
    workspace_discovered: ToadResult<Workspace>,
    path: Option<String>,
    yes: bool,
) -> Result<Option<HomeReport>> {
    let mut config = GlobalConfig::load(None)?.unwrap_or_else(|| GlobalConfig {
        home_pointer: PathBuf::from("."),
        active_context: None,
        project_contexts: std::collections::HashMap::new(),
        auto_sync: true,
        budget: toad_core::ContextBudget::default(),
    });

    if let Some(new_path) = path {
        let p = PathBuf::from(new_path);
        if !p.exists() {
            anyhow::bail!("Path does not exist: {:?}", p);
        }
        let abs_path = fs::canonicalize(p)?;
        let mut is_new = false;

        if !abs_path.join(".toad-root").exists() {
            if !yes {
                // Return a signal that confirmation is needed
                return Ok(Some(HomeReport {
                    path: abs_path,
                    name: String::new(),
                    is_new: true,
                    already_registered: false,
                }));
            }
            let marker_content = "# Primatif Toad Workspace Root\n# This file identifies this directory as a Toad Control Plane home.\n# Do not delete this file if you want the 'toad' CLI to recognize this workspace.\n";
            fs::write(abs_path.join(".toad-root"), marker_content)?;
            is_new = true;
        }

        let name = if config.project_contexts.is_empty() {
            "default".to_string()
        } else {
            abs_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("new-home")
                .to_string()
        };

        config.home_pointer = abs_path.clone();
        config.project_contexts.insert(
            name.clone(),
            ProjectContext {
                path: abs_path.clone(),
                description: Some("Registered via 'toad home'".to_string()),
                context_type: if abs_path.join(".gitmodules").exists() {
                    ContextType::Hub
                } else {
                    ContextType::Generic
                },
                ai_vendors: Vec::new(),
                registered_at: std::time::SystemTime::now(),
            },
        );
        config.active_context = Some(name.clone());

        let ctx_shadows = GlobalConfig::context_dir(&name, None)?.join("shadows");
        fs::create_dir_all(&ctx_shadows)?;

        config.save(None)?;

        Ok(Some(HomeReport {
            path: abs_path,
            name,
            is_new,
            already_registered: true,
        }))
    } else {
        // Just querying current home
        match workspace_discovered {
            Ok(ws) => Ok(Some(HomeReport {
                path: ws.projects_dir.clone(),
                name: ws
                    .active_context
                    .clone()
                    .unwrap_or_else(|| "unknown".to_string()),
                is_new: false,
                already_registered: true,
            })),
            Err(_) => Ok(None),
        }
    }
}
