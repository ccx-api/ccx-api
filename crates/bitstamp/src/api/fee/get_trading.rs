use crate::api::RL_GENERAL_KEY;
use crate::api::fee::TradingFee;
use crate::api::prelude::*;

pub type TradingFeeResponse = TradingFee;

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
    pub fn trading_fee<C: AsRef<str>>(
        &self,
        currency_pair: C,
    ) -> BitstampResult<Task<TradingFeeResponse>> {
        fn endpoint(currency_pair: &str) -> String {
            format!("fees/trading/{}/", currency_pair)
        }

        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint(currency_pair.as_ref()))?
                    .signed_now()?
                    .request_body(())?,
            )
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
