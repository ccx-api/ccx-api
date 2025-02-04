#![allow(dead_code, unused_variables)]

use ccx_api_lib::ApiCred;
use ccx_binance_pay::Api;
use ccx_binance_pay::LibError;
use ccx_binance_pay::Time;
use ccx_binance_pay::TransferType;
use ccx_binance_pay::V1TransferFundRequest;
use ccx_binance_pay::V2TransferFundRequest;
use ccx_binance_pay_examples_util::*;
use uuid::Uuid;

#[actix_rt::main]
async fn main() {
    let _ = main_v1().await;
    // let _ = main_v2().await;
}

async fn main_v1() -> Result<(), LibError> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let api = Api::<ApiCred>::from_env();

    let merchant_id = api.merchant_id();
    let request_id = Uuid::new_v4();
    let asset_code = "BUSD".to_string();
    let request = V1TransferFundRequest {
        request_id,
        merchant_id: merchant_id,
        currency: asset_code,
        amount: d("4"),
        transfer_type: TransferType::ToMain,
    };
    let _response = print_res(api.v1_transfer_fund(request, Time::now()).await)?;

    Ok(())
}

async fn main_v2() -> Result<(), LibError> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let api = Api::<ApiCred>::from_env();

    let request_id = Uuid::new_v4();
    let asset_code = "BUSD".to_string();
    let request = V2TransferFundRequest {
        request_id,
        currency: asset_code,
        amount: d("4"),
        transfer_type: TransferType::ToMain,
    };
    // let _response = print_res(api.v2_transfer_fund(request, Time::now()).await)?;

    Ok(())
}
