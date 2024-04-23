use super::prelude::*;
use crate::api::exchange::RL_PRIVATE_KEY;

/// .
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct GetFeesResponse {
    /// Taker fee rate.
    pub taker_fee_rate: Decimal,
    /// Maker fee rate.
    pub maker_fee_rate: Decimal,
    /// The 30 days trailing volume in USD.
    pub usd_volume: Option<Decimal>,
}

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get fees.
    ///
    /// Get fees rates and 30 days trailing volume. This request returns your current maker & taker
    /// fee rates, as well as your 30-day trailing volume. Quoted rates are subject to change.
    /// For more information, see
    /// [What are the fees on Coinbase Pro?](https://help.coinbase.com/en/pro/trading-and-funding/trading-rules-and-fees/fees.html).
    ///
    /// API Key Permissions.
    ///
    /// This endpoint requires the "view" permission.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getfees]
    pub fn get_fees(&self) -> CoinbaseResult<Task<GetFeesResponse>> {
        let endpoint = "/fees";
        Ok(self
            .rate_limiter
            .task(self.client.get(endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
