pub mod commands;
pub mod models;
pub mod services;
pub mod utils;

use crate::commands::workspace::AppState;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            current_workspace: Mutex::new(None),
        })
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            crate::commands::workspace::create_workspace,
            crate::commands::workspace::open_workspace,
            crate::commands::workspace::get_current_workspace,
            crate::commands::workspace::list_files,
            crate::commands::filesystem::read_file,
            crate::commands::filesystem::write_file,
            crate::commands::filesystem::create_dir,
            crate::commands::filesystem::delete_path,
            crate::commands::filesystem::rename_path,
            crate::commands::assets::save_asset,
            crate::commands::assets::list_assets,
            crate::commands::search::search_files,
        ])
        .setup(|_app| {
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
