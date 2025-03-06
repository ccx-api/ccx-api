use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Commission {
    pub maker: Decimal,
    pub taker: Decimal,
    pub buyer: Decimal,
    pub seller: Decimal,
}
