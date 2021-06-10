use ccx_binance::api::spot::SpotApi;
use ccx_binance::{LibResult, TimeWindow};
use ccx_binance_examples_util::*;

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> LibResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance_spot = SpotApi::from_env();

    print_res(binance_spot.all_coins_information(TimeWindow::now()).await)?;
    Ok(())
}
