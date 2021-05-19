use std::fmt;

use ccx_binance::Api;
use ccx_binance::LibError;
use ccx_binance::LibResult;

#[actix_rt::main]
async fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance = Api::from_env();

    let res = async move {
        println!("Running...");

        print_res(binance.ping().await)?;

        let time = print_res(binance.time().await)?.server_time;
        println!("Server Time: {}", time);

        let info = print_res(binance.exchange_info().await)?;
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
    .await;

    if let Err::<(), LibError>(e) = res {
        println!("Error: {}", e)
    }
}

fn print_res<T: fmt::Debug>(res: LibResult<T>) -> LibResult<T> {
    match &res {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    res
}
