use std::collections::HashMap;

use ccx_api_lib::dec;
use ccx_api_lib::env_logger_util::log_format_clean;
use ccx_api_lib::Atom;
use ccx_coinbase::api::prime::types::*;
use ccx_coinbase::api::prime::*;
use ccx_coinbase::CoinbaseResult;
use uuid1::Uuid;

#[actix_rt::main]
async fn main() {
    if let Err(err) = main_().await {
        println!("{:?}", err);
    };
}

async fn main_() -> CoinbaseResult<()> {
    let _ = dotenv::dotenv();
    env_logger::Builder::from_default_env()
        .format(log_format_clean)
        .init();

    let cb_prime = PrimeApi::from_env_with_prefix("PRIME_API");

    // let portfolio_id = uuid!("xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx");
    let portfolios = dbg!(cb_prime.get_portfolios()?.await)?;
    let portfolio = portfolios.portfolios.first().unwrap();
    let portfolio_id = portfolio.id;

    let portfolio_wallets = dbg!(
        cb_prime
            .get_wallets(
                portfolio_id,
                PortfolioWalletType::Trading,
                &[],
                Page::default(),
            )?
            .await,
    )?;
    assert!(portfolio_wallets.pagination.next().is_none());
    let wallets: HashMap<Atom, AccountPortfolioWallet> = portfolio_wallets
        .wallets
        .into_iter()
        .map(|w| (w.symbol.clone(), w))
        .collect();

    let source_symbol = "USD".into();
    let source_wallet_id = wallets.get(&source_symbol).unwrap().id;
    let destination_symbol = "USDC".into();
    let destination_wallet_id = wallets.get(&destination_symbol).unwrap().id;

    let amount = dec!(10.0);

    let conversion_idempotency_key = Uuid::new_v4().to_string();
    println!("\n==============================================================================\n");
    println!("  portfolio_id:               {}", portfolio_id);
    println!();
    println!(
        "  source_wallet_id:           {} ({})",
        source_wallet_id, source_symbol
    );
    println!(
        "  destination_wallet_id:      {} ({})",
        destination_wallet_id, destination_symbol
    );
    println!();
    println!(
        "  conversion amount:          {} {} -> {}",
        amount, source_symbol, destination_symbol
    );
    println!();
    println!(
        "  conversion_idempotency_key: {}",
        conversion_idempotency_key
    );
    println!("\n==============================================================================\n");

    let _conversion = dbg!(
        cb_prime
            .create_conversion(
                portfolio_id,
                &conversion_idempotency_key,
                source_wallet_id,
                &source_symbol,
                amount,
                destination_wallet_id,
                &destination_symbol,
            )?
            .await,
    )?;

    Ok(())
}
