use std::fmt;

use ccx_mexc::MexcResult;
use ccx_mexc::Decimal;

pub fn print_res<T: fmt::Debug>(res: MexcResult<T>) -> MexcResult<T> {
    match &res {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    res
}

pub fn d(v: &'static str) -> Decimal {
    v.parse().unwrap()
}
