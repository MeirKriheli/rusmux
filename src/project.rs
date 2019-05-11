use yaml_rust::yaml::{Hash, Yaml, YamlLoader};

use super::command::Cmd;

#[derive(Debug)]
pub struct Project {
    name: String,
    root: Option<String>,
    pre: Option<String>,
    pre_window: Option<String>,
    windows: Vec<Window>,
}

fn get_value(hash: &Hash, key: &str) -> Option<String> {
    match hash.get(&Yaml::from_str(key)) {
        Some(p) => p.to_owned().into_string(),
        _ => None,
    }
}

impl Project {
    pub fn new(project: &Yaml) -> Self {
        let hash = project.as_hash().unwrap();

        let windows = match hash.get(&Yaml::from_str("windows")) {
            Some(c) => c.to_owned().into_iter().map(|x| Window::new(&x)).collect(),
            _ => vec![Window::new(
                &YamlLoader::load_from_str("default: #").unwrap()[0],
            )],
        };

        Project {
            name: get_value(&hash, &"project_name").unwrap(),
            root: get_value(&hash, &"project_root"),
            pre: get_value(&hash, &"pre"),
            pre_window: get_value(&hash, &"pre_window"),
            windows: windows,
        }
    }

    pub fn get_commands(&self) -> Vec<Cmd> {
        let mut commands = vec![Cmd::new(vec!["tmux", " start-server"], false, None)];

        if let Some(root_dir) = &self.root {
            commands.push(Cmd::new(vec!["cd", root_dir], false, None));
        }

        if let Some(pre_command) = &self.pre {
            commands.push(Cmd::new(pre_command.split_whitespace().collect(), false, None));
        }

        let first_window_name = &self.windows[0].name;
        commands.push(Cmd::new(
            vec!["tmux", "new-session", "-d", "-s", self.name.as_str(), "-n", first_window_name],
            true, Some("Create the session and the first window")
        ));

        for (i, window) in (&self.windows).into_iter().enumerate() {
            println!("{}: {:?}", i, window);
        }

        commands
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
                    Some(Yaml::Array(cmds)) => cmds
                        .into_iter()
                        .map(|x| x.to_owned().into_string())
                        .collect(),
                    _ => vec![],
                },
                get_value(&config, &"layout"),
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
