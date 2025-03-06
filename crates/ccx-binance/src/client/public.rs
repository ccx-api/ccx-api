use crate::prelude::BinanceErrorWithMeta;
use crate::proto::{PublicRequest, RequestReadyToSend};

use super::meta::BinanceResponseWithMeta;
use super::{BinanceClient, handle_response};

impl<T> RequestReadyToSend<T> for T
where
    T: PublicRequest,
{
    async fn send(
        self,
        client: &BinanceClient,
    ) -> Result<BinanceResponseWithMeta<T::Response>, BinanceErrorWithMeta> {
        let inner = &client.inner;
        let mut url = inner.config.api_base.join(T::ENDPOINT)?;
        let query = serde_urlencoded::to_string(&self)?;
        if !query.is_empty() {
            url.set_query(Some(&query));
        }

        let request = inner.client.request(T::HTTP_METHOD, url);

        handle_response(request.send().await?).await
    }
}
