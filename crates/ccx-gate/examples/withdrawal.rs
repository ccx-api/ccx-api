use ccx_gate::prelude::*;
use envconfig::Envconfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_GATE_API_KEY")]
    api_key: String,
    #[envconfig(from = "EXAMPLE_GATE_API_SECRET")]
    api_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_gate=debug,ccx_lib=trace".into()),
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
        GateCredential::new(config.api_key, config.api_secret)
    };

    let client = {
        let client = reqwest::Client::new();
        let config = config::production();
        GateClient::new(client, config)
    };

    let withdraw = withdrawal::Withdraw::builder()
        .currency("USDT".into())
        .address("0xBF182Ff3aAf061779fEfd2452bAEE2F4Eca3bD50".into())
        .amount(1.into())
        .chain("ETH".into())
        .build()
        .sign_now_and_send(&credential, &client)
        .await?;

    dbg!(withdraw);

    Ok(())
}
