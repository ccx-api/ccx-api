use std::marker::PhantomData;

use smart_string::SmartString;

use crate::spot::client::handle_response;
use crate::spot::client::BinanceSpotClient;
use crate::spot::error::BinanceSpotSendError;
use crate::spot::meta::BinanceSpotResponseMeta;
use crate::spot::proto::BinanceSpotReadyToSend;
use crate::spot::proto::BinanceSpotSigned;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SignedRequest<T> {
    api_key: SmartString<64>,
    query: String,
    _phantom: PhantomData<T>,
}

impl<T> SignedRequest<T> {
    pub(super) fn new(query: String, api_key: &str) -> Self {
        let api_key = SmartString::from(api_key);
        SignedRequest {
            api_key,
            query,
            _phantom: PhantomData,
        }
    }
}

impl<T> BinanceSpotReadyToSend<T> for SignedRequest<T>
where
    T: BinanceSpotSigned,
{
    async fn send(
        self,
        client: &BinanceSpotClient,
    ) -> Result<BinanceSpotResponseMeta<T::Response>, BinanceSpotSendError> {
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
