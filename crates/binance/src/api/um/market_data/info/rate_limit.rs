use crate::api::um::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub interval: RateLimitInterval,
    pub interval_num: u32,
    pub limit: u32,
    pub rate_limit_type: RateLimitType,
}

// TODO check variants
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RateLimitInterval {
    #[serde(rename = "SECOND")]
    Second,
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "DAY")]
    Day,
}

// TODO check variants
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub enum RateLimitType {
    #[serde(rename = "REQUEST_WEIGHT")]
    RequestWeight,
    #[serde(rename = "ORDERS")]
    Orders,
    // #[serde(rename = "RAW_REQUESTS")]
    // RawRequests,
}
