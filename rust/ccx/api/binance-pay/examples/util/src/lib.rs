use std::fmt;

use ccx_binance_pay::Decimal;
use ccx_binance_pay::LibError;
use ccx_binance_pay::Uuid;

pub fn print_res<T: fmt::Debug>(res: Result<T, LibError>) -> Result<T, LibError> {
    match &res {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    res
}

pub fn d(v: &'static str) -> Decimal {
    v.parse().unwrap()
}

pub fn uuid(v: &'static str) -> Uuid {
    v.parse().unwrap()
}
