use ccx_kraken::api::spot::trading::{CancelOrderId, OrderSide, OrderType};
use ccx_kraken::prelude::*;
use ccx_kraken::rate_limiter::Tier;
use ccx_kraken::types::trading::OrderParams;
use envconfig::Envconfig;
use rust_decimal_macros::dec;
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

    let order = spot::trading::AddOrder::builder()
        .pair("USDC/EUR")
        .params(
            OrderParams::builder()
                .volume(dec!(0.01))
                .side(OrderSide::Buy)
                .ordertype(OrderType::Market)
                .build(),
        )
        .validate(true)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&order);

    let query_info = spot::account_data::QueryOrderInfo::builder()
        .txid(order.txid.clone())
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(query_info);

    let open_orders = spot::account_data::OpenOrders::builder()
        .trades(false)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(open_orders);

    let cancel_order = spot::trading::CancelOrder::builder()
        .id(CancelOrderId::TxId(order.txid.first().unwrap().clone()))
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(cancel_order);

    let closed_orders = spot::account_data::ClosedOrders::builder()
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(closed_orders);

    Ok(())
}
