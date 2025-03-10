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
        let config = config::production();

        GateClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    let all_currencies = spot::AllCurrencies
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    println!("len of all currencies: {}", all_currencies.len());

    let btc_currency = spot::Currency::new("BTC")
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(btc_currency);

    let all_currency_pairs = spot::AllCurrencyPairs
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    println!("len of all currency pairs: {}", all_currency_pairs.len());

    let currency_pair = spot::CurrencyPair::new("BTC", "USDT")
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(currency_pair);

    let currency_pair = spot::CurrencyPair::new("TRX", "USDT")
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(currency_pair);

    let currency_pair = spot::CurrencyPair::new("TRX", "USDT")
        .throttle(&rate_limiter)
        .await?
        .send(&client)
        .await?
        .into_payload();

    dbg!(currency_pair);

    Ok(())
}
