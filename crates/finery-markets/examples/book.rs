use ccx_finery_markets::ApiCred;
use ccx_finery_markets::LibResult;
use ccx_finery_markets::SpotApi;
use ccx_finery_markets::types::BookRequest;
use ccx_finery_markets::types::Nonce;
use ccx_finery_markets::types::Pair;
use ccx_finery_markets::types::Time;

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
    let request = BookRequest {
        instrument: Pair {
            base: "ETH".to_owned(),
            quote: "EUR".to_owned(),
        },
        tradable: false,
    };
    let book = finery_market.book(nonce, time, request).await?;
    println!("XXX {:?}", book);

    Ok(())
}
