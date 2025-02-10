use http::StatusCode;
use uuid::Uuid;

use crate::spot::types::rate_limits::RateLimitInterval;
use crate::spot::types::rate_limits::RateLimitType;

#[derive(Debug)]
pub struct BinanceSpotResponseMeta<T> {
    pub meta: BinanceSpotMeta,
    pub payload: T,
}

#[derive(Debug)]
pub struct BinanceSpotMeta {
    pub http_status: StatusCode,
    pub id: Option<Uuid>,
    pub usage: Vec<(RateLimitType, RateLimitInterval, u32, u64)>,
}

impl<T> BinanceSpotResponseMeta<T> {
    pub fn new(meta: BinanceSpotMeta, payload: T) -> Self {
        BinanceSpotResponseMeta { meta, payload }
    }

    pub fn into_parts(self) -> (BinanceSpotMeta, T) {
        (self.meta, self.payload)
    }

    pub fn into_meta(self) -> BinanceSpotMeta {
        self.meta
    }

    pub fn into_payload(self) -> T {
        self.payload
    }
}

impl BinanceSpotMeta {
    pub(super) fn from_response(resp: &reqwest::Response) -> Self {
        let http_status = resp.status();
        println!("Response status: {http_status}");

        let id = resp
            .headers()
            .get("x-mbx-uuid")
            .and_then(|v| Uuid::parse_str(v.to_str().ok()?).ok());
        println!("Response id: {id:?}");

        // println!("Response headers:");
        let mut usage = Vec::new();
        for (k, v) in resp.headers() {
            let k = k.as_str();
            // println!("  {}: {:?}", k, v);
            if let Some(value) = v.to_str().ok() {
                for (typ, prefix) in [
                    (RateLimitType::RequestWeight, "x-mbx-used-weight-"),
                    (RateLimitType::Orders, "x-mbx-order-count-"),
                ] {
                    if let Some((interval, quantity, used)) = parse_usage(prefix, k, value) {
                        println!("    ::  {typ:?} {quantity} {interval:?} {used}");
                        usage.push((typ, interval, quantity, used));
                    }
                }
            }
        }
        BinanceSpotMeta {
            http_status,
            id,
            usage,
        }
    }
}

fn parse_usage(prefix: &str, name: &str, value: &str) -> Option<(RateLimitInterval, u32, u64)> {
    if !name.starts_with(prefix) {
        None?
    }
    // Safety: prefix is a valid UTF-8 string and name starts with prefix, so prefix.len() is at a
    //  valid UTF-8 boundary.
    let suffix = name.split_at(prefix.len()).1;
    if suffix.len() < 2 {
        None?
    }
    let interval = RateLimitInterval::from_letter(*suffix.as_bytes().last()?)?;
    let interval_quantity = suffix[..suffix.len() - 1].parse().ok()?;
    let used = value.parse().ok()?;
    Some((interval, interval_quantity, used))
}
