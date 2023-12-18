pub enum ApiMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl ApiMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiMethod::Get => "GET",
            ApiMethod::Post => "POST",
            ApiMethod::Put => "PUT",
            ApiMethod::Delete => "DELETE",
        }
    }
}
