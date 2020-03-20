use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    project_name: String,
    project_root: Option<String>,
    pre: Option<String>,
    pre_window: Option<String>,
    windows: Option<Vec<BTreeMap<String, WindowContent>>>,
}

impl From<String> for Project {
    fn from(yaml: String) -> Self {
        serde_yaml::from_str(&yaml).unwrap()
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

    #[test]
    fn empty_project_test() {
        let name = "meir";
        let empty_project = format!("project_name: {}", name);

        let project = Project::from(empty_project);
        assert_eq!(project.project_name, name);
        assert_eq!(project.windows, None);
    }

    #[test]
    fn invalid_project_test() {
        let empty_project: String = "".into();
        let project = Project::from(empty_project);
    }
}
