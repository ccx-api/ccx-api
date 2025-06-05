use macro_rules_attribute::apply;
use serde::{Deserialize, Serialize};

use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::{Request, Response};
use crate::types::rate_limits::RateLimitType;

/// Request to list bank accounts
#[apply(Request)]
pub struct ListBankAccounts {
    /// Filter by verification state
    verification_state: Option<VerificationState>,

    /// Filter by bank account ID hash
    bank_account_id_hash: Option<String>,

    /// Filter by bank account ID
    bank_account_id: Option<String>,
}

/// Verification state of a bank account
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationState {
    /// Account is pending verification
    Pending,
    /// Account has been approved
    Approved,
    /// Account has been rejected
    Rejected,
    /// Account has been removed
    Removed,
}

/// Type of bank transfer
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferType {
    /// Wire transfer
    Wire,
    /// CBIT transfer
    Cbit,
    /// Null type
    Null,
}

/// Currency of the bank account
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BankCurrency {
    /// USD currency
    FiatUsd,
    /// EUR currency
    FiatEur,
    /// Null currency
    Null,
}

/// Fee associated with a bank account
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BankAccountFee {
    /// The bank that the fee is associated with.
    /// Allowed values: customers_sftp, customers_api, bcbgroup
    pub bank: String,

    /// The type of fee.
    /// Allowed values: static, variable
    #[serde(rename = "type")]
    pub fee_type: String,

    /// The fee amount.
    pub amount: String,

    /// The currency of the bank account. If null, defaults to `fiatusd`.
    /// Allowed values: fiatusd, fiateur, null
    pub coin: Option<BankCurrency>,
}

/// Bank account information
#[apply(Response)]
pub struct BankAccount {
    /// Bank account number or IBAN
    pub account_number: Option<String>,

    /// Enterprise ID
    pub enterprise_id: Option<String>,

    /// Bank address
    pub address: Option<String>,

    /// Address line 1
    pub address1: Option<String>,

    /// Address line 2
    pub address2: Option<String>,

    /// Address line 3
    pub address3: Option<String>,

    /// Bank account ID
    pub id: Option<String>,

    /// Bank name
    pub name: Option<String>,

    /// Account owner name
    pub owner_name: Option<String>,

    /// Account owner address
    pub owner_address: Option<String>,

    /// US bank routing number (required for US bank accounts)
    pub routing_number: Option<String>,

    /// Two-letter country code (ISO 3166-1 alpha-2)
    pub short_country_code: Option<String>,

    /// SWIFT code (required for non-US bank accounts)
    pub swift_code: Option<String>,

    /// Type of transfer
    pub transfer_type: Option<TransferType>,

    /// Currency of the bank account
    pub currency: Option<BankCurrency>,

    /// External ID
    pub external_id: Option<String>,

    /// Further credit to
    pub further_credit_to: Option<String>,

    /// Intermediary bank name
    pub intermediary_bank_name: Option<String>,

    /// Intermediary bank ID
    pub intermediary_bank_id: Option<String>,

    /// Unique identifier for this account
    pub id_hash: Option<String>,

    /// BitGo Organization related to this entity
    pub trust_org: Option<String>,

    /// Token
    pub token: Option<String>,

    /// Verification state
    pub verification_state: Option<VerificationState>,

    /// Pending activity
    pub pending_activity: Option<String>,

    /// Fee associated with the bank account
    pub fee: Option<BankAccountFee>,
}

/// Response for ListBankAccounts request
#[apply(Response)]
pub struct ListBankAccountsResponse {
    /// List of bank accounts
    pub bank_accounts: Vec<BankAccount>,
}

impl Response for ListBankAccountsResponse {}

impl Request for ListBankAccounts {
    type Response = ListBankAccountsResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        "/api/v2/bankaccounts".into()
    }
}

impl SignedRequest for ListBankAccounts {}
