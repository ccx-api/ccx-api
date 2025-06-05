use macro_rules_attribute::apply;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::prelude::Coin;
use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::{Request, Response};
use crate::types::rate_limits::RateLimitType;

/// Request to get account balance information
#[apply(Request)]
pub struct AccountBalance {
    /// The id of the trading account
    #[serde(skip)]
    account_id: String,
}

/// Response for AccountBalance request
#[apply(Response)]
pub struct AccountBalanceResponse {
    /// List of balances for different currencies
    #[serde(rename = "data")]
    pub balances: Vec<CurrencyBalance>,
}

/// Balance information for a specific currency
#[apply(Response)]
pub struct CurrencyBalance {
    /// Currency ID
    pub currency_id: Uuid,
    /// Currency symbol
    pub currency: Coin,
    /// The total balance in the account
    pub balance: Decimal,
    /// The total balance reserved for some purpose, e.g. a pending withdrawal
    pub held_balance: Decimal,
    /// The total balance available for trading
    pub tradable_balance: Decimal,
    /// The total balance available for withdrawal
    pub withdrawable_balance: Decimal,
}

impl Response for AccountBalanceResponse {}

impl Request for AccountBalance {
    type Response = AccountBalanceResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let account_id = &self.account_id;
        format!("/api/prime/trading/v1/accounts/{account_id}/balances").into()
    }
}

impl SignedRequest for AccountBalance {}
