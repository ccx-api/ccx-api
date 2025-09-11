use crate::api::RL_GENERAL_KEY;
use crate::api::order::MarketOrder;
use crate::api::prelude::*;

pub type BuyMarketOrderResponse = MarketOrder;

#[derive(Debug, Serialize)]
struct BuyMarketOrderRequest<'a> {
    amount: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_order_id: Option<&'a str>,
}

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// Buy market order
    ///
    /// By placing a market order you acknowledge that the execution of
    /// your order depends on the market conditions and that these conditions
    /// may be subject to sudden changes that cannot be foreseen.
    ///
    /// This call will be executed on the account (Sub or Main),
    /// to which the used API key is bound to.
    ///
    /// [https://www.bitstamp.net/api/#buy-market-order]
    pub fn buy_market_order<C: AsRef<str>, O: AsRef<str>>(
        &self,
        currency_pair: C,
        amount: Decimal,
        client_order_id: Option<O>,
    ) -> BitstampResult<Task<BuyMarketOrderResponse>> {
        fn endpoint(currency_pair: &str) -> String {
            format!("buy/market/{}/", currency_pair)
        }
        let client_order_id = client_order_id.as_ref().map(|c| c.as_ref());

        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint(currency_pair.as_ref()))?
                    .signed_now()?
                    .request_body(BuyMarketOrderRequest {
                        amount,
                        client_order_id,
                    })?,
            )
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
