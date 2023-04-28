use chrono::Utc;

use super::prelude::*;
use crate::api::exchange::RL_PRIVATE_KEY;
use crate::client::Task;
use crate::dt_coinbase::DtCoinbase;

pub type GetAddressBookResponse = Vec<Address>;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Address {
    pub id: String,
    pub address: String,
    pub destination_tag: Option<String>,
    pub currency: Atom,
    pub label: String,
    pub address_book_added_at: String,
    pub last_used: Option<DtCoinbase>,
}

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// Get address book
    ///
    /// Get all addresses stored in the address book.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getaddressbook]
    pub fn get_address_book(&self) -> CoinbaseResult<Task<GetAddressBookResponse>> {
        let endpoint = "/address-book";
        Ok(self
            .rate_limiter
            .task(self.client.get(endpoint)?.signed_now()?.request_body(())?)
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
