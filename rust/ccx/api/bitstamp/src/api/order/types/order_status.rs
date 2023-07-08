use std::collections::HashMap;

use serde::Deserialize;

use super::OrderId;
use crate::Atom;
use crate::Decimal;
use crate::DtBitstamp;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum OrderStatusType {
    Open,
    Finished,
    Expired,
    Canceled,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum OrderStatusTransactionType {
    Deposit,
    Withdrawal,
    MarketTrade,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OrderStatus {
    pub id: OrderId,
    pub status: OrderStatusType,
    pub transactions: Vec<OrderStatusTransaction>,
    pub amount_remaining: Decimal,
    pub client_order_id: Option<String>,
    pub currency_pair: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OrderStatusTransaction {
    pub tid: OrderId,
    pub price: Decimal,
    pub fee: Decimal,
    pub datetime: DtBitstamp,
    #[serde(with = "order_status_transaction_type")]
    pub r#type: OrderStatusTransactionType,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl OrderStatusTransaction {
    pub fn find_volume<P: AsRef<str>>(&self, pair: P) -> Option<Decimal> {
        self.other
            .get(pair.as_ref())
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse::<Decimal>().ok())
    }
}

mod order_status_transaction_type {
    use serde::de::{self, Deserialize, Deserializer};

    use super::OrderStatusTransactionType as Type;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Type, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 0 - deposit; 1 - withdrawal; 2 - market trade
        let n = u8::deserialize(deserializer)?;
        match n {
            0 => Ok(Type::Deposit),
            1 => Ok(Type::Withdrawal),
            2 => Ok(Type::MarketTrade),
            _ => Err(de::Error::custom(format!("invalid type: {}", n))),
        }
    }
}
