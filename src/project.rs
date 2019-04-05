use yaml_rust::yaml::{Yaml, YamlLoader};

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
            _ => vec![Window::new(
                &YamlLoader::load_from_str("default: #").unwrap()[0],
            )],
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
    panes: Vec<Option<String>>,
    layout: Option<String>,
}

impl Window {
    pub fn new(window: &Yaml) -> Self {
        let (name, content) = window.as_hash().unwrap().iter().next().unwrap();

        let (panes, layout) = match content {
            Yaml::String(cmd) => (vec![Some(cmd.to_owned())], None),
            Yaml::Hash(config) => (
                match config.get(&Yaml::from_str("panes")) {
                    Some(Yaml::Array(cmds)) => cmds.into_iter().map(|x| x.to_owned().into_string()).collect(),
                    _ => vec![],
                },
                config.get(&Yaml::from_str("layout")).unwrap().to_owned().into_string()
            ),
            _ => (vec![], None),
        };

        Window {
            name: name.to_owned().into_string().unwrap(),
            panes: panes,
            layout: layout,
        }
    }
}
