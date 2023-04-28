use crate::api::exchange::prelude::*;
use crate::api::exchange::profile::Profile;
use crate::api::exchange::RL_PUBLIC_KEY;

pub type GetProfileResponse = Profile;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get a currency.
    ///
    /// Gets a single currency by id.
    ///
    ///
    /// Currency Codes
    ///
    /// Currency codes conform to the ISO 4217 standard where possible. Currencies that have
    /// no representation in ISO 4217 can use a custom code.
    ///
    ///
    /// * `currency_id` - .
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getcurrency]
    pub fn get_profile(&self, profile_id: Uuid) -> CoinbaseResult<Task<GetProfileResponse>> {
        fn endpoint(profile_id: Uuid) -> String {
            format!("profiles/{profile_id}")
        }

        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint(profile_id))?
                    .signed_now()?
                    .request_body(())?,
            )
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
