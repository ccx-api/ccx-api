use std::fmt;

use serde::de::Error;
use serde::de::Visitor;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

pub struct EmptyStr;

impl Serialize for EmptyStr {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str("")
    }
}

impl<'de> Deserialize<'de> for EmptyStr {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EmptyStringVisitor;

        impl<'de> Visitor<'de> for EmptyStringVisitor {
            type Value = EmptyStr;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an empty string")
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                if value.is_empty() {
                    Ok(EmptyStr)
                } else {
                    Err(Error::invalid_value(
                        serde::de::Unexpected::Str(value),
                        &self,
                    ))
                }
            }
        }

        deserializer.deserialize_any(EmptyStringVisitor)
    }
}
