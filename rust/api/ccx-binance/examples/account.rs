use ccx_binance::spot::prelude::*;
use envconfig::Envconfig;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_BINANCE_SPOT_KEY_NAME", default = "default")]
    key_name: String,
    #[envconfig(from = "EXAMPLE_BINANCE_SPOT_API_KEY")]
    api_key: String,
    #[envconfig(from = "EXAMPLE_BINANCE_SPOT_API_SECRET")]
    api_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    let credential = {
        let config = match EnvConfig::init_from_env() {
            Ok(config) => config,
            Err(err) => {
                println!("{err}");
                std::process::exit(1);
            }
        };
        BinanceSpotCredential::new(
            config.key_name,
            config.api_key,
            config.api_secret.as_bytes(),
        )?
    };

    let spot = {
        let client = reqwest::Client::new();
        let config = spot::config::production();
        BinanceSpotClient::new(client, config)
    };
    let rate_limiter = RateLimiter::spawn();

    let account_info = spot::api::GetAccountInfo::with_omit_zero_balances(true)
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &spot)
        .await?;
    println!("{:#?}", account_info);

    // let my_trades = spot::api::GetAccountTradeList::new("ADAUSDT".into())
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &spot)
    //     .await?;
    // println!("{:#?}", my_trades);

    // let unfilled_order_limits = spot::api::GetUnfilledOrderCount::new()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &spot)
    //     .await?;
    // println!("{:#?}", unfilled_order_limits);

    // let commission_rates = spot::api::GetCommissionRates::new("ADAUSDT".into())
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &spot)
    //     .await?;
    // println!("{:#?}", commission_rates);

    Ok(())
}
