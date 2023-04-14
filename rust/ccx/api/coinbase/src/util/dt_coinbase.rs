use std::ops;

use chrono::DateTime;
use chrono::TimeZone;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;
use derive_more::From;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, From)]
pub struct DtCoinbase(#[serde(with = "self")] pub DateTime<Utc>);

impl DtCoinbase {
    #[inline]
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }
}

pub fn serialize<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date_time.format("%Y-%m-%dT%H:%M:%S%.fZ").to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_time_str = String::deserialize(deserializer)?;
    Utc.datetime_from_str(&date_time_str, "%Y-%m-%dT%H:%M:%S%.fZ")
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        let date_time = Utc.ymd(2020, 3, 11).and_hms_micro(20, 48, 46, 622052);
        let serialized = serde_plain::to_string(&DtCoinbase(date_time)).unwrap();
        assert_eq!(serialized, "2020-03-11T20:48:46.622052Z")
    }

    #[test]
    fn test_deserialize() {
        let deserialized: DtCoinbase = serde_plain::from_str("2020-03-11T20:48:46.622052Z").unwrap();
        let date_time = Utc.ymd(2020, 3, 11).and_hms_micro(20, 48, 46, 622052);
        assert_eq!(deserialized.0, date_time)
    }
}
