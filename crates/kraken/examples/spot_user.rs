use ccx_kraken::KrakenApiError;
use ccx_kraken::KrakenApiResult;
use ccx_kraken::api::spot::SpotApi;
#[allow(unused_imports)]
use ccx_kraken::client::ApiCred;
use ccx_kraken::client::NonceSeq;
use ccx_kraken_examples_util::*;

// const SYMBOL: &str = "XXBTZUSD";

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> KrakenApiResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let mut seq = NonceSeq::new();

    let kraken_spot = SpotApi::from_env();

    print_res(kraken_spot.get_account_balance(seq.ts_next())?.await)?;
    KrakenApiError::ok(())
}
