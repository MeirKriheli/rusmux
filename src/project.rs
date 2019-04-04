use yaml_rust::yaml::Yaml;

#[derive(Debug)]
pub struct Project {
    name: String,
    root: String,
    pre: Option<String>,
    pre_window: Option<String>,
    windows: Vec<Window>,
}

impl Project {
    pub fn new(project: &Yaml) -> Self {
        let hash = project.as_hash().unwrap();
        let pre = match hash.get(&Yaml::from_str("pre")) {
            Some(p) => p.to_owned().into_string(),
            _ => None,
        };
        let pre_window = match hash.get(&Yaml::from_str("pre_window")) {
            Some(c) => c.to_owned().into_string(),
            _ => None,
        };

        let windows = match hash.get(&Yaml::from_str("windows")) {
            Some(c) => c.to_owned().into_iter().map(|x| Window::new(&x)).collect(),
            _ => Vec::new(),
        };
        Project {
            name: hash[&Yaml::from_str("project_name")]
                .to_owned()
                .into_string()
                .unwrap(),
            root: hash[&Yaml::from_str("project_root")]
                .to_owned()
                .into_string()
                .unwrap(),
            pre: pre,
            pre_window: pre_window,
            windows: windows,
        }
    }
}

#[derive(Debug)]
struct Window {
    name: String,
}

impl Window {
    pub fn new(window: &Yaml) -> Self {
        let (name, content) = window.as_hash().unwrap().iter().next().unwrap();
        Window {
            name: name.to_owned().into_string().unwrap(),
        }
    }
}
