use ccx_kraken::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_kraken=debug,ccx_lib=trace".into()),
        )
        .init();

    let client = {
        let client = reqwest::Client::new();
        let config = config::production();

        KrakenClient::new(client, config)
    };

    let asset_info = spot::market::AssetInfo::builder()
        .asset(["USDC", "USDT", "BTC"].map(ToString::to_string).to_vec())
        .build()
        .send(&client)
        .await?
        .into_payload();

    dbg!(asset_info);

    let asset_pairs = spot::market::AssetPairs::builder()
        .pair(["BTC/USD", "ETH/BTC"].map(ToString::to_string).to_vec())
        .build()
        .send(&client)
        .await?
        .into_payload();

    dbg!(asset_pairs);

    let ticker = spot::market::Ticker::builder()
        .pair("BTC/USD")
        .build()
        .send(&client)
        .await?
        .into_payload();

    dbg!(ticker);

    let depth = spot::market::Depth::builder()
        .count(5)
        .pair("BTC/USD")
        .build()
        .send(&client)
        .await?
        .into_payload();

    dbg!(depth);

    Ok(())
}
