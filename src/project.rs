use crate::error::AppError;
use crate::stringorvec;
use crate::window::Window;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub project_name: String,
    pub project_root: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
    pub on_project_start: Option<Vec<String>>,
    #[serde(default)]
    #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
    pub on_project_exit: Option<Vec<String>>,
    #[serde(default)]
    #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
    pub pre_window: Option<Vec<String>>,
    pub windows: Option<Vec<Window>>,
}

impl TryFrom<String> for ProjectConfig {
    type Error = AppError;

    fn try_from(yaml: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&yaml)
            .map_err(|e| AppError::Message(format!("Cannot parse yaml: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectConfig;
    use std::convert::TryFrom;

    #[test]
    fn empty_project_test() {
        let name = "empty";
        let yaml = format!("project_name: {}", name);

        let project = ProjectConfig::try_from(yaml).unwrap();
        assert_eq!(project.project_name, name);
        assert_eq!(project.windows, None);
    }

    #[test]
    fn invalid_project_test() {
        let empty_project: String = "".into();
        let project = ProjectConfig::try_from(empty_project);
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
        let project = ProjectConfig::try_from(yaml).unwrap();
        assert_eq!(project.project_name, name);
        assert_eq!(project.project_root, Some(project_root));
        assert!(project.windows.is_some(), "windows is none");

        let windows = project.windows.unwrap();
        assert_eq!(windows.len(), 1);
        let first = windows.get(0).unwrap();
        assert_eq!(first.name, window_name);
        assert_eq!(first.panes, vec![Some(window_command)]);
    }
}
