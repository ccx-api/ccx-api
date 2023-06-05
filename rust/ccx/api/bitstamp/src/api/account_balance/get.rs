use crate::api::account_balance::AccountBalance;
use crate::api::prelude::*;
use crate::api::RL_GENERAL_KEY;

pub type ListAccountBalanceResponse = AccountBalance;

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// Account balances
    ///
    /// This call will be executed on the account (Sub or Main),
    /// to which the used API key is bound to.
    ///
    /// [https://www.bitstamp.net/api/#account-balances]
    pub fn get_account_balance<C: AsRef<str>>(
        &self,
        currency: C,
    ) -> BitstampResult<Task<ListAccountBalanceResponse>> {
        fn endpoint(currency: &str) -> String {
            format!("account_balances/{}/", currency)
        }

        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint(currency.as_ref()))?
                    .signed_now()?
                    .request_body(())?,
            )
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
