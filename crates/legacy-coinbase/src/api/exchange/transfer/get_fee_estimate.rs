use crate::api::exchange::FeeEstimate;
use crate::api::exchange::RL_PRIVATE_KEY;
use crate::api::exchange::prelude::*;

pub type FeeEstimateResponse = FeeEstimate;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Get fee estimate for crypto withdrawal.
    ///
    /// Gets the fee estimate for the crypto withdrawal to crypto address.
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires the "transfer" permission.
    /// API key must belong to default profile.
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getwithdrawfeeestimate]
    pub fn get_fee_estimate<C: AsRef<str>, N: AsRef<str>, A: AsRef<str>>(
        &self,
        currency: C,
        network: N,
        crypto_address: A,
    ) -> CoinbaseResult<Task<FeeEstimateResponse>> {
        let endpoint = "/withdrawals/fee-estimate";
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(endpoint)?
                    .query_arg("currency", currency.as_ref())?
                    .query_arg("network", network.as_ref())?
                    .query_arg("crypto_address", crypto_address.as_ref())?
                    .signed_now()?
                    .request_body(())?,
            )
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
