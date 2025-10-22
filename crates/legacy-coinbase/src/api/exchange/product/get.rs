use crate::api::exchange::Product;
use crate::api::exchange::RL_PUBLIC_KEY;
use crate::api::exchange::prelude::*;

pub type GetProductResponse = Product;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Get single product.
    ///
    /// Get information on a single product.
    ///
    /// ## Parameters
    ///
    /// * `product_id` - (undocumented).
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getproduct]
    pub fn get_product(&self, product_id: &str) -> CoinbaseResult<Task<GetProductResponse>> {
        let endpoint = format!("products/{product_id}");
        Ok(self
            .rate_limiter
            .task(self.client.get(&endpoint)?.request_body(())?)
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
