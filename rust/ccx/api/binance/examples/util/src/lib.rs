use std::fmt;

use ccx_binance::Decimal;
use ccx_binance::LibResult;

pub fn print_res<T: fmt::Debug>(res: LibResult<T>) -> LibResult<T> {
    match &res {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    res
}

pub fn d(v: &'static str) -> Decimal {
    v.parse().unwrap()
}
