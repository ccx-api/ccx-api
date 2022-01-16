use std::collections::BTreeMap;

use futures::{stream, FutureExt, StreamExt};
use string_cache::DefaultAtom as Atom;

use ccx_binance::api::spot::OrderBookLimit;
use ccx_binance::util::OrderBook;
use ccx_binance::util::OrderBookUpdater;
use ccx_binance::BinanceError;
use ccx_binance::SpotApi;
use ccx_binance::UpstreamWebsocketMessage;
use ccx_binance::WsEvent;
use ccx_binance::WsStream;
// use ccx_binance_examples_util::*;

enum X {
    Snapshot((Atom, OrderBook)),
    Event(WsEvent),
}

#[actix_rt::main]
async fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance_spot = SpotApi::from_env();

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
        let mut snapshots = Vec::new();

        for symbol in &listen {
            state.insert(symbol.clone(), OrderBookUpdater::new());

            let f = Box::pin(
                binance_spot
                    .depth(symbol.clone(), OrderBookLimit::N1000)
                    .into_stream()
                    .filter_map({
                        let symbol = symbol.clone();
                        move |r| {
                            let symbol = symbol.clone();
                            async move {
                                println!("Received {}", symbol);
                                r.ok().map(|v| (X::Snapshot((symbol, v.into()))))
                            }
                        }
                    }),
            );

            snapshots.push(f);
        }

        let mut stream = Box::pin(stream.filter_map(move |e| async move {
            match e {
                UpstreamWebsocketMessage::Event(e) => Some(X::Event(e)),
                UpstreamWebsocketMessage::Response(e) => {
                    println!("{:?}", e);
                    None
                }
            }
        }));
        let mut stream = stream::select(&mut stream, stream::select_all(snapshots));

        while let Some(e) = stream.next().await {
            match e {
                X::Event(e) => {
                    if let WsEvent::DiffOrderBook(diff) = e {
                        state
                            .get_mut(&diff.symbol)
                            .unwrap()
                            .push_diff(diff)
                            .unwrap();
                    }
                }
                X::Snapshot((symbol, snapshot)) => {
                    let book = state.get_mut(&symbol).unwrap();
                    book.init(snapshot).unwrap();
                }
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
