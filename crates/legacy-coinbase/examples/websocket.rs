use ccx_coinbase::CoinbaseResult;
use ccx_coinbase::ExchangeApiCred;
use ccx_coinbase::api::exchange::ExchangeApi as CoinbaseExchangeApi;
use ccx_coinbase::proto::WsCommand;
use ccx_coinbase::proto::subscribe::ChannelType;
use ccx_coinbase::proto::subscribe::Subscribe;
use futures::stream::StreamExt;
use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> CoinbaseResult<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install rustls crypto provider");

    #[allow(unused_variables)]
    let coinbase = CoinbaseExchangeApi::<ExchangeApiCred>::from_env();

    let stream = coinbase.ws().await?;

    let (tx, mut rx) = stream.split();

    tx.send_command(WsCommand::Subscribe(Subscribe {
        product_ids: vec!["BTC-USD".into()],
        channels: vec![ChannelType::Ticker],
    }))
    .await?;

    let (waiting_tx, waiting_rx) = oneshot::channel();

    tokio::spawn(async move {
        while let Some(msg) = rx.next().await {
            println!("{:?}", msg);
        }
        waiting_tx.send(()).unwrap();
    });

    waiting_rx.await.unwrap();

    Ok(())
}
