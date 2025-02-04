use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LiveOrderEvent {
    /// Order ID.
    pub id: u64,

    /// Order amount.
    pub amount: Decimal,
    // pub amount_str: Decimal,
    /// Order price.
    pub price: Decimal,
    // pub price_str: Decimal,
    /// Order type (0 - buy; 1 - sell).
    pub order_type: u8,

    /// Order datetime.
    pub datetime: String,

    /// Order action timestamp represented microseconds.
    pub microtimestamp: String,

    /// Order amount that already had been filled.
    pub amount_traded: Decimal,

    /// Original order amount at the moment it was created.
    pub amount_at_create: Decimal,

    /// Type of this [`LiveOrderEvent`].
    #[serde(default)]
    pub event_type: LiveOrderEventType,
}

/// Event types related to orders.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LiveOrderEventType {
    #[serde(rename = "order_created")]
    OrderCreated,
    #[serde(rename = "order_changed")]
    OrderChanged,
    #[serde(rename = "order_deleted")]
    OrderDeleted,
    /// Fallback event type used in cases of new/unknown events.
    #[serde(other)]
    Unknown,
}

impl Default for LiveOrderEventType {
    fn default() -> Self {
        Self::Unknown
    }
}

#[cfg(test)]
mod tests {

    use crate::ws_stream::Event;

    #[test]
    fn test_order_created() {
        let json = r#"
            {
                "data":{
                    "id":1651478886379522,
                    "id_str":"1651478886379522",
                    "order_type":0,
                    "datetime":"1692028062",
                    "microtimestamp":"1692028062152000",
                    "amount":0.34,
                    "amount_str":"0.34000000",
                    "amount_traded":"0",
                    "amount_at_create":"0.34000000",
                    "price":29422,
                    "price_str":"29422"
                },
                "channel":"live_orders_btcusd",
                "event":"order_created"
            }"#;

        let res = serde_json::from_str::<Event>(json);
        assert!(
            res.is_ok(),
            "Failed to deserialize order_created: {:?}",
            res
        );
    }

    #[test]
    fn test_order_changed() {
        let json = r#"{
            "data":{
                "id":1651483095416833,
                "id_str":"1651483095416833",
                "order_type":0,
                "datetime":"1692029091",
                "microtimestamp":"1692029090635000",
                "amount":0.04944669,
                "amount_str":"0.04944669",
                "amount_traded":"0.00055331",
                "amount_at_create":"0.05000000",
                "price":29591,
                "price_str":"29591"
            },
            "channel":"live_orders_btcusd",
            "event":"order_changed"
        }"#;

        let res = serde_json::from_str::<Event>(json);
        assert!(
            res.is_ok(),
            "Failed to deserialize order_changed: {:?}",
            res
        );
    }

    #[test]
    fn test_order_deleted() {
        let json = r#"{
            "data":{
                "id":1651483074093058,
                "id_str":"1651483074093058",
                "order_type":1,
                "datetime":"1692029085",
                "microtimestamp":"1692029085457000",
                "amount":0.5,
                "amount_str":"0.50000000",
                "amount_traded":"0",
                "amount_at_create":"0.50000000",
                "price":29599,
                "price_str":"29599"
            },
            "channel":"live_orders_btcusd",
            "event":"order_deleted"
        }"#;

        let res = serde_json::from_str::<Event>(json);
        assert!(
            res.is_ok(),
            "Failed to deserialize order_deleted: {:?}",
            res
        );
    }
}
