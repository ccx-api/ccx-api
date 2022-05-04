use std::time::Duration;

use actix_http::header::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum IntervalLetter {
    #[serde(rename = "S")]
    Second,
    #[serde(rename = "M")]
    Minute,
    #[serde(rename = "H")]
    Hour,
    #[serde(rename = "D")]
    Day,
}

impl IntervalLetter {
    pub const SECOND: &'static str = "S";
    pub const SECOND_L: &'static str = "s";
    pub const MINUTE: &'static str = "M";
    pub const MINUTE_L: &'static str = "m";
    pub const HOUR: &'static str = "H";
    pub const HOUR_L: &'static str = "h";
    pub const DAY: &'static str = "D";
    pub const DAY_L: &'static str = "d";

    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            Self::SECOND | Self::SECOND_L => Self::Second,
            Self::MINUTE | Self::MINUTE_L => Self::Minute,
            Self::HOUR | Self::HOUR_L => Self::Hour,
            Self::DAY | Self::DAY_L => Self::Day,
            _ => None?,
        })
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Second => Self::SECOND,
            Self::Minute => Self::MINUTE,
            Self::Hour => Self::HOUR,
            Self::Day => Self::DAY,
        }
    }

    pub fn as_seconds(self) -> u32 {
        match self {
            Self::Second => 1,
            Self::Minute => 60,
            Self::Hour => 60 * 60,
            Self::Day => 24 * 60 * 60,
        }
    }
}

#[derive(Debug)]
pub struct TimeSpan {
    pub interval: Duration,
}

impl TimeSpan {
    pub fn new(interval_num: u32, interval_letter: IntervalLetter) -> Self {
        let interval = Duration::new((interval_num * interval_letter.as_seconds()).into(), 0);
        TimeSpan { interval }
    }

    pub fn from_str(ts_code: &str) -> Option<Self> {
        if ts_code.len() < 2 {
            None?;
        }
        let (num, ltr) = ts_code.split_at(ts_code.len() - 1);
        let interval_letter = IntervalLetter::from_str(ltr)?;
        let interval_num = num.parse().ok()?;
        Some(TimeSpan::new(interval_num, interval_letter))
    }
}

#[derive(Default, Debug)]
pub struct UsedRateLimits {
    /// The limits on the API are based on the IPs, not the API keys.
    pub weight_per_ip: Vec<(TimeSpan, u32)>,
    /// The order rate limit is counted against each account.
    pub order_count_per_account: Vec<(TimeSpan, u32)>,
}

impl UsedRateLimits {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        static OC_Z: &str = "x-mbx-order-count";
        static OC_PREFIX: &str = "x-mbx-order-count-";
        static UW_Z: &str = "x-mbx-used-weight";
        static UW_PREFIX: &str = "x-mbx-used-weight-";

        let mut u = UsedRateLimits::default();
        for (header_name, _hv) in headers.iter() {
            if let Some(value) = headers.get(header_name) {
                if let Ok(header_value) = value.to_str() {
                    let header_name = header_name.as_str();
                    if header_name.starts_with(UW_Z) {
                        log::debug!("  {}: {}", header_name, header_value);
                    }
                    if header_name.starts_with(UW_PREFIX) {
                        if let Some(time_span) = TimeSpan::from_str(&header_name[UW_PREFIX.len()..])
                        {
                            if let Some(weight) = header_value.parse().ok() {
                                u.weight_per_ip.push((time_span, weight))
                            }
                        };
                    }
                    if header_name.starts_with(OC_Z) {
                        log::debug!("  {}: {}", header_name, header_value);
                    }
                    if header_name.starts_with(OC_PREFIX) {
                        if let Some(time_span) = TimeSpan::from_str(&header_name[OC_PREFIX.len()..])
                        {
                            if let Some(count) = header_value.parse().ok() {
                                u.order_count_per_account.push((time_span, count))
                            }
                        };
                    }
                }
            }
        }
        u
    }
}
