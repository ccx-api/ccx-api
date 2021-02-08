#[allow(unused_imports)]
use ccx_binance::{client::ApiCred, Api, ChartInterval};
use ccx_binance::OrderBookLimit;

const SYMBOL: &str = "BNBBTC";

#[actix_rt::main]
async fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance = Api::with_cred(ApiCred::from_env());

    let future = binance.depth(SYMBOL, OrderBookLimit::N1000);
    // let future = binance.depth(SYMBOL, None);
    // let future = binance.trades(SYMBOL, None);
    // let future = binance.historical_trades(SYMBOL, None, None);
    // let future = binance.agg_trades(SYMBOL, None, None, None, None);
    // let future = binance.klines(SYMBOL, ChartInterval::Minute1, None, None, None);
    // let future = binance.avg_price(SYMBOL);
    // let future = binance.ticker_24hr(SYMBOL);
    // let future = binance.ticker_24hr_all();
    // let future = binance.ticker_price(SYMBOL);
    // let future = binance.ticker_price_all();
    // let future = binance.ticker_book(SYMBOL);
    // let future = binance.ticker_book_all();
    // let future = binance.user_data_stream();
    match future.await {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
}
