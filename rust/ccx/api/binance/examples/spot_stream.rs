use std::collections::BTreeMap;

use futures::{stream, FutureExt, StreamExt};
use string_cache::DefaultAtom as Atom;

use ccx_binance::api::spot::OrderBookLimit;
use ccx_binance::util::OrderBook;
use ccx_binance::util::OrderBookUpdater;
use ccx_binance::ApiCred;
use ccx_binance::BinanceError;
use ccx_binance::SpotApi;
use ccx_binance::UpstreamWebsocketMessage;
use ccx_binance::WsEvent;
use ccx_binance::WsStream;
// use ccx_binance_examples_util::*;

#[actix_rt::main]
async fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance_spot = SpotApi::<ApiCred>::from_env();

    let res = async move {
        let (sink, stream) = binance_spot.ws().await?.split();
        println!("Connected");

        let listen: Vec<Atom> = vec![
            "BTCUSDT".into(),
            "ETHUSDT".into(),
            "LTCUSDT".into(),
            "ZECUSDT".into(),
        ];

        sink.subscribe_list(
            listen
                .iter()
                .map(|v| (v.to_lowercase(), WsStream::Depth100ms).into())
                .collect::<Vec<_>>()
                .into(),
        )
        .await
        .unwrap();
        println!("Subscribed");

        let mut state = BTreeMap::new();

        for symbol in &listen {
            let mut book_updater = OrderBookUpdater::new();

            let result = binance_spot
                .depth(symbol.clone(), OrderBookLimit::N1000)
                .await;
            if let Ok(snapshot) = result {
                let _ = book_updater.init(snapshot.into());
            }

            state.insert(symbol.clone(), book_updater);
        }

        let mut stream = Box::pin(stream.filter_map(move |e| async move {
            match e {
                UpstreamWebsocketMessage::Event(e) => Some(e),
                UpstreamWebsocketMessage::Response(e) => {
                    println!("{:?}", e);
                    None
                }
            }
        }));

        while let Some(e) = stream.next().await {
            match e {
                WsEvent::DiffOrderBook(diff) => {
                    state
                        .get_mut(&diff.symbol)
                        .unwrap()
                        .push_diff(diff)
                        .unwrap();
                }
                _ => {}
            }
            for (symbol, updater) in &state {
                let s = match updater.state() {
                    None => format!("<uninitialized>"),
                    Some(book) => format!(
                        "{:?}  <=>  {:?}",
                        book.bids().iter().next_back(),
                        book.asks().iter().next(),
                    ),
                };
                println!("{}\t{}", symbol, s);
            }
            println!();
        }
        Ok::<(), BinanceError>(())
    };
    println!("Execution stopped with: {:?}", res.await);
}
