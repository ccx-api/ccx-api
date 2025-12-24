//! Deserialize `f64`/`Option<f64>` while tolerating `serde_json`'s
//! `arbitrary_precision` tagged map representation.
use core::fmt;

use serde::Deserialize;
use serde::Deserializer;
use serde::de::Error;
use serde::de::MapAccess;
use serde::de::Visitor;

const SERDE_JSON_NUMBER_TOKEN: &str = "$serde_json::private::Number";

/// Deserialize a floating number from normal JSON numbers/strings or the
/// `$serde_json::private::Number` map used by `serde_json` with
/// `arbitrary_precision`.
pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct F64Compat;

    impl<'de> Visitor<'de> for F64Compat {
        type Value = f64;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str(
                "a JSON number, a numeric string, or serde_json arbitrary_precision tagged number",
            )
        }

        fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
            Ok(v)
        }

        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            Ok(v as f64)
        }

        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v as f64)
        }

        fn visit_str<E: Error>(self, s: &str) -> Result<Self::Value, E> {
            s.parse::<f64>().map_err(E::custom)
        }

        fn visit_string<E: Error>(self, s: String) -> Result<Self::Value, E> {
            self.visit_str(&s)
        }

        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            while let Some((k, v)) = map.next_entry::<String, String>()? {
                if k == SERDE_JSON_NUMBER_TOKEN {
                    return v.parse::<f64>().map_err(A::Error::custom);
                }
            }
            Err(A::Error::custom("unexpected map for f64"))
        }
    }

    deserializer.deserialize_any(F64Compat)
}

/// Deserialize `Option<f64>` with the same compatibility rules.
pub fn deserialize_option<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Option::<CompatF64>::deserialize(deserializer)?.map(|v| v.0))
}

struct CompatF64(f64);

impl<'de> Deserialize<'de> for CompatF64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize(deserializer).map(CompatF64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_plain_number() {
        let v: f64 = deserialize(1.5_f64.into_deserializer()).unwrap();
        assert_eq!(v, 1.5);
    }

    #[test]
    fn parse_string_number() {
        let v: f64 = deserialize("1.5".into_deserializer()).unwrap();
        assert_eq!(v, 1.5);
    }

    #[test]
    fn parse_tagged_map() {
        // Mimic serde_json arbitrary_precision tagged map.
        use serde::de::value::MapDeserializer;
        use serde::de::value::StrDeserializer;

        let entries = core::iter::once((
            StrDeserializer::<serde::de::value::Error>::new(SERDE_JSON_NUMBER_TOKEN),
            StrDeserializer::<serde::de::value::Error>::new("1.2345"),
        ));
        let map_de = MapDeserializer::new(entries);
        let v: f64 = deserialize(map_de).unwrap();
        assert_eq!(v, 1.2345);
    }
}
