use macro_rules_attribute::apply;
use uuid::Uuid;

use crate::proto::{Request, Response, SignedRequest};
use crate::types::derive::Request;
use crate::types::derive::Response;
use crate::types::rate_limits::RateLimitType;

#[apply(Request)]
pub struct CurrentUser {}

#[apply(Response)]
pub struct CurrentUserResponse {
    /// Unique identifier for the user
    pub id: Uuid,
    /// User's first name
    pub first_name: String,
    /// User's last name
    pub last_name: String,
    /// User's email address
    pub email: String,
}

impl Response for CurrentUserResponse {}

impl Request for CurrentUser {
    type Response = CurrentUserResponse;

    const HTTP_METHOD: http::Method = http::Method::GET;

    const COSTS: &'static RateLimitType = &RateLimitType::Authenticated;

    fn path(&self) -> std::borrow::Cow<'static, str> {
        "/api/prime/trading/v1/user/current".into()
    }
}

impl SignedRequest for CurrentUser {}
