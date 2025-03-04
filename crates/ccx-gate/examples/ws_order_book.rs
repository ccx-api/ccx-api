use api_ws::*;
use ccx_gate::prelude::*;
use ccx_lib::console::Style;
use ccx_lib::console::Term;
use ccx_lib::nice_num::NiceNum;
use futures::StreamExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_gate=debug,ccx_lib=trace".into()),
        )
        .init();

    let term = Term::stdout();
    let ask_style = Style::new().green().bold();
    let bid_style = Style::new().red().bold();
    let symbol_style = Style::new().yellow().bold();

    let client = {
        let client = reqwest::Client::new();
        let config = config::production();

        GateClient::new(client, config)
    };

    let mut ws = client.websocket().await?;

    ws.send(api_ws::WsRequest::order_book(
        api_ws::WsRequestEvent::Subscribe,
        OrderBookRequest {
            pair: "BTC_USDT".into(),
            level: Level::L10,
            interval: Interval::Ms1000,
        },
    ))
    .await?;

    while let Some(response) = ws.next().await {
        match response.event {
            Event::Pong(_) => {
                tracing::info!("pong");
            }
            Event::OrderBook(event_inner) => match event_inner {
                EventInner::Subscribe(_) => {
                    tracing::info!("Subscribed");
                }
                EventInner::Unsubscribe(_) => {
                    tracing::info!("Unsubscribed");
                }
                EventInner::Update(snapshot) => {
                    let order_book = match snapshot {
                        Ok(snapshot) => snapshot,
                        Err(err) => {
                            tracing::error!(?err);
                            break;
                        }
                    };

                    for _ in 0..5 {
                        term.clear_line()?;
                        term.write_line("")?;
                    }

                    term.clear_line()?;
                    term.write_line(&format!(
                        "                 {:^20}",
                        symbol_style.apply_to(order_book.currency_pair)
                    ))?;

                    term.clear_line()?;
                    term.write_line("")?;

                    let len = order_book.asks.len();
                    let begin = len - len.min(20);

                    for ask in order_book.asks.iter().rev().skip(begin) {
                        term.clear_line()?;
                        term.write_line(&format!(
                            "          {} :: {}",
                            NiceNum(&ask_style, ask.price, 8),
                            NiceNum(&ask_style, ask.amount, 8),
                        ))?;
                    }

                    term.clear_line()?;
                    term.write_line("")?;

                    term.clear_line()?;
                    term.write_line("")?;

                    for bid in order_book.bids.iter().rev().take(20) {
                        term.clear_line()?;
                        term.write_line(&format!(
                            "          {} :: {}",
                            NiceNum(&bid_style, bid.price, 8),
                            NiceNum(&bid_style, bid.amount, 8),
                        ))?;
                    }

                    term.move_cursor_up(54)?;
                }
            },
        }
    }

    Ok(())
}
