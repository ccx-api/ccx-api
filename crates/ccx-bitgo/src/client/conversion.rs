use crate::proto::Request;

use super::meta::BitGoError;

pub struct RequestContent {
    pub query: Option<String>,
    pub body: Option<String>,
}

pub fn to_request_content<T: Request>(request: &T) -> Result<RequestContent, BitGoError> {
    Ok(match T::HTTP_METHOD {
        // NOTE: for now POST/PUT methods converts the whole request data to request body,
        // although in some cases it could be necessary to encode different parameters to
        // different parts of the request.
        http::Method::POST | http::Method::PUT => RequestContent {
            query: None,
            body: Some(serde_json::to_string(request)?),
        },
        _ => RequestContent {
            query: Some(serde_html_form::to_string(request)?),
            body: None,
        },
    })
}
