#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RateLimitType {
    Normal,
    History,
    Order,
}
