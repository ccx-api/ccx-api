use crate::api::RL_GENERAL_KEY;
use crate::api::fee::WithdrawalFee;
use crate::api::prelude::*;

pub type ListWithdrawalFeeResponse = Vec<WithdrawalFee>;

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// Withdrawal fees
    ///
    /// This call will be executed on the account (Sub or Main),
    /// to which the used API key is bound to.
    ///
    /// [https://www.bitstamp.net/api/#withdrawal-fees]
    pub fn list_withdrawal_fee(&self) -> BitstampResult<Task<ListWithdrawalFeeResponse>> {
        let endpoint = "fees/withdrawal/";

        Ok(self
            .rate_limiter
            .task(self.client.post(endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
