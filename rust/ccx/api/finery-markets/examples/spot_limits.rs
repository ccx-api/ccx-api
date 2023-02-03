use ccx_finery_markets::types::LimitsRequest;
use ccx_finery_markets::types::Nonce;
use ccx_finery_markets::types::Time;
use ccx_finery_markets::ApiCred;
use ccx_finery_markets::LibResult;
use ccx_finery_markets::SpotApi;

#[actix_rt::main]
async fn main() {
    if let Err(e) = main_().await {
        log::error!("main error: {:?}", e);
    }
}

async fn main_() -> LibResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let finery_market = SpotApi::<ApiCred>::from_env();
    let time = Time::now();
    let nonce = Nonce::from(time);
    let limits = finery_market.limits(nonce, time, LimitsRequest {}).await?;
    println!("XXX {:?}", limits);

    Ok(())
}
