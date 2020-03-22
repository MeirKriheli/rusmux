use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use crate::error::AppError;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    project_name: String,
    project_root: Option<String>,
    pre: Option<String>,
    pre_window: Option<String>,
    windows: Option<Vec<BTreeMap<String, WindowContent>>>,
}

impl TryFrom<String> for Project {

    type Error = AppError;

    fn try_from(yaml: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&yaml).map_err(|_| AppError::Message("Cannot parse yaml".into()))
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
    use super::Project;
    use std::convert::TryFrom;
    use crate::error::AppError;

    #[test]
    fn empty_project_test() {
        let name = "meir";
        let empty_project = format!("project_name: {}", name);

        let project = Project::try_from(empty_project).unwrap();
        assert_eq!(project.project_name, name);
        assert_eq!(project.windows, None);
    }

    #[test]
    fn invalid_project_test() {
        let empty_project: String = "".into();
        let project = Project::try_from(empty_project);
        assert!(project.is_err(), "Should return an error");
    }
}
