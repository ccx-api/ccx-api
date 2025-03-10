use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MexcTimestamp(DateTime<Utc>);

impl MexcTimestamp {
    pub fn new(timestamp: DateTime<Utc>) -> Self {
        Self(timestamp)
    }

    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_epoch_millis(epoch_millis: i64) -> Option<Self> {
        Some(Self(Utc.timestamp_millis_opt(epoch_millis).single()?))
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.0
    }

    pub fn to_epoch_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

impl From<DateTime<Utc>> for MexcTimestamp {
    fn from(timestamp: DateTime<Utc>) -> Self {
        Self::new(timestamp)
    }
}

impl From<MexcTimestamp> for DateTime<Utc> {
    fn from(timestamp: MexcTimestamp) -> Self {
        timestamp.0
    }
}

impl Serialize for MexcTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.timestamp_millis().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MexcTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<MexcTimestamp, D::Error>
    where
        D: Deserializer<'de>,
    {
        let epoch_millis = i64::deserialize(deserializer)?;
        Ok(MexcTimestamp::from_epoch_millis(epoch_millis)
            .ok_or_else(|| serde::de::Error::custom("invalid epoch millis"))?)
    }
}
