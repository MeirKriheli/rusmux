//! Handles a [Window] de-serialization, an optional section
//! of project's configuration.
use super::error::ProjectParseError;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::{self, Value};
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
    /// The window's panes, each with an optional command to run.
    pub panes: Vec<Option<String>>,
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
        };

        match val {
            Value::Null => w.panes.push(None),
            Value::String(value) => w.panes.push(Some(value)),
            Value::Mapping(map) => {
                w.layout = map
                    .get(&Value::String("layout".into()))
                    .map(|v| v.as_str().unwrap().into())
                    .unwrap_or_else(|| "tiled".into());

                if let Some(Value::Sequence(panes)) = map.get(&Value::String("panes".into())) {
                    for pane in panes {
                        if pane.is_string() {
                            w.panes.push(pane.as_str().map(|v| v.into()))
                        } else {
                            w.panes.push(None)
                        }
                    }
                }
            }
            _ => return Err("invalid window struct").map_err(de::Error::custom),
        }
        Ok(w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_test() {
        let windows_strings = vec![
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
        assert_eq!(windows[1].panes, vec![Some("vim".into())]);

        assert_eq!(windows[2].name, "window3");
        assert_eq!(windows[2].layout, "tiled".to_string());
        assert_eq!(
            windows[2].panes,
            vec![Some("vim".into()), None, Some("npm run serve".into())]
        );

        assert_eq!(windows[3].name, "window4");
        assert_eq!(windows[3].layout, "main-vertical".to_string());
        assert_eq!(
            windows[3].panes,
            vec![Some("vim".into()), None, Some("npm run serve".into())]
        );
    }
}
