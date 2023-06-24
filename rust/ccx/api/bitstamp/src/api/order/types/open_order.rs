use serde::Deserialize;

use crate::Atom;
use crate::Decimal;

#[derive(Clone, Debug, Deserialize)]
pub struct OpenOrder {
    pub id: u64,
    pub datetime: String,
    #[serde(with = "open_order_type")]
    pub r#type: OpenOrderType,
    pub price: Decimal,
    pub amount: Decimal,
    pub amount_at_create: Decimal,
    pub currency_pair: Atom,
    pub client_order_id: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum OpenOrderType {
    Buy,
    Sell,
}

mod open_order_type {
    use serde::de::{self, Deserialize, Deserializer};

    use super::OpenOrderType;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<OpenOrderType, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 0 - Buy; 1 - Sell.
        let n = u8::deserialize(deserializer)?;
        match n {
            0 => Ok(OpenOrderType::Buy),
            1 => Ok(OpenOrderType::Sell),
            _ => Err(de::Error::custom(format!("invalid type: {}", n))),
        }
    }
}
