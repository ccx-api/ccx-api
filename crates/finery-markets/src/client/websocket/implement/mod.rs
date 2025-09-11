mod handler;
mod message;
mod public;

#[cfg(test)]
mod tests {

    use std::time::Duration;

    use actix::System;
    use actix::clock::sleep;
    use futures::StreamExt;

    use crate::ApiCred;
    use crate::SpotApi;
    use crate::error::LibResult;
    use crate::types::FeedId;
    use crate::types::FeedRequest;
    use crate::types::Nonce;
    use crate::types::Pair;
    use crate::types::Time;

    type Api = SpotApi<ApiCred>;

    #[test]
    #[ignore = "manual testing"]
    fn test_finery_api_ws() {
        let system = System::new();
        system.block_on(async move {
            let f = async move {
                let api = Api::from_env();

                let time = Time::now();
                let nonce = Nonce::from(time);

                let (ws, mut rx) = api.ws(nonce, time).await?;

                sleep(Duration::from_secs(1)).await;

                // let msg = String::from(r#"{"event": "bind", "feed": "I"}"#);
                // let msg = String::from(r#"{"event": "bind", "feed": "F", "feedId": 4955410050}"#);
                // let result = ws.send_message(msg).await;
                // let feed_request = FeedRequest::Instruments;
                // let feed_request = FeedRequest::PositionOrders;
                // let feed_request = FeedRequest::GlobalLimits;
                // let feed_request = FeedRequest::CounterpartyLimits;
                // let feed_request = FeedRequest::CounterpartyMutualLimits;
                let feed_request = FeedRequest::GlobalOrderBooks;
                // let feed_request = FeedRequest::TradableOrderBooks;
                // let feed_request = FeedRequest::SettlementRequests;
                // let feed_request = FeedRequest::SettlementTransactions;
                // let feed_request = FeedRequest::PositionFeed;
                // let feed_request = FeedRequest::Orders;
                // let feed_request = FeedRequest::SettlementOrders;
                // let feed_id = FeedId::Instrument(4955410050);
                let feed_id = FeedId::Pair(Pair::new("BTC", "EUR"));
                let result = ws.subscribe_feed(feed_request, feed_id.clone()).await;
                log::debug!("send_message result :: {:#?}", result);

                actix_rt::spawn(async move {
                    // sleep(Duration::from_secs(5)).await;
                    // let result = ws.unsubscribe_feed(feed_request, feed_id).await;
                    // log::debug!("send_message result :: {:#?}", result);
                    sleep(Duration::from_secs(600)).await;
                    log::debug!("close web socket");
                    ws.die().await;
                });

                loop {
                    match rx.next().await {
                        Some(Ok(message)) => {
                            // log::debug!("message :: {:?}", message);
                            log::debug!("message feed :: {:?}", message.feed);
                            log::debug!("message feed_id :: {:?}", message.feed_id);
                            log::debug!("message action :: {:?}", message.action);
                            log::debug!("message data :: {:?}", message.data());
                        }
                        Some(Err(error)) => {
                            log::debug!("message error :: {:?}", error);
                        }
                        None => {
                            log::debug!("channel was closed");
                            break;
                        }
                    }
                }

                Ok(())
            };
            let res: LibResult<()> = f.await;
            log::debug!("f res :: {:#?}", res);
            sleep(Duration::from_secs(5)).await;
            log::debug!("f res :: {:#?}", res);
            System::current().stop();
        });
        let _ = system.run();
    }
}
