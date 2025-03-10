use ccx_mexc::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_mexc=debug,ccx_lib=trace".into()),
        )
        .init();

    let spot_client = {
        let client = reqwest::Client::new();
        let config = config::production();
        MexcClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    let order_book = spot::GetOrderBook::new("ADAUSDT".into())
        .with_limit(5)
        .throttle(&rate_limiter)
        .await?
        .send(&spot_client)
        .await?
        .into_payload();

    dbg!(order_book);

    let recent_trades = spot::GetRecentTrades::new("ADAUSDT".into())
        .with_limit(5)
        .throttle(&rate_limiter)
        .await?
        .send(&spot_client)
        .await?;
    dbg!(recent_trades);

    let old_trades = spot::GetOldTrades::new("ADAUSDT".into())
        .with_limit(5)
        .throttle(&rate_limiter)
        .await?
        .send(&spot_client)
        .await?
        .into_payload();

    dbg!(old_trades);

    let old_trades = spot::GetOldTrades::new("ADAUSDT".into())
        .with_limit(5)
        .throttle(&rate_limiter)
        .await?
        .send(&spot_client)
        .await?
        .into_payload();

    dbg!(old_trades);

    Ok(())
}
