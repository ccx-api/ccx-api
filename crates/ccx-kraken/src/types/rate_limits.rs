#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RateLimitPrivateType {
    Normal,
    History,
}

pub enum RateLimitType {
    Public,
    Private(RateLimitPrivateType),
    Order,
}
