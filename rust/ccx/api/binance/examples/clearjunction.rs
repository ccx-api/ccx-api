use std::fmt;

use ccx_binance::client::Config;
use ccx_binance::Api;
use ccx_binance::Decimal;
use ccx_binance::LibResult;
use ccx_binance::TimeWindow;

const EUR: &str = "EUR";

#[actix_rt::main]
async fn main() {
    main_().await.unwrap();
}

async fn main_() -> LibResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance = Api::with_config(Config::from_env());

    let time = print_res(binance.time().await)?;
    let init_time = time.server_time;

    let amount = d("1");

    let _withdraw = print_res(
        binance
            .clearjunction_withdraw(EUR, amount, init_time, TimeWindow::now())
            .await,
    )?;

    Ok(())
}

fn print_res<T: fmt::Debug>(res: LibResult<T>) -> LibResult<T> {
    match &res {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    res
}

fn d(v: &'static str) -> Decimal {
    v.parse().unwrap()
}
