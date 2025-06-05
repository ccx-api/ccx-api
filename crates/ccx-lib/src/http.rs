pub fn is_json_response(resp: &reqwest::Response) -> bool {
    resp.headers()
        .get(http::header::CONTENT_TYPE)
        .map(|c| c.as_bytes().starts_with(b"application/json"))
        .unwrap_or_default()
}
