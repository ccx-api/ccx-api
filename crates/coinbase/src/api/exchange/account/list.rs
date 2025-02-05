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
    /// # Get all accounts for a profile.
    ///
    /// Get a list of trading accounts from the profile of the API key.
    ///
    /// > Note: Your trading accounts are separate from your Coinbase accounts.
    /// > See [Deposit from Coinbase account](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_postdepositcoinbaseaccount/)
    /// > for documentation on how to deposit funds to begin trading.
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires either the "view" or "trade" permission.
    ///
    /// ## Rate Limits
    ///
    /// This endpoint has a custom rate limit by profile ID: 25 requests per second,
    /// up to 50 requests per second in bursts
    ///
    /// ## Funds on Hold
    ///
    /// When you place an order, the funds for the order are placed on hold.
    /// They cannot be used for other orders or withdrawn.
    /// Funds will remain on hold until the order is filled or canceled.
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getaccounts]
    pub fn list_accounts(&self) -> CoinbaseResult<Task<ListAccountResponse>> {
        let endpoint = "accounts";
        Ok(self
            .rate_limiter
            .task(self.client.get(endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
