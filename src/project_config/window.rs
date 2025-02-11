//! Handles a [Window] de-serialization, an optional section
//! of project's configuration.
use super::error::ProjectParseError;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::{self, Value};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;

/// Represents each windows in the project configuration. Since it
/// can be specified in several ways, it implements a custom visitor
/// for [serde].
///
/// For example:
///
/// ```yaml
/// test:
/// ```
/// Will be a window named `test` with a single pane running nothing with
/// `tiled` layout.
///
/// ```yaml
/// test2 window: vim
/// ```
/// Will be a window named `test2 window` with a single pane running `vim` with
/// `tiled` layout.
///
/// ```yaml
/// window3:
///   panes:
///     - vim
///     - #
///     - npm run serve,
/// ```
/// Will be a window named `window3` with 3 panes running `vim`, nothing,
/// & `npm run serve` and `tiled` layout.
///
/// ```yml
///
/// window4:
///   layout: main-vertical
///   options:
///     main-pane-width: 60%
///   panes:
///     - vim
///     - #
///     - npm run serve,
/// ```
/// Will be a window named `window4` with 3 panes running `vim`, nothing,
/// & `npm run serve` and `main-vertical` layout.
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Window {
    /// The name of the window
    pub name: String,
    /// The tmux layout of the window. Defaults to `tiled`.
    #[serde(default = "tiled")]
    pub layout: String,
    /// The window's panes, each with an optional command(s) to run.
    pub panes: Vec<Option<Vec<String>>>,
    /// Per window options
    pub options: Option<HashMap<String, String>>,
    /// Root directory for the window (optional). If exists will be used
    /// as the window's working directory with `-c` flag.
    ///
    /// Takes precedence over the project_root.
    pub root: Option<String>,
}

impl TryFrom<String> for Window {
    type Error = ProjectParseError;

    fn try_from(yaml: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&yaml).map_err(|e| ProjectParseError(format!("{e}")))
    }
}

impl<'de> Deserialize<'de> for Window {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(WindowVisitor)
    }
}

/// Implements the visitor which supports the various ways a [Window],
/// its pane(s) and layout can be specified.
struct WindowVisitor;

impl<'de> Visitor<'de> for WindowVisitor {
    type Value = Window;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A window struct")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let (key, val) = map.next_entry::<String, Value>().unwrap().unwrap();
        let mut w = Window {
            name: key,
            layout: "tiled".into(),
            panes: vec![],
            options: None,
            root: None,
        };

        match val {
            Value::Null => w.panes.push(None),
            Value::String(value) => w.panes.push(Some(vec![value])),
            Value::Mapping(map) => {
                w.layout = map
                    .get(Value::String("layout".into()))
                    .map(|v| v.as_str().unwrap().into())
                    .unwrap_or_else(|| "tiled".into());

                w.root = map
                    .get(Value::String("root".into()))
                    .map(|v| v.as_str().unwrap().into());

                w.options = map.get(Value::String("options".into())).map(|v| {
                    v.as_mapping()
                        .unwrap()
                        .iter()
                        .map(|(option, value)| {
                            (
                                option.as_str().unwrap().into(),
                                value.as_str().unwrap().into(),
                            )
                        })
                        .collect()
                });

                if let Some(Value::Sequence(panes)) = map.get(Value::String("panes".into())) {
                    for pane in panes {
                        match pane {
                            Value::String(pane_cmd) => w.panes.push(Some(vec![pane_cmd.into()])),
                            Value::Mapping(multicommand_pane) => {
                                for pane_commands in multicommand_pane.values() {
                                    match pane_commands {
                                        Value::String(pane_cmd) => {
                                            w.panes.push(Some(vec![pane_cmd.into()]))
                                        }
                                        Value::Sequence(pane_cmds) => w.panes.push(Some(
                                            pane_cmds
                                                .iter()
                                                .filter_map(|cmd| cmd.as_str())
                                                .map(|cmd| cmd.into())
                                                .collect(),
                                        )),
                                        _ => w.panes.push(None),
                                    }
                                }
                            }
                            _ => w.panes.push(None),
                        }
                    }
                }
            }
            _ => return Err(de::Error::custom("invalid window struct")),
        }
        Ok(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_test() {
        let windows_strings = [
            "test: #",
            "test2 window: vim",
            "window3:\n  panes:\n    - vim\n    - #\n    - npm run serve",
            "window4:\n  layout: main-vertical\n  panes:\n    - vim\n    - #\n    - npm run serve",
        ];

        let windows: Vec<Window> = windows_strings
            .iter()
            .map(|yaml| Window::try_from(yaml.to_string()).unwrap())
            .collect();

        assert_eq!(windows[0].name, "test");
        assert_eq!(windows[0].layout, "tiled".to_string());
        assert_eq!(windows[0].panes, vec![None]);

        assert_eq!(windows[1].name, "test2 window");
        assert_eq!(windows[1].layout, "tiled".to_string());
        assert_eq!(windows[1].panes, vec![Some(vec!["vim".into()])]);

        assert_eq!(windows[2].name, "window3");
        assert_eq!(windows[2].layout, "tiled".to_string());
        assert_eq!(
            windows[2].panes,
            vec![
                Some(vec!["vim".into()]),
                None,
                Some(vec!["npm run serve".into()])
            ]
        );

        assert_eq!(windows[3].name, "window4");
        assert_eq!(windows[3].layout, "main-vertical".to_string());
        assert_eq!(
            windows[3].panes,
            vec![
                Some(vec!["vim".into()]),
                None,
                Some(vec!["npm run serve".into()])
            ]
        );
    }

    #[test]
    fn multicommand_panes() {
        let yaml = "\
greek-window:
  panes:
    - alpha-pane:
      - echo alpha1
      - echo alpha2
    - beta-pane:
      - echo beta
    - gamma-pane: echo gamma
    - echo delta # a good old single command pane";
        let window = Window::try_from(yaml.to_string()).unwrap();
        assert_eq!(window.name, "greek-window");
        assert_eq!(
            window.panes,
            vec![
                Some(vec!["echo alpha1".into(), "echo alpha2".into()]),
                Some(vec!["echo beta".into()]),
                Some(vec!["echo gamma".into()]),
                Some(vec!["echo delta".into()]),
            ]
        );
    }

    #[test]
    fn invalid_multicommand_pane() {
        let yaml = "\
aircraft-window:
  panes:
    - bad-plane:
        aero-bullet: echo 'object is not a valid command, also the worst airplane in history'
    - ok-plane: echo 'Boeing 747'";
        let window = Window::try_from(yaml.to_string()).unwrap();
        assert_eq!(window.name, "aircraft-window");
        assert_eq!(
            window.panes,
            vec![None, Some(vec!["echo 'Boeing 747'".into()]),]
        );
    }

    #[test]
    fn invalid_multicommand_pane_item() {
        let yaml = "\
roman-window:
  panes:
    - roman-pane:
      # comments are fine
      - echo I
      - # empty command is ignored
      - echo II
      - bad-command: is ignored!";
        let window = Window::try_from(yaml.to_string()).unwrap();
        assert_eq!(window.name, "roman-window");
        assert_eq!(
            window.panes,
            vec![Some(vec!["echo I".into(), "echo II".into()])]
        );
    }

    #[test]
    fn window_options() {
        let yaml = "\
window-with-options:
    options:
        main-pane-height: 70%
        main-pane-width: 75%";

        let window = Window::try_from(yaml.to_string()).unwrap();
        assert!(window.options.is_some());

        let options = window.options.unwrap();
        assert_eq!(options.len(), 2);
        assert_eq!(options.get("main-pane-height"), Some(&"70%".to_string()));
        assert_eq!(options.get("main-pane-width"), Some(&"75%".to_string()));
    }

    #[test]
    fn window_root() {
        let yaml = "\
window-with-root:
    root: /home/dummy/void";
        let window = Window::try_from(yaml.to_string()).unwrap();

        assert_eq!(window.root, Some("/home/dummy/void".into()));

        let yaml = "\
window-wiout-root:";
        let window = Window::try_from(yaml.to_string()).unwrap();

        assert_eq!(window.root, None);
    }
}
