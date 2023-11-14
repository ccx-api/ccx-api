use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use derive_more::Deref;
use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, From, Deref)]
pub struct DtBitstamp(#[serde(with = "datetime")] DateTime<Utc>);

impl From<NaiveDateTime> for DtBitstamp {
    fn from(value: NaiveDateTime) -> Self {
        Self(DateTime::from_utc(value, Utc))
    }
}

mod datetime {
    use super::*;

    pub fn serialize<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&date_time.format("%Y-%m-%d %H:%M:%S%.f").to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let date_time_str = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&date_time_str, "%Y-%m-%d %H:%M:%S%.f")
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    const DATE_TIME_STR: &str = "2020-03-11 20:48:46.622052";

    fn date_time_sample() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2020, 3, 11, 20, 48, 46)
            .unwrap()
            .with_nanosecond(622052000)
            .unwrap()
    }

    #[test]
    fn test_serialize() {
        let serialized = serde_plain::to_string(&DtBitstamp(date_time_sample())).unwrap();
        assert_eq!(serialized, DATE_TIME_STR)
    }

    #[test]
    fn test_deserialize() {
        let deserialized: DtBitstamp = serde_plain::from_str(DATE_TIME_STR).unwrap();
        assert_eq!(deserialized.0, date_time_sample())
    }
}
