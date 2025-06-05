use api_ws::*;
use ccx_kraken::{api_ws, prelude::*};
use ccx_lib::console::Style;
use ccx_lib::console::Term;
use ccx_lib::nice_num::NiceNum;
use ccx_lib::order_book::{OrderBook, PriceAndAmount};
use futures::StreamExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_kraken=debug,ccx_lib=trace".into()),
        )
        .init();

    let term = Term::stdout();
    let main_style = Style::new().blue().bold();
    let num_style = Style::new().white().bold();
    let ask_style = Style::new().green().bold();
    let bid_style = Style::new().red().bold();
    let symbol_style = Style::new().yellow().bold();

    let client = {
        let client = reqwest::Client::new();
        let config = config::production();

        KrakenClient::new(client, config)
    };

    let mut ws = client.websocket().await?;

    ws.send(api_ws::WsRequest::ping()).await?;

    ws.send(api_ws::WsRequest::order_book(
        api_ws::WsRequestEvent::Subscribe,
        OrderBookChannel::builder()
            .symbol(vec!["BTC/USD".into()])
            .depth(Depth::L1000)
            .build(),
    ))
    .await?;

    let mut order_book = OrderBookSync::new();

    while let Some(response) = ws.next().await {
        match response.event {
            Event::Pong(_) => {
                tracing::info!("pong");
            }
            Event::Status(status) => {
                tracing::info!(?status, "Got status");
            }
            Event::OrderBook(channel) => match channel {
                ChannelResponse::Snapshot(snapshot) => {
                    order_book.set_from_snapshot(snapshot);
                }
                ChannelResponse::Update(update) => {
                    order_book.update(&update);

                    let bids = order_book.bids();
                    let asks = order_book.asks();

                    for _ in 0..5 {
                        term.clear_line()?;
                        term.write_line("")?;
                    }

                    term.clear_line()?;
                    term.write_line(&format!(
                        "                 {:^20}",
                        symbol_style.apply_to(update.symbol)
                    ))?;

                    let volume = 1_000_000.into();
                    let fill = order_book.ask_base_depth(volume)?;

                    term.clear_line()?;
                    term.write_line("")?;

                    term.clear_line()?;
                    term.write_line(&format!(
                        "{}",
                        main_style.apply_to(&format!(
                            "ask depth {} :: {}",
                            NiceNum(&ask_style, fill.price(), 8),
                            NiceNum(&ask_style, fill.base_value, 8),
                        )),
                    ))?;

                    term.clear_line()?;
                    term.write_line("")?;

                    let len = asks.len();
                    let begin = len - len.min(20);

                    for PriceAndAmount { price, amount } in asks.skip(begin) {
                        term.clear_line()?;
                        term.write_line(&format!(
                            "          {} :: {}",
                            NiceNum(&ask_style, price, 8),
                            NiceNum(&ask_style, amount, 8),
                        ))?;
                    }

                    term.clear_line()?;
                    term.write_line("")?;

                    term.clear_line()?;
                    // term.write_line(&format!("spread: {}", order_book.spread()))?;
                    let spread = order_book.spread().unwrap_or_default();
                    term.write_line(&format!(
                        "{}",
                        main_style
                            .apply_to(&format!("spread:   {}", NiceNum(&num_style, spread, 8),))
                    ))?;

                    term.clear_line()?;
                    term.write_line("")?;

                    for PriceAndAmount { price, amount } in bids.take(20) {
                        term.clear_line()?;
                        term.write_line(&format!(
                            "          {} :: {}",
                            NiceNum(&bid_style, price, 8),
                            NiceNum(&bid_style, amount, 8),
                        ))?;
                    }

                    let fill = order_book.bid_base_depth(volume)?;

                    term.clear_line()?;
                    term.write_line("")?;

                    term.clear_line()?;
                    term.write_line(&format!(
                        "{}",
                        main_style.apply_to(&format!(
                            "bid depth {} :: {}",
                            NiceNum(&bid_style, fill.price(), 8),
                            NiceNum(&bid_style, fill.base_value, 8),
                        )),
                    ))?;

                    term.move_cursor_up(54)?;
                }
            },
            _ => {}
        }
    }

    Ok(())
}
