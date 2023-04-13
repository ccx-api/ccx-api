use serde::Deserialize;
use serde::Serialize;

// ???
// #[derive(Debug, Deserialize, Serialize)]
// pub enum PushPaymentMethod {
//     #[serde(rename = "crypto")]
//     Crypto,
//     #[serde(rename = "ach")]
//     ACH,
//     #[serde(rename = "sepa")]
//     SEPA,
//     #[serde(rename = "wire")]
//     Wire,
//     #[serde(rename = "faster_payments")]
//     FasterPayments,
// }

// #[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
// #[serde(rename_all = "snake_case")]
// pub enum PushPaymentMethod {
//     Crypto,
//     BankWire,
//     Fedwire,
//     SwiftBankAccount,
//     IntraBankAccount,
//     SepaBankAccount,
//     UkBankAccount,
// }
