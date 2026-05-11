use std::fs;
use std::path::{Path, PathBuf};
use crate::utils::errors::AppResult;

pub struct AssetService;

impl AssetService {
    pub fn save_asset(workspace_path: &Path, asset_name: &str, data: &[u8]) -> AppResult<PathBuf> {
        let assets_dir = workspace_path.join("assets");
        if !assets_dir.exists() {
            fs::create_dir_all(&assets_dir)?;
        }

        let asset_path = assets_dir.join(asset_name);
        fs::write(&asset_path, data)?;

        Ok(asset_path)
    }

    pub fn get_assets(workspace_path: &Path) -> AppResult<Vec<PathBuf>> {
        let assets_dir = workspace_path.join("assets");
        if !assets_dir.exists() {
            return Ok(vec![]);
        }

        let mut assets = vec![];
        for entry in fs::read_dir(assets_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                assets.push(path);
            }
        }

        Ok(assets)
    }
}
