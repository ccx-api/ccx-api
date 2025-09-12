use super::types::ProductList;
use crate::CoinbaseResult;
use crate::api::prime::prelude::*;

pub type PortfolioProductsResponse = ProductList;

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # List Portfolio Products.
    ///
    /// List tradable products for a given portfolio.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getportfolioproducts]
    pub fn get_products(
        &self,
        portfolio_id: Uuid,
        page: Page,
    ) -> CoinbaseResult<Task<PortfolioProductsResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/products");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .try_query_arg("cursor", &page.cursor())?
                    .try_query_arg("limit", &page.limit())?
                    .try_query_arg("sort_direction", &page.sort_direction())?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
