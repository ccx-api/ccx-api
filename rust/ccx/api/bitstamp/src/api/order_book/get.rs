use crate::api::order_book::OrderBook;
use crate::api::prelude::*;
use crate::api::RL_PUBLIC_KEY;

pub type GetOrderBookResponse = OrderBook;

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// Get the order book for a given pair.
    ///
    /// The group parameter is used for accessing different data from order book.
    /// Possible values are 0 (orders are not grouped at same price),
    /// 1 (orders are grouped at same price - default) or
    /// 2 (orders with their order ids are not grouped at same price).
    ///
    /// * `pair` - btcusd, btceur, etc.
    /// * `group` - Group orders with the same price (0 - false; 1 - true). Default: 1
    ///
    /// [https://www.bitstamp.net/api/#order-book]
    pub fn get_order_book<P: AsRef<str>>(
        &self,
        pair: P,
        group: Option<bool>,
    ) -> BitstampResult<Task<GetOrderBookResponse>> {
        fn endpoint(pair: &str) -> String {
            format!("order_book/{pair}")
        }

        let group = group.map(|v| v as u8);

        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint(pair.as_ref()))?
                    .try_query_arg("group", &group)?
                    .request_body(())?,
            )
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
