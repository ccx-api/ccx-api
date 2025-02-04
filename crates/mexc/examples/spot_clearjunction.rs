use ccx_mexc::ApiCred;
use ccx_mexc::MexcResult;
use ccx_mexc::SpotApi;
use ccx_mexc::TimeWindow;
use ccx_mexc_examples_util::*;

const EUR: &str = "EUR";

#[actix_rt::main]
async fn main() {
    main_().await.unwrap();
}

async fn main_() -> MexcResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let mexc_spot = SpotApi::<ApiCred>::from_env();

    let time = print_res(mexc_spot.time()?.await)?;
    let init_time = time.server_time;

    let amount = d("1");

    let _balance = print_res(
        mexc_spot
            .clearjunction_get_balance(EUR, TimeWindow::now())?
            .await,
    )?;

    // Prevent an accidental withdrawal.
    if true {
        return Ok(());
    }

    let _withdraw = print_res(
        mexc_spot
            .clearjunction_withdraw(EUR, amount, init_time, TimeWindow::now())?
            .await,
    )?;

    Ok(())
}
