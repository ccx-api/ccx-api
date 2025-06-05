use crate::types::derive::Request;
use crate::types::derive::Response;
use macro_rules_attribute::apply;

use crate::proto::{PublicRequest, Request, Response};
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
#[derive(Default)]
pub struct PingExpress {}

#[apply(Response)]
pub struct PingExpressResponse {
    pub status: String,
}

impl Response for PingExpressResponse {}

impl Request for PingExpress {
    type Response = PingExpressResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Public;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        "/api/v2/pingexpress".into()
    }
}

impl PublicRequest for PingExpress {}
