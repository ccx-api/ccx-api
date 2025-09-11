use crate::api::exchange::RL_PRIVATE_KEY;
use crate::api::exchange::account_coinbase::CoinbaseAccount;
use crate::api::exchange::prelude::*;

pub type ListAccountCoinbaseResponse = Vec<CoinbaseAccount>;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Get all Coinbase wallets.
    ///
    /// Gets all the user's available Coinbase wallets (These are the wallets/accounts
    /// that are used for buying and selling on www.coinbase.com).
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getcoinbaseaccounts]
    pub fn list_coinbase_accounts(&self) -> CoinbaseResult<Task<ListAccountCoinbaseResponse>> {
        let endpoint = "/coinbase-accounts";
        Ok(self
            .rate_limiter
            .task(self.client.get(endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
