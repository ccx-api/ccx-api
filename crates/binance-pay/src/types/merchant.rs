use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Merchant {
    #[serde(rename = "subMerchantId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_merchant_id: Option<String>,
}
