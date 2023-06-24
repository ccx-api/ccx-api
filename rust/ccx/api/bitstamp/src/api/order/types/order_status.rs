use serde::Deserialize;

use crate::Atom;
use crate::Decimal;

#[derive(Clone, Debug, Deserialize)]
pub struct OrderStatus {
    pub status: OrderStatusType,
    pub id: u64,
    pub transactions: Vec<OrderStatusTransaction>,
    pub amount_remaining: Decimal,
    pub client_order_id: Option<String>,
    pub currency_pair: String,
}

// tid, usd, price, fee, btc, datetime and type ()
#[derive(Clone, Debug, Deserialize)]
pub struct OrderStatusTransaction {
    pub tid: u64,
    pub usd: Decimal,
    pub price: Decimal,
    pub fee: Decimal,
    pub btc: Decimal,
    pub datetime: Atom,
    #[serde(with = "order_status_transaction_type")]
    pub r#type: OrderStatusTransactionType,
}

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
