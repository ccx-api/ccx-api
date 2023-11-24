use serde::de;
use serde::Serialize;

use crate::api::ApiMethod;
use crate::api::ApiVersion;

pub trait Request: Serialize {
    const METHOD: ApiMethod;
    const VERSION: ApiVersion;
    const PATH: &'static str;
    const IS_PUBLIC: bool;

    type Response: de::DeserializeOwned;
}
