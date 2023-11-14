use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use derive_more::Deref;
use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, From, Deref)]
pub struct DtCoinbase(#[serde(with = "self")] DateTime<Utc>);

impl From<NaiveDateTime> for DtCoinbase {
    fn from(value: NaiveDateTime) -> Self {
        Self(DateTime::from_utc(value, Utc))
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
    use chrono::Timelike;

    use super::*;

    const DATE_TIME_STR: &str = "2020-03-11 20:48:46.622052Z";

    fn date_time_sample() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2020, 3, 11, 20, 48, 46)
            .unwrap()
            .with_nanosecond(622052000)
            .unwrap()
    }

    #[test]
    fn test_serialize() {
        let serialized = serde_plain::to_string(&DtCoinbase(date_time_sample())).unwrap();
        assert_eq!(serialized, DATE_TIME_STR)
    }

    #[test]
    fn test_deserialize() {
        let deserialized: DtCoinbase = serde_plain::from_str(DATE_TIME_STR).unwrap();
        assert_eq!(deserialized.0, date_time_sample())
    }
}
