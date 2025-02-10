use ccx_binance::spot::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spot_client = {
        let client = reqwest::Client::new();
        let config = spot::config::production();
        BinanceSpotClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    let order_book = spot::api::GetOrderBook::new("ADAUSDT".into())
        .with_limit(5)
        .throttle(&rate_limiter)
        .await?
        .send(&spot_client)
        .await?;
    println!("{:#?}", order_book);

    // let recent_trades = spot::api::GetRecentTrades::new("ADAUSDT".into())
    //     .with_limit(5)
    //     .throttle(&rate_limiter)
    //     .await?
    //     .send(&spot_client)
    //     .await?;
    // println!("{:#?}", recent_trades);

    // let old_trades = spot::api::GetOldTrades::new("ADAUSDT".into())
    //     .with_limit(5)
    //     .throttle(&rate_limiter)
    //     .await?
    //     .send(&spot_client)
    //     .await?;
    // println!("{:#?}", old_trades);
    //
    // let prev_trade_id = old_trades.payload.first().unwrap().id - 5;
    //
    // let old_trades = spot::api::GetOldTrades::new("ADAUSDT".into())
    //     .with_limit(5)
    //     .with_from_id(prev_trade_id)
    //     .throttle(&rate_limiter)
    //     .await?
    //     .send(&spot_client)
    //     .await?;
    // println!("{:#?}", old_trades);

    Ok(())
}
