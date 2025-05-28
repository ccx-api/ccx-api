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

    let deposit_methods = spot::funding::DepositMethods::builder()
        .asset("BTC")
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(deposit_methods);

    let deposit_addresses = spot::funding::DepositAddresses::builder()
        .asset("BTC")
        .method("Bitcoin")
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(deposit_addresses);

    let deposit_statuses = spot::funding::DepositStatus::builder()
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(deposit_statuses);

    let withdrawal_methods = spot::funding::WithdrawalMethods::builder()
        .asset("BTC")
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(withdrawal_methods);

    let withdrawal_addresses = spot::funding::WithdrawalAddresses::builder()
        .asset("BTC")
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(withdrawal_addresses);

    // TODO: test this
    // let withdrawal_information = spot::funding::WithdrawalInformation::builder()
    //     .asset("BTC")
    //     .key("test_withdrawal")
    //     .amount("100")
    //     .build()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(withdrawal_information);

    // let withdraw = spot::funding::WithdrawFunds::builder()
    //     .asset("BTC")
    //     .key("test")
    //     .amount("1")
    //     .build()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(&withdraw);

    // let withdrawal_cancel = spot::funding::WithdrawalCancel::builder()
    //     .asset("BTC")
    //     .refid(withdraw.refid)
    //     .build()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(withdrawal_cancel);

    // let wallet_transfer = spot::funding::WalletTransfer::builder()
    //     .asset("BTC")
    //     .amount("0.0000001")
    //     .build()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(wallet_transfer);

    let withdrawal_statuses = spot::funding::WithdrawalStatus::builder()
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(withdrawal_statuses);

    Ok(())
}
