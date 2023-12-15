use ccx_api_lib::GateApiCred;
use ccx_gate::client::rest::RequestError;
use ccx_gate::GateApi;

#[actix_rt::main]
async fn main() -> Result<(), RequestError> {
    let _ = dotenvy::dotenv();

    env_logger::init();

    let api = GateApi::<GateApiCred>::from_env();

    // dbg!(api.wallet_balances(None).await)?;
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

    Ok(())
}
