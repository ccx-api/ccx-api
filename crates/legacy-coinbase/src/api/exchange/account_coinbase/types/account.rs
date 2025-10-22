use serde::Deserialize;
use serde::Serialize;

use super::CoinbaseAccountId;
use crate::Atom;
use crate::Decimal;

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinbaseAccount {
    pub id: CoinbaseAccountId,
    pub name: Atom,
    pub balance: Decimal,
    pub currency: Atom,
    pub r#type: String,
    pub primary: bool,
    pub active: bool,
    pub available_on_consumer: Option<bool>,
    pub wire_deposit_information: Option<DepositInformation>,
    pub swift_deposit_information: Option<DepositInformation>,
    pub sepa_deposit_information: Option<DepositInformation>,
    pub uk_deposit_information: Option<DepositInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepositInformation {
    pub account_number: Option<String>,
    pub routing_number: Option<String>,
    pub bank_name: Option<String>,
    pub bank_address: Option<String>,
    pub bank_country: Option<BankCountry>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub account_name: Option<String>,
    pub account_address: Option<String>,
    pub reference: Option<String>,
    pub iban: Option<String>,
    pub swift: Option<String>,
    pub sort_code: Option<String>,
    pub destination_tag_name: Option<String>,
    pub destination_tag_regex: Option<String>,
    pub hold_balance: Option<String>,
    pub hold_currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BankCountry {
    pub name: String,
    pub code: String,
}
