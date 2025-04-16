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

    let server_time = spot::market::ServerTime.send(&client).await?.into_payload();

    dbg!(server_time);

    let system_status = spot::market::SystemStatus
        .send(&client)
        .await?
        .into_payload();

    dbg!(system_status);

    Ok(())
}
