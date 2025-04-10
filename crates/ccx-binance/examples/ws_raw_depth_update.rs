use ccx_binance::prelude::*;
use ccx_binance::types::ws_depth_updater::OrderBookSync;
use ccx_binance::types::ws_stream_name::DepthUpdateSpeed;
use ccx_lib::console::Style;
use ccx_lib::console::Term;
use ccx_lib::nice_num::NiceNum;
use ccx_lib::order_book::{OrderBook, PriceAndAmount};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Term::stdout();
    let main_style = Style::new().blue().bold();
    let num_style = Style::new().white().bold();
    let ask_style = Style::new().green().bold();
    let bid_style = Style::new().red().bold();
    let symbol_style = Style::new().yellow().bold();

    let spot_client = {
        let client = reqwest::Client::new();
        let config = config::production();
        BinanceClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    let mut raw_stream = spot_client
        .websocket()
        .raw_depth_update("ADAUSDT", DepthUpdateSpeed::Ms100)
        .await?;

    let order_book = spot::GetOrderBook::new("ADAUSDT".into())
        .with_limit(5000)
        .throttle(&rate_limiter)
        .await?
        .send(&spot_client)
        .await?
        .into_payload();

    let mut order_book = OrderBookSync::new(order_book);

    while let Some(depth_update) = raw_stream.next().await {
        let depth_update = depth_update?;
        // println!(
        //     "{}    {}    {:16}  vs  {:16} .. {:16}",
        //     depth_update.event_time.timestamp(),
        //     depth_update.symbol,
        //     order_book.last_update_id(),
        //     depth_update.first_update_id,
        //     depth_update.last_update_id,
        // );
        order_book.update(&depth_update)?;

        let bids = order_book.bids();
        let asks = order_book.asks();

        // for (price, qty) in bids.iter().rev().take(5) {
        //     println!("BID {:16.8} {:16.8}", price, qty);
        // }
        //
        // println!();
        //
        // for (price, qty) in asks.iter().take(5) {
        //     println!("ASK {:16.8} {:16.8}", price, qty);
        // }

        for _ in 0..5 {
            term.clear_line()?;
            term.write_line("")?;
        }

        term.clear_line()?;
        term.write_line(&format!(
            "                 {:^20}",
            symbol_style.apply_to(depth_update.symbol)
        ))?;

        let volume = 1_000_000.into();
        let fill = order_book.ask_base_depth(volume);

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
            main_style.apply_to(&format!("spread:   {}", NiceNum(&num_style, spread, 8),))
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

        let fill = order_book.bid_base_depth(volume);

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

    Ok(())
}
