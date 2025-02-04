use crate::api::fee::TradingFee;
use crate::api::prelude::*;
use crate::api::RL_GENERAL_KEY;

pub type ListTradingFeeResponse = Vec<TradingFee>;

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// Trading fees
    ///
    /// This API call is cached for 10 seconds.
    /// This call will be executed on the account (Sub or Main),
    /// to which the used API key is bound to.
    ///
    /// [https://www.bitstamp.net/api/#trading-fees]
    pub fn list_trading_fee(&self) -> BitstampResult<Task<ListTradingFeeResponse>> {
        let endpoint = "fees/trading/";

        Ok(self
            .rate_limiter
            .task(self.client.post(endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
