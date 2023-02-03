use crate::types::Nonce;
use crate::types::Time;

#[derive(Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Payload<T>
where
    T: std::fmt::Debug,
    T: serde::Serialize,
{
    #[serde(flatten)]
    pub content: T,
    pub nonce: Nonce,
    #[serde(rename = "timestamp")]
    pub time: Time,
}
