use crate::api::exchange::currency::CurrencyInfo;
use crate::api::exchange::prelude::*;
use crate::api::exchange::RL_PUBLIC_KEY;

pub type ListCurrencyResponse = Vec<CurrencyInfo>;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get all known currencies.
    ///
    /// Gets a list of all known currencies.
    ///
    ///     Note: Not all currencies may be currently in use for trading.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getcurrencies]
    pub fn list_currencies(&self) -> CoinbaseResult<Task<ListCurrencyResponse>> {
        let endpoint = format!("currencies");
        Ok(self
            .rate_limiter
            .task(self.client.get(&endpoint)?.request_body(())?)
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
