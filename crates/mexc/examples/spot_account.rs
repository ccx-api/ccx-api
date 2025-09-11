use ccx_mexc::ApiCred;
use ccx_mexc::Decimal;
use ccx_mexc::MexcResult;
use ccx_mexc::SpotApi;
use ccx_mexc::TimeWindow;
use ccx_mexc::api::spot::NewOrder;
use ccx_mexc::api::spot::OrderSide;
use ccx_mexc::api::spot::OrderType;
use ccx_mexc::client::Task;
use ccx_mexc_examples_util::d;
use ccx_mexc_examples_util::print_res;

// prevent some operations that could affect balance
const ENABLE_BALANCE_OPERATION: bool = false;
const SYMBOL: &str = "MXUSDT";

#[actix_rt::main]
async fn main() {
    let _ = main_().await;
}

async fn main_() -> MexcResult<()> {
    let _ = dotenv::dotenv();
    env_logger::init();

    let mexc = SpotApi::<ApiCred>::from_env();

    print_res(mexc.account(TimeWindow::now())?.await)?;
    print_res(
        mexc.my_trades(SYMBOL, None, None, Some(10), TimeWindow::now())?
            .await,
    )?;

    print_res(
        mexc.all_orders(SYMBOL, None, None, Some(10), TimeWindow::now())?
            .await,
    )?;
    print_res(mexc.open_orders(SYMBOL, TimeWindow::now())?.await)?;

    if ENABLE_BALANCE_OPERATION {
        let _ = limit_order(
            &mexc,
            SYMBOL,
            OrderSide::Buy,
            d("440"),
            Quantity::Base(d("1")),
        )?
        .await;

        let _ = market_order(&mexc, SYMBOL, OrderSide::Sell, Quantity::Quote(d("22")))?.await;

        print_res(mexc.open_orders(SYMBOL, TimeWindow::now())?.await)?;
        print_res(mexc.cancel_all_orders(SYMBOL, TimeWindow::now())?.await)?;
        print_res(
            mexc.all_orders(SYMBOL, None, None, None, TimeWindow::now())?
                .await,
        )?;

        let order = market_order(&mexc, SYMBOL, OrderSide::Buy, Quantity::Base(d("20")))?.await?;

        print_res(
            mexc.cancel_order(
                &order.symbol,
                Some(order.order_id.clone()),
                None::<&str>,
                None::<&str>,
                TimeWindow::now(),
            )?
            .await,
        )?;

        print_res(
            mexc.get_order(
                &order.symbol,
                Some(order.order_id),
                None::<&str>,
                TimeWindow::now(),
            )?
            .await,
        )?;
    }

    Ok(())
}

fn limit_order(
    mexc: &SpotApi<ApiCred>,
    symbol: &str,
    side: OrderSide,
    price: Decimal,
    quantity: Quantity,
) -> MexcResult<Task<NewOrder>> {
    let (quantity, quote_quantity) = quantity.to_arg();
    let task = mexc.create_order(
        symbol,
        side,
        OrderType::Limit,
        quantity,
        quote_quantity,
        Some(price),
        None::<&str>,
        TimeWindow::now(),
    )?;
    Ok(task)
}

fn market_order(
    mexc: &SpotApi<ApiCred>,
    symbol: &str,
    side: OrderSide,
    quantity: Quantity,
) -> MexcResult<Task<NewOrder>> {
    let (quantity, quote_quantity) = quantity.to_arg();
    let task = mexc.create_order(
        symbol,
        side,
        OrderType::Market,
        quantity,
        quote_quantity,
        None,
        None::<&str>,
        TimeWindow::now(),
    )?;
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
