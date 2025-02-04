// use ccx_mexc::api::spot::NewOrder;
use ccx_mexc::api::spot::NewOrderResult;
use ccx_mexc::api::spot::OrderResponseType;
use ccx_mexc::api::spot::OrderSide;
use ccx_mexc::api::spot::OrderType;
use ccx_mexc::client::Task;
use ccx_mexc::ApiCred;
use ccx_mexc::MexcResult;
use ccx_mexc::Decimal;
use ccx_mexc::SpotApi;
use ccx_mexc::TimeWindow;
use ccx_mexc_examples_util::d;

const BTCBUSD: &str = "BTCBUSD";
// const EURBUSD: &str = "EURBUSD";

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> MexcResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let mexc = SpotApi::<ApiCred>::from_env();

    // let book = print_res(mexc.ticker_book(BTCBUSD).await)?;
    // let time = print_res(mexc.time().await)?;

    // print_res(mexc.account(TimeWindow::now()).await)?;
    // print_res(mexc.my_trades(BTCBUSD, None, None, None, Some(10), TimeWindow::now()).await)?;
    // print_res(mexc.my_trades(EURBUSD, None, None, None, Some(10), TimeWindow::now()).await)?;

    // print_res(mexc.all_orders(SYMBOL, None, None, None, Some(10), TimeWindow::now()).await)?;
    // print_res(mexc.open_orders(Some(SYMBOL), TimeWindow::now()).await)?;
    // print_res(mexc.open_orders(Some(SYMBOL), TimeWindow::now()).await)?;
    // print_res(mexc.open_orders(None::<&str>, TimeWindow::now()).await)?;

    // print_res(
    //     mexc
    //         .cancel_all_orders(
    //             SYMBOL,
    //             TimeWindow::now(),
    //         )
    //         .await,
    // )?;

    // limit_order(
    //     &mexc,
    //     BTCBUSD,
    //     OrderSide::Buy,
    //     d("44000"),
    //     Quantity::Base(d("0.0005")),
    // )?
    // .await?;
    //
    // limit_order(
    //     &mexc,
    //     BTCBUSD,
    //     OrderSide::Buy,
    //     d("43000"),
    //     Quantity::Base(d("0.0005")),
    // )?
    // .await?;
    //
    // market_order(&mexc, BTCBUSD, OrderSide::Sell, Quantity::Quote(d("22")))?.await?;

    // market_order(
    //     &mexc,
    //     EURBUSD,
    //     OrderSide::Buy,
    //     Quantity::Base(d("20")),
    // )
    // .await?;

    // let order = order.as_result().unwrap();

    // print_res(
    //     mexc
    //         .cancel_order(
    //             &order.symbol,
    //             Some(order.order_id),
    //             None::<&str>,
    //             None::<&str>,
    //             TimeWindow::now(),
    //         )
    //         .await,
    // )?;

    // print_res(
    //     mexc
    //         .get_order(
    //             &order.symbol,
    //             Some(order.order_id),
    //             None::<&str>,
    //             TimeWindow::now(),
    //         )
    //         .await,
    // );
    Ok(())
}

fn limit_order(
    mexc: &SpotApi<ApiCred>,
    symbol: &str,
    side: OrderSide,
    price: Decimal,
    quantity: Quantity,
) -> MexcResult<Task<NewOrderResult>> {
    let (quantity, quote_quantity) = quantity.to_arg();
    let task = mexc
        .create_order(
            symbol,
            side,
            OrderType::LimitMaker,
            None,
            quantity,
            quote_quantity,
            None,
            Some(price),
            None,
            None::<&str>,
            Some(OrderResponseType::Result),
            TimeWindow::now(),
        )?
        .as_result()
        .unwrap();
    Ok(task)
}

fn market_order(
    mexc: &SpotApi<ApiCred>,
    symbol: &str,
    side: OrderSide,
    quantity: Quantity,
) -> MexcResult<Task<NewOrderResult>> {
    let (quantity, quote_quantity) = quantity.to_arg();
    let task = mexc
        .create_order(
            symbol,
            side,
            OrderType::Market,
            None,
            quantity,
            quote_quantity,
            None,
            None,
            None,
            None::<&str>,
            Some(OrderResponseType::Result),
            TimeWindow::now(),
        )?
        .as_result()
        .unwrap();
    Ok(task)
}

enum Quantity {
    Base(Decimal),
    Quote(Decimal),
}

impl Quantity {
    pub fn to_arg(self) -> (Option<Decimal>, Option<Decimal>) {
        match self {
            Quantity::Base(quantity) => (Some(quantity), None),
            Quantity::Quote(quantity) => (None, Some(quantity)),
        }
    }
}
