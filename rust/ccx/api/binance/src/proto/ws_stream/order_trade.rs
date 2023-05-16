//
//#[derive(Debug, Serialize, Deserialize, Clone)]
//#[serde(rename_all = "camelCase")]
//pub struct OrderTradeEvent {
//    #[serde(rename = "e")] pub event_type: String,
//
//    #[serde(rename = "E")] pub event_time: u64,
//
//    #[serde(rename = "s")] pub symbol: String,
//
//    #[serde(rename = "c")] pub new_client_order_id: String,
//
//    #[serde(rename = "S")] pub side: String,
//
//    #[serde(rename = "o")] pub order_type: String,
//
//    #[serde(rename = "f")] pub time_in_force: String,
//
//    #[serde(rename = "q")] pub qty: String,
//
//    #[serde(rename = "p")] pub price: String,
//
//    #[serde(skip_serializing, rename = "P")] pub p_ignore: String,
//
//    #[serde(skip_serializing, rename = "F")] pub f_ignore: String,
//
//    #[serde(skip_serializing)] pub g: i32,
//
//    #[serde(skip_serializing, rename = "C")] pub c_ignore: Option<String>,
//
//    #[serde(rename = "x")] pub execution_type: String,
//
//    #[serde(rename = "X")] pub order_status: String,
//
//    #[serde(rename = "r")] pub order_reject_reason: String,
//
//    #[serde(rename = "i")] pub order_id: u64,
//
//    #[serde(rename = "l")] pub qty_last_filled_trade: String,
//
//    #[serde(rename = "z")] pub accumulated_qty_filled_trades: String,
//
//    #[serde(rename = "L")] pub price_last_filled_trade: String,
//
//    #[serde(rename = "n")] pub commission: String,
//
//    #[serde(skip_serializing, rename = "N")] pub asset_commisioned: Option<String>,
//
//    #[serde(rename = "T")] pub trade_order_time: u64,
//
//    #[serde(rename = "t")] pub trade_id: i64,
//
//    #[serde(skip_serializing, rename = "I")] pub i_ignore: u64,
//
//    #[serde(skip_serializing)] pub w: bool,
//
//    #[serde(rename = "m")] pub is_buyer_maker: bool,
//
//    #[serde(skip_serializing, rename = "M")] pub m_ignore: bool,
//}
//
