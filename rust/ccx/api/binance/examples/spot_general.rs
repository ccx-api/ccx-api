use ccx_binance::BinanceResult;
use ccx_binance::SpotApi;
use ccx_binance_examples_util::*;

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> BinanceResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance_spot = SpotApi::from_env();

    println!("Running...");

    print_res(binance_spot.ping().await)?;

    let time = print_res(binance_spot.time().await)?.server_time;
    println!("Server Time: {}", time);

    let info = print_res(binance_spot.exchange_info().await)?;
    for symbol in info.symbols {
        if &symbol.base_asset != "BTC"
            || &symbol.quote_asset != "USDT" && &symbol.quote_asset != "EUR"
        {
            continue;
        }
        println!("{}/{}", symbol.base_asset, symbol.quote_asset);
        println!("base precision: {}", symbol.base_asset_precision);
        println!("quote precision: {}", symbol.quote_asset_precision);
    }

    Ok(())
}
