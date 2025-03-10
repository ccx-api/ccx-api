use ccx_binance::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_binance=debug,ccx_lib=trace".into()),
        )
        .init();

    let spot_client = {
        let client = reqwest::Client::new();
        let config = config::production();
        BinanceClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    let pong = spot::Ping::new()
        .throttle(&rate_limiter)
        .await?
        .send(&spot_client)
        .await?
        .into_payload();

    dbg!(pong);

    let server_time = spot::GetServerTime::new()
        .send(&spot_client)
        .await?
        .into_payload();

    dbg!(server_time);

    let exchange_info = spot::GetExchangeInfo::new()
        .throttle(&rate_limiter)
        .await?
        .send(&spot_client)
        .await?;

    let (meta, exchange_info) = exchange_info.into_parts();
    dbg!(meta);

    for symbol in exchange_info.symbols.iter().take(3) {
        dbg!(symbol);
    }
    for rate_limit in &exchange_info.rate_limits {
        dbg!(rate_limit);
    }
    for filter in exchange_info.exchange_filters.iter().take(3) {
        dbg!(filter);
    }

    let exchange_info = spot::GetExchangeInfo::with_symbols(&["BNBBTC", "BTCUSDT"])
        .send(&spot_client)
        .await?
        .into_payload();

    dbg!(exchange_info);

    let exchange_info = spot::GetExchangeInfo::with_symbol("BNBBTC")
        .send(&spot_client)
        .await?
        .into_payload();

    dbg!(exchange_info);

    Ok(())
}
