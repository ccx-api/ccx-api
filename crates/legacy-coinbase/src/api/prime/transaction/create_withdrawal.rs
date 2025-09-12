use super::RequestedWithdrawal;
use crate::api::prime::prelude::*;

pub type WithdrawalResponse = RequestedWithdrawal;

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
struct WithdrawalRequest<'a> {
    /// The amount in whole units of the withdrawal
    amount: Decimal,
    // /// Possible values: [DESTINATION_PAYMENT_METHOD, DESTINATION_BLOCKCHAIN]
    // destination_type: DestinationType,
    /// The idempotency key associated with the withdrawal
    idempotency_key: &'a str,
    /// The currency symbol for the withdrawal
    currency_symbol: &'a str,
    // /// Payment method
    // payment_method: PaymentMethod,
    // /// Blockchain address
    // blockchain_address: BlockchainAddress,
    #[serde(flatten)]
    destination: Destination<'a>,
}

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
#[serde(tag = "destination_type")]
enum Destination<'a> {
    // #[serde(rename = "DESTINATION_PAYMENT_METHOD")]
    // PaymentMethod { payment_method: PaymentMethod<'a> },
    #[serde(rename = "DESTINATION_BLOCKCHAIN")]
    Blockchain {
        blockchain_address: BlockchainAddress<'a>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
struct PaymentMethod<'a> {
    /// Payment method ID
    payment_method_id: &'a str,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
struct BlockchainAddress<'a> {
    /// Address
    address: &'a str,
    /// Account identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    account_identifier: Option<&'a str>,
}

#[cfg(feature = "with_network")]
impl<S> PrimeApi<S>
where
    S: crate::client::CoinbasePrimeSigner,
    S: Unpin + 'static,
{
    /// # Create Withdrawal.
    ///
    /// Create a withdrawal.
    ///
    /// ## Parameters
    ///
    /// * `portfolio_id` - The portfolio ID.
    /// * `wallet_id` - The wallet ID.
    /// * `idempotency_key` - The idempotency key.
    /// * `amount` - The amount in whole units of the withdrawal.
    /// * `currency_symbol` - The currency symbol for the withdrawal.
    /// * `address` - The blockchain address.
    ///
    /// [https://docs.cdp.coinbase.com/prime/reference/primerestapi_createwalletwithdrawal]
    pub fn create_withdrawal_blockchain(
        &self,
        portfolio_id: Uuid,
        wallet_id: Uuid,
        idempotency_key: &str,
        amount: Decimal,
        currency_symbol: &str,
        address: &str,
    ) -> CoinbaseResult<Task<WithdrawalResponse>> {
        let timestamp = Utc::now().timestamp() as u32;
        let endpoint = format!("/v1/portfolios/{portfolio_id}/wallets/{wallet_id}/withdrawals");
        Ok(self
            .rate_limiter
            .task(
                self.client
                    .post(&endpoint)?
                    .signed(timestamp)?
                    .request_body(WithdrawalRequest {
                        amount,
                        idempotency_key,
                        currency_symbol,
                        destination: Destination::Blockchain {
                            blockchain_address: BlockchainAddress {
                                address,
                                account_identifier: None,
                            },
                        },
                    })?,
            )
            .cost(RL_PORTFOLIO_KEY, 1)
            .send())
    }
}
