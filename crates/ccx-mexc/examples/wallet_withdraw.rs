use ccx_mexc::prelude::*;
use envconfig::Envconfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_MEXC_KEY_NAME", default = "default")]
    key_name: String,
    #[envconfig(from = "EXAMPLE_MEXC_API_KEY")]
    api_key: String,
    #[envconfig(from = "EXAMPLE_MEXC_API_SECRET")]
    api_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_mexc=debug,ccx_lib=trace".into()),
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
        MexcCredential::new(
            config.key_name,
            config.api_key,
            config.api_secret.as_bytes(),
        )?
    };

    let client = {
        let client = reqwest::Client::new();
        let config = config::production();
        MexcClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    let withdraw_history = wallet::WithdrawHistory::builder()
        // .coin("USDT")
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(withdraw_history);

    let withdraw = wallet::Withdraw::builder()
        .coin("USDT")
        .network("ETH")
        .address("0x5c2b938cb78931f95d26a781f5279f2dc112ca1d")
        .amount(1.into())
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?;

    dbg!(withdraw);

    Ok(())
}
