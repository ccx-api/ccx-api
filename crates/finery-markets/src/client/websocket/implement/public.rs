use crate::client::WebSocket;
use crate::client::websocket::implement::message::Die;
use crate::client::websocket::implement::message::WsCommand;
use crate::error::LibResult;
use crate::types::FeedId;
use crate::types::FeedRequest;
use crate::types::WsRequest;

impl WebSocket {
    pub(crate) async fn die(&self) {
        let _ = self.addr.send(Die).await;
    }

    pub(crate) async fn send_message(&self, msg: String) -> LibResult<()> {
        self.addr.send(WsCommand(msg)).await?
    }

    pub async fn subscribe(&self, feed: FeedRequest) -> LibResult<()> {
        self.addr.send(WsRequest::subscribe(feed)).await?
    }

    pub async fn unsubscribe(&self, feed: FeedRequest) -> LibResult<()> {
        self.addr.send(WsRequest::unsubscribe(feed)).await?
    }

    pub async fn subscribe_feed(&self, feed: FeedRequest, feed_id: FeedId) -> LibResult<()> {
        self.addr
            .send(WsRequest::subscribe_feed(feed, feed_id))
            .await?
    }

    pub async fn unsubscribe_feed(&self, feed: FeedRequest, feed_id: FeedId) -> LibResult<()> {
        self.addr
            .send(WsRequest::unsubscribe_feed(feed, feed_id))
            .await?
    }
}
