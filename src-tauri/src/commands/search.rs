use tauri::State;
use crate::commands::workspace::AppState;
use crate::services::search_service::{SearchResult, SearchService};
use crate::utils::errors::{AppError, AppResult};

#[tauri::command]
pub async fn search_files(
    query: String,
    state: State<'_, AppState>,
) -> AppResult<Vec<SearchResult>> {
    let current_workspace = state.current_workspace.lock().unwrap();
    let workspace = current_workspace.as_ref().ok_or_else(|| {
        AppError::Workspace("No workspace open".into())
    })?;

    Ok(SearchService::search(&workspace.path, &query))
}
