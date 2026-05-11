use std::path::PathBuf;
use tauri::State;
use crate::commands::workspace::AppState;
use crate::services::asset_service::AssetService;
use crate::utils::errors::{AppError, AppResult};

#[tauri::command]
pub async fn save_asset(
    name: String,
    data: Vec<u8>,
    state: State<'_, AppState>,
) -> AppResult<PathBuf> {
    let current_workspace = state.current_workspace.lock().unwrap();
    let workspace = current_workspace.as_ref().ok_or_else(|| {
        AppError::Workspace("No workspace open".into())
    })?;

    AssetService::save_asset(&workspace.path, &name, &data)
}

#[tauri::command]
pub async fn list_assets(
    state: State<'_, AppState>,
) -> AppResult<Vec<PathBuf>> {
    let current_workspace = state.current_workspace.lock().unwrap();
    let workspace = current_workspace.as_ref().ok_or_else(|| {
        AppError::Workspace("No workspace open".into())
    })?;

    AssetService::get_assets(&workspace.path)
}
