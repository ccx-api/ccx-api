use ccx_mexc::api::spot::SpotApi;
use ccx_mexc::ApiCred;
use ccx_mexc::MexcResult;
use ccx_mexc::TimeWindow;
use ccx_mexc_examples_util::*;

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> MexcResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let mexc_spot = SpotApi::<ApiCred>::from_env();

    print_res(mexc_spot.all_coins_information(TimeWindow::now())?.await)?;
    Ok(())
}
