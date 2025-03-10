use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::proto::Request;
use crate::proto::Response;
use crate::proto::SignedRequest;
use crate::types::timestamp::MexcTimestamp;

impl Request for GetAccountTradeList {
    type Response = Vec<AccountTradeInfo>;
    const HTTP_METHOD: http::Method = http::Method::GET;
    const ENDPOINT: &'static str = "/api/v3/myTrades";
    const COST: u32 = 10;
}

impl SignedRequest for GetAccountTradeList {}

impl Response for Vec<AccountTradeInfo> {}

/// Account Trade List (USER_DATA).
///
/// Get trades for a specific account and symbol.
///
/// Weight(IP): 10
///
/// * limit: Default 500; max 1000.
///
/// If fromId is set, it will get id >= that fromId. Otherwise most recent trades are returned.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountTradeList {
    symbol: SmartString,
    order_id: Option<u64>,
    start_time: Option<MexcTimestamp>,
    end_time: Option<MexcTimestamp>,
    from_id: Option<u64>,
    limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AccountTradeInfo {
    pub symbol: SmartString,
    pub id: String,
    pub order_id: String,
    // FIXME make None when -1.
    pub order_list_id: i64,
    pub price: Decimal,
    pub qty: Decimal,
    pub quote_qty: Decimal,
    pub commission: Decimal,
    pub commission_asset: SmartString,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
    pub is_self_trade: bool,
    pub client_order_id: Option<String>,
}

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
        start_time: Option<MexcTimestamp>,
        end_time: Option<MexcTimestamp>,
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
        Self {
            limit: Some(limit),
            ..self
        }
    }
}
