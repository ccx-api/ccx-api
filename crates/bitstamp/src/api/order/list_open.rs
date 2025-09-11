use crate::api::RL_GENERAL_KEY;
use crate::api::order::OpenOrder;
use crate::api::prelude::*;

pub type ListOpenOrdersResponse = Vec<OpenOrder>;

#[cfg(feature = "with_network")]
impl<S> Api<S>
where
    S: crate::client::BitstampSigner,
    S: Unpin + 'static,
{
    /// Open orders
    ///
    /// This API call is cached for 10 seconds.
    /// This call will be executed on the account (Sub or Main),
    /// to which the used API key is bound to.
    ///
    /// [https://www.bitstamp.net/api/#open-orders]
    pub fn list_open_orders<C: AsRef<str>>(
        &self,
        currency: Option<C>,
    ) -> BitstampResult<Task<ListOpenOrdersResponse>> {
        fn endpoint(currency: Option<&str>) -> String {
            let currency = currency.unwrap_or("all");
            format!("open_orders/{currency}/")
        }
        let currency = currency.as_ref().map(|c| c.as_ref());

        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint(currency))?
                    .signed_now()?
                    .request_body(())?,
            )
            .cost(RL_GENERAL_KEY, 1)
            .send())
    }
}
