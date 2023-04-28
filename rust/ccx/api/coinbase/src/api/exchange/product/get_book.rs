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
    /// Get product book.
    ///
    /// Get a list of open orders for a product.
    /// The amount of detail shown can be customized with the level parameter.
    ///
    ///
    /// By default, only the inside (i.e., the best) bid and ask are returned.
    /// This is equivalent to a book depth of 1 level.
    /// To see a larger order book, specify the level query parameter.
    ///
    /// If a level is not aggregated, all of the orders at each price are returned.
    /// Aggregated levels return only one size for each active price
    /// (as if there was only a single order for that size at the level).
    ///
    /// Levels:
    /// 1 - The best bid, ask and auction info
    /// 2 - Full order book (aggregated) and auction info
    /// 3 - Full order book (non aggregated) and auction info
    ///
    /// * `product_id` - BTC-USD, ETH-EUR, etc.
    /// * `level_id` - The level of detail.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getproductbook]
    pub fn get_product_book(
        &self,
        product_id: Atom,
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
