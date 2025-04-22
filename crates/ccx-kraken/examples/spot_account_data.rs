use ccx_kraken::prelude::*;
use ccx_kraken::rate_limiter::Tier;
use envconfig::Envconfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_KRAKEN_API_KEY")]
    api_key: String,
    #[envconfig(from = "EXAMPLE_KRAKEN_API_SECRET")]
    api_secret: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_kraken=debug,ccx_lib=trace".into()),
        )
        .init();

    let credential = {
        let config = match EnvConfig::init_from_env() {
            Ok(config) => config,
            Err(err) => {
                tracing::error!("{err}");
                std::process::exit(1);
            }
        };
        KrakenCredential::new(config.api_key, config.api_secret)
    };
    let rate_limiter = RateLimiter::spawn(Tier::default());

    let client = {
        let client = reqwest::Client::new();
        let config = config::production();

        KrakenClient::new(client, config)
    };

    let balance = spot::account_data::AccountBalance::new()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(balance);

    Ok(())
}
