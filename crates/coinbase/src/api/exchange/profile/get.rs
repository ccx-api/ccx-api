use crate::api::exchange::RL_PUBLIC_KEY;
use crate::api::exchange::prelude::*;
use crate::api::exchange::profile::Profile;

pub type GetProfileResponse = Profile;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Get profile by id.
    ///
    /// Information for a single profile. Use this endpoint when you know the profile_id.
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires the "transfer" permission.
    ///
    /// ## Parameters
    ///
    /// * `profile_id` - (undocumented).
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getprofile]
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
