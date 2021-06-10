use ccx_binance::SpotApi;
use ccx_binance::LibResult;
use ccx_binance::TimeWindow;
use ccx_binance_examples_util::*;

const EUR: &str = "EUR";

#[actix_rt::main]
async fn main() {
    main_().await.unwrap();
}

async fn main_() -> LibResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance_spot = SpotApi::from_env();

    let time = print_res(binance_spot.time().await)?;
    let init_time = time.server_time;

    let amount = d("1");

    let _balance = print_res(
        binance_spot
            .clearjunction_get_balance(EUR, TimeWindow::now())
            .await,
    )?;

    // Prevent an accidental withdrawal.
    if true {
        return Ok(());
    }

    let _withdraw = print_res(
        binance_spot
            .clearjunction_withdraw(EUR, amount, init_time, TimeWindow::now())
            .await,
    )?;

    Ok(())
}
