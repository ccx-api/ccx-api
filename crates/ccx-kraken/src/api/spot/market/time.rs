use serde::{Deserialize, Serialize};

use crate::proto::{PublicRequest, Request, Response};
use crate::types::rate_limits::RateLimitType;

/// Get Server Time.
///
/// Get the server's time.
#[derive(Serialize, Debug)]
pub struct ServerTime;

#[derive(Deserialize, Debug)]
pub struct ServerTimeResponse {
    pub unixtime: u64,
    pub rfc1123: String,
}

impl Response for ServerTimeResponse {}

impl Request for ServerTime {
    type Response = ServerTimeResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const ENDPOINT: &'static str = "/0/public/Time";

    const COSTS: &'static RateLimitType = &RateLimitType::Public;
}

impl PublicRequest for ServerTime {}
