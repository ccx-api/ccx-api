use crate::client::WebSocket;
use crate::LibResult;

use super::FeedRequest;

#[derive(Clone)]
pub struct Transactions;

impl Transactions {
    pub async fn subscribe(ws: WebSocket) -> LibResult<()> {
        let feed_request = FeedRequest::SettlementTransactions;
        let _result = ws.subscribe(feed_request).await?;
        Ok(())
    }

    pub async fn unsubscribe(ws: WebSocket) -> LibResult<()> {
        let feed_request = FeedRequest::SettlementTransactions;
        let _result = ws.unsubscribe(feed_request).await?;
        Ok(())
    }
}
