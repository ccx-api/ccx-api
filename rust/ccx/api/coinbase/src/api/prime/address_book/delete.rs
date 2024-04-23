use crate::api::prime::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
pub struct DeletePortfolioAddressBookEntryResponse {
    pub activity_type: Atom,
    pub num_approvals_remaining: i32,
    pub activity_id: Uuid,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// Delete Address Book Entry.
    ///
    /// Removes an entry for a portfolio's trusted addresses.
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `address_id` - The address ID.
    ///
    /// _undocumented_
    pub fn delete_address_book_entry(
        &self,
        portfolio_id: Uuid,
        address_id: Uuid,
    ) -> CoinbaseResult<Task<DeletePortfolioAddressBookEntryResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/address_book/{address_id}");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .delete(&endpoint)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
