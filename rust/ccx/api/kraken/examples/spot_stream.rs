use std::collections::BTreeMap;
use std::println;
use std::thread;

use actix_rt::time::sleep;
use futures::StreamExt;
use string_cache::DefaultAtom as Atom;

use actix::prelude::*;
use ccx_kraken::api::spot::SpotApi;
use ccx_kraken::util::OrderBookUpdater;
use ccx_kraken::ws_stream::UpstreamWebsocketMessage;
use ccx_kraken::ws_stream::UpstreamWebsocketResult;
use ccx_kraken::ws_stream::WsEvent;
use ccx_kraken::ws_stream::WsStream;
use ccx_kraken::ws_stream::WsStreamBookParams;
use ccx_kraken::{KrakenApiError, KrakenApiResult, KrakenError};

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

        let listen: Vec<Atom> = vec![
            // "XBT/EUR".into(),
            // "ETH/EUR".into(),
            "XBT/USD".into(),
            // "ETH/USD".into(),
        ];
        let params = WsStreamBookParams { depth: 10 };
        let mut state: BTreeMap<Atom, OrderBookUpdater> = BTreeMap::new();

        for pair in &listen {
            state.insert(pair.clone(), OrderBookUpdater::new());
        }

        sink.subscribe((listen, WsStream::Book(params)))
            .await
            .unwrap();
        println!("Subscribed");

        // accumulate last N events
        const trace_capacity: usize = 50;
        enum TraceEvent {
            Event(WsEvent),
            Print,
        }
        let (tx, rx) = std::sync::mpsc::channel::<TraceEvent>();
        thread::spawn(move || {
            let mut cache: [WsEvent; trace_capacity] = core::array::from_fn(|_| {
                WsEvent::Heartbeat(ccx_kraken::ws_stream::Heartbeat {
                    event: ccx_kraken::ws_stream::HeartbeatEvent::Heartbeat,
                })
            });
            let mut i = 0;

            while let Ok(trace_event) = rx.recv() {
                let ws_event = match trace_event {
                    TraceEvent::Event(e) => e,
                    TraceEvent::Print => {
                        // cache.iter().enumerate().for_each(|(index, item)| {
                        //     println!("Cache item #{index}");
                        //     println!("{:?}", item);
                        //     println!();
                        // });
                        continue;
                    }
                };

                cache[i % trace_capacity] = ws_event;
                i += 1;
            }
        });

        let txx = tx.clone();
        let mut stream = Box::pin(stream.filter_map(move |e| {
            let tx = txx.clone();
            async move {
                match e {
                    UpstreamWebsocketMessage::Event(e) => {
                        // println!("Event {:?}", e);
                        tx.send(TraceEvent::Event(e.clone())).unwrap();
                        Some(e)
                    }
                    UpstreamWebsocketMessage::Response(e) => {
                        // println!("Response {:?}", e);
                        if let UpstreamWebsocketResult::Ok(e) = e.payload {
                            tx.send(TraceEvent::Event(e)).unwrap();
                        }
                        None
                    }
                }
            }
        }));

        let txx = tx.clone();
        'outer: while let Some(e) = stream.next().await {
            let tx = txx.clone();

            // OrderBook 10 - это capacity

            match e {
                WsEvent::OrderBookSnap(snap) => {
                    state
                        .get_mut(&snap.pair)
                        .unwrap()
                        .init(snap.into())
                        .unwrap();
                }
                WsEvent::OrderBookDiff(diff) => {
                    state.get_mut(&diff.pair).unwrap().push_diff(diff).unwrap();
                }
                _ => continue,
            }
            for (_symbol, updater) in &state {
                match updater.state() {
                    None => {}
                    Some(book) => {
                        // let (ask_price, _ask_volume) = book.ask_low().unwrap();
                        // let (bid_price, _bid_volume) = book.bid_high().unwrap();

                        // if ask_price < bid_price {
                        //     tx.send(TraceEvent::Print).unwrap();
                        //     sleep(std::time::Duration::from_secs(10)).await;
                        //     println!("book: {:?}", book);
                        //     break 'outer;
                        // }

                        let mut lines = vec![];

                        book.asks()
                            .iter()
                            .for_each(|(p, v)| lines.push(format!("{}: {}", p, v)));

                        let ask_avg = book.ask_avg().unwrap_or_default();
                        lines.push(format!("ask avg. {}: {}", ask_avg.0, ask_avg.1));
                        lines.push(format!("-----------------"));

                        book.bids()
                            .iter()
                            .for_each(|(p, v)| lines.push(format!("{}: {}", p, v)));

                        let bid_avg = book.bid_avg().unwrap_or_default();
                        lines.push(format!("bid avg. {}: {}", bid_avg.0, bid_avg.1));
                        lines.push(format!("spread: {}", book.spread()));

                        print!("{}\n\n.\n\n", lines.join("\n"));
                    }
                };

                // println!("{}\t{:?}", symbol, updater);
                // let s = match updater.state() {
                //     None => format!("<uninitialized>"),
                //     Some(book) => format!(
                //         "{:?}  <=>  {:?}",
                //         book.bids().iter().next_back(),
                //         book.asks().iter().next(),
                //     ),
                // };
                // println!("{}\t{}", symbol, s);
            }
            // println!();
        }

        Ok::<_, KrakenError>(())
    };
    println!("Execution stopped with: {:?}", res.await);

    KrakenApiError::ok(())
}
