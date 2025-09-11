use ccx_bitstamp::ApiCred;
use ccx_bitstamp::BitstampResult;
use ccx_bitstamp::api::Api as BitstampApi;
use ccx_bitstamp_examples_util::*;

#[actix_rt::main]
async fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    if let Err(e) = main_bitstamp().await {
        log::error!("bitstamp err: {:?}", e);
    }
}

async fn main_bitstamp() -> BitstampResult<()> {
    #[allow(unused_variables)]
    let bitstamp = BitstampApi::<ApiCred>::from_env();

    let res = bitstamp.get_order_book("btcusdt", None)?.await;
    let _ = print_res(res);

    Ok(())
}
