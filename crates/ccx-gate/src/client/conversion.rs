use crate::proto::Request;

use super::meta::GateError;

pub struct RequestContent {
    pub query: Option<String>,
    pub body: Option<String>,
}

pub fn to_request_content<T: Request>(request: &T) -> Result<RequestContent, GateError> {
    Ok(match T::HTTP_METHOD {
        // NOTE: for now POST/PUT methods converts the whole request data to request body,
        // although in some cases it could be necessary to encode different parameters to
        // different parts of the request. https://www.gate.io/docs/developers/apiv4/#http-convention
        http::Method::POST | http::Method::PUT => RequestContent {
            query: None,
            // TODO: remove unwrap
            body: Some(serde_json::to_string(request)?),
        },
        _ => RequestContent {
            query: Some(serde_urlencoded::to_string(request)?),
            body: None,
        },
    })
}
