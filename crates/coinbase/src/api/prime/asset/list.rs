use super::AssetList;
use crate::api::prime::prelude::*;
use crate::CoinbaseResult;

pub type EntityAssetsResponse = AssetList;

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # List Assets.
    ///
    /// List all assets available for a given entity.
    ///
    /// ## Parameters
    ///
    /// * `entity_id` - The entity ID.
    ///
    /// To retrieve your entity_id, use List Portfolios.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_getentityassets]
    pub fn get_assets(&self, entity_id: Uuid) -> CoinbaseResult<Task<EntityAssetsResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/entities/{entity_id}/assets");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .get(&endpoint)?
                    .signed(timestamp)?
                    .request_body(())?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
