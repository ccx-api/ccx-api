use ccx_api_lib::GatepayApiCred;
use ccx_gatepay::MerchantApi;

#[actix_rt::main]
async fn main() {
    let _ = dotenvy::dotenv();

    env_logger::init();

    let api = MerchantApi::<GatepayApiCred>::from_env();

    dbg!(api.balance().await).unwrap();
}
