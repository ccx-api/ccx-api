#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RateLimitPrivateType {
    Normal,
    History,
    Order,
}

pub enum RateLimitType {
    Public,
    Private(RateLimitPrivateType),
}
