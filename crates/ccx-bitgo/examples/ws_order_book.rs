use api_ws::*;
use ccx_bitgo::prelude::*;
use ccx_lib::console::Style;
use ccx_lib::console::Term;
use ccx_lib::nice_num::NiceNum;
use envconfig::Envconfig;
use futures::StreamExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_BITGO_API_TOKEN")]
    api_token: String,
    #[envconfig(from = "EXAMPLE_BITGO_ACCOUNT_ID")]
    account_id: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_bitgo=debug,ccx_lib=trace".into()),
        )
        .init();

    let term = Term::stdout();
    let ask_style = Style::new().green().bold();
    let bid_style = Style::new().red().bold();
    let symbol_style = Style::new().yellow().bold();

    let client = {
        let client = reqwest::Client::new();
        let config = config::testing();

        BitGoClient::new(client, config)
    };
    let config = EnvConfig::init_from_env()?;
    let credential = BitGoCredential::new(config.api_token);

    let mut ws = client.websocket(&credential).await?;

    ws.send(api_ws::WsRequest::order_book(
        api_ws::WsRequestEvent::Subscribe,
        OrderBookRequest::builder()
            .account_id(config.account_id)
            .product_id("TBTC-TUSD*")
            .build(),
    ))
    .await?;

    while let Some(response) = ws.next().await {
        match response {
            WsResponse::Error(error) => {
                tracing::error!(?error);

                Err(error)?;
            }
            WsResponse::System(system) => {
                tracing::info!(?system);
            }
            WsResponse::Channel(channel) => match channel {
                Channel::OrderBook(OrderBookEvent::Snapshot(order_book)) => {
                    for _ in 0..5 {
                        term.clear_line()?;
                        term.write_line("")?;
                    }

                    term.clear_line()?;
                    term.write_line(&format!(
                        "                 {:^20}",
                        symbol_style.apply_to(order_book.product)
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
