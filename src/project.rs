use yaml_rust::yaml::Yaml;

#[derive(Debug)]
pub struct Project {
    name: String,
    root: String,
    pre: Option<String>,
    pre_window: Option<String>,
}

impl Project {
    pub fn new_from_yaml(project_hash: &Yaml) -> Self {
        let hash = project_hash.as_hash().unwrap();
        let pre = match hash.get(&Yaml::from_str("pre")) {
            Some(p) => p.to_owned().into_string(),
            _ => None,
        };
        let pre_window = match hash.get(&Yaml::from_str("pre_window")) {
            Some(c) => c.to_owned().into_string(),
            _ => None,
        };
        Project {
            name: hash[&Yaml::from_str("project_name")].to_owned().into_string().unwrap(),
            root: hash[&Yaml::from_str("project_root")].to_owned().into_string().unwrap(),
            pre: pre,
            pre_window: pre_window,
        }
    }
}
