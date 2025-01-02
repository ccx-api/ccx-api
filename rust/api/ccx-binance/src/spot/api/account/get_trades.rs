use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::spot::proto::BinanceSpotRequest;
use crate::spot::proto::BinanceSpotResponse;
use crate::spot::proto::BinanceSpotSigned;
use crate::spot::types::rate_limits::RateLimitType;

impl BinanceSpotRequest for GetAccountTradeList {
    type Response = Vec<TradeInfo>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/myTrades";
    const COSTS: &'static [(RateLimitType, u32)] = &[(RateLimitType::RequestWeight, 20)];
}

impl BinanceSpotSigned for GetAccountTradeList {}

impl BinanceSpotResponse for Vec<TradeInfo> {}

/// [Account trade list (USER_DATA)](https://developers.binance.com/docs/binance-spot-api-docs/rest-api/account-endpoints#account-trade-list-user_data).
///
/// Get trades for a specific account and symbol.
///
/// Weight: 20
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountTradeList {
    symbol: SmartString,
    order_id: Option<u64>,
    start_time: Option<Decimal>,
    end_time: Option<Decimal>,
    from_id: Option<u64>,
    limit: Option<u32>,
}

// symbol	STRING	YES
// orderId	LONG	NO	This can only be used in combination with symbol.
// startTime	LONG	NO
// endTime	LONG	NO
// fromId	LONG	NO	TradeId to fetch from. Default gets most recent trades.
// limit	INT	NO	Default 500; max 1000.
// recvWindow	LONG	NO	The value cannot be greater than 60000
// timestamp	LONG	YES

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TradeInfo {
    pub symbol: SmartString,
    pub id: u64,
    pub order_id: u64,
    pub order_list_id: i64,
    pub price: Decimal,
    pub qty: Decimal,
    pub order_qty: Option<Decimal>,
    pub commission: Decimal,
    pub commission_asset: SmartString,
    pub time: Decimal,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

// [
//   {
//     "symbol": "BNBBTC",
//     "id": 28457,
//     "orderId": 100234,
//     "orderListId": -1,
//     "price": "4.00000100",
//     "qty": "12.00000000",
//     "quoteQty": "48.000012",
//     "commission": "10.10000000",
//     "commissionAsset": "BNB",
//     "time": 1499865549590,
//     "isBuyer": true,
//     "isMaker": false,
//     "isBestMatch": true
//   }
// ]

impl GetAccountTradeList {
    pub fn new(symbol: SmartString) -> Self {
        Self {
            symbol,
            order_id: None,
            start_time: None,
            end_time: None,
            from_id: None,
            limit: None,
        }
    }

    pub fn new_with_time(
        symbol: SmartString,
        start_time: Option<Decimal>,
        end_time: Option<Decimal>,
    ) -> Self {
        Self {
            symbol,
            order_id: None,
            start_time,
            end_time,
            from_id: None,
            limit: None,
        }
    }

    /// * from_id — TradeId to fetch from. Default gets most recent trades.
    pub fn new_with_id(symbol: SmartString, from_id: Option<u64>, order_id: Option<u64>) -> Self {
        Self {
            symbol,
            order_id,
            start_time: None,
            end_time: None,
            from_id,
            limit: None,
        }
    }

    /// * limit — Default 500; max 1000.
    pub fn limit(self, limit: u32) -> Self {
        GetAccountTradeList {
            limit: Some(limit),
            ..self
        }
    }
}
