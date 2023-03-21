use directories::ProjectDirs;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::error::AppError;

// Returns the path of a file/pattern inside the config dir
pub fn get_path(pattern: &str) -> Result<PathBuf, AppError> {
    let proj_dirs =
        ProjectDirs::from("org", crate_authors!(), crate_name!()).ok_or(AppError::Path)?;
    let config_dir = proj_dirs.config_dir();

    let mut path = PathBuf::from(config_dir);
    path.push(pattern);
    Ok(path)
}

// Return the path of a project file, adding `.yml` extension it
pub fn get_project_path(project_name: &str) -> Result<PathBuf, AppError> {
    let mut file_path = get_path(project_name)?;
    file_path.set_extension("yml");

    Ok(file_path)
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
