use futures::StreamExt;

use ccx_binance::{client::ApiCred, Api, LibError, WsStream, WsEvent, UpstreamWebsocketMessage};

#[actix_rt::main]
async fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance = Api::with_cred(ApiCred::from_env());

    let res = async move {
        // let mut ws = api.ws_diff_depth("btcusdt").await?;
        let (sink, mut stream) = binance.ws().await?.split();
        println!("Connected");

        sink.subscribe("btcusdt", WsStream::Depth).await.unwrap();
        println!("Subscribed");

        while let Some(e) = stream.next().await {
            match e {
                UpstreamWebsocketMessage::Response(e) => {
                    println!("{:?}", e);
                }
                UpstreamWebsocketMessage::Event(e) => {
                    if let WsEvent::DiffOrderBook(diff) = e {
                        println!("{:?}  <=>  {:?}", diff.bids.first(), diff.asks.first());
                    }
                }
            }
        }
        Ok::<(), LibError>(())
    };
    println!("Execution stopped with: {:?}", res.await);
}
