use crate::api::exchange::prelude::*;
use crate::api::exchange::Product;
use crate::api::exchange::RL_PUBLIC_KEY;

pub type GetProductResponse = Product;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get single product.
    ///
    /// Get information on a single product.
    ///
    /// * `product_id` - .
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getproduct]
    pub fn get_product(&self, product_id: Atom) -> CoinbaseResult<Task<GetProductResponse>> {
        // let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("products/{product_id}");
        Ok(self
            .rate_limiter
            .task(self.client
                    .get(&endpoint)?
                    // .signed(timestamp)?
                    .request_body(())?)
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
