use serde::Serialize;

use super::order_book::OrderBookRequest;

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WsRequestEvent {
    Subscribe,
    Unsubscribe,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "channel")]
pub enum WsRequest {
    #[serde(rename = "level2")]
    OrderBook {
        #[serde(rename = "type")]
        event: WsRequestEvent,
        #[serde(flatten)]
        payload: OrderBookRequest,
    },
}

impl WsRequest {
    /// Periodically notify top bids and asks snapshot with limited levels.
    ///
    /// <https://www.gate.io/docs/developers/apiv4/ws/en/#limited-level-full-order-book-snapshot>
    pub fn order_book(event: WsRequestEvent, payload: OrderBookRequest) -> Self {
        WsRequest::OrderBook { event, payload }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use similar_asserts::assert_eq;

    use super::*;

    #[test]
    fn serialize_order_book() {
        let actual = WsRequest::order_book(
            WsRequestEvent::Subscribe,
            OrderBookRequest::builder()
                .account_id("acc")
                .product_id("TBTC-TUSD*")
                .build(),
        );

        let expected = json!({
          "type": "subscribe",
          "channel": "level2",
          "accountId": "acc",
          "productId": "TBTC-TUSD*"
        });

        assert_eq!(serde_json::to_value(actual).unwrap(), expected,);
    }
}
