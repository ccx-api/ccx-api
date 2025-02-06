use core::fmt;

use ccx_mexc::util::OrderBook;
use ccx_mexc::util::OrderBookUpdater;
use ccx_mexc::ws_stream::UpstreamWebsocketMessage;
use ccx_mexc::ws_stream::WsEvent;
use ccx_mexc::ws_stream::WsStream;
use ccx_mexc::ApiCred;
use ccx_mexc::MexcError;
use ccx_mexc::SpotApi;
use console::Style;
use console::Term;
use futures::stream;
use futures::FutureExt;
use futures::StreamExt;
use rust_decimal::Decimal;
use smart_string::DisplayExt;
use smart_string::SmartString;

enum X {
    Snapshot(OrderBook),
    SnapshotErr(MexcError),
    Event(WsEvent),
}

#[actix_rt::main]
async fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    let term = Term::stdout();
    let main_style = Style::new().blue().bold();
    let num_style = Style::new().white().bold();
    let ask_style = Style::new().green().bold();
    let bid_style = Style::new().red().bold();
    let symbol_style = Style::new().yellow().bold();

    let mexc_spot = SpotApi::<ApiCred>::from_env();

    let res =
        async move {
            let (sink, stream) = mexc_spot.ws().await?.split();
            println!("Connected");

            let symbol = "BTCUSDT";

            let subscribe_list = [symbol]
                .into_iter()
                .map(|v| (v.to_lowercase(), WsStream::Depth100ms).into())
                .collect();

            sink.subscribe_list(subscribe_list).await.unwrap();
            println!("Subscribed");

            let mut updater = OrderBookUpdater::new();

            let snapshot = Box::pin(mexc_spot.depth(symbol, Some(1000))?.into_stream().map(
                move |r| match r {
                    Ok(book) => X::Snapshot(book.into()),
                    Err(e) => X::SnapshotErr(e),
                },
            ));

            let mut stream = Box::pin(stream.filter_map(move |e| async move {
                match e {
                    UpstreamWebsocketMessage::Event(e) => Some(X::Event(e)),
                    UpstreamWebsocketMessage::Response(e) => {
                        println!("{:?}", e);
                        None
                    }
                }
            }));
            let mut stream = stream::select(&mut stream, snapshot);

            while let Some(e) = stream.next().await {
                match e {
                    X::Event(e) => {
                        if let WsEvent::OrderBookDiff(diff) = e {
                            updater.push_diff(diff).unwrap();
                        }
                    }
                    X::Snapshot(snapshot) => {
                        updater.init(snapshot).unwrap();
                    }
                    X::SnapshotErr(e) => {
                        log::error!("SnapshotErr: {:?}", e);
                        break;
                    }
                }
                match updater.state() {
                    None => {}
                    Some(book) => {
                        for _ in 0..5 {
                            term.clear_line()?;
                            term.write_line("")?;
                        }

                        term.clear_line()?;
                        term.write_line(&format!(
                            "                 {:^20}",
                            symbol_style.apply_to(symbol)
                        ))?;

                        term.clear_line()?;
                        term.write_line("")?;

                        let (a, b) = book.ask_avg().unwrap_or_default();

                        term.clear_line()?;
                        term.write_line(&format!(
                            "{}",
                            main_style.apply_to(&format!(
                                "ask avg.: {} :: {}",
                                NiceNum(&ask_style, a, 6),
                                NiceNum(&ask_style, b, 6),
                            )),
                        ))?;

                        term.clear_line()?;
                        term.write_line("")?;

                        let len = book.asks().len();
                        let begin = len - len.min(10);

                        for (&p, &v) in book.asks().iter().rev().skip(begin) {
                            term.clear_line()?;
                            term.write_line(&format!(
                                "          {} :: {}",
                                NiceNum(&ask_style, p, 6),
                                NiceNum(&ask_style, v, 6),
                            ))?;
                        }

                        term.clear_line()?;
                        term.write_line("")?;

                        term.clear_line()?;
                        // term.write_line(&format!("spread: {}", book.spread()))?;
                        term.write_line(&format!(
                            "{}",
                            main_style.apply_to(&format!(
                                "spread:   {}",
                                NiceNum(&num_style, book.spread(), 6),
                            ))
                        ))?;

                        term.clear_line()?;
                        term.write_line("")?;

                        for (&p, &v) in book.bids().iter().rev().take(10) {
                            term.clear_line()?;
                            term.write_line(&format!(
                                "          {} :: {}",
                                NiceNum(&bid_style, p, 6),
                                NiceNum(&bid_style, v, 6),
                            ))?;
                        }

                        let (a, b) = book.bid_avg().unwrap_or_default();

                        term.clear_line()?;
                        term.write_line("")?;

                        term.clear_line()?;
                        term.write_line(&format!(
                            "{}",
                            main_style.apply_to(&format!(
                                "bid avg.: {} :: {}",
                                NiceNum(&bid_style, a, 6),
                                NiceNum(&bid_style, b, 6),
                            )),
                        ))?;

                        term.move_cursor_up(34)?;
                    }
                }
            }
            Ok::<(), MexcError>(())
        };
    println!("Execution stopped with: {:?}", res.await);
}

struct NiceNum<'a>(&'a Style, Decimal, usize);

impl fmt::Display for NiceNum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self(style, num, left) = *self;

        let num: SmartString<62> = format_args!("{num:0.8}").to_fmt();
        let dot_pos = num.bytes().position(|c| c == b'.');
        if let Some(dot_pos) = dot_pos {
            let int_part = left.min(dot_pos).min(10);
            let left_pad = left - int_part.min(left);
            for _ in 0..left_pad {
                write!(f, " ")?;
            }
        }
        let s = num
            .trim_end_matches('0')
            .trim_end_matches('.')
            .trim_end_matches('0');

        format_args!("{}{}", &style.apply_to(&num[..s.len()]), &num[s.len()..]).write_to_fmt(f)
    }
}
