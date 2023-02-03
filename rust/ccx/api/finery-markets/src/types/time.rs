use chrono::Utc;

use crate::types::Timestamp;

#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub struct Time(pub Timestamp);

impl Time {
    pub fn new(v: impl Into<Timestamp>) -> Self {
        Self(v.into())
    }

    pub fn now() -> Self {
        Self(Utc::now().timestamp_millis() as Timestamp)
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::now()
    }
}
