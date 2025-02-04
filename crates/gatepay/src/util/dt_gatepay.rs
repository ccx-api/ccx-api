use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use derive_more::Deref;
use derive_more::From;
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Deref,
)]
pub struct DtGatepay(#[serde(with = "self")] DateTime<Utc>);

impl DtGatepay {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_timestamp_ms(timestamp: i64) -> Self {
        Self(Utc.timestamp_millis_opt(timestamp).single().unwrap())
    }

    pub fn timestamp_ms(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

impl From<NaiveDateTime> for DtGatepay {
    fn from(value: NaiveDateTime) -> Self {
        Self(DateTime::from_naive_utc_and_offset(value, Utc))
    }
}

pub fn serialize<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_i64(date_time.timestamp_millis())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;
    Utc.timestamp_millis_opt(timestamp)
        .single()
        .ok_or(serde::de::Error::invalid_value(
            serde::de::Unexpected::Signed(timestamp),
            &"a valid millis based timestamp",
        ))
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;

    use super::*;

    const TIMESTAMP: &str = "1700073707111";

    fn date_time_sample() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 11, 15, 18, 41, 47)
            .unwrap()
            .with_nanosecond(111000000)
            .unwrap()
    }

    #[test]
    fn test_serialize() {
        let serialized = serde_plain::to_string(&DtGatepay(date_time_sample())).unwrap();
        assert_eq!(serialized, TIMESTAMP)
    }

    #[test]
    fn test_deserialize() {
        let deserialized: DtGatepay = serde_plain::from_str(TIMESTAMP).unwrap();
        assert_eq!(deserialized.0, date_time_sample())
    }
}
