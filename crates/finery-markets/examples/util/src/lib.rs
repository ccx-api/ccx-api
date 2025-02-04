use std::fmt;

use ccx_finery_markets::LibResult;

pub fn print_res<T: fmt::Debug>(res: LibResult<T>) -> LibResult<T> {
    match &res {
        Ok(answer) => println!("Answer: {:#?}", answer),
        Err(e) => println!("Error: {:?}", e),
    }
    res
}
