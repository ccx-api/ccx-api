use crate::api::exchange::account_coinbase::CoinbaseAccountId;
use crate::api::exchange::account_coinbase::GenerateCryptoAddress;
use crate::api::exchange::prelude::*;
use crate::api::exchange::RL_PRIVATE_KEY;

pub type GenerateCryptoAddressResponse = GenerateCryptoAddress;

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
    /// Generate crypto address.
    ///
    /// Generates a one-time crypto address for depositing crypto.
    ///
    /// You can generate an address for crypto deposits.
    /// See the Coinbase Accounts section for information on how to retrieve your coinbase account ID.
    ///
    ///    Note: This endpoint requires the "transfer" permission.
    ///          API key must belong to default profile.
    ///
    /// * `account_id` - .
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_getaccount]
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
