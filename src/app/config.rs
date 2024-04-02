//! Configuration directory path helpers.
use directories::ProjectDirs;
use serde::Deserialize;
use serde_yaml::Value;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::PathBuf;

use crate::error::AppError;

/// Returns the path of a file/pattern inside the configuration directory.
pub fn get_path(pattern: &str) -> Result<PathBuf, AppError> {
    let proj_dirs =
        ProjectDirs::from("org", crate_authors!(), crate_name!()).ok_or(AppError::ConfigPath)?;
    let config_dir = proj_dirs.config_dir();

    let mut path = PathBuf::from(config_dir);
    create_dir_all(&path).map_err(|e| AppError::ProjectCreateConfigDir(path.clone(), e))?;
    path.push(pattern);
    Ok(path)
}

/// Returns the path of a project file, adding `.yml` extension it.
pub fn get_project_path(project_name: &str) -> Result<PathBuf, AppError> {
    let mut file_path = get_path(project_name)?;
    file_path.set_extension("yml");

    Ok(file_path)
}

/// Read project file, parse it to [`serde_yaml::Value`].
pub fn get_project_yaml(project_name: &str) -> Result<Value, AppError> {
    let project_file_path = get_project_path(project_name)?;
    let mut contents = String::new();
    File::open(&project_file_path)
        .map_err(|_| AppError::ProjectFileNotFound(project_file_path.clone()))?
        .read_to_string(&mut contents)
        .map_err(|e| AppError::ProjectFileRead(project_file_path.clone(), e))?;

    let de = serde_yaml::Deserializer::from_str(&contents);
    Value::deserialize(de)
        .map_err(|e| AppError::YamlParse(project_file_path.clone(), format!("{e}")))
}
