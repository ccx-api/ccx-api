use ccx_mexc::api::spot::SpotApi;
use ccx_mexc::ApiCred;
use ccx_mexc::MexcResult;
use ccx_mexc::TimeWindow;
use ccx_mexc_examples_util::*;

// prevent some operations that could affect balance
const ENABLE_BALANCE_OPERATION: bool = false;

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> MexcResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let mexc_spot = SpotApi::<ApiCred>::from_env();

    print_res(mexc_spot.all_coins_information(TimeWindow::now())?.await)?;

    if ENABLE_BALANCE_OPERATION {
        let _ = print_res(
            mexc_spot
                .withdraw(
                    "ETH",
                    None::<&str>,
                    Some("ETH"),
                    None::<&str>,
                    "0x822330d165d511a855d474010a629a08b6fe1e8c",
                    None::<&str>,
                    200.into(),
                    None::<&str>,
                    TimeWindow::now(),
                )?
                .await,
        );
    }

    print_res(
        mexc_spot
            .deposit_history(None::<&str>, None, None, None, None, TimeWindow::now())?
            .await,
    )?;

    print_res(
        mexc_spot
            .withdraw_history(None::<&str>, None, None, None, None, TimeWindow::now())?
            .await,
    )?;

    print_res(
        mexc_spot
            .get_deposit_address("ETH", "ETH", TimeWindow::now())?
            .await,
    )?;

    Ok(())
}
