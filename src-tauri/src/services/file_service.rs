use std::fs;
use std::path::PathBuf;
use crate::utils::errors::{AppError, AppResult};

pub struct FileService;

impl FileService {
    pub fn create_file(path: PathBuf, content: String) -> AppResult<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)?;
        Ok(())
    }

    pub fn read_file(path: PathBuf) -> AppResult<String> {
        if !path.exists() {
            return Err(AppError::NotFound(format!("File not found: {:?}", path)));
        }
        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    pub fn delete_file(path: PathBuf) -> AppResult<()> {
        if path.is_file() {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    pub fn create_directory(path: PathBuf) -> AppResult<()> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    pub fn delete_directory(path: PathBuf) -> AppResult<()> {
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        }
        Ok(())
    }

    pub fn rename(from: PathBuf, to: PathBuf) -> AppResult<()> {
        fs::rename(from, to)?;
        Ok(())
    }
}
