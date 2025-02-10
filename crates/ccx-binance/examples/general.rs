use ccx_binance::spot::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spot_client = {
        let client = reqwest::Client::new();
        let config = spot::config::production();
        BinanceSpotClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    // let pong = spot::api::Ping::new()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .send(&spot_client)
    //     .await?;
    // println!("{:?}", pong);

    // let server_time = spot::api::GetServerTime::new()
    //     .send(&spot_client)
    //     .await
    //     .unwrap();
    // println!("{:?}", server_time);

    let exchange_info = spot::api::GetExchangeInfo::new()
        .throttle(&rate_limiter)
        .await?
        .send(&spot_client)
        .await?;

    let (meta, exchange_info) = exchange_info.into_parts();
    println!("{:#?}", meta);

    for symbol in exchange_info.symbols.iter().take(3) {
        println!("{:#?}", symbol);
    }
    for rate_limit in &exchange_info.rate_limits {
        println!("{:#?}", rate_limit);
    }
    for filter in exchange_info.exchange_filters.iter().take(3) {
        println!("{:#?}", filter);
    }

    // let exchange_info = spot::api::GetExchangeInfo::with_symbols(&["BNBBTC", "BTCUSDT"])
    //     .send(&spot_client)
    //     .await
    //     .unwrap();
    // println!("{:#?}", exchange_info);

    // let exchange_info = spot::api::GetExchangeInfo::with_symbol("BNBBTC")
    //     .send(&spot_client)
    //     .await
    //     .unwrap();
    // println!("{:#?}", exchange_info);

    Ok(())
}
