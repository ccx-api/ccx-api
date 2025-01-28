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

    let mut raw_stream = spot_client.websocket().raw_trade("btcusdt".into()).await?;
    while let Some(trade) = raw_stream.next().await {
        println!("{:#?}", trade);
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
