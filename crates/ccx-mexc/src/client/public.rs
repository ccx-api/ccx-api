use crate::prelude::MexcErrorWithMeta;
use crate::proto::{PublicRequest, RequestReadyToSend};

use super::meta::MexcResponseWithMeta;
use super::{MexcClient, handle_response};

impl<T> RequestReadyToSend<T> for T
where
    T: PublicRequest,
{
    async fn send(
        self,
        client: &MexcClient,
    ) -> Result<MexcResponseWithMeta<T::Response>, MexcErrorWithMeta> {
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
