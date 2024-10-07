use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::spot::proto::BinanceSpotRequest;
use crate::spot::proto::BinanceSpotResponse;
use crate::spot::proto::BinanceSpotSigned;
use crate::spot::types::rate_limits::RateLimitType;
use crate::spot::types::symbols::SymbolPermission;

impl BinanceSpotRequest for GetAccountInfo {
    type Response = AccountInfo;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/account";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 20)];
}

impl BinanceSpotSigned for GetAccountInfo {}

impl BinanceSpotResponse for AccountInfo {}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountInfo {
    omit_zero_balances: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub maker_commission: Decimal,
    pub taker_commission: Decimal,
    pub buyer_commission: Decimal,
    pub seller_commission: Decimal,
    pub commission_rates: CommissionRates,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub brokered: bool,
    pub require_self_trade_prevention: bool,
    pub prevent_sor: bool,
    pub update_time: u64,
    pub account_type: AccountType,
    pub balances: Vec<Balance>,
    // FIXME choose apropriate kind of permission.
    pub permissions: Vec<SymbolPermission>,
    pub uid: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRates {
    pub maker: Decimal,
    pub taker: Decimal,
    pub buyer: Decimal,
    pub seller: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub enum AccountType {
    #[serde(rename = "SPOT")]
    Spot,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: SmartString,
    pub free: Decimal,
    pub locked: Decimal,
}

// {
//   "makerCommission": 15,
//   "takerCommission": 15,
//   "buyerCommission": 0,
//   "sellerCommission": 0,
//   "commissionRates": {
//     "maker": "0.00150000",
//     "taker": "0.00150000",
//     "buyer": "0.00000000",
//     "seller": "0.00000000"
//   },
//   "canTrade": true,
//   "canWithdraw": true,
//   "canDeposit": true,
//   "brokered": false,
//   "requireSelfTradePrevention": false,
//   "preventSor": false,
//   "updateTime": 123456789,
//   "accountType": "SPOT",
//   "balances": [
//     {
//       "asset": "BTC",
//       "free": "4723846.89208129",
//       "locked": "0.00000000"
//     },
//     {
//       "asset": "LTC",
//       "free": "4763368.68006011",
//       "locked": "0.00000000"
//     }
//   ],
//   "permissions": [
//     "SPOT"
//   ],
//   "uid": 354937868
// }

impl GetAccountInfo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_omit_zero_balances(omit_zero_balances: bool) -> Self {
        GetAccountInfo {
            omit_zero_balances: Some(omit_zero_balances),
        }
    }

    pub fn omit_zero_balances(self) -> Self {
        GetAccountInfo {
            omit_zero_balances: Some(true),
            ..self
        }
    }
}
