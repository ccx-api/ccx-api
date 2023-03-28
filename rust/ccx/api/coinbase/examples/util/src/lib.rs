use std::fmt;

use ccx_coinbase::CoinbaseApiResult;
use ccx_coinbase::Decimal;

pub fn print_res<T: fmt::Debug>(res: CoinbaseApiResult<T>) -> CoinbaseApiResult<T> {
    match &res {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    res
}

pub fn d(v: &'static str) -> Decimal {
    v.parse().unwrap()
}
