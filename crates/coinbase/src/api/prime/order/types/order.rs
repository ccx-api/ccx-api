use crate::api::prime::prelude::*;
use crate::api::prime::PortfolioOrderSide;
use crate::api::prime::PortfolioOrderStatus;
use crate::api::prime::PortfolioOrderTimeInForce;
use crate::api::prime::PortfolioOrderType;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AccountPortfolioOrder {
    /// The unique order ID generated by Coinbase.
    pub id: Uuid,
    /// The ID of the user that created the order.
    pub user_id: Uuid,
    /// The ID of the portfolio that owns the order.
    pub portfolio_id: Uuid,
    /// The ID of the product being traded by the order.
    pub product_id: Atom,
    pub side: PortfolioOrderSide,
    /// A client-generated order ID used for reference purposes (note: order will be rejected
    /// if this ID is not unique among all currently active orders).
    pub client_order_id: String,
    pub r#type: PortfolioOrderType,
    /// Order size in base asset units (either `base_quantity` or `quote_value` is required).
    #[serde(default, with = "maybe_str")]
    pub base_quantity: Option<Decimal>,
    /// Order size in quote asset units, i.e. the amount the user wants to spend (when buying) or
    /// receive (when selling); the quantity in base units will be determined based on the market
    /// liquidity and indicated `quote_value`. Either `base_quantity` or `quote_value` is required.
    #[serde(default, with = "maybe_str")]
    pub quote_value: Option<Decimal>,
    /// The limit price (required for limit orders).
    #[serde(default, with = "maybe_str")]
    pub limit_price: Option<Decimal>,
    /// The start time of the order in UTC (only applies to TWAP orders).
    pub start_time: Option<DtCoinbasePrime>,
    /// The expiry time of the order in UTC (applies to TWAP orders and limit orders
    /// with `time_in_force` set to `GTD`).
    pub expiry_time: Option<DtCoinbasePrime>,
    pub status: PortfolioOrderStatus,
    pub time_in_force: PortfolioOrderTimeInForce,
    /// The order creation time as a UTC timestamp.
    pub created_at: DtCoinbasePrime,
    /// Size filled (in base asset units).
    pub filled_quantity: Decimal,
    /// Market value filled (in quote asset units).
    pub filled_value: Decimal,
    /// Indicates the average `filled_price`.
    pub average_filled_price: Decimal,
    /// Total commission paid on this order (in quote asset units)
    /// -- only applicable for partially- or fully-filled orders.
    pub commission: Decimal,

    #[serde(default, with = "maybe_str")]
    pub exchange_fee: Option<Decimal>,
    #[serde(default, with = "maybe_str")]
    pub historical_pov: Option<Decimal>,
    #[serde(default, with = "maybe_str")]
    pub liquidity: Option<Decimal>,
}

#[cfg(test)]
mod tests {
    use ccx_api_lib::dec;

    use super::*;

    fn uuid(s: &str) -> Uuid {
        Uuid::parse_str(s).unwrap()
    }

    #[test]
    fn test_deserialize_order_live1() {
        let json = r#"{
            "id": "876f946a-0000-0000-0000-000000000000",
            "user_id": "d74922fc-0000-0000-0000-000000000000",
            "portfolio_id": "20de012c-0000-0000-0000-000000000000",
            "product_id": "USDT-USD",
            "side": "SELL",
            "client_order_id": "8af9d1c1-0000-0000-0000-000000000000",
            "type": "MARKET",
            "base_quantity": "10.1",
            "quote_value": "",
            "limit_price": "",
            "start_time": null,
            "expiry_time": null,
            "status": "FILLED",
            "time_in_force": "IMMEDIATE_OR_CANCEL",
            "created_at": "2022-11-22T11:33:44.857992Z",
            "filled_quantity": "10.1",
            "filled_value": "10.101515",
            "average_filled_price": "1.00015",
            "commission": "0.0252537875",
            "exchange_fee": "",
            "historical_pov": "",
            "stop_price": ""
        }"#;

        let sample = AccountPortfolioOrder {
            id: uuid("876f946a-0000-0000-0000-000000000000"),
            user_id: uuid("d74922fc-0000-0000-0000-000000000000"),
            portfolio_id: uuid("20de012c-0000-0000-0000-000000000000"),
            product_id: "USDT-USD".into(),
            side: PortfolioOrderSide::Sell,
            client_order_id: "8af9d1c1-0000-0000-0000-000000000000".to_string(),
            r#type: PortfolioOrderType::Market,
            base_quantity: Some(dec!(10.1)),
            quote_value: None,
            limit_price: None,
            start_time: None,
            expiry_time: None,
            status: PortfolioOrderStatus::Filled,
            time_in_force: PortfolioOrderTimeInForce::ImmediateOrCancel,
            created_at: DtCoinbasePrime::parse_from_str("2022-11-22T11:33:44.857992Z").unwrap(),
            filled_quantity: dec!(10.1),
            filled_value: dec!(10.101515),
            average_filled_price: dec!(1.00015),
            commission: dec!(0.0252537875),
            exchange_fee: None,
            historical_pov: None,
            liquidity: None,
        };

        let order: AccountPortfolioOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order, sample);
    }

    #[test]
    fn test_deserialize_order_live2() {
        let json = r#"{
            "id": "c658cfd3-f946-4dcd-bbd6-61e00f6cf006",
            "user_id":"6a6ff611-33e8-5012-9989-a15bd5aaa3f6",
            "portfolio_id":"20de012c-7ad7-46ca-88fd-f785fab4e6e9",
            "product_id":"BTC-USDT",
            "side":"BUY",
            "client_order_id":"12345678-1234-5678-abcd-000000000001",
            "type":"MARKET",
            "base_quantity":"",
            "quote_value":"5",
            "limit_price":"",
            "start_time":null,
            "expiry_time":null,
            "status":"FILLED",
            "time_in_force":"IMMEDIATE_OR_CANCEL",
            "created_at":"2024-03-26T17:51:55.469280Z",
            "filled_quantity":"0.0000711050557919",
            "filled_value":"4.9875311720698254",
            "average_filled_price":"70143.1300000187153077",
            "commission":"0.0124688279301746",
            "exchange_fee":"",
            "historical_pov":"",
            "stop_price":""
        }"#;

        let sample = AccountPortfolioOrder {
            id: uuid("c658cfd3-f946-4dcd-bbd6-61e00f6cf006"),
            user_id: uuid("6a6ff611-33e8-5012-9989-a15bd5aaa3f6"),
            portfolio_id: uuid("20de012c-7ad7-46ca-88fd-f785fab4e6e9"),
            product_id: "BTC-USDT".into(),
            side: PortfolioOrderSide::Buy,
            client_order_id: "12345678-1234-5678-abcd-000000000001".to_string(),
            r#type: PortfolioOrderType::Market,
            base_quantity: None,
            quote_value: Some(dec!(5)),
            limit_price: None,
            start_time: None,
            expiry_time: None,
            status: PortfolioOrderStatus::Filled,
            time_in_force: PortfolioOrderTimeInForce::ImmediateOrCancel,
            created_at: DtCoinbasePrime::parse_from_str("2024-03-26T17:51:55.469280Z").unwrap(),
            filled_quantity: dec!(0.0000711050557919),
            filled_value: dec!(4.9875311720698254),
            average_filled_price: dec!(70143.1300000187153077),
            commission: dec!(0.0124688279301746),
            exchange_fee: None,
            historical_pov: None,
            liquidity: None,
        };

        let order: AccountPortfolioOrder = serde_json::from_str(json).unwrap();
        assert_eq!(order, sample);
    }
}
