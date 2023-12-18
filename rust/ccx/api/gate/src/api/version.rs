pub enum ApiVersion {
    V4,
}

impl ApiVersion {
    pub fn as_str(&self) -> &'static str {
        match self {
            ApiVersion::V4 => "v4",
        }
    }
}
