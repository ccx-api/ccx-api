use chrono::Utc;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Time {
    pub timestamp: i64,
}

impl Time {
    pub fn new(timestamp: i64) -> Self {
        Time { timestamp }
    }

    pub fn now() -> Self {
        Time::new(Utc::now().timestamp_millis())
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}
