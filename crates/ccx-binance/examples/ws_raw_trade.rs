use ccx_binance::spot::prelude::*;
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spot_client = {
        let client = reqwest::Client::new();
        let config = spot::config::production();
        BinanceSpotClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    let mut raw_stream = spot_client.websocket().raw_trade("adausdt".into()).await?;
    while let Some(trade) = raw_stream.next().await {
        let trade = trade?;
        // println!("{:#?}", trade);
        let market_side = if trade.is_buyer_market_maker {
            "📈"
        } else {
            "📉"
        };
        println!(
            "{}    {}    {:16.8}  {market_side} {:16.8}",
            trade.trade_time.timestamp(),
            trade.symbol,
            trade.price,
            trade.quantity,
        );
    }

    // let recent_trades = spot::api::GetRecentTrades::new("ADAUSDT".into())
    //     .with_limit(5)
    //     .throttle(&rate_limiter)
    //     .await?
    //     .send(&spot_client)
    //     .await?;
    // println!("{:#?}", recent_trades);

    Ok(())
}
