#![allow(dead_code, unused_variables)]

use ccx_api_lib::ApiCred;
use ccx_binance_pay::Api;
use ccx_binance_pay::LibError;
use ccx_binance_pay::Time;
use ccx_binance_pay::V1QueryOrderRequest;
use ccx_binance_pay::V2QueryOrderRequest;
use ccx_binance_pay_examples_util::*;

#[actix_rt::main]
async fn main() {
    // let _ = main_v1().await;
    let _ = main_v2().await;
}

async fn main_v1() -> Result<(), LibError> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let api = Api::<ApiCred>::from_env();

    let merchant_id = api.merchant_id();
    let merchant_trade_no = None;
    let prepay_id = Some("180958457718611968".to_string());
    let request = V1QueryOrderRequest {
        merchant_id,
        sub_merchant_id: None,
        merchant_trade_no,
        prepay_id,
    };
    let _response = print_res(api.v1_query_order(request, Time::now()).await)?;

    Ok(())
}

async fn main_v2() -> Result<(), LibError> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let api = Api::<ApiCred>::from_env();

    let merchant_id = api.merchant_id();
    let merchant_trade_no = None;
    let prepay_id = Some("180959074046418944".to_string());
    let request = V2QueryOrderRequest {
        merchant_trade_no,
        prepay_id,
    };
    let _response = print_res(api.v2_query_order(request, Time::now()).await)?;

    Ok(())
}
