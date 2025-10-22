#[cfg(feature = "db")]
use diesel_derives::AsExpression;
#[cfg(feature = "db")]
use diesel_derives::FromSqlRow;

use crate::api::prime::prelude::*;

/// The type of a transaction
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(test, derive(enum_iterator::Sequence))]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum TransactionType {
    /// An unknown transaction type
    #[serde(rename = "TRANSACTION_TYPE_UNKNOWN")]
    Unknown,
    /// A fiat or crypto deposit
    #[serde(rename = "DEPOSIT")]
    Deposit,
    /// A fiat or crypto withdrawal
    #[serde(rename = "WITHDRAWAL")]
    Withdrawal,
    /// An internal fiat or crypto deposit
    #[serde(rename = "INTERNAL_DEPOSIT")]
    InternalDeposit,
    /// An internal fiat or crypto withdrawal
    #[serde(rename = "INTERNAL_WITHDRAWAL")]
    InternalWithdrawal,
    /// Internal automated deposit to a cold address from a restored address
    #[serde(rename = "SWEEP_DEPOSIT")]
    SweepDeposit,
    /// Internal automated withdrawal from a restored address to a cold address
    #[serde(rename = "SWEEP_WITHDRAWAL")]
    SweepWithdrawal,
    /// On-chain deposit of funds into proxy contract from cold address
    #[serde(rename = "PROXY_DEPOSIT")]
    ProxyDeposit,
    /// On-chain withdrawal of funds from proxy contract to cold address
    #[serde(rename = "PROXY_WITHDRAWAL")]
    ProxyWithdrawal,
    /// Coinbase Prime automated invoice settlement payment
    #[serde(rename = "BILLING_WITHDRAWAL")]
    BillingWithdrawal,
    /// Reward payment to an associated address for a staked asset
    #[serde(rename = "REWARD")]
    Reward,
    /// Coinbase Prime refund for the leftover amount for a CPFP (child pays for parent) transaction
    #[serde(rename = "COINBASE_REFUND")]
    CoinbaseRefund,
    /// An OTHER type of transaction
    #[serde(rename = "TRANSACTION_TYPE_OTHER")]
    Other,
    /// A manual adjustment withdrawal transaction
    #[serde(rename = "WITHDRAWAL_ADJUSTMENT")]
    WithdrawalAdjustment,
    /// A manual adjustment deposit transaction
    #[serde(rename = "DEPOSIT_ADJUSTMENT")]
    DepositAdjustment,
    /// An on-chain registration for an address
    #[serde(rename = "KEY_REGISTRATION")]
    KeyRegistration,
    /// An on-chain delegation transaction
    #[serde(rename = "DELEGATION")]
    Delegation,
    /// An on-chain undelegation transaction
    #[serde(rename = "UNDELEGATION")]
    Undelegation,
    /// On-chain restaking transaction
    #[serde(rename = "RESTAKE")]
    Restake,
    /// On-chain unbonding event transaction
    #[serde(rename = "COMPLETE_UNBONDING")]
    CompleteUnbonding,
    /// On-chain event indicating unbonding period is over
    #[serde(rename = "WITHDRAW_UNBONDED")]
    WithdrawUnbonded,
    /// On-chain transaction to begin staking from an address
    #[serde(rename = "STAKE_ACCOUNT_CREATE")]
    StakeAccountCreate,
    /// On-chain transaction alter validator
    #[serde(rename = "CHANGE_VALIDATOR")]
    ChangeValidator,
    /// On-chain transaction to begin staking in Cryptocurrency network
    #[serde(rename = "STAKE")]
    Stake,
    /// On-chain transaction to stop staking in Cryptocurrency network
    #[serde(rename = "UNSTAKE")]
    Unstake,
    /// On-chain transaction to remove a party from a multi-signature wallet
    #[serde(rename = "REMOVE_AUTHORIZED_PARTY")]
    RemoveAuthorizedParty,
    /// On-chain transaction to begin staking from a seed account
    #[serde(rename = "STAKE_AUTHORIZE_WITH_SEED")]
    StakeAuthorizeWithSeed,
    /// On-chain transaction indicating a slash event has occurred
    #[serde(rename = "SLASH")]
    Slash,
    /// On-chain transaction deposit for the purpose of transaction operations
    #[serde(rename = "COINBASE_DEPOSIT")]
    CoinbaseDeposit,
    /// Internal conversion between two assets
    #[serde(rename = "CONVERSION")]
    Conversion,
    /// On-chain transaction to claim rewards from Vote Account
    #[serde(rename = "CLAIM_REWARDS")]
    ClaimRewards,
    /// On-chain transaction to transfer the reward claiming permission to other pubkey
    #[serde(rename = "VOTE_AUTHORIZE")]
    VoteAuthorize,
    /// On-chain transaction initiated with Prime Web3 Wallet
    #[serde(rename = "WEB3_TRANSACTION")]
    Web3Transaction,
}
#[cfg(feature = "db")]
forward_display_to_serde!(TransactionType);
#[cfg(feature = "db")]
forward_from_str_to_serde!(TransactionType);

impl TransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            TransactionType::Unknown => "TRANSACTION_TYPE_UNKNOWN",
            TransactionType::Deposit => "DEPOSIT",
            TransactionType::Withdrawal => "WITHDRAWAL",
            TransactionType::InternalDeposit => "INTERNAL_DEPOSIT",
            TransactionType::InternalWithdrawal => "INTERNAL_WITHDRAWAL",
            TransactionType::SweepDeposit => "SWEEP_DEPOSIT",
            TransactionType::SweepWithdrawal => "SWEEP_WITHDRAWAL",
            TransactionType::ProxyDeposit => "PROXY_DEPOSIT",
            TransactionType::ProxyWithdrawal => "PROXY_WITHDRAWAL",
            TransactionType::BillingWithdrawal => "BILLING_WITHDRAWAL",
            TransactionType::Reward => "REWARD",
            TransactionType::CoinbaseRefund => "COINBASE_REFUND",
            TransactionType::Other => "TRANSACTION_TYPE_OTHER",
            TransactionType::WithdrawalAdjustment => "WITHDRAWAL_ADJUSTMENT",
            TransactionType::DepositAdjustment => "DEPOSIT_ADJUSTMENT",
            TransactionType::KeyRegistration => "KEY_REGISTRATION",
            TransactionType::Delegation => "DELEGATION",
            TransactionType::Undelegation => "UNDELEGATION",
            TransactionType::Restake => "RESTAKE",
            TransactionType::CompleteUnbonding => "COMPLETE_UNBONDING",
            TransactionType::WithdrawUnbonded => "WITHDRAW_UNBONDED",
            TransactionType::StakeAccountCreate => "STAKE_ACCOUNT_CREATE",
            TransactionType::ChangeValidator => "CHANGE_VALIDATOR",
            TransactionType::Stake => "STAKE",
            TransactionType::Unstake => "UNSTAKE",
            TransactionType::RemoveAuthorizedParty => "REMOVE_AUTHORIZED_PARTY",
            TransactionType::StakeAuthorizeWithSeed => "STAKE_AUTHORIZE_WITH_SEED",
            TransactionType::Slash => "SLASH",
            TransactionType::CoinbaseDeposit => "COINBASE_DEPOSIT",
            TransactionType::Conversion => "CONVERSION",
            TransactionType::ClaimRewards => "CLAIM_REWARDS",
            TransactionType::VoteAuthorize => "VOTE_AUTHORIZE",
            TransactionType::Web3Transaction => "WEB3_TRANSACTION",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serde_roundtrip() {
        for typ in enum_iterator::all::<TransactionType>() {
            let s = serde_json::to_string(&typ).unwrap();
            let typ2: TransactionType = serde_json::from_str(&s).unwrap();
            assert_eq!(typ, typ2);
        }
    }

    #[test]
    fn test_as_str() {
        for typ in enum_iterator::all::<TransactionType>() {
            let s = serde_plain::to_string(&typ).unwrap();
            assert_eq!(s, typ.as_str());
        }
    }
}
