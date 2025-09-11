use crate::api::exchange::RL_PRIVATE_KEY;
use crate::api::exchange::account_coinbase::CoinbaseAccountId;
use crate::api::exchange::account_coinbase::GenerateCoinbaseAccount;
use crate::api::exchange::prelude::*;

pub type GenerateCryptoAddressResponse = GenerateCoinbaseAccount;

#[derive(Debug, Serialize, Deserialize)]
struct GenerateCryptoAddressRequest<'a> {
    account_id: CoinbaseAccountId,
    profile_id: Uuid,
    network: &'a str,
}

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Generate crypto address.
    ///
    /// Generates a one-time crypto address for depositing crypto.
    ///
    /// > Note: You can generate an address for crypto deposits.
    /// > See the [Coinbase Accounts](https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_getcoinbaseaccounts/)
    /// > section for information on how to retrieve your coinbase account ID.
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires the "transfer" permission. API key must belong to default profile.
    ///
    /// ## Parameters
    ///
    /// * `account_id` - (undocumented).
    /// * `profile_id` - (undocumented).
    /// * `network` - (undocumented).
    ///
    /// [https://docs.cdp.coinbase.com/exchange/reference/exchangerestapi_postcoinbaseaccountaddresses]
    pub fn generate_coinbase_account<N: AsRef<str>>(
        &self,
        account_id: CoinbaseAccountId,
        profile_id: Uuid,
        network: N,
    ) -> CoinbaseResult<Task<GenerateCryptoAddressResponse>> {
        fn endpoint(account_id: CoinbaseAccountId) -> String {
            format!("coinbase-accounts/{account_id}/addresses")
        }
        fn body(
            account_id: CoinbaseAccountId,
            profile_id: Uuid,
            network: &str,
        ) -> GenerateCryptoAddressRequest {
            GenerateCryptoAddressRequest {
                account_id,
                profile_id,
                network,
            }
        }

        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint(account_id.clone()))?
                    .signed_now()?
                    .request_body(body(account_id, profile_id, network.as_ref()))?,
            )
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
