use ccx_api_lib::GateApiCred;
use ccx_gate::api::wallet::WalletAccountEnum;
use ccx_gate::GateApi;
use rust_decimal_macros::dec;

#[actix_rt::main]
async fn main() {
    let _ = dotenvy::dotenv();

    env_logger::init();

    let api = GateApi::<GateApiCred>::from_env();

    // dbg!(api.wallet_balances(None).await).unwrap();
    dbg!(
        api.wallet_transfer(
            "USDT".into(),
            WalletAccountEnum::Payment,
            WalletAccountEnum::Spot,
            dec!(0.01),
            None,
            None,
        )
        .await
    )
    .unwrap();
}
