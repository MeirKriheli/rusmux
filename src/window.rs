use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_yaml::Value;
use std::fmt;

#[derive(Debug, PartialEq, Serialize)]
pub struct Window {
    pub name: String,
    pub layout: Option<String>,
    pub panes: Vec<Option<String>>,
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
            Value::Null => (),
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
