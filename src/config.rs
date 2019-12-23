use app_dirs::{app_root, AppDataType, AppInfo};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use super::error::AppError;

const APP_INFO: AppInfo = AppInfo {
    name: crate_name!(),
    author: crate_authors!(),
};

// Return config directory, which stores project files
fn get_dir() -> Result<PathBuf, AppError> {
    Ok(app_root(AppDataType::UserConfig, &APP_INFO)?)
}

// Returns the path of a file/pattern inside the donfig dir
pub fn get_path(pattern: &str) -> Result<String, AppError> {
    let config_dir = get_dir()?;

    match Path::new(&config_dir).join(pattern).to_str() {
        None => Err(AppError::Path),
        Some(s) => Ok(s.into()),
    }
}

// Read project file
pub fn get_project_yaml(project_name: &str) -> Result<String, AppError> {
    let mut filename = project_name.to_owned();
    filename.push_str(".yml");

    let project_file_path = get_path(&filename)?;
    let mut contents = String::new();
    File::open(project_file_path)?.read_to_string(&mut contents)?;

    Ok(contents)
}
