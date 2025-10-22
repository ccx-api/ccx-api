use std::borrow::Cow;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::de::value::StrDeserializer;

pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    match value {
        Some(value) => value.serialize(serializer),
        None => "".serialize(serializer),
    }
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    match StringOrOption::<T>::deserialize(deserializer)? {
        StringOrOption::String(s) if s.is_empty() => Ok(None),
        StringOrOption::String(s) => T::deserialize(StrDeserializer::new(&s)).map(Some),
        StringOrOption::Option(option) => Ok(option),
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StringOrOption<'a, T> {
    String(#[serde(borrow)] Cow<'a, str>),
    Option(Option<T>),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use rust_decimal::Decimal;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Test {
        #[serde(with = "super", default)]
        rate: Option<Decimal>,
        #[serde(with = "super", default)]
        description: Option<String>,
    }

    #[test]
    fn all_none() {
        let case = Test {
            rate: None,
            description: None,
        };
        let json = r#"{}"#;
        assert_eq!(case, serde_json::from_str(json).unwrap());
        assert_eq!(
            r#"{"rate":"","description":""}"#,
            serde_json::to_string(&case).unwrap()
        );
    }

    #[test]
    fn all_some() {
        let case = Test {
            rate: Decimal::from_str("1.25").ok(),
            description: Some("Sample text".to_owned()),
        };
        let json = r#"{"rate":"1.25","description":"Sample text"}"#;
        assert_eq!(case, serde_json::from_str(json).unwrap());
        assert_eq!(json, serde_json::to_string(&case).unwrap());
    }

    #[test]
    fn all_some_escaped_str() {
        let case = Test {
            rate: Decimal::from_str("1.25").ok(),
            description: Some(r#"Sample text: \"Sample\""#.to_owned()),
        };
        let json = r#"{"rate":"1.25","description":"Sample text: \\\"Sample\\\""}"#;
        assert_eq!(case, serde_json::from_str(json).unwrap());
        assert_eq!(json, serde_json::to_string(&case).unwrap());
    }

    #[test]
    fn all_empty_str() {
        let case = Test {
            rate: None,
            description: None,
        };
        let json = r#"{"rate":"","description":""}"#;
        assert_eq!(case, serde_json::from_str(json).unwrap());
        assert_eq!(json, serde_json::to_string(&case).unwrap());
    }
}
