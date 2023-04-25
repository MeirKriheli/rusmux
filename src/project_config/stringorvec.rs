//! Implements custom visitors to handle de-serializing string or vec
//! of strings.
use serde::de;
use serde::Deserialize;
use std::fmt;
use std::marker::PhantomData;

/// Actually utilized [OptionalVecOrStringVisitor]. Used with serde's attribute
/// macro.
///
/// e.g.:
/// ```ignore
/// #[serde(default)]
/// #[serde(deserialize_with = "stringorvec::deserialize_optional_vec_or_string")]
/// pub on_project_start: Option<Vec<String>>,
/// ```
pub fn deserialize_optional_vec_or_string<'de, D>(d: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_option(OptionalVecOrStringVisitor)
}

/// Visitor de-serializing optional string (to a vec of single
/// string if specified) or optional vec of strings.
struct OptionalVecOrStringVisitor;

impl<'de> de::Visitor<'de> for OptionalVecOrStringVisitor {
    type Value = Option<Vec<String>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Optional String or Vec of Strings")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Option<Vec<String>>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(Some(d.deserialize_any(StringOrVecVisitor(PhantomData))?))
    }
}

/// Visitor de-serializing string (to a vec of single string) or vec of strings.
/// Visited from [OptionalVecOrStringVisitor].
struct StringOrVecVisitor(PhantomData<Vec<String>>);

impl<'de> de::Visitor<'de> for StringOrVecVisitor {
    type Value = Vec<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("string or list of strings")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(vec![value.to_owned()])
    }

    fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
    where
        S: de::SeqAccess<'de>,
    {
        Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
    }
}
