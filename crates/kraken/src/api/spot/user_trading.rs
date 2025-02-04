use super::prelude::*;
use super::RlPriorityLevel;
use super::RL_MATCHING_ENGINE_PER_MINUTE;
use super::RL_PRIVATE_PER_MINUTE;
use crate::client::Task;

pub const API_0_PRIVATE_ADD_ORDER: &str = "/0/private/AddOrder";

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
struct AddOrderRequest<'a> {
    userref: Option<u32>,
    ordertype: OrderType,
    r#type: OrderSide,
    volume: Option<Decimal>,
    pair: &'a str,
    price: Option<Decimal>,
    price2: Option<Decimal>,
    leverage: Option<&'a str>,
    oflags: Option<OrderFlags>,
    timeinforce: Option<TimeInForce>,
    starttm: Option<&'a str>,
    expiretm: Option<&'a str>,
    #[serde(rename = "close[ordertype]")]
    close_ordertype: Option<&'a str>,
    #[serde(rename = "close[price]")]
    close_price: Option<Decimal>,
    #[serde(rename = "close[price2]")]
    close_price2: Option<Decimal>,
    deadline: Option<&'a str>,
    validate: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AddOrderResponse {
    /// Transaction IDs for order.
    pub txid: Vec<String>,
    /// Order description info.
    pub descr: AddedOrderDescription,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ValidateOrderResponse {
    /// Order description info.
    pub descr: AddedOrderDescription,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct AddedOrderDescription {
    /// Order description.
    pub order: String,
    /// Conditional close order description, if applicable.
    pub close: Option<String>,
}

#[cfg(feature = "with_network")]
pub use with_network::*;

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;

    impl<S> SpotApi<S>
    where
        S: crate::client::KrakenSigner,
        S: Unpin + 'static,
    {
        /// Validate inputs only. Do not submit order.
        ///
        /// Note: See the AssetPairs endpoint for details on the available trading pairs,
        /// their price and quantity precisions, order minimums, available leverage, etc.
        #[allow(clippy::too_many_arguments)]
        pub fn validate_order(
            &self,
            nonce: Nonce,
            userref: Option<u32>,
            ordertype: OrderType,
            r#type: OrderSide,
            volume: Option<Decimal>,
            pair: &str,
            price: Option<Decimal>,
            price2: Option<Decimal>,
            leverage: Option<&str>,
            oflags: Option<OrderFlags>,
            timeinforce: Option<TimeInForce>,
            starttm: Option<&str>,
            expiretm: Option<&str>,
            close_ordertype: Option<&str>,
            close_price: Option<Decimal>,
            close_price2: Option<Decimal>,
            deadline: Option<&str>,
        ) -> KrakenResult<Task<ValidateOrderResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_ADD_ORDER)?
                        .signed(nonce)?
                        .request_body(AddOrderRequest {
                            userref,
                            ordertype,
                            r#type,
                            volume,
                            pair,
                            price,
                            price2,
                            leverage,
                            oflags,
                            timeinforce,
                            starttm,
                            expiretm,
                            close_ordertype,
                            close_price,
                            close_price2,
                            deadline,
                            validate: true,
                        })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .priority(RlPriorityLevel::High as u8)
                .send())
        }

        /// Place a new order.
        ///
        /// Note: See the AssetPairs endpoint for details on the available trading pairs,
        /// their price and quantity precisions, order minimums, available leverage, etc.
        #[allow(clippy::too_many_arguments)]
        pub fn add_order(
            &self,
            nonce: Nonce,
            userref: Option<u32>,
            ordertype: OrderType,
            r#type: OrderSide,
            volume: Option<Decimal>,
            pair: &str,
            price: Option<Decimal>,
            price2: Option<Decimal>,
            leverage: Option<&str>,
            oflags: Option<OrderFlags>,
            timeinforce: Option<TimeInForce>,
            starttm: Option<&str>,
            expiretm: Option<&str>,
            close_ordertype: Option<&str>,
            close_price: Option<Decimal>,
            close_price2: Option<Decimal>,
            deadline: Option<&str>,
        ) -> KrakenResult<Task<AddOrderResponse>> {
            Ok(self
                .rate_limiter
                .task(
                    self.client
                        .post(API_0_PRIVATE_ADD_ORDER)?
                        .signed(nonce)?
                        .request_body(AddOrderRequest {
                            userref,
                            ordertype,
                            r#type,
                            volume,
                            pair,
                            price,
                            price2,
                            leverage,
                            oflags,
                            timeinforce,
                            starttm,
                            expiretm,
                            close_ordertype,
                            close_price,
                            close_price2,
                            deadline,
                            validate: false,
                        })?,
                )
                .cost(RL_PRIVATE_PER_MINUTE, 1)
                .cost(RL_MATCHING_ENGINE_PER_MINUTE, 1)
                .priority(RlPriorityLevel::High as u8)
                .send())
        }
    }
}
