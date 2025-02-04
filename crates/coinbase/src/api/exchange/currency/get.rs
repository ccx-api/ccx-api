use crate::api::exchange::currency::CurrencyInfo;
use crate::api::exchange::prelude::*;
use crate::api::exchange::RL_PUBLIC_KEY;

pub type GetCurrencyResponse = CurrencyInfo;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Get a currency.
    ///
    /// Gets a single currency by id.
    ///
    /// ## Currency Codes
    ///
    /// Currency codes conform to the ISO 4217 standard where possible. Currencies
    /// that have no representation in ISO 4217 can use a custom code.
    ///
    /// ## Parameters
    ///
    /// * `currency_id` - The currency code.
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getcurrency]
    pub fn get_currency(&self, currency_id: Atom) -> CoinbaseResult<Task<GetCurrencyResponse>> {
        let endpoint = format!("currencies/{currency_id}");
        Ok(self
            .rate_limiter
            .task(self.client.get(&endpoint)?.request_body(())?)
            .cost(RL_PUBLIC_KEY, 1)
            .send())
    }
}
