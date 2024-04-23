use crate::api::exchange::account::Account;
use crate::api::exchange::prelude::*;
use crate::api::exchange::RL_PUBLIC_KEY;

pub type ListAccountResponse = Vec<Account>;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get all accounts for a profile.
    ///
    /// Get a list of trading accounts from the profile of the API key.
    ///
    /// Note: This endpoint requires either the "view" or "trade" permission.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getaccounts]
    pub fn list_accounts(&self) -> CoinbaseResult<Task<ListAccountResponse>> {
        let endpoint = "accounts";
        Ok(self
            .rate_limiter
            .task(self.client.get(endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
