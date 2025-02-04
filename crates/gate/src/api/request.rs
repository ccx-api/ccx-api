use serde::de;
use serde::Serialize;

use crate::api::ApiMethod;
use crate::api::ApiVersion;

pub trait Request: Serialize {
    const METHOD: ApiMethod;
    const VERSION: ApiVersion;

    type Response: de::DeserializeOwned;
}

/// Request doesn't require signature
pub trait PublicRequest: Request {}

/// Request requires signature
pub trait PrivateRequest: Request {}
