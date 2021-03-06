use crate::error::AppError;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::{self, Value};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, PartialEq, Serialize)]
pub struct Window {
    pub name: String,
    pub layout: Option<String>,
    pub panes: Vec<Option<String>>,
}

impl TryFrom<String> for Window {
    type Error = AppError;

    fn try_from(yaml: String) -> Result<Self, Self::Error> {
        serde_yaml::from_str(&yaml)
            .map_err(|e| AppError::Message(format!("Cannot parse yaml: {}", e)))
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
            layout: None,
            panes: vec![],
        };

        match val {
            Value::Null => w.panes.push(None),
            Value::String(value) => w.panes.push(Some(value)),
            Value::Mapping(map) => {
                w.layout = map
                    .get(&Value::String("layout".into()))
                    .map(|v| v.as_str().unwrap().into());

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
        assert_eq!(windows[0].layout, None);
        assert_eq!(windows[0].panes, vec![None]);

        assert_eq!(windows[1].name, "test2 window");
        assert_eq!(windows[1].layout, None);
        assert_eq!(windows[1].panes, vec![Some("vim".into())]);

        assert_eq!(windows[2].name, "window3");
        assert_eq!(windows[2].layout, None);
        assert_eq!(
            windows[2].panes,
            vec![Some("vim".into()), None, Some("npm run serve".into())]
        );

        assert_eq!(windows[3].name, "window4");
        assert_eq!(windows[3].layout, Some("main-vertical".into()));
        assert_eq!(
            windows[3].panes,
            vec![Some("vim".into()), None, Some("npm run serve".into())]
        );
    }
}
