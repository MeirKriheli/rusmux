use crate::error::AppError;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    project_name: String,
    project_root: Option<String>,
    on_project_start: Option<StringOrList>,
    pre_window: Option<StringOrList>,
    windows: Option<Vec<BTreeMap<String, WindowContent>>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum StringOrList {
    Single(String),
    List(Vec<String>),
}

impl TryFrom<String> for Project {
    type Error = AppError;

    fn try_from(yaml: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&yaml).map_err(|_| AppError::Message("Cannot parse yaml".into()))
    }
}

impl Project {
    /// Get the shell shebang, e.g.: "#!/bin/zsh"
    fn get_shell_shebang(&self) -> Option<String> {
        let shell = env::var("SHELL");

        match shell {
            Ok(shell) => Some(format!("#!{}", shell)),
            _ => None,
        }
    }

    pub fn debug(&self) -> String {
        todo!()
    }

    pub fn execute(&self) -> Result<(), AppError> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WindowWithSetup {
    layout: Option<String>,
    panes: Vec<Option<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum WindowContent {
    SingleCommand(String),
    WithSetup(WindowWithSetup),
}

#[cfg(test)]
mod tests {
    use super::{Project, WindowContent};
    use std::convert::TryFrom;

    #[test]
    fn empty_project_test() {
        let name = "empty";
        let yaml = format!("project_name: {}", name);

        let project = Project::try_from(yaml).unwrap();
        assert_eq!(project.project_name, name);
        assert_eq!(project.windows, None);
    }

    #[test]
    fn invalid_project_test() {
        let empty_project: String = "".into();
        let project = Project::try_from(empty_project);
        assert!(project.is_err(), "Should return an error");
    }

    #[test]
    fn test_windows() {
        let name: String = "with-windows".into();
        let project_root: String = "/home/dummy/void".into();
        let window_name: String = "editor".into();
        let window_command: String = "vim".into();
        let yaml = format!(
            "project_name: {}
project_root: {}
windows:
  - {}: {}",
            name, project_root, window_name, window_command
        );
        let project = Project::try_from(yaml).unwrap();
        assert_eq!(project.project_name, name);
        assert_eq!(project.project_root, Some(project_root));
        assert!(project.windows.is_some(), "windows is none");

        let windows = project.windows.unwrap();
        assert_eq!(windows.len(), 1);
        let first = windows.get(0).unwrap();
        let window_names: Vec<_> = first.keys().collect();
        assert_eq!(window_names, [&window_name]);
        assert_eq!(
            first.get(&window_name).unwrap(),
            &WindowContent::SingleCommand(window_command)
        );
    }
}
