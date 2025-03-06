use ccx_binance::prelude::*;
use envconfig::Envconfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_BINANCE_KEY_NAME", default = "default")]
    key_name: String,
    #[envconfig(from = "EXAMPLE_BINANCE_API_KEY")]
    api_key: String,
    #[envconfig(from = "EXAMPLE_BINANCE_API_SECRET")]
    api_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_binance=debug,ccx_lib=trace".into()),
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
        BinanceCredential::new(
            config.key_name,
            config.api_key,
            config.api_secret.as_bytes(),
        )?
    };

    let client = {
        let client = reqwest::Client::new();
        let config = config::production();
        BinanceClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    let account_status = wallet::AccountStatus
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(account_status);

    let account_trading_status = wallet::AccountTradingStatus
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(account_trading_status);

    let _ = wallet::EnableFastWithdraw
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?;

    let _ = wallet::DisableFastWithdraw
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?;

    Ok(())
}
