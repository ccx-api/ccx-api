mod get_wallet_by_id;
mod get_wallet_by_id_coin;
mod list_wallets;
mod send_coins;
mod send_many;
mod total_balances;

pub use crate::types::wallet;
pub use get_wallet_by_id::*;
pub use get_wallet_by_id_coin::*;
pub use list_wallets::*;
pub use send_coins::*;
pub use send_many::*;
pub use total_balances::*;
