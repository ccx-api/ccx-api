use crate::error::BitGoResult;
use crate::proto::{PublicRequest, RequestReadyToSend};

use super::BitGoClient;
use super::conversion::to_request_content;
use super::ready::ReadyRequest;

impl<T> RequestReadyToSend<T> for T
where
    T: PublicRequest,
{
    async fn send(self, client: &BitGoClient) -> BitGoResult<T::Response> {
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
