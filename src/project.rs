use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    project_name: String,
    project_root: Option<String>,
    pre: Option<String>,
    pre_window: Option<String>,
    windows: Vec<BTreeMap<String, WindowContent>>,
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
