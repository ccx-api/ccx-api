use ccx_binance::spot;
use ccx_binance::spot::client::BinanceSpotClient;
use ccx_binance::spot::proto::BinanceSpotPublic;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let config = spot::config::production();
    let spot_client = BinanceSpotClient::new(client, config);

    let pong = spot::api::Ping::new().send(&spot_client).await.unwrap();
    println!("{:?}", pong);

    // let server_time = spot::api::GetServerTime::new()
    //     .send(&spot_client)
    //     .await
    //     .unwrap();
    // println!("{:?}", server_time);

    // let exchange_info = spot::api::GetExchangeInfo::new()
    //     .send(&spot_client)
    //     .await
    //     .unwrap();
    // for symbol in exchange_info.symbols.iter().take(3) {
    //     println!("{:#?}", symbol);
    // }
    // for rate_limit in &exchange_info.rate_limits {
    //     println!("{:#?}", rate_limit);
    // }
    // for filter in exchange_info.exchange_filters.iter().take(3) {
    //     println!("{:#?}", filter);
    // }

    // let exchange_info = spot::api::GetExchangeInfo::with_symbols(&["BNBBTC", "BTCUSDT"])
    //     .send(&spot_client)
    //     .await
    //     .unwrap();
    // println!("{:#?}", exchange_info);

    // let exchange_info = spot::api::GetExchangeInfo::with_symbol("BNBBTC")
    //     .send(&spot_client)
    //     .await
    //     .unwrap();
    // println!("{:#?}", exchange_info);
}
