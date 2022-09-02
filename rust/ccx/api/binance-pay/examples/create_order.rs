#![allow(dead_code, unused_variables)]

use ccx_api_lib::ApiCred;
use ccx_binance_pay::Api;
use ccx_binance_pay::Buyer;
use ccx_binance_pay::LibError;
use ccx_binance_pay::Merchant;
use ccx_binance_pay::OrderEnv;
use ccx_binance_pay::OrderGoods;
use ccx_binance_pay::Shipping;
use ccx_binance_pay::TerminalType;
use ccx_binance_pay::Time;
use ccx_binance_pay::TradeType;
use ccx_binance_pay::V1CreateOrderRequest;
use ccx_binance_pay::V2CreateOrderRequest;
use ccx_binance_pay_examples_util::*;

use url::Url;
use uuid::Uuid;

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
    let auction_id = Uuid::new_v4();
    let asset_code = "USDT".to_string();
    let request = V1CreateOrderRequest {
        merchant_id,
        sub_merchant_id: Some(merchant_id),
        merchant_trade_no: auction_id,
        trade_type: TradeType::Web,
        total_fee: d("10.1"),
        currency: asset_code,
        product_type: String::new(),
        product_name: String::new(),
        product_detail: None,
        return_url: None,
    };
    let _response = print_res(api.v1_create_order(request, Time::now()).await)?;

    Ok(())
}

async fn main_v2() -> Result<(), LibError> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let api = Api::<ApiCred>::from_env();

    // let merchant_id = api.merchant_id();
    let auction_id = Uuid::new_v4();
    let asset_code = "USDT".to_string();
    let request = V2CreateOrderRequest {
        merchant: None::<Merchant>,
        order_env: OrderEnv::from(TerminalType::Web),
        merchant_trade_no: auction_id,
        order_amount: d("10.1"),
        currency: asset_code,
        goods: OrderGoods::default(),
        shipping: None::<Shipping>,
        buyer: None::<Buyer>,
        return_url: None::<Url>,
        cancel_url: None::<Url>,
        order_expire_time: None::<i64>,
        support_pay_currency: None::<String>,
        app_id: None::<String>,
        universal_url_attach: None::<Url>,
        pass_through_info: None::<String>,
        webhook_url: None::<Url>,
    };
    let _response = print_res(api.v2_create_order(request, Time::now()).await)?;

    Ok(())
}
