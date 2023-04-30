use crate::api::exchange::account::Account;
use crate::api::exchange::prelude::*;
use crate::api::exchange::RL_PUBLIC_KEY;

pub type GetAccountResponse = Account;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get a single account by id
    ///
    /// Information for a single account.
    /// Use this endpoint when you know the account_id.
    /// API key must belong to the same profile as the account.
    ///
    ///    Note: This endpoint requires either the "view" or "trade" permission.
    ///
    /// * `account_id` - .
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getaccount]
    pub fn get_account(&self, account_id: Uuid) -> CoinbaseResult<Task<GetAccountResponse>> {
        fn endpoint(account_id: Uuid) -> String {
            format!("accounts/{account_id}")
        }

        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint(account_id))?
                    .signed_now()?
                    .request_body(())?,
            )
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
