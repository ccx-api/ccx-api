use crate::api::exchange::Product;
use crate::api::exchange::RL_PUBLIC_KEY;
use crate::api::exchange::prelude::*;

pub type ListProductsResponse = Vec<Product>;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get all known trading pairs.
    ///
    /// Gets a list of available currency pairs for trading.
    ///
    /// ## Parameters
    ///
    /// * `type` - ?
    ///
    /// This is not a full copy of the documentation.
    /// Please refer to the official documentation for more details.
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getproducts]
    pub fn list_products(
        &self,
        r#type: Option<Atom>,
    ) -> CoinbaseResult<Task<ListProductsResponse>> {
        let endpoint = "/products";
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(endpoint)?
                    .try_query_arg("type", &r#type)?
                    .request_body(())?,
            )
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
