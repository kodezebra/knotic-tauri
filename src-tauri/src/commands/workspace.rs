use std::path::PathBuf;
use tauri::State;
use crate::models::workspace::Workspace;
use crate::services::workspace_service::{WorkspaceService, FileEntry};

#[tauri::command]
pub async fn list_files(
    path: PathBuf,
) -> AppResult<Vec<FileEntry>> {
    WorkspaceService::list_files(&path)
}
use crate::utils::errors::AppResult;
use std::sync::Mutex;

pub struct AppState {
    pub current_workspace: Mutex<Option<Workspace>>,
}

#[tauri::command]
pub async fn create_workspace(
    path: PathBuf,
    name: String,
    scaffold: Option<bool>,
    state: State<'_, AppState>,
) -> AppResult<Workspace> {
    let workspace = WorkspaceService::create_workspace(path, name, scaffold.unwrap_or(true))?;
    let mut current = state.current_workspace.lock().unwrap();
    *current = Some(workspace.clone());
    Ok(workspace)
}

#[tauri::command]
pub async fn open_workspace(
    path: PathBuf,
    state: State<'_, AppState>,
) -> AppResult<Workspace> {
    let workspace = WorkspaceService::open_workspace(path)?;
    let mut current = state.current_workspace.lock().unwrap();
    *current = Some(workspace.clone());
    Ok(workspace)
}

#[tauri::command]
pub async fn get_current_workspace(
    state: State<'_, AppState>,
) -> AppResult<Option<Workspace>> {
    let current = state.current_workspace.lock().unwrap();
    Ok(current.clone())
}
