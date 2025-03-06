use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: RateLimitInterval,
    pub interval_num: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash, strum::EnumIter)]
pub enum RateLimitType {
    #[serde(rename = "REQUEST_WEIGHT")]
    RequestWeight,
    #[serde(rename = "ORDERS")]
    Orders,
    #[serde(rename = "RAW_REQUESTS")]
    RawRequests,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RateLimitInterval {
    #[serde(rename = "SECOND")]
    Second,
    #[serde(rename = "MINUTE")]
    Minute,
    #[serde(rename = "HOUR")]
    Hour,
    #[serde(rename = "DAY")]
    Day,
}

impl RateLimitInterval {
    pub fn from_letter(ascii: u8) -> Option<Self> {
        // lowercase and uppercase letters are the same
        match ascii {
            b's' | b'S' => Some(RateLimitInterval::Second),
            b'm' | b'M' => Some(RateLimitInterval::Minute),
            b'h' | b'H' => Some(RateLimitInterval::Hour),
            b'd' | b'D' => Some(RateLimitInterval::Day),
            _ => None,
        }
    }

    pub fn as_secs(&self) -> u64 {
        match self {
            RateLimitInterval::Second => 1,
            RateLimitInterval::Minute => 60,
            RateLimitInterval::Hour => 3_600,
            RateLimitInterval::Day => 86_400,
        }
    }
}
