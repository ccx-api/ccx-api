use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::TimeZone;
use chrono::Utc;
use derive_more::Deref;
use derive_more::From;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::de;
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, From, Deref,
)]
pub struct DtGate(#[serde(with = "self")] DateTime<Utc>);

impl DtGate {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn from_timestamp(timestamp: i64) -> Self {
        Self(Utc.timestamp_millis_opt(timestamp).single().unwrap())
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp_millis()
    }

    pub fn header_timestamp(&self) -> i64 {
        self.0.timestamp()
    }
}

impl From<NaiveDateTime> for DtGate {
    fn from(value: NaiveDateTime) -> Self {
        Self(DateTime::from_naive_utc_and_offset(value, Utc))
    }
}

pub fn serialize<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let ts = Decimal::from(date_time.timestamp_millis()) / dec!(1000);
    <Decimal as Serialize>::serialize(&ts, serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    const OUT_OF_BOUNDS: &str = "out of bounds";
    const VALID_VALUE: &str = "a valid seconds based timestamp with 3 digits precision";

    let val: Decimal = Deserialize::deserialize(deserializer)?;

    let mk_err = || de::Error::invalid_value(de::Unexpected::Other(OUT_OF_BOUNDS), &VALID_VALUE);

    let val = val.checked_mul(dec!(1000)).ok_or_else(mk_err)?;

    if !val.fract().is_zero() {
        Err(mk_err())?;
    }

    let millis = val.to_i64().ok_or_else(mk_err)?;

    Utc.timestamp_millis_opt(millis).single().ok_or_else(mk_err)
}

#[cfg(test)]
mod tests {
    use chrono::Timelike;

    use super::*;

    const TIMESTAMP_INT: &str = "1700073707";
    const TIMESTAMP_FRAC: &str = "1700073707.111";

    const TIMESTAMP_CASES: &str = r#"{
        "a": 1700073707,
        "b": 1700073707.111,
        "c": "1700073707",
        "d": "1700073707.111"
    }"#;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TimestampCases {
        a: DtGate,
        b: DtGate,
        c: DtGate,
        d: DtGate,
    }

    fn dt_sample_int() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 11, 15, 18, 41, 47).unwrap()
    }

    fn dt_sample_frac() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2023, 11, 15, 18, 41, 47)
            .unwrap()
            .with_nanosecond(111000000)
            .unwrap()
    }

    fn dt_samples_any() -> TimestampCases {
        TimestampCases {
            a: DtGate(dt_sample_int()),
            b: DtGate(dt_sample_frac()),
            c: DtGate(dt_sample_int()),
            d: DtGate(dt_sample_frac()),
        }
    }

    #[test]
    fn test_serialize_int() {
        let serialized = serde_plain::to_string(&DtGate(dt_sample_int())).unwrap();
        assert_eq!(serialized, TIMESTAMP_INT)
    }

    #[test]
    fn test_deserialize_int() {
        let deserialized: DtGate = serde_plain::from_str(TIMESTAMP_INT).unwrap();
        assert_eq!(deserialized.0, dt_sample_int())
    }

    #[test]
    fn test_serialize_frac() {
        let serialized = serde_plain::to_string(&DtGate(dt_sample_frac())).unwrap();
        assert_eq!(serialized, TIMESTAMP_FRAC)
    }

    #[test]
    fn test_deserialize_frac() {
        let deserialized: DtGate = serde_plain::from_str(TIMESTAMP_FRAC).unwrap();
        assert_eq!(deserialized.0, dt_sample_frac())
    }

    #[test]
    fn test_deserialize_any() {
        let deserialized: TimestampCases = serde_json::from_str(TIMESTAMP_CASES).unwrap();
        assert_eq!(deserialized, dt_samples_any())
    }
}
