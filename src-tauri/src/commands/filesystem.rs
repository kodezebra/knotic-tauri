use std::path::PathBuf;
use crate::services::file_service::FileService;
use crate::utils::errors::AppResult;

#[tauri::command]
pub async fn read_file(path: PathBuf) -> AppResult<String> {
    FileService::read_file(path)
}

#[tauri::command]
pub async fn write_file(path: PathBuf, content: String) -> AppResult<()> {
    FileService::create_file(path, content)
}

#[tauri::command]
pub async fn create_dir(path: PathBuf) -> AppResult<()> {
    FileService::create_directory(path)
}

#[tauri::command]
pub async fn delete_path(path: PathBuf) -> AppResult<()> {
    if path.is_file() {
        FileService::delete_file(path)
    } else {
        FileService::delete_directory(path)
    }
}

#[tauri::command]
pub async fn rename_path(from: PathBuf, to: PathBuf) -> AppResult<()> {
    FileService::rename(from, to)
}
