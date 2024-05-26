use crate::api::exchange::prelude::*;
use crate::api::exchange::profile::Profile;
use crate::api::exchange::RL_PUBLIC_KEY;

pub type ListProfileResponse = Vec<Profile>;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Get profiles.
    ///
    /// Gets a list of all of the current user's profiles.
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires the "view" permission and is accessible by any profile's API key.
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getprofiles]
    pub fn list_profiles(&self) -> CoinbaseResult<Task<ListProfileResponse>> {
        let endpoint = "profiles";
        Ok(self
            .rate_limiter
            .task(self.client.get(endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
