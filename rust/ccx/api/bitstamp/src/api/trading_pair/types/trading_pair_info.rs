use serde::Deserialize;

use crate::Atom;
use crate::Decimal;

#[derive(Debug, Deserialize)]
pub struct TradingPairInfo {
    pub name: Atom,
    pub url_symbol: Atom,
    pub base_decimals: u8,
    pub counter_decimals: u8,
    pub instant_order_counter_decimals: u8,
    #[serde(with = "minimum_order")]
    pub minimum_order: MinimumOrder,
    pub trading: Status,
    pub instant_and_market_orders: Status,
    pub description: Atom,
}

#[derive(Debug, Deserialize)]
pub struct MinimumOrder {
    pub currency: Atom,
    pub amount: Decimal,
}

#[derive(Debug, Deserialize)]
pub enum Status {
    Disabled,
    Enabled,
}

impl Status {
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled)
    }
}

mod minimum_order {
    use serde::de::{self, Deserializer};
    use serde::Deserialize;

    use super::MinimumOrder;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<MinimumOrder, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        let index = value.find(char::is_whitespace).ok_or_else(|| {
            serde::de::Error::custom(format!("Invalid MinimumOrder value: {}", value))
        })?;
        let (amount, currency) = value.split_at(index);

        Ok(MinimumOrder {
            amount: amount.parse().map_err(de::Error::custom)?,
            currency: currency.trim().into(),
        })
    }
}
