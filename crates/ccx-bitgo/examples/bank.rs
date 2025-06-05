use ccx_bitgo::prelude::*;
use envconfig::Envconfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_BITGO_API_TOKEN")]
    api_token: String,
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

    let credential = {
        let config = match EnvConfig::init_from_env() {
            Ok(config) => config,
            Err(err) => {
                tracing::error!("{err}");
                std::process::exit(1);
            }
        };
        BitGoCredential::new(config.api_token)
    };
    let rate_limiter = RateLimiter::spawn();

    let client = {
        let client = reqwest::Client::new();
        let config = config::testing();

        BitGoClient::new(client, config)
    };

    let bank_accounts = bank::ListBankAccounts::builder()
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&bank_accounts);

    Ok(())
}
