use std::fmt;

use crate::error::ApiFineryError;
use crate::types::CancelReason;
use crate::types::ClientId;
use crate::types::ClientOrderId;
use crate::types::DealId;
use crate::types::OrderId;
use crate::types::Price;
use crate::types::Size;

mod add;
mod add_incoming_settlement_request;
mod add_outgoing_settlement_transaction;
mod book;
mod climits;
mod commit_incoming_settlement_transaction;
mod deal_history;
mod del;
mod del_all;
mod del_climit;
mod del_incoming_settlement_cp_request;
mod del_incoming_settlement_request;
mod del_limit;
mod del_outgoing_settlement_transaction;
mod get_subaccounts;
mod get_user_name;
mod instruments;
mod limits;
mod positions;
mod send_outgoing_settlement_transaction;
mod set_climit;
mod set_limit;
mod settlement_history;
mod settlement_requests;
mod settlement_transaction_history;
mod settlement_transactions;

pub use add::*;
pub use add_incoming_settlement_request::*;
pub use add_outgoing_settlement_transaction::*;
pub use book::*;
pub use climits::*;
pub use commit_incoming_settlement_transaction::*;
pub use deal_history::*;
pub use del::*;
pub use del_all::*;
pub use del_climit::*;
pub use del_incoming_settlement_cp_request::*;
pub use del_incoming_settlement_request::*;
pub use del_limit::*;
pub use del_outgoing_settlement_transaction::*;
pub use get_subaccounts::*;
pub use get_user_name::*;
pub use instruments::*;
pub use limits::*;
pub use positions::*;
pub use send_outgoing_settlement_transaction::*;
pub use set_climit::*;
pub use set_limit::*;
pub use settlement_history::*;
pub use settlement_requests::*;
pub use settlement_transaction_history::*;
pub use settlement_transactions::*;

pub const API_MOD: &str = "api/mod";

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ModRequest {
    /// Efx::OrderId
    /// Id of order to replace
    #[serde(rename = "orderId")]
    pub order_id: OrderId,
    /// Efx::ClientOrderId
    /// Optional user data attached to a new order
    #[serde(rename = "clientOrderId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<ClientOrderId>,
    /// Efx::Price
    /// New Price
    pub price: Price,
    /// Efx::Size
    /// Initial size of replace result will be size - (old order initial size - old order remaining size)
    pub size: Size,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ModResponse {
    /// Efx::OrderId
    /// New Order Id
    pub id: OrderId,
    /// Efx::Size
    /// New Order initial size
    #[serde(rename = "initialSize")]
    pub initial_size: Size,
    /// Efx::Size
    /// New Order remaining size (after aggressive deals)
    #[serde(rename = "remainingSize")]
    pub remaining_size: Size,
    /// unsigned int16
    /// Cancel reason
    ///     0 - in place or filled
    ///     1 - by client
    ///     2 - as non-book order
    ///     3 - by self-trade prevention
    ///     4 - cancel-on-disconnect
    #[serde(rename = "cancelReason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<CancelReason>,
    /// Efx::OrderId
    /// Original Order Id
    #[serde(rename = "origId")]
    pub orig_id: OrderId,
    /// Efx::Size
    /// Original Order size on removal
    #[serde(rename = "origRemainingSize")]
    pub orig_remaining_size: Size,
    /// Array of Deals
    /// Initial (taker) deals
    pub deals: Vec<Deal>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Deal {
    /// Efx::DealId
    /// Deal Id
    pub id: DealId,
    /// Efx::Price
    /// Deal price
    pub price: Price,
    /// Efx::Size
    /// DealSize
    pub size: Size,
    /// Efx::Size
    /// Deal volume
    pub volume: Size,
    /// Efx::Size
    /// Deal delta in quote (balance) currency
    pub delta: Size,
    /// Efx::ClientId
    /// Counterparty id
    #[serde(rename = "counterpartyId")]
    pub counterparty_id: ClientId,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok(T),
    Err(ApiError),
}

impl<T> ApiResponse<T> {
    pub fn ok(value: T) -> Self {
        Self::Ok(value)
    }

    pub fn err(error: ApiError) -> Self {
        Self::Err(error)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ApiError {
    pub error: ApiFineryError,
}

impl ApiError {
    fn new(error: ApiFineryError) -> Self {
        Self { error }
    }
}

impl<'de, T> serde::Deserialize<'de> for ApiResponse<T>
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
    T: fmt::Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<ApiResponse<T>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;

        let value: Value = serde_json::Value::deserialize(deserializer)?;
        // log::debug!("value :: {:?}", value);
        let error = match value.get("error") {
            Some(code) => {
                // log::debug!("code :: {:?}", code);
                ApiFineryError::deserialize(code).map_err(serde::de::Error::custom)?
            }
            None => ApiFineryError::Ok,
        };
        // log::debug!("error :: {:?}", error);
        match error {
            ApiFineryError::Ok => Ok(ApiResponse::ok(
                T::deserialize(value).map_err(serde::de::Error::custom)?,
            )),
            finery_error => Ok(ApiResponse::err(ApiError::new(finery_error))),
        }
    }
}

#[allow(dead_code)]
pub(in crate::types) fn test_serde_value_type<T>(json: &str)
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
    T: fmt::Debug,
    T: std::cmp::PartialEq,
{
    log::debug!("test_serde_value_type json old :: {}", json);
    let value1: T = serde_json::from_str(json).expect("Failed from_str");
    log::debug!("test_serde_value_type value1 :: {:?}", value1);
    let json = serde_json::to_string(&value1).expect("Failed to_string");
    log::debug!("test_serde_value_type json new :: {}", json);
    let value2: T = serde_json::from_str(&json).expect("Failed from_str");
    log::debug!("test_serde_value_type value2 :: {:?}", value2);
    assert_eq!(value1, value2);
}

#[allow(dead_code)]
pub(in crate::types) fn test_serde_response<T>(json: &str)
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
    T: fmt::Debug,
    T: std::cmp::PartialEq,
{
    log::debug!("test_serde_response json old :: {}", json);
    let value1: ApiResponse<T> = serde_json::from_str(json).expect("Failed from_str");
    log::debug!("test_serde_response value1 :: {:?}", value1);
    let json = serde_json::to_string(&value1).expect("Failed to_string");
    log::debug!("test_serde_response json new :: {}", json);
    let value2: ApiResponse<T> = serde_json::from_str(&json).expect("Failed from_str");
    log::debug!("test_serde_response value2 :: {:?}", value2);
    assert_eq!(value1, value2);
}

#[allow(dead_code)]
pub(in crate::types::rest) fn test_serde_response_err<T>()
where
    T: serde::Serialize,
    T: serde::de::DeserializeOwned,
    T: std::fmt::Debug,
    T: std::cmp::PartialEq,
{
    let json = r#"
    {
        "error": 2
    }
    "#;
    log::debug!("test_serde_response_err json old :: {}", json);
    let value1: ApiResponse<T> = serde_json::from_str(json).expect("Failed from_str");
    log::debug!("test_serde_response_err value1 :: {:?}", value1);
    let json = serde_json::to_string(&value1).expect("Failed to_string");
    log::debug!("test_serde_response_err json new :: {}", json);
    let value2: ApiResponse<T> = serde_json::from_str(&json).expect("Failed from_str");
    log::debug!("test_serde_response_err value2 :: {:?}", value2);
    assert_eq!(value1, value2);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::types::rest;

    #[test]
    pub(crate) fn test_serde_mod() {
        let json = r#"
        {
            "orderId": 1235,
            "price": 1000000000000,
            "size": 20000000
        }
        "#;
        test_serde_value_type::<ModRequest>(json);

        let json = r#"
        {
            "id": 1246,
            "initialSize": 18000000,
            "remainingSize": 18000000,
            "deals": [ ],
            "origId": 1245,
            "origRemainingSize": 8000000
        }
        "#;
        test_serde_response::<ModResponse>(json);
        test_serde_response_err::<ModResponse>();
    }

    #[test]
    pub(crate) fn test_all_serde() {
        rest::tests::test_serde_mod();
        rest::add::tests::test_serde_add();
        rest::add_incoming_settlement_request::tests::test_serde_add_incoming_settlement_request();
        rest::add_outgoing_settlement_transaction::tests::test_serde_add_outgoing_settlement_transaction();
        rest::book::tests::test_serde_book();
        rest::climits::tests::test_serde_climits();
        rest::commit_incoming_settlement_transaction::tests::test_serde_commit_incoming_settlement_transaction();
        rest::deal_history::tests::test_serde_deal_history();
        rest::del::tests::test_serde_del();
        rest::del_all::tests::test_serde_del_all();
        rest::del_climit::tests::test_serde_del_climit();
        rest::del_incoming_settlement_cp_request::tests::test_serde_del_incoming_settlement_cp_request();
        rest::del_incoming_settlement_request::tests::test_serde_del_incoming_settlement_request();
        rest::del_limit::tests::test_serde_del_limit();
        rest::del_outgoing_settlement_transaction::tests::test_serde_del_outgoing_settlement_transaction();
        rest::instruments::tests::test_serde_instruments();
        rest::limits::tests::test_serde_limits();
        rest::positions::tests::test_serde_positions();
        rest::send_outgoing_settlement_transaction::tests::test_serde_send_outgoing_settlement_transaction();
        rest::set_climit::tests::test_serde_set_climit();
        rest::set_limit::tests::test_serde_set_limit();
        rest::settlement_history::tests::test_serde_settlement_history();
        rest::settlement_requests::tests::test_serde_settlement_requests();
        rest::settlement_transaction_history::tests::test_serde_settlement_transaction_history();
        rest::settlement_transactions::tests::test_serde_settlement_transactions();
    }
}
