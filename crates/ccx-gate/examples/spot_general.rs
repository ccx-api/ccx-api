use ccx_gate::prelude::*;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_gate=debug,ccx_lib=trace".into()),
        )
        .init();

    let client = {
        let client = reqwest::Client::new();
        let config = config::testing();

        GateClient::new(client, config)
    };

    let server_time = spot::GetServerTime.send(&client).await?.into_payload();

    dbg!(server_time);

    Ok(())
}
