use super::PortfolioAddressBookEntry;
use crate::api::prime::prelude::*;

/// List all wallets associated with a given portfolio.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PortfoliAddressBookResponse {
    /// A list of transactions.
    pub addresses: Vec<PortfolioAddressBookEntry>,
    pub pagination: NextPage,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # Get Address Book.
    ///
    /// Gets a list of address book addresses.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `currency_symbol` - Cryptocurrency symbol -- if nothing is passed, all addresses are returned.
    /// * `search` - Query string that matches the address name.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getportfolioaddressbook]
    pub fn list_address_book(
        &self,
        portfolio_id: Uuid,
        currency_symbol: Option<&str>,
        search: Option<&str>,
        page: Page,
    ) -> CoinbaseResult<Task<PortfoliAddressBookResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/address_book");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .try_query_arg("currency_symbol", &currency_symbol)?
                    .try_query_arg("search", &search)?
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
