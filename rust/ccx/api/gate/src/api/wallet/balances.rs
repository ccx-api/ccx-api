use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use smart_string::SmartString;

use crate::api::ApiMethod;
use crate::api::ApiVersion;
use crate::api::Request;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletBalancesRequest {
    pub currency: Option<SmartString>,
}

impl Request for WalletBalancesRequest {
    const METHOD: ApiMethod = ApiMethod::Get;
    const VERSION: ApiVersion = ApiVersion::V4;
    const PATH: &'static str = "wallet/total_balance";
    const IS_PUBLIC: bool = false;
    type Response = WalletBalancesResponse;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletBalancesResponse {
    /// Total balances calculated with specified currency unit
    pub total: WalletBalance,
    /// Total balances in different accounts
    pub details: WalletBalanceDetails,
}

/// Total balances calculated with specified currency unit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletBalanceDetails {
    pub cbbc: Option<WalletBalance>,
    pub cross_margin: Option<WalletBalance>,
    pub delivery: Option<WalletBalance>,
    pub finance: Option<WalletBalance>,
    pub futures: Option<WalletBalance>,
    pub margin: Option<WalletBalance>,
    // missing in docs
    pub options: Option<WalletBalance>,
    // missing in docs
    pub payment: Option<WalletBalance>,
    pub quant: Option<WalletBalance>,
    pub spot: Option<WalletBalance>,
    pub warrant: Option<WalletBalance>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WalletBalance {
    /// Currency
    pub currency: SmartString,
    /// Account total balance amount
    pub amount: Decimal,
}

#[cfg(feature = "with_network")]
mod with_network {
    use super::*;
    use crate::client::rest::RequestError;
    use crate::client::signer::GateSigner;
    use crate::GateApi;

    impl<S: GateSigner> GateApi<S> {
        /// # Retrieve user's total balances
        ///
        /// Retrieve user's total balances
        ///
        /// This endpoint returns an approximate sum of exchanged amount from all currencies
        /// to input currency for each account.The exchange rate and account balance could have been
        /// cached for at most 1 minute. It is not recommended to use its result for any trading
        /// calculation.
        ///
        /// For trading calculation, use the corresponding account query endpoint for each account
        /// type. For example:
        ///
        /// - GET /spot/accounts to query spot account balance
        /// - GET /margin/accounts to query margin account balance
        /// - GET /futures/{settle}/accounts to query futures account balance
        ///
        /// ## Parameters
        ///
        /// * `currency` - Currency unit used to calculate the balance amount.
        ///    BTC, CNY, USD and USDT are allowed. USDT is the default.
        pub async fn wallet_balances(
            &self,
            currency: Option<SmartString>,
        ) -> Result<<WalletBalancesRequest as Request>::Response, RequestError> {
            self.request(&WalletBalancesRequest { currency }).await
        }
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;

    use super::*;

    #[test]
    fn test_example_from_docs() {
        let json = r#"{
  "details": {
    "cross_margin": {
      "amount": "0",
      "currency": "USDT"
    },
    "spot": {
      "currency": "USDT",
      "amount": "42264489969935775.5160259954878034182418"
    },
    "finance": {
      "amount": "662714381.70310327810191647181",
      "currency": "USDT"
    },
    "margin": {
      "amount": "1259175.664137668554329559",
      "currency": "USDT"
    },
    "quant": {
      "amount": "591702859674467879.6488202650892478553852",
      "currency": "USDT"
    },
    "futures": {
      "amount": "2384175.5606114082065",
      "currency": "USDT"
    },
    "delivery": {
      "currency": "USDT",
      "amount": "1519804.9756702"
    },
    "warrant": {
      "amount": "0",
      "currency": "USDT"
    },
    "cbbc": {
      "currency": "USDT",
      "amount": "0"
    }
  },
  "total": {
    "currency": "USDT",
    "amount": "633967350312281193.068368815439797304437"
  }
}"#;
        let res: WalletBalancesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            WalletBalancesResponse {
                total: WalletBalance {
                    currency: "USDT".into(),
                    amount: "633967350312281193.068368815439797304437".parse().unwrap(),
                },
                details: WalletBalanceDetails {
                    cbbc: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    cross_margin: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    delivery: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(1519804.9756702),
                    }),
                    finance: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(662714381.70310327810191647181),
                    }),
                    futures: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(2384175.5606114082065),
                    }),
                    margin: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(1259175.664137668554329559),
                    }),
                    options: None,
                    payment: None,
                    quant: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: "591702859674467879.6488202650892478553852".parse().unwrap(),
                    }),
                    spot: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: "42264489969935775.5160259954878034182418".parse().unwrap(),
                    }),
                    warrant: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                },
            }
        );
    }

    #[test]
    fn test_real_response() {
        let json = "{\"details\":{\"cbbc\":{\"currency\":\"USDT\",\"amount\":\"0\"},\
        \"delivery\":{\"currency\":\"USDT\",\"amount\":\"0\"},\"finance\":{\"currency\":\"USDT\",\
        \"amount\":\"0\"},\"futures\":{\"currency\":\"USDT\",\"amount\":\"0\"},\"margin\":\
        {\"currency\":\"USDT\",\"amount\":\"0\"},\"options\":{\"currency\":\"USDT\",\"amount\":\
        \"0\"},\"payment\":{\"currency\":\"USDT\",\"amount\":\"0\"},\"quant\":{\"currency\":\
        \"USDT\",\"amount\":\"0\"},\"spot\":{\"currency\":\"USDT\",\"amount\":\"0\"}},\"total\":\
        {\"amount\":\"0\",\"currency\":\"USDT\"}}";
        let res: WalletBalancesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(
            res,
            WalletBalancesResponse {
                total: WalletBalance {
                    currency: "USDT".into(),
                    amount: dec!(0),
                },
                details: WalletBalanceDetails {
                    cbbc: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    cross_margin: None,
                    delivery: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    finance: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    futures: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    margin: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    options: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    payment: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    quant: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    spot: Some(WalletBalance {
                        currency: "USDT".into(),
                        amount: dec!(0),
                    }),
                    warrant: None,
                },
            }
        );
    }
}
