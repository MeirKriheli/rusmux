use app_dirs::{app_root, AppDataType, AppInfo};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use yaml_rust::yaml;

const APP_INFO: AppInfo = AppInfo {
    name: crate_name!(),
    author: crate_authors!(),
};

// Return config directory, which stores project files
fn get_dir() -> PathBuf {
    app_root(AppDataType::UserConfig, &APP_INFO).unwrap()
}

// Returns the path of a file/pattern inside the donfig dir
pub fn get_path(pattern: &str) -> String {
    let config_dir = get_dir();

    Path::new(&config_dir)
        .join(pattern)
        .into_os_string()
        .into_string()
        .unwrap()
}

// Read project file
pub fn get_project_yaml(project_name: &str) -> Vec<yaml::Yaml> {
    let mut filename = project_name.to_owned();
    filename.push_str(".yml");

    let project_file_path = get_path(&filename);
    let mut project_file = File::open(project_file_path).unwrap();
    let mut contents = String::new();
    project_file.read_to_string(&mut contents).unwrap();
    yaml::YamlLoader::load_from_str(&contents).unwrap()
}
