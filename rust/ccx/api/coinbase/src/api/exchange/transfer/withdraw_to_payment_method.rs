use crate::api::exchange::prelude::*;
use crate::api::exchange::RequestedWithdrawal;
use crate::api::exchange::RL_PRIVATE_KEY;

pub type WithdrawToPaymentMethodResponse = RequestedWithdrawal;

#[derive(Debug, Serialize, Deserialize)]
struct WithdrawToPaymentMethodRequest {
    profile_id: Option<Uuid>,
    payment_method_id: Uuid,
    amount: Decimal,
    currency: Atom,
}

#[cfg(feature = "with_network")]
impl<S> ExchangeApi<S>
where
    S: crate::client::CoinbaseExchangeSigner,
    S: Unpin + 'static,
{
    /// # Withdraw to payment method.
    ///
    /// Withdraws funds from the specified profile_id to a linked external payment method.
    ///
    ///
    /// ## API Key Permissions
    ///
    /// This endpoint requires the "transfer" permission. API key must belong to default profile.
    ///
    /// [https://docs.cloud.coinbase.com/exchange/reference/exchangerestapi_postwithdrawcrypto]
    pub fn withdraw_to_payment_method(
        &self,
        profile_id: Option<Uuid>,
        payment_method_id: Uuid,
        amount: Decimal,
        currency: Atom,
    ) -> CoinbaseResult<Task<WithdrawToPaymentMethodResponse>> {
        let endpoint = "/withdrawals/payment-method";
        Ok(self
            .rate_limiter
            .task(self.client.post(&endpoint)?.signed_now()?.request_body(
                WithdrawToPaymentMethodRequest {
                    profile_id,
                    payment_method_id,
                    amount,
                    currency,
                },
            )?)
            .cost(RL_PRIVATE_KEY, 1)
            .send())
    }
}
