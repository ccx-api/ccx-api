use crate::api::exchange::prelude::*;
use crate::api::exchange::Transfer;
use crate::api::exchange::RL_PRIVATE_KEY;

pub type GetTransferResponse = Transfer;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Get a single transfer.
    ///
    /// Get information on a single transfer.
    ///
    /// * `transfer_id` - .
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires either the "view" or "trade" permission.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_gettransfer]
    pub fn get_transfer(&self, transfer_id: Uuid) -> CoinbaseResult<Task<GetTransferResponse>> {
        let endpoint = format!("transfers/{transfer_id}");
        Ok(self
            .rate_limiter
            .task(self.client.get(&endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
