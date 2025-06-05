use ccx_bitgo::prelude::*;
use envconfig::Envconfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_BITGO_API_TOKEN")]
    api_token: String,
    #[envconfig(from = "EXAMPLE_BITGO_GO_WALLET_ID")]
    go_wallet_id: String,
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

    let config = EnvConfig::init_from_env()?;
    let credential = BitGoCredential::new(config.api_token);
    let rate_limiter = RateLimiter::spawn();

    let client = {
        let client = reqwest::Client::new();
        let config = config::testing();

        BitGoClient::new(client, config)
    };

    let fee = transfer::FeeEstimate::builder()
        .coin("teth")
        .amount(12354654)
        .build()
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(&fee);

    let fee = transfer::FeeEstimate::builder()
        .coin("teth")
        .build()
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(&fee);

    // Example: List transfers for a wallet
    println!("📝 Listing wallet transfers...");
    let transfers = transfer::ListTransfers::builder()
        .coin("ofctbtc")
        .wallet_id(config.go_wallet_id)
        .limit(10u32)
        .state(TransferState::Confirmed)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(transfers);

    Ok(())
}
