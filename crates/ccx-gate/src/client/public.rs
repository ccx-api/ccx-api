use crate::error::GateResult;
use crate::proto::{PublicRequest, RequestReadyToSend};

use super::GateClient;
use super::conversion::to_request_content;
use super::ready::ReadyRequest;

impl<T> RequestReadyToSend<T> for T
where
    T: PublicRequest,
{
    #[tracing::instrument(skip_all, fields(http_method = %T::HTTP_METHOD, endpoint = %self.path()), err)]
    async fn send(self, client: &GateClient) -> GateResult<T::Response> {
        let content = to_request_content(&self)?;

        let path = self.path();

        let ready_request: ReadyRequest<T> = ReadyRequest::builder()
            .path(path.into_owned())
            .query(content.query)
            .body(content.body)
            .build();

        ready_request.send(client).await
    }
}
