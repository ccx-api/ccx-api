use crate::api::exchange::prelude::*;
use crate::api::exchange::RequestedWithdrawal;
use crate::api::exchange::RL_PRIVATE_KEY;

pub type WithdrawToAddressResponse = RequestedWithdrawal;

#[derive(Debug, Serialize, Deserialize)]
struct WithdrawToAddressRequest {
    profile_id: Option<Uuid>,
    amount: Decimal,
    currency: Atom,
    crypto_address: String,
    destination_tag: Option<u64>,
    no_destination_tag: bool,
    two_factor_code: Option<String>,
    nonce: Option<u32>,
    network: Option<Atom>,
    /// A boolean flag to add the network fee on top of the amount. If this is blank, it will
    /// default to deducting the network fee from the amount.
    add_network_fee_to_total: Option<bool>,
}

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Withdraw to crypto address.
    ///
    /// Withdraws funds from the specified profile_id to an external crypto address.
    ///
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires the "transfer" permission. API key must belong to default profile.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_postwithdrawcrypto]
    pub fn withdraw_to_address(
        &self,
        profile_id: Option<Uuid>,
        amount: Decimal,
        currency: Atom,
        crypto_address: String,
        destination_tag: Option<u64>,
        no_destination_tag: bool,
        two_factor_code: Option<String>,
        nonce: Option<u32>,
        network: Option<Atom>,
        add_network_fee_to_total: Option<bool>,
    ) -> CoinbaseResult<Task<WithdrawToAddressResponse>> {
        let endpoint = "/withdrawals/crypto";
        Ok(self
            .rate_limiter
            .task(self.client.post(&endpoint)?.signed_now()?.request_body(
                WithdrawToAddressRequest {
                    profile_id,
                    amount,
                    currency,
                    crypto_address,
                    destination_tag,
                    no_destination_tag,
                    two_factor_code,
                    nonce,
                    network,
                    add_network_fee_to_total,
                },
            )?)
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
