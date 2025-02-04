use ccx_mexc::api::spot::SpotApi;
use ccx_mexc::ApiCred;
use ccx_mexc::BinanceResult;
use ccx_mexc::TimeWindow;
use ccx_mexc_examples_util::*;

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> BinanceResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance_spot = SpotApi::<ApiCred>::from_env();

    print_res(binance_spot.all_coins_information(TimeWindow::now())?.await)?;
    Ok(())
}
