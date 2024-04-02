//! The project configuration.
use super::error::ProjectParseError;
use super::stringorvec;
use super::window::Window;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::convert::TryFrom;

/// The `.yml` file is de-serialized to this struct. For example;
///
/// ```yml
/// project_name: demo
/// project_root: ~/src/demo
/// on_project_start:
///   - sudo systemctl start postgresql
///   - sudo systemctl start mysqld
/// # on_project_stop:
/// pre_window:
///   - workon demo
///   - cd demo
/// windows:
///   - editor: vim
///   - shells:
///       layout: main-vertical
///       panes:
///         - #
///         - grunt serve
///   - mail: python -m smtpd -n -c DebuggingServer localhost:1025
/// ```
///
/// The struct implements [TryFrom] for:
///
/// - [String] containing the yaml.
/// - [Value] for the deserialized yaml from a string.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Required project name
    #[serde(alias = "name")]
    pub project_name: String,
    /// The root directory for the project (optional).
    /// Will be shell expanded and `cd` into before starting the session.
    pub project_root: Option<String>,
    /// Optional Command(s) to run upon session start, before starting
    /// the tmux session. Can be a single command (string), or several commands
    /// (list of strings).
    #[serde(default)]
    #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
    pub on_project_start: Option<Vec<String>>,
    /// Optional Command(s) to run when session setup is done.
    /// Can be a single command (string), or several commands
    /// (list of strings).
    #[serde(default)]
    #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
    pub on_project_exit: Option<Vec<String>>,
    /// Optional Command(s) to run when the tmux session is killed, using the
    /// `stop` command. Can be a single command (string), or several commands
    /// (list of strings).
    #[serde(default)]
    #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
    pub on_project_stop: Option<Vec<String>>,
    /// Optional Command(s) to run when in each newly created pane (e.g. activate
    /// a virtualenv). Can be a single command (string), or several commands
    /// (list of strings).
    #[serde(default)]
    #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
    pub pre_window: Option<Vec<String>>,
    /// Optional list of [`Window`]s to create.
    pub windows: Option<Vec<Window>>,
}

impl TryFrom<Value> for ProjectConfig {
    type Error = ProjectParseError;

    fn try_from(yaml: Value) -> Result<Self, Self::Error> {
        serde_yaml::from_value(yaml).map_err(|e| ProjectParseError(format!("{e}")))
    }
}

impl TryFrom<String> for ProjectConfig {
    type Error = ProjectParseError;

    fn try_from(yaml: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&yaml).map_err(|e| ProjectParseError(format!("{e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectConfig;
    use std::convert::TryFrom;

    #[test]
    fn empty_project_test() {
        let name = "empty";
        let yaml = format!("project_name: {name}");

        let project = ProjectConfig::try_from(yaml).unwrap();
        assert_eq!(project.project_name, name);
        assert_eq!(project.windows, None);
    }

    #[test]
    fn empty_project_with_name_instead_of_project_name_test() {
        let name = "empty";
        let yaml = format!("name: {name}");

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
            "project_name: {name}
project_root: {project_root}
windows:
  - {window_name}: {window_command}"
        );
        let project = ProjectConfig::try_from(yaml).unwrap();
        assert_eq!(project.project_name, name);
        assert_eq!(project.project_root, Some(project_root));
        assert!(project.windows.is_some(), "windows is none");

        let windows = project.windows.unwrap();
        assert_eq!(windows.len(), 1);
        let first = windows.first().unwrap();
        assert_eq!(first.name, window_name);
        assert_eq!(first.panes, vec![Some(vec![window_command])]);
    }
}
