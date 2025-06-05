use ccx_bitgo::prelude::*;
use envconfig::Envconfig;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

#[derive(Debug, Envconfig)]
struct EnvConfig {
    #[envconfig(from = "EXAMPLE_BITGO_EXPRESS_URL")]
    express_url: Url,
    #[envconfig(from = "EXAMPLE_BITGO_API_TOKEN")]
    api_token: String,
    #[envconfig(from = "EXAMPLE_BITGO_GO_WALLET_ID")]
    go_wallet_id: String,
    #[envconfig(from = "EXAMPLE_BITGO_GO_WALLET_PASSPHRASE")]
    go_wallet_passphrase: String,
    #[envconfig(from = "EXAMPLE_BITGO_HOT_WALLET_ID")]
    hot_wallet_id: String,
    #[envconfig(from = "EXAMPLE_BITGO_HOT_WALLET_ADDRESS")]
    hot_wallet_address: String,
    #[envconfig(from = "EXAMPLE_BITGO_HOT_WALLET_PASSPHRASE")]
    hot_wallet_passphrase: String,
    // #[envconfig(from = "EXAMPLE_BITGO_HOT_WALLET_PRIVATE_KEY")]
    // hot_wallet_private_key: String,
    #[envconfig(from = "EXAMPLE_BITGO_RECIPIENT_ADDRESS")]
    recipient_address: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_err| "info,ccx_bitgo=trace,ccx_lib=trace".into()),
        )
        .init();

    let config = EnvConfig::init_from_env()?;
    let credential = BitGoCredential::new(config.api_token);
    let rate_limiter = RateLimiter::spawn();

    let client = {
        let client = reqwest::Client::new();
        let config = config::ConnectionConfig::new(config.express_url);

        BitGoClient::new(client, config)
    };

    let wallets = wallet::ListWallets::builder()
        .limit(10u32)
        .expand_balance(true)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&wallets);

    // Get wallet by ID (without requiring coin parameter)
    let wallet_by_id = wallet::GetWalletById::builder()
        .wallet_id(config.hot_wallet_id.clone())
        .include_balance(true) // Include wallet balance
        .expand_balance(true) // Include detailed balance information
        .include_staking_balances(true) // Include staking balances if available
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&wallet_by_id);

    let wallet_info = wallet::GetWalletByIdCoin::builder()
        .coin("tsol:usdcv2")
        .wallet_id(config.hot_wallet_id.clone())
        .include_balance(true) // Include wallet balance
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&wallet_info);

    // Get total balances across all wallets
    let total_balances = wallet::TotalBalances::builder()
        .exclude_empty_balances(true)
        .expand_custodial_wallet(true)
        .build()
        .throttle(&rate_limiter)
        .await?
        .sign_now_and_send(&credential, &client)
        .await?
        .into_payload();

    dbg!(&total_balances);

    // // Go Account -> Hot Wallet withdrawal
    // let send_coins = wallet::SendCoins::builder()
    //     .coin("ofctbtc")
    //     .wallet_id(config.go_wallet_id.clone())
    //     .address(config.hot_wallet_address)
    //     .wallet_passphrase(config.go_wallet_passphrase)
    //     .amount(100u64)
    //     .comment("Test transaction")
    //     .build()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(&send_coins);

    // let transfer_id = send_coins.transfer.id.clone();
    // let transfer_status = transfer::GetTransfer::builder()
    //     .coin("ofctbtc")
    //     .wallet_id(config.go_wallet_id.clone())
    //     .transfer_id(transfer_id)
    //     .build()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(transfer_status);

    // // Hot wallet -> crypto address withdrawal
    // let send_coins = wallet::SendCoins::builder()
    //     .coin("tsol")
    //     .wallet_id(config.hot_wallet_id.clone())
    //     .address(config.recipient_address.clone())
    //     .wallet_passphrase(config.hot_wallet_passphrase.clone())
    //     .tx_type(wallet::TransactionType::Transfer)
    //     .amount(1000i128)
    //     .comment("Test transaction")
    //     // .token_name("tsol:usdcv2")
    //     .build()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(&send_coins);

    // // If we have a transfer ID from the send_coins response, we can fetch its specific status
    // let transfer_id = send_coins.transfer.id.clone();
    // let transfer_status = transfer::GetTransfer::builder()
    //     .coin("tsol")
    //     .wallet_id(config.hot_wallet_id.clone())
    //     .transfer_id(transfer_id)
    //     .build()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(transfer_status);

    // Example: Send to multiple recipients in a single transaction
    // let recipient1 = wallet::Recipient::builder()
    //     .address(config.recipient_address)
    //     .amount(1000u64)
    //     // .token_name("tsol")
    //     .build();

    // let send_many_result = wallet::SendMany::builder()
    //     .coin("tsol")
    //     .wallet_id(config.hot_wallet_id.clone())
    //     .recipients(vec![recipient1])
    //     .wallet_passphrase(config.hot_wallet_passphrase.clone())
    //     .tx_type(wallet::TransactionType::Transfer)
    //     .comment("Multi-recipient test transaction")
    //     .build()
    //     .throttle(&rate_limiter)
    //     .await?
    //     .sign_now_and_send(&credential, &client)
    //     .await?
    //     .into_payload();

    // dbg!(&send_many_result);

    Ok(())
}
