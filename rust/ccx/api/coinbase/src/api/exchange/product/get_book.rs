use crate::api::exchange::prelude::*;
use crate::api::exchange::ProductBook;
use crate::api::exchange::RL_PUBLIC_KEY;

pub type GetProductBookResponse = ProductBook;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Get product book.
    ///
    /// Get a list of open orders for a product.
    /// The amount of detail shown can be customized with the level parameter.
    ///
    /// ## Details
    ///
    /// By default, only the inside (i.e., the best) bid and ask are returned.
    /// This is equivalent to a book depth of 1 level.
    /// To see a larger order book, specify the level query parameter.
    ///
    /// If a level is not aggregated, all of the orders at each price are returned.
    /// Aggregated levels return only one size for each active price
    /// (as if there was only a single order for that size at the level).
    ///
    /// ## Levels
    ///
    /// 1 - The best bid, ask and auction info
    /// 2 - Full order book (aggregated) and auction info
    /// 3 - Full order book (non aggregated) and auction info
    ///
    /// **Levels 1 and 2 are aggregated**. The size field is the sum of the size
    /// of the orders at that price, and num-orders is the count of orders
    /// at that price; size should not be multiplied by num-orders.
    ///
    /// **Level 3 is non-aggregated** and returns the entire order book.
    ///
    /// ....
    ///
    /// ## Parameters
    ///
    /// * `product_id` - BTC-USD, ETH-EUR, etc.
    /// * `level_id` - The level of detail.
    ///
    /// This is not a full copy of the documentation.
    /// Please refer to the official documentation for more details.
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getproductbook]
    pub fn get_product_book(
        &self,
        product_id: &str,
        level_id: Option<u8>,
    ) -> CoinbaseResult<Task<GetProductBookResponse>> {
        if level_id == Some(3) {
            // TODO: implement level 3
            Err(CoinbaseError::other("Level 3 isn't implemented yet"))?
        }

        let endpoint = format!(
            "products/{}/book?level={}",
            product_id,
            level_id.unwrap_or(1)
        );
        Ok(self
            .rate_limiter
            .task(self.client.get(&endpoint)?.request_body(())?)
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
