use std::time::Instant;

use ccx_binance::Api;
use ccx_binance::LibError;

#[actix_rt::main]
async fn main() {
    let _ = dotenv::dotenv();
    env_logger::init();

    let binance = Api::new();

    let ts = Instant::now();

    let res = async move {
        println!("Running...");

        let ping = binance.ping().await?;
        let ping_el = ts.elapsed();
        println!("{:?};  elapsed {} ms", ping, ping_el.as_millis());

        let time = binance.time().await?.server_time;
        let time_el = ts.elapsed() - ping_el;
        println!("Server Time: {};  elapsed {} ms", time, time_el.as_millis());

        let info = binance.exchange_info().await?;
        let info_el = ts.elapsed() - time_el;
        println!(
            "Exchenge Info: {:#?};\nelapsed {} ms",
            info,
            info_el.as_millis()
        );

        Ok(())
    }
    .await;
    let el = ts.elapsed();
    println!("\nTotal elapsed {} ms", el.as_millis());

    if let Err::<(), LibError>(e) = res {
        println!("Error: {}", e)
    }
}
