use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveTradeEvent {
    /// Trade unique ID.
    pub id: u64,

    /// Trade amount.
    pub amount: Decimal,
    // amount_str: Decimal,
    /// Trade price.
    pub price: Decimal,
    // price_str: Decimal,
    /// Trade type (0 - buy; 1 - sell)
    pub r#type: u8,

    /// Trade mictotimestamp.
    pub microtimestamp: String,

    /// Trade buy order ID
    pub buy_order_id: u64,

    /// Trade sell order ID
    pub sell_order_id: u64,
}

#[cfg(test)]
mod tests {
    use crate::ws_stream::Event;

    #[test]
    fn test_deserialize() {
        let json = r#"
        {
            "data":{
                "id":296045814,
                "timestamp":"1692025525",
                "amount":0.01611591,
                "amount_str":"0.01611591",
                "price":29452,
                "price_str":"29452",
                "type":1,
                "microtimestamp":"1692025525441000",
                "buy_order_id":1651468495040514,
                "sell_order_id":1651468496011265
            },
            "channel":"live_trades_btcusd",
            "event":"trade"
        }"#;

        let res = serde_json::from_str::<Event>(json);
        assert!(res.is_ok(), "Failed to deserialize: {:?}", res);
    }
}
