use std::{fmt, marker::PhantomData};

use serde::{
    de::{self, Error, MapAccess, Visitor},
    Deserialize, Deserializer,
};

use crate::configuration::{ColorNamespace, NamespacedColor, NamespacedThemeElement};

// Based on https://serde.rs/string-or-struct.html
pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + From<&'de str> + 'de,
    D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + From<&'de str> + 'de,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_borrowed_str<E>(self, value: &'de str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(From::from(value))
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

impl<'de: 'a, 'a> Deserialize<'de> for NamespacedColor<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;

        let (namespace, color_name) = if let Some((namespace, color_name)) = s.split_once('.') {
            (ColorNamespace::Group(namespace), color_name)
        } else {
            (ColorNamespace::Colors, s)
        };

        Ok(NamespacedColor {
            namespace,
            color_name,
        })
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for NamespacedThemeElement<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;

        let mut iter = s.splitn(2, '.');
        let theme_namespace = iter
            .next()
            .ok_or(D::Error::custom("expected a theme element name"))?;
        let element_name = iter
            .next()
            .ok_or(D::Error::custom("expected a theme element name"))?;

        Ok(NamespacedThemeElement {
            theme_namespace,
            element_name,
        })
    }
}
