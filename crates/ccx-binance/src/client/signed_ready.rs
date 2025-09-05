use std::marker::PhantomData;

use smart_string::SmartString;

use crate::client::BinanceClient;
use crate::client::handle_response;
use crate::client::meta::BinanceResponseWithMeta;
use crate::error::BinanceErrorWithMeta;
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
    #[tracing::instrument(skip_all, fields(http_method = %T::HTTP_METHOD, endpoint = %T::ENDPOINT), err)]
    async fn send(
        self,
        client: &BinanceClient,
    ) -> Result<BinanceResponseWithMeta<T::Response>, BinanceErrorWithMeta> {
        let inner = &client.inner;

        let mut url = inner.config.api_base.join(T::ENDPOINT)?;
        url.set_query(Some(&self.query));

        let request = inner
            .client
            .request(T::HTTP_METHOD, url)
            .header("X-MBX-APIKEY", self.api_key.as_str());

        handle_response(request.send().await?).await
    }
}
