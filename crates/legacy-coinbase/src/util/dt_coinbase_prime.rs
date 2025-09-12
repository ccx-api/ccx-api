use std::ops;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use derive_more::Deref;
use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, From, Deref)]
pub struct DtCoinbasePrime(#[serde(with = "self")] DateTime<Utc>);

impl DtCoinbasePrime {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn parse_from_str(s: &str) -> Result<Self, chrono::ParseError> {
        Ok(NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ")?
            .and_utc()
            .into())
    }
}

impl From<NaiveDateTime> for DtCoinbasePrime {
    fn from(value: NaiveDateTime) -> Self {
        Self(DateTime::from_naive_utc_and_offset(value, Utc))
    }
}

impl ops::Add<chrono::Duration> for DtCoinbasePrime {
    type Output = Self;

    fn add(self, rhs: chrono::Duration) -> Self::Output {
        Self(self.0 + rhs)
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
    NaiveDateTime::parse_from_str(&date_time_str, "%Y-%m-%dT%H:%M:%S%.fZ")
        .map_err(serde::de::Error::custom)
        .map(|ndt| ndt.and_utc())
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono::Timelike;

    use super::*;

    const DATE_TIME_STR: &str = "2020-03-11T20:48:46.622052Z";

    fn date_time_sample() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2020, 3, 11, 20, 48, 46)
            .unwrap()
            .with_nanosecond(622052000)
            .unwrap()
    }

    #[test]
    fn test_serialize() {
        let serialized = serde_plain::to_string(&DtCoinbasePrime(date_time_sample())).unwrap();
        assert_eq!(serialized, DATE_TIME_STR)
    }

    #[test]
    fn test_deserialize() {
        let deserialized: DtCoinbasePrime = serde_plain::from_str(DATE_TIME_STR).unwrap();
        assert_eq!(deserialized.0, date_time_sample())
    }
}
