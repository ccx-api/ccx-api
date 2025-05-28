use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

pub type AssetName = SmartString<10>;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum AssetClass {
    #[serde(rename = "currency")]
    Currency,
    // TODO other classes ?
}
