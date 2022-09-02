#![allow(dead_code, unused_variables)]

use ccx_api_lib::ApiCred;
use ccx_binance_pay::Api;
use ccx_binance_pay::LibError;
use ccx_binance_pay::Time;
use ccx_binance_pay::V1CertificateRequest;
use ccx_binance_pay::V2CertificateRequest;
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
    let request = V1CertificateRequest { merchant_id };
    let _response = print_res(api.v1_certificates(request, Time::now()).await)?;

    Ok(())
}

async fn main_v2() -> Result<(), LibError> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let api = Api::<ApiCred>::from_env();

    let request = V2CertificateRequest;
    let _response = print_res(api.v2_certificates(request, Time::now()).await)?;

    Ok(())
}
