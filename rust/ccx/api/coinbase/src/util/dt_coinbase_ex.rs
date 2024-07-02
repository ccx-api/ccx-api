use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use derive_more::Deref;
use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, From, Deref)]
pub struct DtCoinbaseEx(#[serde(with = "self")] DateTime<Utc>);

impl DtCoinbaseEx {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    // 2024-03-26 13:52:30.819928+00
    pub fn parse_from_str(s: &str) -> Result<Self, chrono::ParseError> {
        Ok(NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f+00")?
            .and_utc()
            .into())
    }
}

impl From<NaiveDateTime> for DtCoinbaseEx {
    fn from(value: NaiveDateTime) -> Self {
        Self(DateTime::from_naive_utc_and_offset(value, Utc))
    }
}

pub fn serialize<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date_time.format("%Y-%m-%d %H:%M:%S%.f+00").to_string())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_time_str = String::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&date_time_str, "%Y-%m-%d %H:%M:%S%.f+00")
        .map_err(serde::de::Error::custom)
        .map(|ndt| ndt.and_utc())
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use chrono::Timelike;

    use super::*;

    const DATE_TIME_STR: &str = "2024-03-26 13:52:30.819928+00";

    fn date_time_sample() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2024, 3, 26, 13, 52, 30)
            .unwrap()
            .with_nanosecond(819928000)
            .unwrap()
    }

    #[test]
    fn test_serialize() {
        let serialized = serde_plain::to_string(&DtCoinbaseEx(date_time_sample())).unwrap();
        assert_eq!(serialized, DATE_TIME_STR)
    }

    #[test]
    fn test_deserialize() {
        let deserialized: DtCoinbaseEx = serde_plain::from_str(DATE_TIME_STR).unwrap();
        assert_eq!(deserialized.0, date_time_sample())
    }
}
