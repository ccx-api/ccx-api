use std::fmt;

use ccx_bitstamp::BitstampApiResult;
use ccx_bitstamp::Decimal;

pub fn print_res<T: fmt::Debug>(res: BitstampApiResult<T>) -> BitstampApiResult<T> {
    match &res {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    res
}

pub fn d(v: &'static str) -> Decimal {
    v.parse().unwrap()
}
