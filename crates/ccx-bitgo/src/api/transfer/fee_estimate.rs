use crate::types::derive::Request;
use crate::types::derive::Response;
use macro_rules_attribute::apply;
use rust_decimal::Decimal;
use std::collections::HashMap;

use crate::proto::{PublicRequest, Request, Response};
use crate::types::base_amount::BaseAmount;
use crate::types::coin::Coin;
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
pub struct FeeEstimate {
    /// A cryptocurrency or token ticker symbol.
    #[serde(skip)]
    coin: Coin,
    /// Target number of blocks
    num_blocks: Option<u32>,
    /// Recipient of the tx to estimate for (only for ETH)
    recipient: Option<String>,
    /// ETH data of the tx to estimate for (only for ETH)
    data: Option<String>,
    /// Amount in base units being sent to estimate for (only for ETH)
    amount: Option<BaseAmount>,
    /// True if we are estimating for a hop tx, false or unspecified for a wallet tx (ETH, AVAXC and POLYGON)
    hop: Option<bool>,
}

/// EIP 1559 fee estimates for Ethereum
#[apply(Response)]
pub struct Eip1559FeeEstimate {
    /// Block base fees, in base units (i.e. Wei), per gas. Zeroes are returned for pre-EIP-1559 blocks
    pub base_fee: BaseAmount,
    /// Block gas used ratio. Calculated as the ratio of gasUsed and gasLimit
    pub gas_used_ratio: Decimal,
    /// 25th percentile of the tips spent in the last block
    pub safe_low_miner_tip: Option<BaseAmount>,
    /// 35th percentile of the tips spent in the last block
    pub normal_miner_tip: Option<BaseAmount>,
    /// 50th percentile of the tips spent in the last block
    pub standard_miner_tip: Option<BaseAmount>,
    /// 75th percentile of the tips spent in the last block
    pub fastest_miner_tip: Option<BaseAmount>,
    /// 97th percentile of the tips spent in the last block
    pub ludicrous_miner_tip: Option<BaseAmount>,
}

#[apply(Response)]
#[serde(untagged)]
pub enum FeeEstimateResponse {
    /// Bitcoin (UTXO) response
    #[serde(rename_all = "camelCase")]
    Bitcoin {
        /// The fee (in base units) per kilobyte (or virtual kilobyte) required
        /// to confirm a transaction on 2 or more blocks.
        fee_per_kb: u32,
        /// Child-Pays-For-Parent (CPFP) fee (in base units) per kilobyte (or virtual kilobyte).
        /// Includes the fees for all unconfirmed transactions dependent on the CPFP transaction.
        cpfp_fee_per_kb: Option<u32>,
        /// The number of blocks required to confirm a transaction.
        num_blocks: u32,
        /// The confidence (as a percentage) in the accuracy of the fee estimate.
        confidence: Option<u32>,
        /// Custom multiplier to the feeRate by block target.
        fee_by_block_target: Option<HashMap<String, u32>>,
    },
    /// Algorand specific response
    #[serde(rename_all = "camelCase")]
    Algo {
        /// Calculated by transaction size. Fee rate is in microAlgo (base unit).
        fee_rate: BaseAmount,
        /// Always 1000.
        minimum_fee: BaseAmount,
    },
    /// Ethereum specific response
    #[serde(rename_all = "camelCase")]
    Eth {
        /// Fee estimate for a transaction for the given account-based coin, denominated in base units (i.e. Wei)
        fee_estimate: BaseAmount,
        /// The amount of gas that the transaction will use, if recipient is provided in the request
        gas_limit_estimate: Option<BaseAmount>,
        /// Minimum gas price that can be provided in base units
        min_gas_price: BaseAmount,
        /// Minimum gas limit that can be provided in base units
        min_gas_limit: BaseAmount,
        /// Maximum gas limit that can be provided in base units
        max_gas_limit: BaseAmount,
        /// Gas price must not be increased by less than this after being introduced to the network
        min_gas_increase_by: BaseAmount,
        /// EIP 1559 fee estimates
        eip1559: Option<Eip1559FeeEstimate>,
    },
    /// Tron specific response
    #[serde(rename_all = "camelCase")]
    Trx {
        /// Maximum fee for a payment transaction, denominated in base units (i.e. sun).
        /// It varies for TRX and TRC20 Token based on the coin parameter
        fee: u64,
        /// Fee for wallet initialization
        new_account_fee: u64,
        /// Fee rate per unit of tx size. Not used currently (fee is a maximum limit,
        /// the network charges the cost of the transaction)
        net_fee: u64,
    },
    /// Account-based coins response
    #[serde(rename_all = "camelCase")]
    AccountBased {
        /// Fee estimate for account-based coins
        fee_estimate: BaseAmount,
    },
}

impl Response for FeeEstimateResponse {}

impl Request for FeeEstimate {
    type Response = FeeEstimateResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        let coin = &self.coin;

        format!("/api/v2/{coin}/tx/fee").into()
    }
}

impl PublicRequest for FeeEstimate {}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    mod serialize {
        use super::*;
        use serde_json::json;
        use similar_asserts::assert_eq;

        #[test]
        fn serialize_basic_fee_estimate() {
            let fee_estimate = FeeEstimate::builder().coin("btc").build();

            let actual = serde_json::to_value(&fee_estimate).unwrap();
            let expected = json!({});

            assert_eq!(actual, expected);
        }

        #[test]
        fn serialize_with_optional_params() {
            let fee_estimate = FeeEstimate::builder()
                .coin("eth")
                .num_blocks(2u32)
                .recipient("0x1234567890123456789012345678901234567890".to_string())
                .amount(1_000_000_000_000_000_000)
                .hop(true)
                .build();

            let actual = serde_json::to_value(&fee_estimate).unwrap();
            let expected = json!({
                "numBlocks": 2,
                "recipient": "0x1234567890123456789012345678901234567890",
                "amount": "1000000000000000000",
                "hop": true
            });

            assert_eq!(actual, expected);
        }
    }

    mod deserialize {
        use super::*;
        use serde_json::json;
        use similar_asserts::assert_eq;

        #[test]
        fn deserialize_bitcoin_response() {
            let json = json!({
                "feePerKb": 15902,
                "cpfpFeePerKb": 20000,
                "numBlocks": 2,
                "confidence": 80,
                "feeByBlockTarget": {
                    "1": 50536,
                    "2": 15902,
                    "3": 1579
                }
            });

            let response: FeeEstimateResponse = serde_json::from_value(json).unwrap();

            match response {
                FeeEstimateResponse::Bitcoin {
                    fee_per_kb,
                    cpfp_fee_per_kb,
                    num_blocks,
                    confidence,
                    fee_by_block_target,
                } => {
                    assert_eq!(fee_per_kb, 15902);
                    assert_eq!(cpfp_fee_per_kb, Some(20000));
                    assert_eq!(num_blocks, 2);
                    assert_eq!(confidence, Some(80));
                    assert!(fee_by_block_target.is_some());
                }
                _ => panic!("Expected Bitcoin variant"),
            }
        }

        #[test]
        fn deserialize_account_based_response() {
            let json = json!({
                "feeEstimate": "1000000"
            });

            let response: FeeEstimateResponse = serde_json::from_value(json).unwrap();

            match response {
                FeeEstimateResponse::AccountBased { fee_estimate } => {
                    assert_eq!(*fee_estimate, 1000000i128);
                }
                _ => panic!("Expected AccountBased variant"),
            }
        }

        #[test]
        fn deserialize_algo_response() {
            let json = json!({
                "feeRate": "1000",
                "minimumFee": "1000"
            });

            let response: FeeEstimateResponse = serde_json::from_value(json).unwrap();

            match response {
                FeeEstimateResponse::Algo {
                    fee_rate,
                    minimum_fee,
                } => {
                    assert_eq!(*fee_rate, 1000i128);
                    assert_eq!(*minimum_fee, 1000i128);
                }
                _ => panic!("Expected Algo variant"),
            }
        }

        #[test]
        fn deserialize_eth_response() {
            let json = json!({
                "feeEstimate": "420000000000000",
                "gasLimitEstimate": "21000",
                "minGasPrice": "1000000000",
                "minGasLimit": "21000",
                "maxGasLimit": "8000000",
                "minGasIncreaseBy": "1100000000",
                "eip1559": {
                    "baseFee": "15000000000",
                    "gasUsedRatio": "0.5",
                    "safeLowMinerTip": "1000000000",
                    "normalMinerTip": "1500000000",
                    "standardMinerTip": "2000000000",
                    "fastestMinerTip": "3000000000",
                    "ludicrousMinerTip": "5000000000"
                }
            });

            let response: FeeEstimateResponse = serde_json::from_value(json).unwrap();

            match response {
                FeeEstimateResponse::Eth {
                    fee_estimate,
                    gas_limit_estimate,
                    min_gas_price,
                    min_gas_limit,
                    max_gas_limit,
                    min_gas_increase_by,
                    eip1559,
                } => {
                    assert_eq!(*fee_estimate, 420000000000000i128);
                    assert_eq!(*gas_limit_estimate.unwrap(), 21000i128);
                    assert_eq!(*min_gas_price, 1000000000i128);
                    assert_eq!(*min_gas_limit, 21000i128);
                    assert_eq!(*max_gas_limit, 8000000i128);
                    assert_eq!(*min_gas_increase_by, 1100000000i128);
                    assert!(eip1559.is_some());

                    let eip1559_data = eip1559.unwrap();
                    assert_eq!(*eip1559_data.base_fee, 15000000000i128);
                    assert_eq!(eip1559_data.gas_used_ratio, dec!(0.5));
                    assert_eq!(*eip1559_data.safe_low_miner_tip.unwrap(), 1000000000i128);
                    assert_eq!(*eip1559_data.standard_miner_tip.unwrap(), 2000000000i128);
                }
                _ => panic!("Expected Eth variant"),
            }
        }

        #[test]
        fn deserialize_trx_response() {
            let json = json!({
                "fee": 100000000,
                "newAccountFee": 100000000,
                "netFee": 1000
            });

            let response: FeeEstimateResponse = serde_json::from_value(json).unwrap();

            match response {
                FeeEstimateResponse::Trx {
                    fee,
                    new_account_fee,
                    net_fee,
                } => {
                    assert_eq!(fee, 100000000);
                    assert_eq!(new_account_fee, 100000000);
                    assert_eq!(net_fee, 1000);
                }
                _ => panic!("Expected Trx variant"),
            }
        }
    }
}
