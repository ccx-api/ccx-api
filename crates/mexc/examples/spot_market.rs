#[allow(unused_imports)]
use ccx_mexc::ChartInterval;
use ccx_mexc::MexcResult;
use ccx_mexc::api::spot::SpotApi;
use ccx_mexc::client::ApiCred;
use ccx_mexc_examples_util::*;

const SYMBOL: &str = "BTCUSDT";

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> MexcResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let mexc_spot = SpotApi::<ApiCred>::from_env();

    print_res(mexc_spot.depth(SYMBOL, Some(10))?.await)?;
    print_res(mexc_spot.depth(SYMBOL, None)?.await)?;
    print_res(mexc_spot.trades(SYMBOL, None)?.await)?;
    print_res(mexc_spot.historical_trades(SYMBOL, None)?.await)?;
    print_res(mexc_spot.agg_trades(SYMBOL, None, None, None)?.await)?;
    print_res(
        mexc_spot
            .klines(SYMBOL, ChartInterval::Minute1, None, None, None)?
            .await,
    )?;
    print_res(mexc_spot.avg_price(SYMBOL)?.await)?;
    print_res(mexc_spot.ticker_24hr(SYMBOL)?.await)?;
    print_res(mexc_spot.ticker_24hr_all()?.await)?;
    print_res(mexc_spot.ticker_price(SYMBOL)?.await)?;
    print_res(mexc_spot.ticker_price_all()?.await)?;
    print_res(mexc_spot.ticker_book(SYMBOL)?.await)?;
    print_res(mexc_spot.ticker_book_all()?.await)?;
    // print_res(mexc_spot.user_data_stream()?.await)?;
    Ok(())
}
