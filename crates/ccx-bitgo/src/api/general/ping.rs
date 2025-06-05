use crate::types::derive::Request;
use crate::types::derive::Response;
use macro_rules_attribute::apply;

use crate::proto::{PublicRequest, Request, Response};
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
#[derive(Default)]
pub struct Ping {}

#[apply(Response)]
pub struct PingResponse {
    pub status: String,
    pub environment: String,
    pub config_env: String,
}

impl Response for PingResponse {}

impl Request for Ping {
    type Response = PingResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Public;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        "/api/v2/ping".into()
    }
}

impl PublicRequest for Ping {}
