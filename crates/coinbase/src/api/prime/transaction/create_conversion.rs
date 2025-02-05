use super::Conversion;
use crate::api::prime::prelude::*;

pub type ConversionResponse = Conversion;

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
struct ConversionRequest<'a> {
    /// The amount in whole units to convert.
    amount: Decimal,
    /// The UUID of the destination wallet.
    destination: Uuid,
    /// The idempotency key associated with this conversion.
    idempotency_key: &'a str,
    /// The currency symbol to convert from.
    source_symbol: &'a str,
    /// The currency symbol to convert to.
    destination_symbol: &'a str,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # Create Conversion.
    ///
    /// Perform a conversion between 2 assets.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The ID of the portfolio.
    /// * `wallet_id` - The wallet ID that the conversion will originate from.
    /// * `idempotency_key` - The idempotency key associated with this conversion.
    /// * `amount` - The amount in whole units to convert.
    /// * `destination` - The UUID of the destination wallet.
    /// * `source_symbol` - The currency symbol to convert from.
    /// * `destination_symbol` - The currency symbol to convert to.
    ///
    /// [See Coinbase documentation for more information](https://docs.cdp.coinbase.com/prime/reference/primerestapi_createconversion)
    pub fn create_conversion(
        &self,
        portfolio_id: Uuid,
        idempotency_key: &str,
        wallet_id: Uuid,
        source_symbol: &str,
        amount: Decimal,
        destination: Uuid,
        destination_symbol: &str,
    ) -> CoinbaseResult<Task<ConversionResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/wallets/{wallet_id}/conversion");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint)?
                    .signed(timestamp)?
                    .request_body(ConversionRequest {
                        idempotency_key,
                        amount,
                        destination,
                        source_symbol,
                        destination_symbol,
                    })?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
