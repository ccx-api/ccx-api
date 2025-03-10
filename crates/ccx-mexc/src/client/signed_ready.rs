use std::marker::PhantomData;

use smart_string::SmartString;

use crate::client::MexcClient;
use crate::client::handle_response;
use crate::client::meta::MexcResponseWithMeta;
use crate::error::MexcErrorWithMeta;
use crate::proto::RequestReadyToSend;
use crate::proto::SignedRequest;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SignedReadyRequest<T> {
    api_key: SmartString<64>,
    query: String,
    _phantom: PhantomData<T>,
}

impl<T> SignedReadyRequest<T> {
    pub(super) fn new(query: String, api_key: &str) -> Self {
        let api_key = SmartString::from(api_key);
        SignedReadyRequest {
            api_key,
            query,
            _phantom: PhantomData,
        }
    }
}

impl<T> RequestReadyToSend<T> for SignedReadyRequest<T>
where
    T: SignedRequest,
{
    #[tracing::instrument(skip_all)]
    async fn send(
        self,
        client: &MexcClient,
    ) -> Result<MexcResponseWithMeta<T::Response>, MexcErrorWithMeta> {
        let inner = &client.inner;

        let mut url = inner.config.api_base.join(T::ENDPOINT)?;
        url.set_query(Some(&self.query));

        tracing::debug!(%url, method = %T::HTTP_METHOD, "Request");

        let request = inner
            .client
            .request(T::HTTP_METHOD, url)
            .header("X-MEXC-APIKEY", self.api_key.as_str());

        handle_response(request.send().await?).await
    }
}
