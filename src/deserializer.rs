use serde::de::{
    self, DeserializeOwned, DeserializeSeed, Deserializer, EnumAccess, MapAccess, SeqAccess,
    VariantAccess, Visitor,
};
use std::fmt;

use crate::eval;

#[derive(Debug)]
pub struct PklDeserializeError {
    message: String,
}

impl fmt::Display for PklDeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pkl deserialization error: {}", self.message)
    }
}

impl std::error::Error for PklDeserializeError {}

impl de::Error for PklDeserializeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        PklDeserializeError {
            message: msg.to_string(),
        }
    }
}

pub fn from_pkl_value<T: DeserializeOwned>(value: &pklr::Value) -> Result<T, PklDeserializeError> {
    let deserializer = PklValueDeserializer { value };
    T::deserialize(deserializer)
}

pub async fn eval_to_typed<T: DeserializeOwned>(
    path: &std::path::Path,
    options: pklr::EvalOptions,
) -> miette::Result<T> {
    let value = eval::eval_to_value(path, options).await?;
    from_pkl_value(&value)
        .map_err(|e| miette::miette!("Failed to deserialize '{}': {e}", path.display()))
}

pub async fn eval_source_to_typed<T: DeserializeOwned>(
    source: &str,
    options: pklr::EvalOptions,
) -> miette::Result<T> {
    let value = eval::eval_source_to_value(source, options).await?;
    from_pkl_value(&value).map_err(|e| miette::miette!("Failed to deserialize Pkl expression: {e}"))
}

pub fn pkl_string_literal(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

pub struct PklValueDeserializer<'de> {
    pub value: &'de pklr::Value,
}

impl<'de> Deserializer<'de> for PklValueDeserializer<'de> {
    type Error = PklDeserializeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self.value {
            pklr::Value::Null => visitor.visit_unit(),
            pklr::Value::Bool(b) => visitor.visit_bool(*b),
            pklr::Value::Int(n) => visitor.visit_i64(*n),
            pklr::Value::Float(f) => visitor.visit_f64(*f),
            pklr::Value::String(s) => visitor.visit_borrowed_str(s.as_str()),
            pklr::Value::Object(map, _) => {
                let mut ma = MapAccessor {
                    iter: map.iter().peekable(),
                    value: None,
                };
                visitor.visit_map(&mut ma)
            }
            pklr::Value::List(items) => {
                let mut sa = SeqAccessor { iter: items.iter() };
                visitor.visit_seq(&mut sa)
            }
            pklr::Value::Lambda(..) => Err(de::Error::custom(
                "lambda expressions cannot be deserialized",
            )),
        }
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Bool(b) = self.value {
            visitor.visit_bool(*b)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Int(n) = self.value {
            visitor.visit_i64(*n)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Int(n) = self.value {
            visitor.visit_i64(*n)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Int(n) = self.value {
            visitor.visit_i64(*n)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Int(n) = self.value {
            visitor.visit_i64(*n)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Int(n) = self.value {
            visitor.visit_i64(*n)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Int(n) = self.value {
            visitor.visit_i64(*n)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Int(n) = self.value {
            visitor.visit_i64(*n)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Int(n) = self.value {
            visitor.visit_i64(*n)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Float(f) = self.value {
            visitor.visit_f64(*f)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Float(f) = self.value {
            visitor.visit_f64(*f)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::String(s) = self.value {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => visitor.visit_char(c),
                _ => Err(de::Error::custom("expected single character")),
            }
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::String(s) = self.value {
            visitor.visit_borrowed_str(s.as_str())
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(de::Error::custom("bytes not supported"))
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(de::Error::custom("byte buffers not supported"))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if matches!(self.value, pklr::Value::Null) {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if matches!(self.value, pklr::Value::Null) {
            visitor.visit_unit()
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::List(items) = self.value {
            let mut sa = SeqAccessor { iter: items.iter() };
            visitor.visit_seq(&mut sa)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        if let pklr::Value::Object(map, _) = self.value {
            let mut ma = MapAccessor {
                iter: map.iter().peekable(),
                value: None,
            };
            visitor.visit_map(&mut ma)
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        if let pklr::Value::String(s) = self.value {
            visitor.visit_enum(PklEnumAccess { value: s.as_str() })
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }
}

struct SeqAccessor<'de> {
    iter: std::slice::Iter<'de, pklr::Value>,
}

impl<'de> SeqAccess<'de> for SeqAccessor<'de> {
    type Error = PklDeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(PklValueDeserializer { value }).map(Some),
            None => Ok(None),
        }
    }
}

struct MapAccessor<'de> {
    iter: std::iter::Peekable<indexmap::map::Iter<'de, String, pklr::Value>>,
    value: Option<&'de pklr::Value>,
}

impl<'de> MapAccess<'de> for MapAccessor<'de> {
    type Error = PklDeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(PklStrDeserializer {
                    value: key.as_str(),
                })
                .map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self
            .value
            .take()
            .expect("next_key_seed must be called before next_value_seed");
        seed.deserialize(PklValueDeserializer { value })
    }
}

struct PklStrDeserializer<'de> {
    value: &'de str,
}

impl<'de> Deserializer<'de> for PklStrDeserializer<'de> {
    type Error = PklDeserializeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_borrowed_str(self.value)
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_any(visitor)
    }
    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_borrowed_str(self.value)
    }
    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_string(self.value.to_owned())
    }
    fn deserialize_bytes<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(de::Error::custom("bytes not supported"))
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(de::Error::custom("byte buffers not supported"))
    }
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_some(self)
    }
    fn deserialize_unit<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(de::Error::custom("expected string, found unit for map key"))
    }
    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_unit(visitor)
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_seq<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(de::Error::custom(
            "expected string, found sequence for map key",
        ))
    }
    fn deserialize_tuple<V: Visitor<'de>>(
        self,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_seq(visitor)
    }
    fn deserialize_map<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(de::Error::custom("expected string, found map for map key"))
    }
    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        self.deserialize_map(visitor)
    }
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_enum(PklEnumAccess { value: self.value })
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }
}

struct PklEnumAccess<'de> {
    value: &'de str,
}

impl<'de> EnumAccess<'de> for PklEnumAccess<'de> {
    type Error = PklDeserializeError;
    type Variant = PklUnitVariant;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(PklStrDeserializer { value: self.value })?;
        Ok((variant, PklUnitVariant))
    }
}

struct PklUnitVariant;

impl<'de> VariantAccess<'de> for PklUnitVariant {
    type Error = PklDeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        Err(de::Error::custom(
            "unit enum expected, found newtype variant",
        ))
    }

    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("unit enum expected, found tuple variant"))
    }

    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom(
            "unit enum expected, found struct variant",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::sync::Arc;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Simple {
        name: String,
        count: i32,
    }

    fn pkl_value(s: &str) -> pklr::Value {
        match s {
            "null" => pklr::Value::Null,
            "true" => pklr::Value::Bool(true),
            "false" => pklr::Value::Bool(false),
            n if n.starts_with('"') => pklr::Value::String(n.trim_matches('"').to_string()),
            n if n.parse::<i64>().is_ok() => pklr::Value::Int(n.parse().unwrap()),
            n if n.parse::<f64>().is_ok() => pklr::Value::Float(n.parse().unwrap()),
            _ => panic!("unsupported test value: {s}"),
        }
    }

    fn pkl_object(pairs: Vec<(&str, &str)>) -> pklr::Value {
        let mut map = indexmap::IndexMap::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), pkl_value(v));
        }
        pklr::Value::Object(Arc::new(map), None)
    }

    fn pkl_list(items: Vec<&str>) -> pklr::Value {
        pklr::Value::List(items.into_iter().map(pkl_value).collect())
    }

    #[test]
    fn deserialize_null_is_unit() {
        let val = pkl_value("null");
        assert_eq!(from_pkl_value::<()>(&val).unwrap(), ());
    }

    #[test]
    fn deserialize_bool() {
        let val = pkl_value("true");
        assert!(from_pkl_value::<bool>(&val).unwrap());

        let val = pkl_value("false");
        assert!(!from_pkl_value::<bool>(&val).unwrap());
    }

    #[test]
    fn deserialize_int() {
        let val = pkl_value("42");
        assert_eq!(from_pkl_value::<i32>(&val).unwrap(), 42);
    }

    #[test]
    fn deserialize_float() {
        let val = pkl_value("2.5");
        assert_eq!(from_pkl_value::<f64>(&val).unwrap(), 2.5);
    }

    #[test]
    fn deserialize_string() {
        let val = pkl_value("\"hello\"");
        assert_eq!(from_pkl_value::<String>(&val).unwrap(), "hello");
    }

    #[test]
    fn deserialize_option_none() {
        let val = pkl_value("null");
        assert_eq!(from_pkl_value::<Option<i32>>(&val).unwrap(), None);
    }

    #[test]
    fn deserialize_option_some() {
        let val = pkl_value("42");
        assert_eq!(from_pkl_value::<Option<i32>>(&val).unwrap(), Some(42));
    }

    #[test]
    fn deserialize_list() {
        let val = pkl_list(vec!["1", "2", "3"]);
        assert_eq!(from_pkl_value::<Vec<i32>>(&val).unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn deserialize_object_to_struct() {
        let val = pkl_object(vec![("name", "\"test\""), ("count", "99")]);
        let result: Simple = from_pkl_value(&val).unwrap();
        assert_eq!(
            result,
            Simple {
                name: "test".to_string(),
                count: 99
            }
        );
    }

    #[test]
    fn deserialize_nested_object() {
        let inner = pkl_object(vec![("x", "1"), ("y", "2")]);
        let inner_arc = match inner {
            pklr::Value::Object(m, _) => m,
            _ => unreachable!(),
        };
        let mut outer_map = indexmap::IndexMap::new();
        outer_map.insert("inner".to_string(), pklr::Value::Object(inner_arc, None));
        let outer = pklr::Value::Object(Arc::new(outer_map), None);

        #[derive(Debug, Deserialize, PartialEq)]
        struct Inner {
            x: i32,
            y: i32,
        }
        #[derive(Debug, Deserialize, PartialEq)]
        struct Outer {
            inner: Inner,
        }

        assert_eq!(
            from_pkl_value::<Outer>(&outer).unwrap(),
            Outer {
                inner: Inner { x: 1, y: 2 }
            }
        );
    }

    #[test]
    fn deserialize_enum_from_string() {
        #[derive(Debug, Deserialize, PartialEq)]
        enum Role {
            #[serde(rename = "admin")]
            Admin,
            #[serde(rename = "user")]
            User,
        }

        let val = pkl_value("\"admin\"");
        assert_eq!(from_pkl_value::<Role>(&val).unwrap(), Role::Admin);

        let val = pkl_value("\"user\"");
        assert_eq!(from_pkl_value::<Role>(&val).unwrap(), Role::User);
    }

    #[test]
    fn pkl_string_literal_escapes_correctly() {
        assert_eq!(
            pkl_string_literal("quote: \" newline:\n slash: \\"),
            "\"quote: \\\" newline:\\n slash: \\\\\""
        );
    }

    #[test]
    fn pkl_string_literal_roundtrip() {
        let s = "hello world";
        let literal = pkl_string_literal(s);
        assert_eq!(literal, "\"hello world\"");
    }
}
