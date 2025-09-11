use super::FeedRequest;
use crate::LibResult;
use crate::client::WebSocket;

#[derive(Clone)]
pub struct Positions;

impl Positions {
    pub async fn subscribe(ws: WebSocket) -> LibResult<()> {
        let feed_request = FeedRequest::PositionOrders;
        ws.subscribe(feed_request).await?;
        Ok(())
    }

    pub async fn unsubscribe(ws: WebSocket) -> LibResult<()> {
        let feed_request = FeedRequest::PositionOrders;
        ws.unsubscribe(feed_request).await?;
        Ok(())
    }
}
