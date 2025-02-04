use ccx_gatepay::util::GatepayApiCred;
use ccx_gatepay::GatepayApi;

#[actix_rt::main]
async fn main() {
    let _ = dotenvy::dotenv();

    env_logger::init();

    let api = GatepayApi::<GatepayApiCred>::from_env();

    dbg!(api.balance().await).unwrap();
}
