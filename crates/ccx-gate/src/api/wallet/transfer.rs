use bon::Builder;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::rate_limits::RateLimitType;

/// # Transfer between trading accounts
///
/// Transfer between trading accounts
///
/// Transfer between different accounts. Currently support transfers between the following:
///
/// * spot - margin
/// * spot - futures(perpetual)
/// * spot - delivery
/// * spot - cross margin
/// * spot - options
///
/// ## Parameters
///
/// * `currency` - Transfer currency. For futures account, currency can be set to POINT or
///   settle currency.
/// * `from` - Account to transfer from
/// * `to` - Account to transfer to
/// * `amount` - Transfer amount
/// * `currency_pair` - Margin currency pair. Required if transfer from or to margin
///    account.
/// * `settle` - Futures settle currency. Required if transferring from or to futures
///    account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder)]
#[builder(on(SmartString, into), on(Decimal, into))]
pub struct Transfer {
    /// Transfer currency. For futures account, currency can be set to POINT or settle currency
    currency: SmartString,
    /// Account to transfer from
    from: AccountEnum,
    /// Account to transfer to
    to: AccountEnum,
    /// Transfer amount
    amount: Decimal,
    /// Margin currency pair. Required if transfer from or to margin account
    currency_pair: Option<SmartString>,
    /// Futures settle currency. Required if transferring from or to futures account
    settle: Option<SmartString>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountEnum {
    /// Currently unsupported.
    Cbbc,
    CrossMargin,
    Delivery,
    /// Currently unsupported.
    Finance,
    Futures,
    Margin,
    Options,
    /// Currently unsupported.
    Payment,
    /// Currently unsupported.
    Quant,
    Spot,
    /// Currently unsupported.
    Warrant,
}

impl Request for Transfer {
    const HTTP_METHOD: http::Method = http::Method::POST;
    const ENDPOINT: &'static str = "/api/v4/wallet/transfers";
    const COSTS: &'static RateLimitType = &RateLimitType::WalletTransferOrBalance;

    type Response = TransferResponse;
}

impl SignedRequest for Transfer {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferResponse {
    /// Order id (Transaction id)
    pub tx_id: i64,
}

impl Response for TransferResponse {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_from_docs() {
        let json = r#"{
            "tx_id": 59636381286
        }"#;
        let res: TransferResponse = serde_json::from_str(json).unwrap();
        assert_eq!(res, TransferResponse { tx_id: 59636381286 });
    }
}
