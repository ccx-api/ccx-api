use ccx_api_lib::GateApiCred;
use ccx_gate::client::rest::RequestError;
use ccx_gate::GateApi;
// use rust_decimal_macros::dec;

#[actix_rt::main]
async fn main() -> Result<(), RequestError> {
    let _ = dotenvy::dotenv();

    env_logger::init();

    let api = GateApi::<GateApiCred>::from_env();

    dbg!(api.wallet_balances(None).await)?;
    // Currently is not working. The only way to transfer funds is to use the website.
    // dbg!(
    //     api.wallet_transfer(
    //         "USDT".into(),
    //         WalletAccountEnum::Payment,
    //         WalletAccountEnum::Spot,
    //         dec!(0.01),
    //         None,
    //         None,
    //     )
    //     .await
    // )?;
    dbg!(
        api.wallet_withdrawal_history(None, None, None, None, None)
            .await
    )?;
    // dbg!(
    //     api.withdrawal_withdraw(
    //         Some("client id".into()),
    //         dec!(2.63),
    //         "USDT".into(),
    //         Some("Txxx ... wallet address here ...".into()),
    //         None,
    //         "TRX".into(),
    //     )
    //     .await
    // )?;

    Ok(())
}
