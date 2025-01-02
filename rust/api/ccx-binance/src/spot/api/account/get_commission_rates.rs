use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::spot::proto::BinanceSpotRequest;
use crate::spot::proto::BinanceSpotResponse;
use crate::spot::proto::BinanceSpotSigned;
use crate::spot::types::commission::Commission;
use crate::spot::types::rate_limits::RateLimitType;

impl BinanceSpotRequest for GetCommissionRates {
    type Response = CommissionRates;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/account/commission";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 20)];
}

impl BinanceSpotSigned for GetCommissionRates {}

impl BinanceSpotResponse for CommissionRates {}

/// [Query Unfilled Order Count (USER_DATA)](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#query-unfilled-order-count-user_data).
///
/// Displays the user's unfilled order count for all intervals.
///
/// Weight: 40
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetCommissionRates {
    symbol: SmartString,
}

// {
//   "symbol": "BTCUSDT",
//   "standardCommission": {         //Commission rates on trades from the order.
//     "maker": "0.00000010",
//     "taker": "0.00000020",
//     "buyer": "0.00000030",
//     "seller": "0.00000040"
//   },
//   "taxCommission": {              //Tax commission rates for trades from the order.
//     "maker": "0.00000112",
//     "taker": "0.00000114",
//     "buyer": "0.00000118",
//     "seller": "0.00000116"
//   },
//   "discount": {                   //Discount commission when paying in BNB
//     "enabledForAccount": true,
//     "enabledForSymbol": true,
//     "discountAsset": "BNB",
//     "discount": "0.75000000"      //Standard commission is reduced by this rate when paying commission in BNB.
//   }
// }

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CommissionRates {
    pub symbol: SmartString,
    /// Commission rates on trades from the order.
    pub standard_commission: Commission,
    /// Tax commission rates for trades from the order.
    pub tax_commission: Commission,
    /// Discount commission when paying in BNB.
    pub discount: Discount,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Discount {
    pub enabled_for_account: bool,
    pub enabled_for_symbol: bool,
    pub discount_asset: SmartString,
    /// Standard commission is reduced by this rate when paying commission in BNB.
    pub discount: Decimal,
}

impl GetCommissionRates {
    pub fn new(symbol: SmartString) -> Self {
        Self { symbol }
    }
}
