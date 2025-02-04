use std::fmt;

use ccx_binance::BinanceResult;
use ccx_binance::Decimal;

pub fn print_res<T: fmt::Debug>(res: BinanceResult<T>) -> BinanceResult<T> {
    match &res {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    res
}

pub fn d(v: &'static str) -> Decimal {
    v.parse().unwrap()
}
