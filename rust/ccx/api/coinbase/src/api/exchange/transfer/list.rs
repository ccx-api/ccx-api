use crate::api::exchange::prelude::*;
use crate::api::exchange::Transfer;
use crate::api::exchange::TransferType;
use crate::api::exchange::RL_PRIVATE_KEY;

pub type ListTransfersResponse = Vec<Transfer>;

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Get all transfers.
    ///
    /// Gets a list of in-progress and completed transfers of funds in/out of any of the user's
    /// accounts.
    ///
    /// * `profile_id` - Returns list of transfers from this portfolio id.
    /// * `type` - Specifies deposit and withdrawal transfer types. Internal transfers represent
    ///   the transfers of a user depositing/withdrawing across their own profiles.
    /// * `after` - Used for pagination. Sets end (?) cursor to `after` date.
    /// * `before` - Used for pagination. Sets start (?) cursor to `before` date.
    /// * `limit` - Limit on number of results to return.
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires either the "view" or "trade" permission.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_gettransfers]
    pub fn list_transfers(
        &self,
        profile_id: Option<Uuid>,
        r#type: Option<TransferType>,
        after: Option<DtCoinbase>,
        before: Option<DtCoinbase>,
        limit: Option<u64>,
    ) -> CoinbaseResult<Task<ListTransfersResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/transfers");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .try_query_arg("profile_id", &profile_id)?
                    .try_query_arg("type", &r#type)?
                    .try_query_arg("after", &after)?
                    .try_query_arg("before", &before)?
                    .try_query_arg("limit", &limit)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
