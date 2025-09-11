use ccx_gate::GateApi;
use ccx_gate::util::GateApiCred;

#[actix_rt::main]
async fn main() {
    let _ = dotenvy::dotenv();

    env_logger::init();

    let api = GateApi::<GateApiCred>::from_env();

    dbg!(api.spot().tickers(&Default::default()).await).unwrap();
}
