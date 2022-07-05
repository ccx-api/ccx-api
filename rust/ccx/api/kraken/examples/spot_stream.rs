use std::collections::BTreeMap;

use futures::StreamExt;
use string_cache::DefaultAtom as Atom;

use actix::prelude::*;
use ccx_kraken::api::spot::SpotApi;
use ccx_kraken::util::OrderBookUpdater;
use ccx_kraken::WsEvent;
use ccx_kraken::WsStream;
use ccx_kraken::{KrakenApiError, KrakenApiResult, KrakenError};
use ccx_kraken::{UpstreamWebsocketMessage, WsStreamBookParams};

fn main() {
    let system = System::new();
    let _addr = system.block_on(async {
        let _ = main_().await;
        System::current().stop();
    });
    system.run().unwrap();
}

async fn main_() -> KrakenApiResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let kraken_spot = SpotApi::from_env();

    let res = async move {
        let (sink, stream) = kraken_spot.ws().await?.split();
        println!("Connected");

        let listen: Vec<Atom> = vec!["XBT/USDT".into(), "ETH/USDT".into()];
        let params = WsStreamBookParams { depth: 10 };
        let mut state: BTreeMap<Atom, OrderBookUpdater> = BTreeMap::new();

        for pair in &listen {
            state.insert(pair.clone(), OrderBookUpdater::new());
        }

        sink.subscribe((listen, WsStream::Book(params)))
            .await
            .unwrap();
        println!("Subscribed");

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
                WsEvent::SnapOrderBook(snap) => {
                    state
                        .get_mut(&snap.pair)
                        .unwrap()
                        .init(snap.into())
                        .unwrap();
                }
                WsEvent::DiffOrderBook(diff) => {
                    state.get_mut(&diff.pair).unwrap().push_diff(diff).unwrap();
                }
                _ => continue,
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
        Ok::<_, KrakenError>(())
    };
    println!("Execution stopped with: {:?}", res.await);

    KrakenApiError::ok(())
}

// async fn main_old_() -> KrakenApiResult<()> {
//     let _ = dotenv::dotenv();
//     env_logger::init();

//     let mut seq = NonceSeq::new();
//     let kraken_spot = SpotApi::from_env();

//     let res = async move {
//         let (sink, mut stream) = kraken_spot.ws().await?.split();
//         println!("Connected");

//         let iso4217_pairs: Vec<Atom> = vec!["XBT/USDT".into(), "ETH/USDT".into()];
//         let listen: Vec<(Atom, Atom)> = iso4217_pairs
//             .iter()
//             .map(|i| (i.replace("/", "").into(), i.clone()))
//             .collect();

//         let params = WsStreamBookParams { depth: 10 }; // PROD 10
//         let mut state: BTreeMap<Atom, OrderBookUpdater> = BTreeMap::new();

//         for (pair, iso4217_pair) in &listen {
//             let mut book_updater = OrderBookUpdater::new();

//             let result = kraken_spot.depth(pair, Some(OrderBookLimit::N500)).await;
//             if let Ok((resp, _)) = result {
//                 if let Some(snapshot) = resp.pair.into_values().next() {
//                     let _ = book_updater.init(snapshot.into());
//                 }
//             }

//             state.insert(iso4217_pair.clone(), book_updater);
//         }

//         sink.subscribe((iso4217_pairs, WsStream::Book(params)))
//             .await
//             .unwrap();
//         println!("Subscribed");

//         let mut stream = Box::pin(stream.filter_map(move |e| async move {
//             match e {
//                 UpstreamWebsocketMessage::Event(e) => Some(e),
//                 UpstreamWebsocketMessage::Response(e) => {
//                     println!("{:?}", e);
//                     None
//                 }
//             }
//         }));

//         while let Some(e) = stream.next().await {
//             match e {
//                 WsEvent::DiffOrderBook(diff) => {
//                     state.get_mut(&diff.pair).unwrap().push_diff(diff).unwrap();
//                 }
//                 _ => {}
//             }
//             for (symbol, updater) in &state {
//                 let s = match updater.state() {
//                     None => format!("<uninitialized>"),
//                     Some(book) => format!(
//                         "{:?}  <=>  {:?}",
//                         book.bids().iter().next_back(),
//                         book.asks().iter().next(),
//                     ),
//                 };
//                 println!("{}\t{}", symbol, s);
//             }
//             println!();
//         }
//         Ok::<_, KrakenError>(())
//     };
//     println!("Execution stopped with: {:?}", res.await);

//     KrakenApiError::ok(())
// }
