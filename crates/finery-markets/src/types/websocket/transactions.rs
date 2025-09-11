use super::FeedRequest;
use crate::LibResult;
use crate::client::WebSocket;

#[derive(Clone)]
pub struct Transactions;

impl Transactions {
    pub async fn subscribe(ws: WebSocket) -> LibResult<()> {
        let feed_request = FeedRequest::SettlementTransactions;
        ws.subscribe(feed_request).await?;
        Ok(())
    }

    pub async fn unsubscribe(ws: WebSocket) -> LibResult<()> {
        let feed_request = FeedRequest::SettlementTransactions;
        ws.unsubscribe(feed_request).await?;
        Ok(())
    }
}
