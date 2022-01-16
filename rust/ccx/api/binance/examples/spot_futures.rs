use ccx_binance::BinanceResult;
use ccx_binance::SpotApi;
use ccx_binance::TimeWindow;
use ccx_binance_examples_util::*;

// const EUR: &str = "EUR";
// const USDT: &str = "USDT";

#[actix_rt::main]
async fn main() {
    main_().await.unwrap();
}

async fn main_() -> BinanceResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance_spot = SpotApi::from_env();

    // let time = print_res(binance_spot.time().await)?;
    // let start_time = time.server_time - 365 * 24 * 3600 * 1000;

    // let amount = d("1");

    let _cross_collateral_info = print_res(
        binance_spot
            .futures_cross_collateral_info_v2(None::<&str>, None::<&str>, TimeWindow::now())
            .await,
    )?;

    // let _balance = print_res(
    //     binance_spot
    //         .futures_transfer_history(USDT, start_time, None, None, Some(100), TimeWindow::now())
    //         .await,
    // )?;

    // Prevent an accidental action with money.
    if true {
        return Ok(());
    }

    // let _withdraw = print_res(
    //     binance_spot
    //         .clearjunction_withdraw(EUR, amount, init_time, TimeWindow::now())
    //         .await,
    // )?;

    Ok(())
}
