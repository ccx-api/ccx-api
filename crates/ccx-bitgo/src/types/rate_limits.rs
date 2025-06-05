#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum RateLimitType {
    Public,
    Authenticated,
}
