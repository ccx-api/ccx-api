use serde::Deserialize;

use crate::Atom;
use crate::Decimal;

#[derive(Clone, Debug, Deserialize)]
pub struct MarketOrder {
    pub id: u64,
    pub market: Atom,
    pub datetime: String,
    #[serde(with = "market_order_type")]
    pub r#type: MarketOrderType,
    pub price: Decimal,
    pub amount: Decimal,
    pub client_order_id: Option<Atom>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum MarketOrderType {
    Buy,
    Sell,
}

mod market_order_type {
    use serde::de::{self, Deserialize, Deserializer};

    use super::MarketOrderType;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<MarketOrderType, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 0 - Buy; 1 - Sell.
        let n = u8::deserialize(deserializer)?;
        match n {
            0 => Ok(MarketOrderType::Buy),
            1 => Ok(MarketOrderType::Sell),
            _ => Err(de::Error::custom(format!("invalid type: {}", n))),
        }
    }
}
