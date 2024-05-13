use crate::api::exchange::prelude::*;
use crate::api::exchange::RequestedWithdrawal;
use crate::api::exchange::RL_PRIVATE_KEY;

pub type WithdrawToAddressResponse = RequestedWithdrawal;

#[derive(Debug, Serialize, Deserialize)]
struct WithdrawToAddressRequest<'a> {
    profile_id: Option<Uuid>,
    amount: Decimal,
    currency: &'a str,
    crypto_address: &'a str,
    destination_tag: Option<&'a str>,
    no_destination_tag: bool,
    two_factor_code: Option<&'a str>,
    nonce: Option<i64>,
    network: Option<&'a str>,
    /// A boolean flag to add the network fee on top of the amount. If this is blank, it will
    /// default to deducting the network fee from the amount.
    add_network_fee_to_total: Option<bool>,

    is_intermediary: bool,
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
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_postwithdrawcrypto]
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires the "transfer" permission. API key must belong to default profile.
    ///
    /// ## Travel Rule
    ///
    /// The Travel Rule requires financial institutions, including custodial cryptocurrency
    /// exchanges, to share basic information about their customers when sending funds over
    /// a certain amount. VASPs that are part of the TRUST consortium use the
    /// [TRUST solution](https://www.coinbase.com/travelrule) when sharing PII to satisfy
    /// the Travel Rule data requirements.
    ///
    /// For more details and examples, see
    /// [Travel Rule for Withdrawals](https://docs.cloud.coinbase.com/exchange/docs/travel-rule-withdrawals).
    #[allow(clippy::too_many_arguments)]
    pub fn withdraw_to_address(
        &self,
        profile_id: Option<Uuid>,
        amount: Decimal,
        currency: &str,
        crypto_address: &str,
        destination_tag: Option<&str>,
        no_destination_tag: bool,
        network: Option<&str>,
        two_factor_code: Option<&str>,
        nonce: Option<i64>,
        add_network_fee_to_total: Option<bool>,
    ) -> CoinbaseResult<Task<WithdrawToAddressResponse>> {
        let endpoint = "/withdrawals/crypto";
        Ok(self
            .rate_limiter
            .task(self.client.post(endpoint)?.signed_now()?.request_body(
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
                    is_intermediary: false,
                },
            )?)
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
