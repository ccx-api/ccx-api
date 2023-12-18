use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::Deserialize;
use serde::Serialize;

use crate::util::empty_str::EmptyStr;

pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match value {
        Some(ref v) => v.serialize(serializer),
        None => serializer.serialize_str(""),
    }
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum EmptyStrOrOptionT<T> {
        EmptyString(EmptyStr),
        Value(Option<T>),
    }

    let val = EmptyStrOrOptionT::deserialize(deserializer)?;
    match val {
        EmptyStrOrOptionT::EmptyString(_) => Ok(None),
        EmptyStrOrOptionT::Value(v) => Ok(v),
    }
}

#[cfg(test)]
mod tests {
    use super::super::maybe_str;
    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct StrExample {
        #[serde(with = "maybe_str")]
        pub value: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct BoolExample {
        #[serde(with = "maybe_str")]
        pub value: Option<bool>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct F64Example {
        #[serde(with = "maybe_str")]
        pub value: Option<f64>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct SeqExample {
        #[serde(with = "maybe_str")]
        pub value: Option<Vec<i32>>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct MapExample {
        #[serde(with = "maybe_str")]
        pub value: Option<Struct>,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Struct {
        pub one: i32,
        pub two: i32,
    }

    #[test]
    fn test_empty_string() {
        let example = StrExample { value: None };

        let serialized = serde_json::to_string(&example).unwrap();
        assert_eq!(serialized, r#"{"value":""}"#);

        let deserialized: StrExample = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, example);
    }

    #[test]
    fn test_value_in_string() {
        let example = StrExample {
            value: Some("Hello, world!".to_string()),
        };

        let serialized = serde_json::to_string(&example).unwrap();
        assert_eq!(serialized, r#"{"value":"Hello, world!"}"#);

        let deserialized: StrExample = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, example);
    }

    #[test]
    fn test_bool() {
        let example = BoolExample { value: Some(true) };

        let serialized = serde_json::to_string(&example).unwrap();
        assert_eq!(serialized, r#"{"value":true}"#);

        let deserialized: BoolExample = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, example);
    }

    #[test]
    fn test_f64() {
        let example = F64Example { value: Some(42.5) };

        let serialized = serde_json::to_string(&example).unwrap();
        assert_eq!(serialized, r#"{"value":42.5}"#);

        let deserialized: F64Example = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, example);
    }

    #[test]
    fn test_seq() {
        let example = SeqExample {
            value: Some(vec![1, 2, 3]),
        };

        let serialized = serde_json::to_string(&example).unwrap();
        assert_eq!(serialized, r#"{"value":[1,2,3]}"#);

        let deserialized: SeqExample = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, example);
    }

    #[test]
    fn test_map() {
        let example = MapExample {
            value: Some(Struct { one: 1, two: 2 }),
        };

        let serialized = serde_json::to_string(&example).unwrap();
        assert_eq!(serialized, r#"{"value":{"one":1,"two":2}}"#);

        let deserialized: MapExample = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, example);
    }
}
