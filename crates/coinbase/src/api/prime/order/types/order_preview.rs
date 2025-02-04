use crate::api::prime::prelude::*;
use crate::api::prime::PortfolioOrderSide;
use crate::api::prime::PortfolioOrderType;
use crate::DtCoinbasePrime;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioOrderPreview {
    /// The ID of the portfolio that owns the order.
    pub portfolio_id: Uuid,
    /// The ID of the product being traded by the order.
    pub product_id: Atom,
    pub side: PortfolioOrderSide,
    pub r#type: PortfolioOrderType,
    /// Order size in base asset units (either `base_quantity` or `quote_value` is required).
    pub base_quantity: Decimal,
    /// Order size in quote asset units, i.e. the amount the user wants to spend (when buying) or
    /// receive (when selling); the quantity in base units will be determined based on the market
    /// liquidity and indicated `quote_value`. Either `base_quantity` or `quote_value` is required.
    pub quote_value: Decimal,
    /// The limit price (required for limit orders).
    #[serde(default, with = "maybe_str")]
    pub limit_price: Option<Decimal>,
    /// The start time of the order in UTC (only applies to TWAP orders).
    pub start_time: Option<DtCoinbasePrime>,
    /// The expiry time of the order in UTC (applies to TWAP orders and limit orders
    /// with `time_in_force` set to `GTD`).
    pub expiry_time: Option<DtCoinbasePrime>,
    /// Time in force used for order creation. Unlike base order it may contain
    /// values not present in [`PortfolioOrderTimeInForce`].
    ///
    /// [`PortfolioOrderTimeInForce`]: crate::api::prime::PortfolioOrderTimeInForce
    pub time_in_force: String,
    /// How much slippage is expected
    #[serde(default, with = "maybe_str")]
    pub slippage: Option<Decimal>,
    /// Current best bid for order book
    pub best_bid: Decimal,
    /// Current best ask for order book
    pub best_ask: Decimal,
    /// Indicates the average `filled_price`.
    pub average_filled_price: Decimal,
    pub order_total: Decimal,
    /// Total commission paid on this order (in quote asset units)
    /// -- only applicable for partially- or fully-filled orders.
    pub commission: Decimal,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_order_preview() {
        let json = r#"{
            "portfolio_id":"a226c3ea-0000-0000-0000-000000000000",
            "product_id":"APE-USDT",
            "side":"BUY",
            "type":"MARKET",
            "base_quantity":"1",
            "quote_value":"1.287",
            "limit_price":"",
            "start_time":null,
            "expiry_time":null,
            "time_in_force":"UNKNOWN_TIME_IN_FORCE",
            "commission":"0.0032175",
            "slippage":"0",
            "best_bid":"1.285",
            "best_ask":"1.287",
            "average_filled_price":"1.287",
            "order_total":"1.2902175",
            "historical_pov":""
        }"#;

        let _decoded: AccountPortfolioOrderPreview = serde_json::from_str(json).unwrap();
    }
}
