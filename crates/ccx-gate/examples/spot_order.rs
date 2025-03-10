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

    let order_book = spot::OrderBook::currency_pair("ETH_BTC")
        .send(&client)
        .await?
        .into_payload();

    dbg!(order_book);

    let list_orders = spot::ListOrders::new("BTC_USDT", spot::OrderStatus::Open)
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(list_orders);

    let order_currency_pair = "BTC_USDT";

    // WARNING: here we create an actual exchange order for 3 USDT.
    // If you lucky enough you may acquire 3 BTC with it, so
    // why not to try anyway, right? It'll cancelled in the code later
    let create_order = spot::CreateOrder::builder()
        .currency_pair(order_currency_pair)
        .order_type(spot::OrderType::Limit)
        .side(spot::OrderSide::Buy)
        .amount(1.into())
        .price(3.into())
        .time_in_force(spot::TimeInForce::ImmediateOrCancelled)
        .build()
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&create_order);

    let list_orders = spot::ListOrders::new(order_currency_pair, spot::OrderStatus::Open)
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(list_orders);

    let get_order = spot::GetOrder::new(&create_order.id, order_currency_pair)
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(get_order);

    let cancel_order = spot::CancelOrder::new(&create_order.id, order_currency_pair)
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(cancel_order);

    let list_orders = spot::ListOrders::new(order_currency_pair, spot::OrderStatus::Open)
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(list_orders);

    Ok(())
}
