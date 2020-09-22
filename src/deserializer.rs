use serde::de;
use serde::Deserialize;
use std::fmt;
use std::marker::PhantomData;

pub fn deserialize_optional_vec_or_string<'de, D>(d: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_option(OptionalVecOrStringVisitor)
}

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
