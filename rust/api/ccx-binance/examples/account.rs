use ccx_binance::spot;
use ccx_binance::spot::client::BinanceSpotClient;
use ccx_binance::spot::client::BinanceSpotCredential;
use ccx_binance::spot::proto::BinanceSpotSigned;
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
async fn main() {
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
        )
        .unwrap()
    };

    let spot = {
        let client = reqwest::Client::new();
        let config = spot::config::production();
        BinanceSpotClient::new(client, config)
    };

    let account_info = spot::api::GetAccountInfo::with_omit_zero_balances(true)
        .sign_now_and_send(&credential, &spot)
        .await
        .unwrap();
    println!("{:#?}", account_info);
}
