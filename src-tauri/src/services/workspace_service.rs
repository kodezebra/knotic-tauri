use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use crate::models::workspace::{Workspace, WorkspaceConfig};
use crate::utils::errors::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Option<Vec<FileEntry>>,
}

pub struct WorkspaceService;

impl WorkspaceService {
    pub fn create_workspace(path: PathBuf, name: String, scaffold: bool) -> AppResult<Workspace> {
        if !path.exists() {
            fs::create_dir_all(&path)?;
        }

        let config_path = path.join("knotic.json");
        if config_path.exists() {
            return Err(AppError::Workspace("Workspace already exists at this location".into()));
        }

        let config = WorkspaceConfig {
            name: name.clone(),
            version: "0.1.0".into(),
        };

        let config_json = serde_json::to_string_pretty(&config)?;
        fs::write(config_path, config_json)?;

        if scaffold {
            fs::create_dir_all(path.join("notes"))?;
            fs::create_dir_all(path.join("assets"))?;
        }

        Ok(Workspace { name, path })
    }

    pub fn open_workspace(path: PathBuf) -> AppResult<Workspace> {
        let config_path = path.join("knotic.json");
        if !config_path.exists() {
            return Err(AppError::Workspace("Not a valid Knotic workspace".into()));
        }

        let config_content = fs::read_to_string(config_path)?;
        let config: WorkspaceConfig = serde_json::from_str(&config_content)?;

        Ok(Workspace {
            name: config.name,
            path,
        })
    }

    pub fn validate_workspace(path: &Path) -> bool {
        path.join("knotic.json").exists()
    }

    pub fn list_files(path: &Path) -> AppResult<Vec<FileEntry>> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().into_owned();
            let is_dir = path.is_dir();
            
            if name.starts_with('.') {
                continue;
            }

            let children = if is_dir {
                Some(Self::list_files(&path)?)
            } else {
                None
            };

            entries.push(FileEntry {
                name,
                path,
                is_dir,
                children,
            });
        }
        
        entries.sort_by(|a, b| {
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir)
            } else {
                a.name.cmp(&b.name)
            }
        });

        Ok(entries)
    }
}
