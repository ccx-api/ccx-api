use ccx_bitgo::prelude::*;
use envconfig::Envconfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_BITGO_EXPRESS_URL")]
    express_url: Url,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_bitgo=trace,ccx_lib=trace".into()),
        )
        .init();
    let rate_limiter = RateLimiter::spawn();

    let client = {
        let client = reqwest::Client::new();
        let config = config::testing();

        BitGoClient::new(client, config)
    };

    let pong = general::Ping::default()
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(&pong);

    let config = EnvConfig::init_from_env()?;
    let client = {
        let client = reqwest::Client::new();
        let config = config::ConnectionConfig::new(config.express_url);

        BitGoClient::new(client, config)
    };

    let pong = general::PingExpress::default()
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(&pong);

    Ok(())
}
