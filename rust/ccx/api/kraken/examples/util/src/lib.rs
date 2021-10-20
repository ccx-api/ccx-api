use std::fmt;

use ccx_kraken::Decimal;
use ccx_kraken::KrakenApiResult;

pub fn print_res<T: fmt::Debug>(res: KrakenApiResult<T>) -> KrakenApiResult<T> {
    match &res {
        Ok((answer, warnings)) => {
            println!("Answer: {:#?}", answer);
            println!("Warnings: {:#?}", warnings);
        },
        Err(e) => println!("Error: {:?}", e),
    }
    res
}

pub fn d(v: &'static str) -> Decimal {
    v.parse().unwrap()
}
