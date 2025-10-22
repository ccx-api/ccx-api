use crate::api::prime::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct CreatePortfolioAddressBookEntryResponse {
    pub activity_type: Atom,
    pub num_approvals_remaining: i32,
    pub activity_id: Uuid,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
struct CreatePortfolioAddressBookEntryRequest<'a> {
    /// Crypto address to add
    address: &'a str,
    /// Currency symbol of address to add
    currency_symbol: &'a str,
    /// Name of address book entry
    name: &'a str,
    /// Account Identifier (memo/destination tag)
    account_identifier: &'a str,
    /// Portfolio ID
    portfolio_id: &'a str,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # Create Address Book Entry.
    ///
    /// Creates an entry for a portfolio's trusted addresses.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `address` - Crypto address to add.
    /// * `currency_symbol` - The currency symbol of the address.
    /// * `name` - Name of address book entry.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_createportfolioaddressbookentry]
    pub fn create_address_book_entry(
        &self,
        portfolio_id: Uuid,
        address: &str,
        currency_symbol: &str,
        name: &str,
    ) -> CoinbaseResult<Task<CreatePortfolioAddressBookEntryResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/address_book");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint)?
                    .signed(timestamp)?
                    .request_body(CreatePortfolioAddressBookEntryRequest {
                        address,
                        currency_symbol,
                        name,
                        account_identifier: "",
                        portfolio_id: &portfolio_id.to_string(),
                    })?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
