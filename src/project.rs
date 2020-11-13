use crate::error::AppError;
use crate::stringorvec;
use crate::window::Window;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::convert::TryFrom;
use std::env;
use std::fmt;

const TMUX: &str = "tmux";

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub project_name: String,
    pub project_root: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
    pub on_project_start: Option<Vec<String>>,
    #[serde(default)]
    #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
    pre_window: Option<Vec<String>>,
    windows: Option<Vec<Window>>,
}

impl TryFrom<String> for Project {
    type Error = AppError;

    fn try_from(yaml: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&yaml)
            .map_err(|e| AppError::Message(format!("Cannot parse yaml: {}", e)))
    }
}

impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shebang = env::var("SHELL").map(|x| format!("#!{}\n#", x)).ok();
        let first_window_name_param = match self.windows.as_ref() {
            Some(windows) => windows
                .first()
                .map(|w| format!(" -s {}", w.name))
                .unwrap_or("".into()),
            _ => "".into(),
        };

        let commands = vec![
            shebang,
            Some(format!("# {} rusmux project\n", self.project_name)),
            Some(format!("{} start server\n", TMUX)),
            self.project_root.as_ref().map(|x| format!("cd {}\n", x)),
            self.on_project_start
                .as_ref()
                .map(|x| format!("# Run on_project_start command(s)\n{}\n", x.join("\n"))),
            Some(format!(
                "# Create new session and first window\nTMUX= {} new-session -d -s {}{}",
                TMUX, self.project_name, first_window_name_param
            )),
        ];

        let joined = commands
            .into_iter()
            .filter_map(|c| c)
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", joined)
    }
}

#[cfg(test)]
mod tests {
    use super::Project;
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
        assert_eq!(first.name, window_name);
        assert_eq!(first.panes, vec![Some(window_command)]);
    }
}
