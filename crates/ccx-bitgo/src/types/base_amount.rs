//! Base amount types for BitGo API
//!
//! This module contains types for handling base unit values that are represented
//! as strings in the BitGo API but should be treated as integers in Rust code.

use derive_more::{Deref, Display, From, FromStr};
use serde_with::{DeserializeFromStr, SerializeDisplay};

/// Represents a value in base units (e.g., satoshis, wei) that is sent as a string
/// by the BitGo API but should be treated as an integer value.
///
/// This wrapper type handles automatic deserialization and serialization
/// from/to string representations
/// while maintaining type safety for base unit values.
///
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Default,
    DeserializeFromStr,
    SerializeDisplay,
    From,
    Deref,
    FromStr,
    Display,
)]
pub struct BaseAmount(i128);

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_deserialization_from_string() {
        let json = r#""1000000""#;
        let value: BaseAmount = serde_json::from_str(json).expect("Failed to deserialize");
        assert_eq!(*value, 1000000);
    }

    #[test]
    fn test_serialization() {
        let value = BaseAmount::from(1000000i128);
        let json = serde_json::to_string(&value).expect("Failed to serialize");
        assert_eq!(json, r#""1000000""#);
    }
}
