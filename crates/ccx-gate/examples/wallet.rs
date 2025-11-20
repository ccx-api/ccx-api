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
                .unwrap_or_else(|_err| "info,ccx_gate=trace,ccx_lib=trace".into()),
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
        let config = config::testing();
        GateClient::new(client, config)
    };

    let balances = wallet::Balances::default()
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(balances);

    let history = wallet::WithdrawalHistory::default()
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(history);

    let deposit_address = wallet::DepositAddress::new("USDT")
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(deposit_address);

    let deposit_history = wallet::DepositHistory::builder()
        .currency("USDT")
        .build()
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(deposit_history);

    // This doesn't work in testing environment
    // let withdraw_status = wallet::WithdrawStatus::with_currency("ETH")
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(withdraw_status);

    Ok(())
}
