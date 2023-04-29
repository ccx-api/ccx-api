use crate::api::exchange::prelude::*;
use crate::api::exchange::RequestedWithdrawal;
use crate::api::exchange::RL_PRIVATE_KEY;

pub type WithdrawToCoinbaseAccountResponse = RequestedWithdrawal;

#[derive(Debug, Serialize, Deserialize)]
struct WithdrawToCoinbaseAccountRequest {
    profile_id: Uuid,
    coinbase_account_id: Uuid,
    amount: Decimal,
    currency: Atom,
}

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Withdraw to Coinbase account.
    ///
    /// Withdraws funds from the specified profile_id to a www.coinbase.com wallet.
    ///
    ///
    /// ## NB Withdraw funds to a Coinbase account
    ///
    /// You can move funds between your Coinbase accounts and your Coinbase Exchange trading
    /// accounts within your daily limits. Moving funds between Coinbase and Coinbase Exchange
    /// is instant and free. See the Coinbase Accounts section for retrieving your Coinbase
    /// accounts.
    ///
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires the "transfer" permission.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_postwithdrawcoinbaseaccount]
    pub fn withdraw_to_coinbase_account(
        &self,
        profile_id: Uuid,
        coinbase_account_id: Uuid,
        amount: Decimal,
        currency: Atom,
    ) -> CoinbaseResult<Task<WithdrawToCoinbaseAccountResponse>> {
        let endpoint = "/withdrawals/coinbase-account";
        Ok(self
            .rate_limiter
            .task(self.client.post(endpoint)?.signed_now()?.request_body(
                WithdrawToCoinbaseAccountRequest {
                    profile_id,
                    coinbase_account_id,
                    amount,
                    currency,
                },
            )?)
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
