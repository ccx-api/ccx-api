use bon::Builder;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::NoneAsEmptyString;
use serde_with::{serde_as, skip_serializing_none};

use crate::prelude::CurrencyPair;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;
use crate::types::trading::{OrderParams, OrderType};

use super::{OrderDescription, TxId};

/// Add a new order.
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Debug, Builder)]
#[builder(on(_, into))]
pub struct AddOrder {
    /// Asset pair id or altname
    pair: CurrencyPair,
    /// RFC3339 timestamp after which the matching engine should reject the new order request (optional)
    deadline: Option<String>,
    /// Validate inputs only. Do not submit order (optional)
    validate: Option<bool>,
    #[serde(flatten)]
    params: OrderParams,
    /// Conditional close order type (optional)
    #[serde(rename = "close[ordertype]")]
    close_ordertype: Option<OrderType>,
    /// Conditional close order price (optional)
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "close[price]")]
    close_price: Option<Decimal>,
    /// Conditional close order price2 (optional)
    #[serde_as(as = "NoneAsEmptyString")]
    #[serde(rename = "close[price2]")]
    close_price2: Option<Decimal>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AddOrderResponse {
    /// Order description
    pub descr: OrderDescription,
    /// Array of transaction IDs for order
    #[serde(default)]
    pub txid: Vec<TxId>,
}

impl Response for AddOrderResponse {}

impl Request for AddOrder {
    type Response = AddOrderResponse;

    const HTTP_METHOD: http::Method = http::Method::POST;

    const ENDPOINT: &'static str = "/0/private/AddOrder";

    const COSTS: &'static RateLimitType = &RateLimitType::Order;
}

impl SignedRequest for AddOrder {}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    mod serialize {
        use crate::types::trading::{ClientId, OrderFlag, OrderSide, OrderType, StpType};

        use super::*;
        use serde_json::json;
        use similar_asserts::assert_eq;

        #[test]
        fn only_required_fields() {
            let add_order = AddOrder::builder()
                .pair("BTC/USD")
                .params(
                    OrderParams::builder()
                        .volume(dec!(2.25))
                        .ordertype(OrderType::Market)
                        .side(OrderSide::Buy)
                        .build(),
                )
                .build();

            let actual = serde_json::to_value(&add_order).unwrap();
            let expected = json!({
                "type": "buy",
                "volume": "2.25",
                "pair": "BTC/USD",
                "ordertype": "market"
            });

            assert_eq!(actual, expected);
        }

        #[test]
        fn options_renaming() {
            let add_order = AddOrder::builder()
                .pair("BTC/USD")
                .params(
                    OrderParams::builder()
                        .volume(dec!(2.25))
                        .ordertype(OrderType::StopLossLimit)
                        .side(OrderSide::Sell)
                        .price(dec!(1.25))
                        .client_id(ClientId::Userref(25))
                        .leverage(dec!(128))
                        .oflags(vec![OrderFlag::Fcib, OrderFlag::Nompp])
                        .stptype(StpType::CancelOldest)
                        .build(),
                )
                .close_ordertype(OrderType::TakeProfitLimit)
                .close_price2(dec!(3))
                .build();

            let actual = serde_json::to_value(&add_order).unwrap();
            let expected = json!({
                "type": "sell",
                "volume": "2.25",
                "pair": "BTC/USD",
                "ordertype": "stop-loss-limit",
                "price": "1.25",
                "userref": 25,
                "oflags": "fcib,nompp",
                "leverage": "128",
                "close[price2]": "3",
                "close[ordertype]": "take-profit-limit",
                "stptype": "cancel-oldest"
            });

            assert_eq!(actual, expected);
        }
    }
}
