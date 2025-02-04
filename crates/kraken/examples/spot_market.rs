// use ccx_kraken::api::spot::SpotApi;
use ccx_kraken::api::spot::SpotApi;
#[allow(unused_imports)]
use ccx_kraken::client::ApiCred;
use ccx_kraken::KrakenApiError;
use ccx_kraken::KrakenApiResult;
use ccx_kraken_examples_util::*;

// const BTCUSD: &str = "XXBTZUSD";
// const BNBUSD: &str = "BNBZUSD";

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> KrakenApiResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let kraken_spot = SpotApi::from_env();

    // print_res(kraken_spot.time().await)?;
    // print_res(kraken_spot.status().await)?;
    // print_res(kraken_spot.asset_info(None, None).await)?;
    let (pairs, _err) = print_res(kraken_spot.asset_pairs(None, None)?.await)?;
    for pair in pairs.pair.keys() {
        if pair.as_ref().contains("BNB") {
            println!("{}", pair);
        }
    }
    // print_res(kraken_spot.ticker(BNBUSD).await)?;
    KrakenApiError::ok(())
}
