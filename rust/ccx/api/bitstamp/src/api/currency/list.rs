use crate::api::currency::Currency;
use crate::api::prelude::*;
use crate::api::RL_GENERAL_KEY;

pub type ListCurrencyResponse = Vec<Currency>;

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// Currencies
    ///
    /// [https://www.bitstamp.net/api/#currencies]
    pub fn list_currencies(&self) -> BitstampResult<Task<ListCurrencyResponse>> {
        let endpoint = "currencies/";

        Ok(self
            .rate_limiter
            .task(self.client.get(endpoint)?.request_body(())?)
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
